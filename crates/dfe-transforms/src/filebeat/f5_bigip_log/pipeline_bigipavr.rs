// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipavr` pipeline.
pub struct PipelineBigipavr;

impl Transform for PipelineBigipavr {
    fn name(&self) -> &str {
        "pipeline_bigipavr"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

        event.set("observer.product", json!("Application Visibility and Reporting"))?;

        let _cond = { event.has_value("json.EOCTimestamp") && event.get_str("json.EOCTimestamp") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.EOCTimestamp") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.eoc.timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.EOCTimestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_eoc_timestamp")?;
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
        if let Some(v) = event.get("f5_bigip.log.eoc.timestamp").cloned() {
            event.set("@timestamp", v)?;
        }
            Ok(())
        })();

        let _cond = { event.get_str("json.ClientIP") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientIP") {
            if let Some(val) = event.get("json.ClientIP") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientIP".into(),
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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.client.ip").cloned() {
            event.set("client.ip", v)?;
        }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.POOLIP") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.POOLIP") {
            if let Some(val) = event.get("json.POOLIP") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.POOLIP".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.pool.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_pool_ip_to_ip")?;
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
        if let Some(v) = event.get("f5_bigip.log.pool.ip").cloned() {
            event.set("destination.ip", v)?;
        }
            Ok(())
        })();

        if let Some(v) = event.get("destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.ip", v)?;
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
            event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.POOLPort") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.POOLPort") {
            if let Some(val) = event.get("json.POOLPort") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.POOLPort".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.pool.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_pool_port_to_long")?;
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
        if let Some(v) = event.get("f5_bigip.log.pool.port").cloned() {
            event.set("destination.port", v)?;
        }
            Ok(())
        })();

        if let Some(v) = event.get("destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.port", v)?;
        }

            if event.has_value("json.QueryName") {
                event.rename("json.QueryName", "f5_bigip.log.query.name")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.query.name").cloned() {
            event.set("dns.question.name", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.QueryType") {
                event.rename("json.QueryType", "f5_bigip.log.query.type")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.query.type").cloned() {
            event.set("dns.question.type", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.Action") {
                event.rename("json.Action", "f5_bigip.log.action")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.action").cloned() {
            event.set("event.action", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.CountryCode") {
                event.rename("json.CountryCode", "f5_bigip.log.country_code")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.country_code").cloned() {
            event.set("host.geo.country_iso_code", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.GeoCountry") {
                event.rename("json.GeoCountry", "f5_bigip.log.geo.country")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.geo.country").cloned() {
            event.set("host.geo.country_name", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.DeviceId") {
                event.rename("json.DeviceId", "f5_bigip.log.device.id")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.device.id").cloned() {
            event.set("host.id", v)?;
        }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("host.id").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.OsName") {
                event.rename("json.OsName", "f5_bigip.log.osname")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.osname").cloned() {
            event.set("host.os.name", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.Method") {
                event.rename("json.Method", "f5_bigip.log.method")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.method").cloned() {
            event.set("http.request.method", v)?;
        }
            Ok(())
        })();

        let _cond = { event.get_str("json.ResponseCode") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ResponseCode") {
            if let Some(val) = event.get("json.ResponseCode") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ResponseCode".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.response.code", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_response_code_to_long")?;
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
        if let Some(v) = event.get("f5_bigip.log.response.code").cloned() {
            event.set("http.response.status_code", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.Severity") {
                event.rename("json.Severity", "f5_bigip.log.severity.name")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.severity.name").cloned() {
            event.set("log.level", v)?;
        }
            Ok(())
        })();

        if event.has_value("log.level") {
            map_strings(event, "log.level", "log.level", str::to_lowercase)?;
        }

            if event.has_value("json.NetworkProtocol") {
                event.rename("json.NetworkProtocol", "f5_bigip.log.network.protocol")?;
            }

        if let Some(v) = event.get("f5_bigip.log.network.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.protocol", v)?;
        }

        if event.has_value("network.protocol") {
            map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
        }

        let _cond = { event.get_str("json.SourceIP") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SourceIP") {
            if let Some(val) = event.get("json.SourceIP") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SourceIP".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.source.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            if event.has_value("json.URL") {
                event.rename("json.URL", "f5_bigip.log.url")?;
            }

        let _cond = { event.has_value("f5_bigip.log.url") };
        if _cond {
            uri_parts(event, "f5_bigip.log.url", "url", true, false)?;
        }

            if event.has_value("json.UserAgent") {
                event.rename("json.UserAgent", "f5_bigip.log.user.agent")?;
            }

        if event.has_value("f5_bigip.log.user.agent") {
            gsub_field(event, "f5_bigip.log.user.agent", "f5_bigip.log.user.agent", cached_regex!("(\\([^)]*)\\+(https?://)"), "$1%2b$2")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.user.agent") {
            if let Some(s) = event.get_string("f5_bigip.log.user.agent") {
                match url_decode(&s) {
                    Some(decoded) => event.set("f5_bigip.log.user.agent", json!(decoded))?,
                    None => return Err(TransformError::ParseError {
                        path: "f5_bigip.log.user.agent".into(),
                        message: format!("cannot url-decode '{s}'"),
                    }),
                }
            }
        }
            Ok(())
        })();

            if let Some(ua_str) = event.get_string("f5_bigip.log.user.agent") {
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

            if event.has_value("json.UserName") {
                event.rename("json.UserName", "f5_bigip.log.user.name")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.user.name").cloned() {
            event.set("user.name", v)?;
        }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.AggrInterval") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AggrInterval") {
            if let Some(val) = event.get("json.AggrInterval") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AggrInterval".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.aggr_interval", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_aggr_interval_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.ApplicationName") {
                event.rename("json.ApplicationName", "f5_bigip.log.application_name")?;
            }

        let _cond = { event.get_str("json.ApplicationResponseTime") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ApplicationResponseTime") {
            if let Some(val) = event.get("json.ApplicationResponseTime") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ApplicationResponseTime".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.application.response.time", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_application_response_time_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.AVRProfileName") {
                event.rename("json.AVRProfileName", "f5_bigip.log.profile.name")?;
            }

            if event.has_value("json.BrowserName") {
                event.rename("json.BrowserName", "f5_bigip.log.browser_name")?;
            }

            if event.has_value("json.ClientIPRouteDomain") {
                event.rename("json.ClientIPRouteDomain", "f5_bigip.log.client.ip_route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.client.ip_route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.ClientLatencyHitCount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientLatencyHitCount") {
            if let Some(val) = event.get("json.ClientLatencyHitCount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientLatencyHitCount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client.latency.hit_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_latency_hit_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ClientLatencyMax") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientLatencyMax") {
            if let Some(val) = event.get("json.ClientLatencyMax") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientLatencyMax".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client.latency.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_latency_max_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ClientLatencyTotal") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientLatencyTotal") {
            if let Some(val) = event.get("json.ClientLatencyTotal") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientLatencyTotal".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client.latency.total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_latency_total_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ClientSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientSideNetworkLatency") {
            if let Some(val) = event.get("json.ClientSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_side.network.latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ClientTtfb") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientTtfb") {
            if let Some(val) = event.get("json.ClientTtfb") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientTtfb".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_ttfb.value", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_ttfb_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ClientTtfbHitcount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ClientTtfbHitcount") {
            if let Some(val) = event.get("json.ClientTtfbHitcount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ClientTtfbHitcount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_ttfb.hit_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_ttfb_hit_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.Entity") {
                event.rename("json.Entity", "f5_bigip.log.entity")?;
            }

            if event.has_value("json.errdefs_msgno") {
                event.rename("json.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
            }

            if event.has_value("json.GeoCode") {
                event.rename("json.GeoCode", "f5_bigip.log.geo.code")?;
            }

        let _cond = { event.get_str("json.HitCount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.HitCount") {
            if let Some(val) = event.get("json.HitCount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.HitCount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.hit_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_hit_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.LatencyHistogram") {
                event.rename("json.LatencyHistogram", "f5_bigip.log.latency_histogram")?;
            }

        let _cond = { event.get_str("json.MaxApplicationResponseTime") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxApplicationResponseTime") {
            if let Some(val) = event.get("json.MaxApplicationResponseTime") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxApplicationResponseTime".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.application.response.max_time", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_application_response_time_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxClientSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxClientSideNetworkLatency") {
            if let Some(val) = event.get("json.MaxClientSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxClientSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_side.network.max_latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_client_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxClientTtfb") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxClientTtfb") {
            if let Some(val) = event.get("json.MaxClientTtfb") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxClientTtfb".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_ttfb.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_client_ttfb_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxRequestDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxRequestDuration") {
            if let Some(val) = event.get("json.MaxRequestDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxRequestDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.request.max_duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_request_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxResponseDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxResponseDuration") {
            if let Some(val) = event.get("json.MaxResponseDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxResponseDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.response.max_duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_response_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxServerSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxServerSideNetworkLatency") {
            if let Some(val) = event.get("json.MaxServerSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxServerSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server_side.network.max_latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_server_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinApplicationResponseTime") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinApplicationResponseTime") {
            if let Some(val) = event.get("json.MinApplicationResponseTime") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinApplicationResponseTime".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.application.response.min_time", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_application_response_time_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinClientSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinClientSideNetworkLatency") {
            if let Some(val) = event.get("json.MinClientSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinClientSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_side.network.min_latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_client_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinClientTtfb") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinClientTtfb") {
            if let Some(val) = event.get("json.MinClientTtfb") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinClientTtfb".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client_ttfb.min", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_client_ttfb_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinRequestDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinRequestDuration") {
            if let Some(val) = event.get("json.MinRequestDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinRequestDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.request.min_duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_request_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinResponseDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinResponseDuration") {
            if let Some(val) = event.get("json.MinResponseDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinResponseDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.response.min_duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_response_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinServerSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinServerSideNetworkLatency") {
            if let Some(val) = event.get("json.MinServerSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinServerSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server_side.network.min_latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_server_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.Module") {
                event.rename("json.Module", "f5_bigip.log.module")?;
            }

            if event.has_value("json.POOLIPRouteDomain") {
                event.rename("json.POOLIPRouteDomain", "f5_bigip.log.pool.ip_route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.pool.ip_route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.RequestDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.RequestDuration") {
            if let Some(val) = event.get("json.RequestDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.RequestDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.request.duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_request_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.RequestDurationHitcount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.RequestDurationHitcount") {
            if let Some(val) = event.get("json.RequestDurationHitcount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.RequestDurationHitcount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.request.duration_hit_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_request_duration_hitcount_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ResponseDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ResponseDuration") {
            if let Some(val) = event.get("json.ResponseDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ResponseDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.response.duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_response_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ResponseDurationHitcount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ResponseDurationHitcount") {
            if let Some(val) = event.get("json.ResponseDurationHitcount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ResponseDurationHitcount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.response.duration_hit_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_response_duration_hitcount_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ServerHitcount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ServerHitcount") {
            if let Some(val) = event.get("json.ServerHitcount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ServerHitcount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server.hit_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_hitcount_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ServerLatencyMax") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ServerLatencyMax") {
            if let Some(val) = event.get("json.ServerLatencyMax") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ServerLatencyMax".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server.latency.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_latency_max_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ServerLatencyMin") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ServerLatencyMin") {
            if let Some(val) = event.get("json.ServerLatencyMin") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ServerLatencyMin".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server.latency.min", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_latency_min_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ServerLatencyTotal") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ServerLatencyTotal") {
            if let Some(val) = event.get("json.ServerLatencyTotal") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ServerLatencyTotal".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server.latency.total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_latency_total_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ServerSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ServerSideNetworkLatency") {
            if let Some(val) = event.get("json.ServerSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ServerSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server_side.network.latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.SlotId") {
                event.rename("json.SlotId", "f5_bigip.log.slot.id")?;
            }

        let _cond = { event.get_str("json.SosApplicationResponseTime") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SosApplicationResponseTime") {
            if let Some(val) = event.get("json.SosApplicationResponseTime") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SosApplicationResponseTime".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sos.application_response_time", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sos_application_response_time_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SosClientSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SosClientSideNetworkLatency") {
            if let Some(val) = event.get("json.SosClientSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SosClientSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sos.client_side_network_latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sos_client_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SosClientTtfb") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SosClientTtfb") {
            if let Some(val) = event.get("json.SosClientTtfb") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SosClientTtfb".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sos.client_ttfb", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sos_client_ttfb_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SosRequestDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SosRequestDuration") {
            if let Some(val) = event.get("json.SosRequestDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SosRequestDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sos.request_duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sos_request_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SosResponseDuration") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SosResponseDuration") {
            if let Some(val) = event.get("json.SosResponseDuration") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SosResponseDuration".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sos.response_duration", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sos_response_duration_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SosServerSideNetworkLatency") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SosServerSideNetworkLatency") {
            if let Some(val) = event.get("json.SosServerSideNetworkLatency") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SosServerSideNetworkLatency".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sos.server_side_network_latency", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sos_server_side_network_latency_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SubnetIP") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SubnetIP") {
            if let Some(val) = event.get("json.SubnetIP") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SubnetIP".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.subnet.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_subnet_ip_to_ip")?;
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
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.subnet.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.SubnetName") {
                event.rename("json.SubnetName", "f5_bigip.log.subnet.name")?;
            }

            if event.has_value("json.SubnetRouteDomain") {
                event.rename("json.SubnetRouteDomain", "f5_bigip.log.subnet.route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.subnet.route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.telemetryEventCategory") {
                event.rename("json.telemetryEventCategory", "f5_bigip.log.telemetry.event.category")?;
            }

        let _cond = { event.get_str("json.ThroughputReqMaxPerSec") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ThroughputReqMaxPerSec") {
            if let Some(val) = event.get("json.ThroughputReqMaxPerSec") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ThroughputReqMaxPerSec".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.throughput.req_per_sec.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_throughput_req_max_per_sec_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ThroughputReqTotalPerInterval") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ThroughputReqTotalPerInterval") {
            if let Some(val) = event.get("json.ThroughputReqTotalPerInterval") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ThroughputReqTotalPerInterval".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.throughput.req_per_interval.total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_throughput_req_total_per_interval_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ThroughputRespMaxPerSec") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ThroughputRespMaxPerSec") {
            if let Some(val) = event.get("json.ThroughputRespMaxPerSec") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ThroughputRespMaxPerSec".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.throughput.resp_per_sec.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_throughput_resp_max_per_sec_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ThroughputRespTotalPerInterval") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ThroughputRespTotalPerInterval") {
            if let Some(val) = event.get("json.ThroughputRespTotalPerInterval") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ThroughputRespTotalPerInterval".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.throughput.resp_per_interval.total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_throughput_resp_total_per_interval_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.TPSMax") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.TPSMax") {
            if let Some(val) = event.get("json.TPSMax") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.TPSMax".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tps.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tps_max_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.UserSessionsNewTotal") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.UserSessionsNewTotal") {
            if let Some(val) = event.get("json.UserSessionsNewTotal") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.UserSessionsNewTotal".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.user.sessions.new_total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_user_sessions_new_total_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.VSName") {
                event.rename("json.VSName", "f5_bigip.log.vs_name")?;
            }

            if event.has_value("json.ObjectTagsList") {
                event.rename("json.ObjectTagsList", "f5_bigip.log.object_tags_list")?;
            }

            if event.has_value("json.DosProfileName") {
                event.rename("json.DosProfileName", "f5_bigip.log.dos.profile_name")?;
            }

            if event.has_value("json.AttackId") {
                event.rename("json.AttackId", "f5_bigip.log.attack.id")?;
            }

            if event.has_value("json.SourceIpRouteDomain") {
                event.rename("json.SourceIpRouteDomain", "f5_bigip.log.source.ip_route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.source.ip_route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.TransactionOutcome") {
                event.rename("json.TransactionOutcome", "f5_bigip.log.transaction_outcome")?;
            }

            if event.has_value("json.AttackVectorName") {
                event.rename("json.AttackVectorName", "f5_bigip.log.attack.vector_name")?;
            }

            if event.has_value("json.AttackTriggerName") {
                event.rename("json.AttackTriggerName", "f5_bigip.log.attack.trigger_name")?;
            }

            if event.has_value("json.AttackMitigationName") {
                event.rename("json.AttackMitigationName", "f5_bigip.log.attack.mitigation_name")?;
            }

        let _cond = { event.get_str("json.IsInternalActivity") == Some("0") };
        if _cond {
        let v = json!(false);
        if !painless_is_empty_value(&v) {
                event.set("f5_bigip.log.is_internal_activity", v)?;
        }
        }

        let _cond = { event.get_str("json.IsInternalActivity") == Some("1") };
        if _cond {
        let v = json!(true);
        if !painless_is_empty_value(&v) {
                event.set("f5_bigip.log.is_internal_activity", v)?;
        }
        }

        let _cond = { event.get_str("json.IsAttackingIp") == Some("0") };
        if _cond {
        let v = json!(false);
        if !painless_is_empty_value(&v) {
                event.set("f5_bigip.log.is_attacking_ip", v)?;
        }
        }

        let _cond = { event.get_str("json.IsAttackingIp") == Some("1") };
        if _cond {
        let v = json!(true);
        if !painless_is_empty_value(&v) {
                event.set("f5_bigip.log.is_attacking_ip", v)?;
        }
        }

            if event.has_value("json.globalBigiqConf") {
                event.rename("json.globalBigiqConf", "f5_bigip.log.global_bigiq_conf")?;
            }

            if event.has_value("json.Policy") {
                event.rename("json.Policy", "f5_bigip.log.policy.name")?;
            }

        let _cond = { event.get_str("json.ViolationRating") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ViolationRating") {
            if let Some(val) = event.get("json.ViolationRating") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ViolationRating".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.violation.rating", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_violation_rating_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.IPReputation") {
                event.rename("json.IPReputation", "f5_bigip.log.ip_reputation")?;
            }

            if event.has_value("json.SessionID") {
                event.rename("json.SessionID", "f5_bigip.log.session.id")?;
            }

        let _cond = { event.get_str("json.IsMobileDevice") == Some("0") };
        if _cond {
        event.set("f5_bigip.log.is_mobile_device", json!(false))?;
        }

        let _cond = { event.get_str("json.IsMobileDevice") == Some("1") };
        if _cond {
        event.set("f5_bigip.log.is_mobile_device", json!(true))?;
        }

            if event.has_value("json.DosMobileAppClientType") {
                event.rename("json.DosMobileAppClientType", "f5_bigip.log.dos.mobile_app.client_type")?;
            }

            if event.has_value("json.DosMobileAppVersion") {
                event.rename("json.DosMobileAppVersion", "f5_bigip.log.dos.mobile_app.version")?;
            }

            if event.has_value("json.DosMobileAppDisplayName") {
                event.rename("json.DosMobileAppDisplayName", "f5_bigip.log.dos.mobile_app.display_name")?;
            }

            if event.has_value("json.STAT_SRC") {
                event.rename("json.STAT_SRC", "f5_bigip.log.stat_src")?;
            }

            if event.has_value("json.AttackType") {
                event.rename("json.AttackType", "f5_bigip.log.attack.type")?;
            }

        let _cond = { event.get_str("json.AttackCount") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AttackCount") {
            if let Some(val) = event.get("json.AttackCount") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AttackCount".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.attack.count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_attack_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.TotalEvents") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.TotalEvents") {
            if let Some(val) = event.get("json.TotalEvents") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.TotalEvents".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.events.total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_total_events_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.SoftwareDrops") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.SoftwareDrops") {
            if let Some(val) = event.get("json.SoftwareDrops") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.SoftwareDrops".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.software_drops", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_software_drops_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.HardwareDrops") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.HardwareDrops") {
            if let Some(val) = event.get("json.HardwareDrops") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.HardwareDrops".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.hardware_drops", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_hardware_drops_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.BadActorEvents") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.BadActorEvents") {
            if let Some(val) = event.get("json.BadActorEvents") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.BadActorEvents".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bad_actor.events", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bad_actor_events_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.BadActorDrops") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.BadActorDrops") {
            if let Some(val) = event.get("json.BadActorDrops") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.BadActorDrops".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bad_actor.drops", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bad_actor_drops_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.WLEvents") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.WLEvents") {
            if let Some(val) = event.get("json.WLEvents") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.WLEvents".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.wl_events", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_wl_events_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgDetection") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgDetection") {
            if let Some(val) = event.get("json.AvgDetection") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgDetection".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.detection.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_detection_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinMitigation") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinMitigation") {
            if let Some(val) = event.get("json.MinMitigation") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinMitigation".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.mitigation.min", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_mitigation_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxMitigation") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxMitigation") {
            if let Some(val) = event.get("json.MaxMitigation") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxMitigation".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.mitigation.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_mitigation_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgBadActorDetection") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgBadActorDetection") {
            if let Some(val) = event.get("json.AvgBadActorDetection") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgBadActorDetection".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bad_actor.detection.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_bad_actor_detection_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MinBadActorMitigation") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MinBadActorMitigation") {
            if let Some(val) = event.get("json.MinBadActorMitigation") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MinBadActorMitigation".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bad_actor.mitigation.min", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_min_bad_actor_mitigation_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxBadActorMitigation") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxBadActorMitigation") {
            if let Some(val) = event.get("json.MaxBadActorMitigation") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxBadActorMitigation".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bad_actor.mitigation.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_bad_actor_mitigation_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.abandoned_conns") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.abandoned_conns") {
            if let Some(val) = event.get("json.abandoned_conns") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.abandoned_conns".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.abandoned_conns", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_abandoned_conns_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.expired_conns") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.expired_conns") {
            if let Some(val) = event.get("json.expired_conns") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.expired_conns".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.expired_conns", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_expired_conns_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.failed_conns") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.failed_conns") {
            if let Some(val) = event.get("json.failed_conns") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.failed_conns".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.failed_conns", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_failed_conns_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.accept_fails") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.accept_fails") {
            if let Some(val) = event.get("json.accept_fails") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.accept_fails".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.accept_fails", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_accept_fails_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.accepts") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.accepts") {
            if let Some(val) = event.get("json.accepts") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.accepts".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.accepts", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_accepts_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.active_conns") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.active_conns") {
            if let Some(val) = event.get("json.active_conns") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.active_conns".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.active_conns", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_active_conns_to_double")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.hw_cookie_valid") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.hw_cookie_valid") {
            if let Some(val) = event.get("json.hw_cookie_valid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.hw_cookie_valid".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.hw.cookie_valid", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_hw_cookie_valid_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.max_active_conns") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.max_active_conns") {
            if let Some(val) = event.get("json.max_active_conns") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.max_active_conns".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.max_active_conns", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_active_conns_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.new_conns") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.new_conns") {
            if let Some(val) = event.get("json.new_conns") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.new_conns".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.new_conns", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_new_conns_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.rxbad_cookie") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.rxbad_cookie") {
            if let Some(val) = event.get("json.rxbad_cookie") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.rxbad_cookie".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.rxbad_cookie", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_rxbad_cookie_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.rxbadseg") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.rxbadseg") {
            if let Some(val) = event.get("json.rxbadseg") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.rxbadseg".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.rxbadseg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_rxbadseg_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.rxbadsum") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.rxbadsum") {
            if let Some(val) = event.get("json.rxbadsum") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.rxbadsum".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.rxbadsum", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_rxbadsum_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.rxcookie") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.rxcookie") {
            if let Some(val) = event.get("json.rxcookie") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.rxcookie".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.rxcookie", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_rxcookie_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.rxooseg") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.rxooseg") {
            if let Some(val) = event.get("json.rxooseg") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.rxooseg".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.rxooseg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_rxooseg_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.rxrst") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.rxrst") {
            if let Some(val) = event.get("json.rxrst") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.rxrst".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.rxrst", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_rxrst_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.sndpack") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.sndpack") {
            if let Some(val) = event.get("json.sndpack") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.sndpack".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.sndpack", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_sndpack_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.syncacheover") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.syncacheover") {
            if let Some(val) = event.get("json.syncacheover") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.syncacheover".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.syncacheover", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_syncacheover_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.tcp_prof") {
                event.rename("json.tcp_prof", "f5_bigip.log.tcp_prof")?;
            }

        let _cond = { event.get_str("json.txrexmits") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.txrexmits") {
            if let Some(val) = event.get("json.txrexmits") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.txrexmits".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.txrexmits", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_txrexmits_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.vip") {
                event.rename("json.vip", "f5_bigip.log.vip")?;
            }

            if event.has_value("json.tenant") {
                event.rename("json.tenant", "f5_bigip.log.tenant")?;
            }

        let _cond = { event.get_str("json.AvgConcurrentConnections") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgConcurrentConnections") {
            if let Some(val) = event.get("json.AvgConcurrentConnections") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgConcurrentConnections".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.concurrent.connections.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_concurrent_connections_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgCpu") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgCpu") {
            if let Some(val) = event.get("json.AvgCpu") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgCpu".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.cpu.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_cpu_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgCpuAnalysisPlane") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgCpuAnalysisPlane") {
            if let Some(val) = event.get("json.AvgCpuAnalysisPlane") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgCpuAnalysisPlane".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.cpu.analysis_plane.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_cpu_analysis_plane_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgCpuDataPlane") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgCpuDataPlane") {
            if let Some(val) = event.get("json.AvgCpuDataPlane") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgCpuDataPlane".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.cpu.data_plane.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_cpu_data_plane_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgMemory") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgMemory") {
            if let Some(val) = event.get("json.AvgMemory") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgMemory".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.memory.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_memory_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgThroughput") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgThroughput") {
            if let Some(val) = event.get("json.AvgThroughput") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgThroughput".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.throughput.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_throughput_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ConcurrentConnectionsHealth") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ConcurrentConnectionsHealth") {
            if let Some(val) = event.get("json.ConcurrentConnectionsHealth") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ConcurrentConnectionsHealth".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.concurrent.connections.health", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_concurrent_connections_health_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.CpuHealth") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.CpuHealth") {
            if let Some(val) = event.get("json.CpuHealth") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.CpuHealth".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.cpu.health", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_cpu_health_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MaxConcurrentConnections") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MaxConcurrentConnections") {
            if let Some(val) = event.get("json.MaxConcurrentConnections") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MaxConcurrentConnections".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.concurrent.connections.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_concurrent_connections_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.MemoryHealth") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.MemoryHealth") {
            if let Some(val) = event.get("json.MemoryHealth") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.MemoryHealth".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.memory.health", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_memory_health_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.ThroughputHealth") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ThroughputHealth") {
            if let Some(val) = event.get("json.ThroughputHealth") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ThroughputHealth".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.throughput.health", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_throughput_health_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.TotalBytes") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.TotalBytes") {
            if let Some(val) = event.get("json.TotalBytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.TotalBytes".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bytes.total", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.AvgCpuControlPlane") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.AvgCpuControlPlane") {
            if let Some(val) = event.get("json.AvgCpuControlPlane") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.AvgCpuControlPlane".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.cpu.control_plane.avg", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_avg_cpu_control_plane_to_long")?;
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
            event.remove("json");
            Ok(())
        })();

        let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.remove("f5_bigip.log.eoc.timestamp");
            event.remove("f5_bigip.log.client.ip");
            event.remove("f5_bigip.log.pool.ip");
            event.remove("f5_bigip.log.pool.port");
            event.remove("f5_bigip.log.query.name");
            event.remove("f5_bigip.log.query.type");
            event.remove("f5_bigip.log.action");
            event.remove("f5_bigip.log.country_code");
            event.remove("f5_bigip.log.geo.country");
            event.remove("f5_bigip.log.device.id");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.osname");
            event.remove("f5_bigip.log.method");
            event.remove("f5_bigip.log.response.code");
            event.remove("f5_bigip.log.severity.name");
            event.remove("f5_bigip.log.network.protocol");
            event.remove("f5_bigip.log.source.ip");
            event.remove("f5_bigip.log.url");
            event.remove("f5_bigip.log.user.agent");
            event.remove("f5_bigip.log.user.name");
            event.remove("f5_bigip.log.application.name");
            Ok(())
        })();
        }

        Ok(TransformResult::Continue)
    }
}
