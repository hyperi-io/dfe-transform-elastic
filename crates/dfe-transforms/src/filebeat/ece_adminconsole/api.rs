// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `api` pipeline.
pub struct Api;

impl Transform for Api {
    fn name(&self) -> &str {
        "api"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("tmp.ece.log.process.thread.name") {
                    event.rename("tmp.ece.log.process.thread.name", "process.thread.name")?;
                }

                if event.has_value("tmp.ece.log.request_payload") {
                    event.rename("tmp.ece.log.request_payload", "http.request.body.content")?;
                }

                if event.has_value("tmp.ece.log.message") {
                    event.rename("tmp.ece.log.message", "message")?;
                }

                if event.has_value("tmp.ece.log.log.logger") {
                    event.rename("tmp.ece.log.log.logger", "log.logger")?;
                }

                if event.has_value("tmp.ece.log.log.level") {
                    event.rename("tmp.ece.log.log.level", "log.level")?;
                }

                if event.has_value("tmp.ece.log.trace.id") {
                    event.rename("tmp.ece.log.trace.id", "trace.id")?;
                }

                if event.has_value("tmp.ece.log.proxy_ip") {
                    event.rename("tmp.ece.log.proxy_ip", "network.forwarded_ip")?;
                }

                if event.has_value("tmp.ece.log.query_parameters") {
                    event.rename("tmp.ece.log.query_parameters", "url.query")?;
                }

            let _cond = { !(event.get("tmp.ece.log.request_url").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("12443")), serde_json::Value::String(s) => s.contains("12443"), _ => false })) };
            if _cond {
            event.set("url.origin", json!(format!("http://{}", event.get("tmp.ece.log.request_url").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { event.get("tmp.ece.log.request_url").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("12443")), serde_json::Value::String(s) => s.contains("12443"), _ => false }) };
            if _cond {
            event.set("url.origin", json!(format!("https://{}", event.get("tmp.ece.log.request_url").map_or_else(String::new, template_to_string))))?;
            }

            if event.has_value("url.origin") {
                uri_parts(event, "url.origin", "url", true, false)?;
            }

                if event.has_value("tmp.ece.log.request_method") {
                    event.rename("tmp.ece.log.request_method", "http.request.method")?;
                }

                if event.has_value("tmp.ece.log.request_length") {
                    event.rename("tmp.ece.log.request_length", "http.request.body.bytes")?;
                }

            if event.has_value("http.request.body.bytes") {
                if let Some(val) = event.get("http.request.body.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.request.body.bytes".into(),
                            message,
                        })?;
                    event.set("http.request.body.bytes", converted)?;
                }
            }

                if event.has_value("tmp.ece.log.status_code") {
                    event.rename("tmp.ece.log.status_code", "http.response.status_code")?;
                }

            if event.has_value("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

                if event.has_value("tmp.ece.log.auth_user") {
                    event.rename("tmp.ece.log.auth_user", "user.name")?;
                }

                if event.has_value("tmp.ece.log.response_length") {
                    event.rename("tmp.ece.log.response_length", "http.response.body.bytes")?;
                }

            if event.has_value("http.response.body.bytes") {
                if let Some(val) = event.get("http.response.body.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.response.body.bytes".into(),
                            message,
                        })?;
                    event.set("http.response.body.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("tmp.ece.log.response_time") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event.duration = Long.parseLong(ctx.tmp.ece.log.response_time) * 1000000\n
                scale_field(event, &ScaleField::new("tmp.ece.log.response_time", "event.duration", Factor::Long(1000000)));
                Ok(())
            })();
            }

            if event.has_value("tmp.ece.log.user_agent") {
                if let Some(ua_str) = event.get_string("tmp.ece.log.user_agent") {
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

                if event.has_value("tmp.ece.log.request_id") {
                    event.rename("tmp.ece.log.request_id", "http.request.id")?;
                }

                if event.has_value("tmp.ece.log.organization_id") {
                    event.rename("tmp.ece.log.organization_id", "organization.id")?;
                }

                if event.has_value("tmp.ece.log.transaction.id") {
                    event.rename("tmp.ece.log.transaction.id", "transaction.id")?;
                }

                if event.has_value("tmp.ece.log.client_ip") {
                    event.rename("tmp.ece.log.client_ip", "source.address")?;
                }

                if event.has_value("tmp.ece.log.region") {
                    event.rename("tmp.ece.log.region", "cloud.region")?;
                }

                if event.has_value("tmp.ece.log.control_plane_use_case") {
                    event.rename("tmp.ece.log.control_plane_use_case", "event.module")?;
                }

            let _cond = { event.has_value("http.request.method") && event.has_value("url.original") };
            if _cond {
                event.append_unique("event.category", json!("api"))?;
            }

            let _cond = { event.has_value("http.response.status_code") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("network.forwarded_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("network.forwarded_ip").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("tmp.extraction_failures.source_ip", json!(true))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.remove("source.address");
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

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("url.domain").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("destination.as.organization_name") {
                    event.rename("destination.as.organization_name", "destination.as.organization.name")?;
                }

            // SKIPPED: condition not transpiled: ctx.url?.path instanceof String && ctx.url.path.contains('/deployments/') && !['_search','traffic-filter/rulesets', 'templates'].contains(ctx.url.path.splitOnToken('/deployments/')[-1])
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(input) = event.get_string("url.path") {
                    // Grok pattern: \\/deployments\\/(?P<ece_adminconsole_log_deployment_id>[^\\/]+).*
                    if !cached_grok_mapped!("\\/deployments\\/(?P<ece_adminconsole_log_deployment_id>[^\\/]+).*", [("ece_adminconsole_log_deployment_id", "ece_adminconsole.log.deployment.id")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("url.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/elasticsearch/main-elasticsearch/proxy/")), serde_json::Value::String(s) => s.contains("/elasticsearch/main-elasticsearch/proxy/"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("url.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("/elasticsearch/main-elasticsearch/proxy/") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/elasticsearch/main-elasticsearch/proxy/") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("ece_adminconsole.log.deployment.es_api", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "url.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("url.original") && event.has_value("http.request.method") && event.get("url.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/api/v1/")), serde_json::Value::String(s) => s.contains("/api/v1/"), _ => false }) };
            if _cond {
                // Painless script
                // Source: def temp = params.get(ctx.http.request.method.toLowerCase());\n// elastic.co/api/v1/deployments\n// and we only want the part after the /api/v1/\nString url_parts = ctx.url.original.splitOnToken(\"/api/v1/\")[1];\nif (url_parts.contains('elasticsearch/elasticsearch/proxy/')){\n  // we handle the special cases here\n  url_parts = url_parts.splitOnToken('elasticsearch/elasticsearch/proxy/')[1];\n  // that should give us `_search`, `_cat/indices`\n  url_parts = url_parts.splitOnToken('?')[0];\n  // we do not care about params ? this will just overload the event.action\n  url_parts = url_parts.splitOnToken('/')[0];\n  // we do not care about '/' like in `_cat/indices`, just getting `_cat` back is good enough\n  ctx.putIfAbsent(\"event\", [:]);\n  ctx.event.action = \"elasticsearch_api_through_ece\" + \"-\" + url_parts.toLowerCase();\n}\nelse if (temp != null){\n    if (temp.get(url_parts) != null){\n        ctx.putIfAbsent(\"event\", [:]);\n        ctx.event.action = temp.get(url_parts);\n    }\n}\nif (ctx.event?.action == null){\n    ctx.putIfAbsent(\"event\", [:]);\n    ctx.event.action = ctx.http.request.method.toLowerCase() + \"_\" + url_parts.splitOnToken(\"/\")[0];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def temp = params.get(ctx.http.request.method.toLowerCase());\n// elastic.co/api/v1/deployments\n// and we only want the part after the /api/v1/\nString url_parts = ctx.url.original.splitOnToken(\"/api/v1/\")[1];\nif (url_parts.contains('elasticsearch/elasticsearch/proxy/')){\n  // we handle the special cases here\n  url_parts = url_parts.splitOnToken('elasticsearch/elasticsearch/proxy/')[1];\n  // that should give us `_search`, `_cat/indices`\n  url_parts = url_parts.splitOnToken('?')[0];\n  // we do not care about params ? this will just overload the event.action\n  url_parts = url_parts.splitOnToken('/')[0];\n  // we do not care about '/' like in `_cat/indices`, just getting `_cat` back is good enough\n  ctx.putIfAbsent(\"event\", [:]);\n  ctx.event.action = \"elasticsearch_api_through_ece\" + \"-\" + url_parts.toLowerCase();\n}\nelse if (temp != null){\n    if (temp.get(url_parts) != null){\n        ctx.putIfAbsent(\"event\", [:]);\n        ctx.event.action = temp.get(url_parts);\n    }\n}\nif (ctx.event?.action == null){\n    ctx.putIfAbsent(\"event\", [:]);\n    ctx.event.action = ctx.http.request.method.toLowerCase() + \"_\" + url_parts.splitOnToken(\"/\")[0];\n}\n"#), cached_params!("{\"get\":{\"deployments\":\"list_deployments\"},\"post\":{\"deployments\":\"create_deployment\"}}"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "http.request.body.content", "tmp.request")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("tmp.json_payload_error", json!(true))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("tmp.json_payload_error") };
            if _cond {
                if event.has_value("tmp.request.name") {
                    event.rename("tmp.request.name", "ece_adminconsole.log.deployment.name")?;
                }
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
