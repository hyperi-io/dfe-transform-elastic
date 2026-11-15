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

            parse_json_field(event, "event.original", "azure.open_ai")?;

            if event.has_value("azure.open_ai.resourceId") {
                event.rename("azure.open_ai.resourceId", "azure.resource.id")?;
            }

            let _cond = {
                event.get_str("azure.open_ai.category") != Some("Audit")
                    && event.get_str("azure.open_ai.category") != Some("RequestResponse")
                    && event.get_str("azure.open_ai.category") != Some("GatewayLogs")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event
                    .get("azure.open_ai.properties")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "azure.open_ai.properties",
                        "azure.open_ai.properties",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json-extract-stringly-Properties",
                    )?;
                    if event.remove("azure.open_ai.properties").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "azure.open_ai.properties".into(),
                        });
                    }
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
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "891ec7306240546833e7cc71b1eed08383bc2aef2fd446d263d8d2485c4632b0",
                    r#"{"params":null,"known":[{"KeysToSnakeCase":["azure.open_ai","CamelBreak"]}]}"#
                ),
            )?;

            let _cond = {
                event
                    .get("azure.open_ai.properties.backend_request_body")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                parse_json_field(
                    event,
                    "azure.open_ai.properties.backend_request_body",
                    "azure.open_ai.properties.backend_request_body",
                )?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "cf73d894c573117bc0f06dfb04e797b02aa0ba4a047cacbabcdc4f3b2befe6e7",
                    r#"{"params":null,"known":["ReplaceDotsInKeys"]}"#,
                    r#"if (ctx.azure.open_ai.properties.backend_request != null) {\n  ctx.temp_request = new HashMap();\n  for (String key : ctx.azure.open_ai.properties.backend_request.keySet()) {\n    ctx.temp_request[key.replace('.', '_')] = ctx.azure.open_ai.properties.backend_request.get(key);\n  }\n  ctx.azure.open_ai.properties.backend_request = ctx.temp_request; ctx.remove('temp_request');\n}"#
                ),
            )?;

            let _cond = {
                event
                    .get("azure.open_ai.properties.backend_response_body")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "azure.open_ai.properties.backend_response_body",
                        "azure.open_ai.properties.backend_response_body",
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "918185ae026adbd67164cfc4fe00720e1564f85c4b64fa09be88c52e4d5d91a8",
                        r#"{"params":null,"known":["ReplaceDotsInKeys"]}"#,
                        r#"if (ctx.azure.open_ai.properties.backend_response != null) {\n  ctx.temp_response = new HashMap();\n  for (String key : ctx.azure.open_ai.properties.backend_response.keySet()) {\n    ctx.temp_response[key.replace('.', '_')] = ctx.azure.open_ai.properties.backend_response.get(key);\n  }\n  ctx.azure.open_ai.properties.backend_response = ctx.temp_response; ctx.remove('temp_response');\n}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.open_ai.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "azure.open_ai.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            event.remove("azure.open_ai.time");

            if event.has_value("azure.open_ai.level") {
                if let Some(val) = event.get("azure.open_ai.level") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "azure.open_ai.level".into(),
                            message,
                        }
                    })?;
                    event.set("log.level", converted)?;
                }
            }

            event.remove("azure.open_ai.level");

            let _cond = { event.get_str("azure.open_ai.category") == Some("GatewayLogs") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("azure.open_ai.caller_ip_address") {
                        if let Some(val) = event.get("azure.open_ai.caller_ip_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "azure.open_ai.caller_ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert-caller_ip_address",
                    )?;
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("azure.open_ai.caller_ip_address") {
                            event.rename("azure.open_ai.caller_ip_address", "source.address")?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "rename")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "rename-caller_ip_address",
                        )?;
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
                                    .get("_ingest.on_failure_pipeline")
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get_str("azure.open_ai.category") == Some("GatewayLogs")
            };
            if _cond {
                event.remove("azure.open_ai.caller_ip_address");
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

            let _cond = {
                event.has_value("azure.open_ai.result_type")
                    && event
                        .get("azure.open_ai.result_type")
                        .is_some_and(|v| v.is_string())
                    && event
                        .get_str("azure.open_ai.result_type")
                        .is_some_and(|s| s.to_lowercase() == "succeeded")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("azure.open_ai.result_type")
                    && event
                        .get("azure.open_ai.result_type")
                        .is_some_and(|v| v.is_string())
                    && event
                        .get_str("azure.open_ai.result_type")
                        .is_some_and(|s| s.to_lowercase() == "failed")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("azure.open_ai.result_type")
                    && event.get_str("azure.open_ai.category") == Some("GatewayLogs")
            };
            if _cond {
                event.remove("azure.open_ai.result_type");
            }

            if event.has_value("azure.open_ai.properties.request_size") {
                event.rename(
                    "azure.open_ai.properties.request_size",
                    "azure.open_ai.properties.request_length",
                )?;
            }

            if event.has_value("azure.open_ai.properties.response_size") {
                event.rename(
                    "azure.open_ai.properties.response_size",
                    "azure.open_ai.properties.response_length",
                )?;
            }

            if event.has_value("azure.open_ai.properties.backend_response_body.usage.prompt_tokens")
            {
                event.rename(
                    "azure.open_ai.properties.backend_response_body.usage.prompt_tokens",
                    "azure.open_ai.properties.backend_response_body.usage.input_tokens",
                )?;
            }

            if event
                .has_value("azure.open_ai.properties.backend_response_body.usage.completion_tokens")
            {
                event.rename(
                    "azure.open_ai.properties.backend_response_body.usage.completion_tokens",
                    "azure.open_ai.properties.backend_response_body.usage.output_tokens",
                )?;
            }

            if event.has_value("azure.open_ai.properties.backend_response_body.choices.content_filter_results.protected_material_code.citation.URL") {
                    event.rename("azure.open_ai.properties.backend_response_body.choices.content_filter_results.protected_material_code.citation.URL", "azure.open_ai.properties.backend_response_body.choices.content_filter_results.protected_material_code.citation.url")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "b061a2c053dc2f2c0920eb44f3c2f7eded31cbb183da087ffea6edb3655ba2c1",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
                Ok(())
            })();

            let _cond =
                { event.has_value("azure.open_ai.properties.backend_response_body.choices") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "9789e2dfe5ab29131825524554984a8c704735ce5edf7f855122cf2affb2645c",
                            r#"{"params":null,"known":[]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("azure.open_ai.properties.method") {
                event.rename("azure.open_ai.properties.method", "http.request.method")?;
            }

            if event.has_value("azure.open_ai.properties.response_code") {
                event.rename(
                    "azure.open_ai.properties.response_code",
                    "http.response.status_code",
                )?;
            }

            let _cond = { event.has_value("azure.open_ai.properties.url") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(event, "azure.open_ai.properties.url", "url", true, false)?
                        && event
                            .get_str("azure.open_ai.properties.url")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "azure.open_ai.properties.url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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
                                .get("_ingest.on_failure_pipeline")
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
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                event.remove("azure.open_ai.properties.url");
            }

            event.remove("azure.open_ai.properties.response_body");
            event.remove("azure.open_ai.properties.request_body");

            if event.has_value("azure.open_ai.duration_ms") {
                event.rename("azure.open_ai.duration_ms", "event.duration")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "9f9c90e4f91775fe67ee2632e8c138fcb8957f6e350220f240467745e1aed012",
                        r#"{"params":"Scale","known":[]}"#,
                        r#"if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}"#
                    ),
                    cached_params!("{\"param_nano\":1000000}"),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Begin nested pipeline: "azure-shared-pipeline"
                event.set("cloud.provider", json!("azure"))?;
                if event.has_value("azure.resource.id") {
                    map_strings(
                        event,
                        "azure.resource.id",
                        "azure.resource.id",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("azure.resource.id") {
                            // Grok pattern: (?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:.+$))
                            if !cached_grok_mapped!("(?i)/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:.+$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                if let Some(v) = event
                    .get("azure.subscription_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.account.id", v)?;
                }
                // End nested pipeline: "azure-shared-pipeline"
                Ok(())
            })();

            // The pipeline's script, emitted as the runtime's equivalent
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(format!(
                        "{} {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
