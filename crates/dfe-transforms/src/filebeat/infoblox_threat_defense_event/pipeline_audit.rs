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
                if event.has_value("cef.extensions.applicationProtocol") {
                    event.rename("cef.extensions.applicationProtocol", "infoblox_threat_defense.event.application_protocol")?;
                }

            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
            }

                if event.has_value("cef.extensions.deviceAction") {
                    event.rename("cef.extensions.deviceAction", "infoblox_threat_defense.event.device.action")?;
                }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("event.action", Value::Array(parts))?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "split")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("event.action") && event.get_str("event.action") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                if let Some(joined) = joined {
                    event.set("event.action", json!(joined))?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "join")?;
                event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename("cef.extensions.deviceEventCategory", "infoblox_threat_defense.event.device.event_category")?;
                }

                if event.has_value("cef.extensions.eventOutcome") {
                    event.rename("cef.extensions.eventOutcome", "infoblox_threat_defense.event.outcome")?;
                }

            let _cond = { event.get_str("infoblox_threat_defense.event.outcome").is_some_and(|s| s.to_lowercase().contains("success")) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("infoblox_threat_defense.event.outcome").is_some_and(|s| s.to_lowercase().contains("fail")) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

                if event.has_value("cef.extensions.InfobloxEventVersion") {
                    event.rename("cef.extensions.InfobloxEventVersion", "infoblox_threat_defense.event.infoblox.event.version")?;
                }

                if event.has_value("cef.extensions.InfobloxHTTPReqBody") {
                    event.rename("cef.extensions.InfobloxHTTPReqBody", "infoblox_threat_defense.event.infoblox.http.req_body")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.http.req_body").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.body.content", v)?;
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.http.req_body") && event.get_str("infoblox_threat_defense.event.infoblox.http.req_body") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "infoblox_threat_defense.event.infoblox.http.req_body", "infoblox_threat_defense.event.infoblox.http.req_body")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_http_req_body")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("cef.extensions.InfobloxHTTPRespBody") {
                    event.rename("cef.extensions.InfobloxHTTPRespBody", "infoblox_threat_defense.event.infoblox.http.resp_body")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.http.resp_body").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.body.content", v)?;
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.http.resp_body") && event.get_str("infoblox_threat_defense.event.infoblox.http.resp_body") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "infoblox_threat_defense.event.infoblox.http.resp_body", "infoblox_threat_defense.event.infoblox.http.resp_body")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_http_resp_body")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("cef.extensions.InfobloxResourceDesc") {
                    event.rename("cef.extensions.InfobloxResourceDesc", "infoblox_threat_defense.event.infoblox.resource.desc")?;
                }

                if event.has_value("cef.extensions.InfobloxResourceId") {
                    event.rename("cef.extensions.InfobloxResourceId", "infoblox_threat_defense.event.infoblox.resource.id")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.resource.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

                if event.has_value("cef.extensions.InfobloxResourceType") {
                    event.rename("cef.extensions.InfobloxResourceType", "infoblox_threat_defense.event.infoblox.resource.type")?;
                }

                if event.has_value("cef.extensions.InfobloxSubjectGroups") {
                    event.rename("cef.extensions.InfobloxSubjectGroups", "infoblox_threat_defense.event.infoblox.subject.groups")?;
                }

                if event.has_value("cef.extensions.InfobloxSubjectType") {
                    event.rename("cef.extensions.InfobloxSubjectType", "infoblox_threat_defense.event.infoblox.subject.type")?;
                }

                if event.has_value("cef.extensions.message") {
                    event.rename("cef.extensions.message", "infoblox_threat_defense.event.message")?;
                }

            let _cond = { event.get_str("cef.extensions.sourceAddress") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.sourceAddress") {
                if let Some(val) = event.get("cef.extensions.sourceAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.sourceAddress".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.source.address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_sourceAddress_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
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

            let _cond = { event.has_value("infoblox_threat_defense.event.source.address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("infoblox_threat_defense.event.source.address").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("cef.extensions.sourceUserName") {
                    event.rename("cef.extensions.sourceUserName", "infoblox_threat_defense.event.source.user_name")?;
                }

            let _cond = { event.has_value("infoblox_threat_defense.event.source.user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("infoblox_threat_defense.event.source.user_name").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
