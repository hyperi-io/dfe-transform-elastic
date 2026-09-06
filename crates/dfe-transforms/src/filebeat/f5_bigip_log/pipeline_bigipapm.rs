// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipapm` pipeline.
pub struct PipelineBigipapm;

impl Transform for PipelineBigipapm {
    fn name(&self) -> &str {
        "pipeline_bigipapm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

        event.set("observer.product", json!("Application Performance Monitoring"))?;

        let _cond = { event.has_value("json.f5telemetry_timestamp") && event.get_str("json.f5telemetry_timestamp") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.f5telemetry_timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.telemetry.timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.f5telemetry_timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_f5telemetry_timestamp")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.telemetry.timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("@timestamp", v)?;
        }

        let _cond = { event.get_str("json.Bytes_In") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.Bytes_In") {
            if let Some(val) = event.get("json.Bytes_In") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.Bytes_In".into(),
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

        let _cond = { event.get_str("json.Bytes_Out") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.Bytes_Out") {
            if let Some(val) = event.get("json.Bytes_Out") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.Bytes_Out".into(),
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

            if event.has_value("json.User_Agent") {
                event.rename("json.User_Agent", "f5_bigip.log.user.agent")?;
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

        let _cond = { event.get_str("json.Client_IP") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.Client_IP") {
            if let Some(val) = event.get("json.Client_IP") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.Client_IP".into(),
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

        if let Some(v) = event.get("client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
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
            event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.Continent") {
                event.rename("json.Continent", "f5_bigip.log.continent")?;
            }

        if let Some(v) = event.get("f5_bigip.log.continent").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.geo.continent_name", v)?;
        }

            if event.has_value("json.Country") {
                event.rename("json.Country", "f5_bigip.log.country")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.country").cloned() {
            event.set("host.geo.country_name", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.Listener") {
                event.rename("json.Listener", "f5_bigip.log.listener")?;
            }

            if event.has_value("json.Reputation") {
                event.rename("json.Reputation", "f5_bigip.log.reputation")?;
            }

            if event.has_value("json.State") {
                event.rename("json.State", "f5_bigip.log.state")?;
            }

        let _cond = { event.get_str("json.Virtual_IP") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.Virtual_IP") {
            if let Some(val) = event.get("json.Virtual_IP") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.Virtual_IP".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.virtual.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_virtual_ip_to_ip")?;
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
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.virtual.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.Access_Policy_Result") {
                event.rename("json.Access_Policy_Result", "f5_bigip.log.access.policy_result")?;
            }

            if event.has_value("json.Access_Profile") {
                event.rename("json.Access_Profile", "f5_bigip.log.access.profile")?;
            }

            if event.has_value("json.errdefs_msgno") {
                event.rename("json.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
            }

            if event.has_value("json.Partition") {
                event.rename("json.Partition", "f5_bigip.log.partition")?;
            }

            if event.has_value("json.partition_name") {
                event.rename("json.partition_name", "f5_bigip.log.partition_name")?;
            }

            if event.has_value("json.session_id") {
                event.rename("json.session_id", "f5_bigip.log.session.id")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("f5_bigip.log.session.id", json!(event.get("json.session_id").map_or_else(String::new, template_to_string)))?;
            event.append_unique("f5_bigip.log.session.id", json!(event.get("json.Session_Id").map_or_else(String::new, template_to_string)))?;
            event.append_unique("f5_bigip.log.session.id", json!(event.get("json.Session_ID").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.telemetryEventCategory") {
                event.rename("json.telemetryEventCategory", "f5_bigip.log.telemetry.event.category")?;
            }

            if event.has_value("json.tenant") {
                event.rename("json.tenant", "f5_bigip.log.tenant")?;
            }

        let _cond = { event.get_str("json.Max_concurrent_Users") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.Max_concurrent_Users") {
            if let Some(val) = event.get("json.Max_concurrent_Users") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.Max_concurrent_Users".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.concurrent.users.max", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_max_concurrent_users_to_long")?;
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
            event.remove("f5_bigip.log.telemetry.timestamp");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.application.name");
            event.remove("f5_bigip.log.user.agent");
            event.remove("f5_bigip.log.client.ip");
            event.remove("f5_bigip.log.continent");
            event.remove("f5_bigip.log.country");
            Ok(())
        })();
        }

        let _cond = { event.has_value("error.message") };
        if _cond {
        event.set("event.kind", json!("pipeline_error"))?;
        }

        Ok(TransformResult::Continue)
    }
}
