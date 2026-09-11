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

            event.set("event.kind", json!("alert"))?;

            parse_json_field(event, "event.original", "json")?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event.event_time") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event.event_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_timestamp_epoch")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.event.severity") {
                if let Some(val) = event.get("json.event.severity") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.severity".into(),
                            message,
                        }
                    })?;
                    event.set("event.severity", converted)?;
                }
            }

            if event.has_value("json.event.full_session_id") {
                event.rename("json.event.full_session_id", "event.id")?;
            }

            if event.has_value("json.event.pe_action") {
                event.rename("json.event.pe_action", "event.action")?;
            }

            event.set("event.outcome", json!("unknown"))?;

            let _cond = {
                event.get_str("event.action") == Some("block")
                    || event.get_str("event.action") == Some("isolate")
                    || event.get_str("event.action") == Some("ssl_exception")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("direct")
                    || event.get_str("event.action") == Some("allow")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            if event.has_value("json.event.pe_reason") {
                event.rename("json.event.pe_reason", "event.reason")?;
            }

            event.set("event.category", json!("web"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.category", json!("threat"))?;

            if event.has_value("json.event.name") {
                event.rename("json.event.name", "menlo.web.request_type")?;
            }

            let _cond = { event.get_str("json.event.soph_dlp_ref") != Some("NA") };
            if _cond {
                if event.has_value("json.event.soph_dlp_ref") {
                    event.rename("json.event.soph_dlp_ref", "event.reference")?;
                }
            }

            if event.has_value("json.event.risk_score") {
                event.rename("json.event.risk_score", "menlo.web.risk_score")?;
            }

            event.append_unique(
                "dns.answers.data",
                json!(
                    event
                        .get("json.event.dst")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.event.product") {
                event.rename("json.event.product", "observer.product")?;
            }

            if event.has_value("json.event.vendor") {
                event.rename("json.event.vendor", "observer.vendor")?;
            }

            if event.has_value("json.event.risk_tally") {
                if let Some(val) = event.get("json.event.risk_tally") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.risk_tally".into(),
                            message,
                        }
                    })?;
                    event.set("menlo.web.tally", converted)?;
                }
            }

            let _cond = { event.get_str("json.event.has_password") != Some("NA") };
            if _cond {
                if event.has_value("json.event.has_password") {
                    if let Some(val) = event.get("json.event.has_password") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event.has_password".into(),
                                message,
                            }
                        })?;
                        event.set("menlo.web.has_password", converted)?;
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event.x-client-ip") {
                    if let Some(val) = event.get("json.event.x-client-ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event.x-client-ip".into(),
                                message,
                            }
                        })?;
                        event.set("client.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.event.origin_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.event.origin_ip".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.event.origin_country", "server.geo.country_iso_code")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.event.x-client-country", "client.geo.country_iso_code")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event.egress_country") {
                    event.rename("json.event.egress_country", "observer.geo.country_iso_code")?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.event.file_size") != Some("NA") };
            if _cond {
                if event.has_value("json.event.file_size") {
                    if let Some(val) = event.get("json.event.file_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event.file_size".into(),
                                message,
                            }
                        })?;
                        event.set("file.size", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("json.event.egress_ip") != Some("NA") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("json.event.egress_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("server")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination", v)?;
            }

            if let Some(v) = event.get("client").cloned() {
                event.set("source", v)?;
            }

            if event.has_value("json.event.content-type") {
                event.rename("json.event.content-type", "menlo.web.content_type")?;
            }

            if event.has_value("json.event.user-agent") {
                if let Some(ua_str) = event.get_string("json.event.user-agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            if event.has_value("json.event.request_type") {
                event.rename("json.event.request_type", "http.request.method")?;
            }

            if event.has_value("json.event.ua_type") {
                event.rename("json.event.ua_type", "menlo.web.ua_type")?;
            }

            if event.has_value("json.event.version") {
                event.rename("json.event.version", "observer.version")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "json.event.url", "url", true, true)?;
                Ok(())
            })();

            if let Some(domain) = event.get_string("url.domain") {
                // Public suffix list lookup for registered domain extraction.
                // A failed lookup writes NO target field, which is what
                // Elasticsearch does.
                if let Some(rd) = registered_domain_lookup(&domain) {
                    event.set("url.domain", json!(domain))?;
                    if let Some(registered) = rd.registered_domain {
                        event.set("url.registered_domain", json!(registered))?;
                    }
                    event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("url.subdomain", json!(sub))?;
                    }
                }
            }

            if event.has_value("json.event.referer") {
                event.rename("json.event.referer", "http.request.referrer")?;
            }

            if event.has_value("json.event.userid") {
                event.rename("json.event.userid", "user.name")?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.protocol") {
                event.rename("json.event.protocol", "network.protocol")?;
            }

            if event.has_value("json.event.response_code") {
                if let Some(val) = event.get("json.event.response_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.response_code".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

            if let Some(v) = event
                .get("json.event.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            let _cond = { event.get_str("json.event.filename") != Some("NA") };
            if _cond {
                if event.has_value("json.event.filename") {
                    event.rename("json.event.filename", "file.name")?;
                }
            }

            if event.has_value("json.event.sha256") {
                event.rename("json.event.sha256", "file.hash.sha256")?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.categories") {
                event.rename("json.event.categories", "menlo.web.categories")?;
            }

            if event.has_value("json.event.threats") {
                event.rename("json.event.threats", "menlo.web.threats")?;
            }

            if event.has_value("json.event.threat_types") {
                event.rename("json.event.threat_types", "menlo.web.threat_types")?;
            }

            if event.has_value("json.event.is_iframe") {
                event.rename("json.event.is_iframe", "menlo.web.is_iframe")?;
            }

            if event.has_value("json.event.product") {
                event.rename("json.event.product", "observer.product")?;
            }

            if event.has_value("json.event.protocol") {
                event.rename("json.event.protocol", "network.protocol")?;
            }

            if event.has_value("json.event.cached") {
                event.rename("json.event.cached", "menlo.web.cached")?;
            }

            if event.has_value("json.event.casb_app_name") {
                event.rename("json.event.casb_app_name", "menlo.web.casb_app_name")?;
            }

            if event.has_value("json.event.casb_cat_name") {
                event.rename("json.event.casb_cat_name", "menlo.web.casb_cat_name")?;
            }

            if event.has_value("json.event.casb_fun_name") {
                event.rename("json.event.casb_fun_name", "menlo.web.casb_fun_name")?;
            }

            if event.has_value("json.event.casb_org_name") {
                event.rename("json.event.casb_org_name", "menlo.web.casb_org_name")?;
            }

            if event.has_value("json.event.casb_profile_id") {
                event.rename("json.event.casb_profile_id", "menlo.web.casb_profile_id")?;
            }

            if event.has_value("json.event.casb_profile_name") {
                event.rename(
                    "json.event.casb_profile_name",
                    "menlo.web.casb_profile_name",
                )?;
            }

            if event.has_value("json.event.casb_profile_type") {
                event.rename(
                    "json.event.casb_profile_type",
                    "menlo.web.casb_profile_type",
                )?;
            }

            if event.has_value("json.event.casb_risk_score") {
                event.rename("json.event.casb_risk_score", "menlo.web.casb_risk_score")?;
            }

            if event.has_value("json.event.connId") {
                event.rename("json.event.connId", "menlo.web.conn_id")?;
            }

            if event.has_value("json.event.reqId") {
                event.rename("json.event.reqId", "menlo.web.req_id")?;
            }

            if event.has_value("json.event.proxyEventType") {
                event.rename("json.event.proxyEventType", "menlo.web.proxy_event_type")?;
            }

            if event.has_value("json.event.proxyEventDetail") {
                event.rename(
                    "json.event.proxyEventDetail",
                    "menlo.web.proxy_event_detail",
                )?;
            }

            if event.has_value("json.event.sbox") {
                event.rename("json.event.sbox", "menlo.web.sbox")?;
            }

            if event.has_value("json.event.sbox_mal_act") {
                event.rename("json.event.sbox_mal_act", "menlo.web.sbox_mal_act")?;
            }

            if event.has_value("json.event.soph") {
                event.rename("json.event.soph", "menlo.web.soph")?;
            }

            if event.has_value("json.event.tab_id") {
                event.rename("json.event.tab_id", "menlo.web.tab_id")?;
            }

            if event.has_value("json.event.virus_details") {
                event.rename("json.event.virus_details", "menlo.web.virus_details")?;
            }

            let _cond = { event.get_str("json.event.xff_ip") != Some("NA") };
            if _cond {
                if event.has_value("json.event.xff_ip") {
                    event.rename("json.event.xff_ip", "menlo.web.xff_ip")?;
                }
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

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
