// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("ecs.version", json!("8.0.0"))?;

        if event.has("azure") {
            event.rename("azure", "azure-eventhub")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Painless script
            // Source: ctx.message = ctx.message.replace(params.empty_field_name, '')
            painless_exec(
                event,
                r#"ctx.message = ctx.message.replace(params.empty_field_name, '')"#,
            )?;
            Ok(())
        })();

        let cond = { !event.has("event.original") };
        if cond {
            if event.has("message") {
                event.rename("message", "event.original")?;
            }
        }

        let cond = { event.has("event.original") };
        if cond {
            event.remove("message");
        }

        if let Some(s) = event.get_string("event.original") {
            let parsed: Value =
                serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                    path: "event.original".into(),
                    message: format!("failed to parse JSON: {}", e),
                })?;
            event.set("azure.platformlogs", parsed)?;
        }

        let cond = {
            event
                .get("azure.platformlogs.identity")
                .is_some_and(|v| v.is_string())
        };
        if cond {
            if event.has("azure.platformlogs.identity") {
                event.rename(
                    "azure.platformlogs.identity",
                    "azure.platformlogs.identity_name",
                )?;
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(date_str) = event.get_string("azure.platformlogs.time") {
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                    })
                {
                    event.set("@timestamp", dt.to_rfc3339())?;
                }
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(date_str) = event.get_string("azure.platformlogs.EventTimeString") {
                // Try ISO8601 format
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str)
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f%:z")
                    })
                    .or_else(|_| {
                        chrono::DateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%:z")
                    })
                {
                    event.set("@timestamp", dt.to_rfc3339())?;
                }
                // Try Java datetime format: CustomTime(\"M/d/yyyy h:mm:ss a XXX\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"M/d/yyyy h:mm:ss a XXX\")")
            }
            Ok(())
        })();

        event.remove("azure.platformlogs.time");

        if event.has("azure.platformlogs.resourceId") {
            event.rename("azure.platformlogs.resourceId", "azure.resource_id")?;
        }

        if event.has("azure.platformlogs.Region") {
            event.rename("azure.platformlogs.Region", "cloud.region")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(s) = event.get_string("azure.platformlogs.EventProperties") {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "azure.platformlogs.EventProperties".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("azure.platformlogs.properties", parsed)?;
            }
            Ok(())
        })();

        let cond = { event.has("azure.platformlogs.properties") };
        if cond {
            event.remove("azure.platformlogs.EventProperties");
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(s) = event.get_string("azure.platformlogs.properties.log") {
                let parsed: Value =
                    serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                        path: "azure.platformlogs.properties.log".into(),
                        message: format!("failed to parse JSON: {}", e),
                    })?;
                event.set("azure.platformlogs.properties.log", parsed)?;
            }
            Ok(())
        })();

        let cond = {
            event.has("azure.platformlogs.properties.log")
                && event
                    .get("azure.platformlogs.properties.log")
                    .is_some_and(|v| v.is_string())
        };
        if cond {
            if event.has("azure.platformlogs.properties.log") {
                event.rename("azure.platformlogs.properties.log", "message")?;
            }
        }

        if event.has("azure.platformlogs.EventName") {
            event.rename("azure.platformlogs.EventName", "event.action")?;
        }

        if event.has("azure.platformlogs.callerIpAddress") {
            if let Some(s) = event.get_string("azure.platformlogs.callerIpAddress") {
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "azure.platformlogs.callerIpAddress".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("source.ip", s)?;
            }
        }

        let cond = { event.has("source.ip") };
        if cond {
            event.remove("azure.platformlogs.callerIpAddress");
        }

        event.set(
            "client.ip",
            event.get("source.ip").cloned().unwrap_or(Value::Null),
        )?;

        let cond = { event.has("source.ip") };
        if cond {
            event.append(
                "related.ip",
                event.get("source.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        if event.has("azure.platformlogs.level") {
            event.rename("azure.platformlogs.level", "log.level")?;
        }

        let cond = {
            event.has("azure.platformlogs.durationMs")
                && event
                    .get("azure.platformlogs.durationMs")
                    .is_some_and(|v| v.is_string())
        };
        if cond {
            if event.has("azure.platformlogs.durationMs") {
                if let Some(val) = event.get("azure.platformlogs.durationMs") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "azure.platformlogs.durationMs".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "azure.platformlogs.durationMs".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "azure.platformlogs.durationMs".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("event.duration", converted)?;
                }
            }
        }

        event.remove("azure.platformlogs.durationMs");

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Painless script
            // Source: if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}
            painless_exec(
                event,
                r#"if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}"#,
            )?;
            Ok(())
        })();

        if event.has("azure.platformlogs.location") {
            event.rename("azure.platformlogs.location", "geo.name")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Painless script
            // Source: if (ctx?.azure?.platformlogs?.properties?.eventCategory != null) {\n  ctx.azure.platformlogs.event_category = ctx.azure.platformlogs.properties.eventCategory;\n} else if (ctx?.azure?.platformlogs?.properties?.policies != null)  {\n  ctx.azure.platformlogs.event_category = 'Policy';\n} else {\n  ctx.azure.platformlogs.event_category = 'Administrative';\n}
            painless_exec(
                event,
                r#"if (ctx?.azure?.platformlogs?.properties?.eventCategory != null) {\n  ctx.azure.platformlogs.event_category = ctx.azure.platformlogs.properties.eventCategory;\n} else if (ctx?.azure?.platformlogs?.properties?.policies != null)  {\n  ctx.azure.platformlogs.event_category = 'Policy';\n} else {\n  ctx.azure.platformlogs.event_category = 'Administrative';\n}"#,
            )?;
            Ok(())
        })();

        if event.has("azure.platformlogs.resultType") {
            event.rename(
                "azure.platformlogs.resultType",
                "azure.platformlogs.result_type",
            )?;
        }

        // result_type is a String whose lowercase form is 'success' or 'failure'
        let cond = event
            .get_str("azure.platformlogs.result_type")
            .is_some_and(|r| {
                let r = r.to_lowercase();
                r == "success" || r == "failure"
            });
        if cond {
            if let Some(val) = event.get("azure.platformlogs.result_type") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.outcome", converted)?;
            }
        }

        let cond = {
            !event.has("event.outcome")
                && event.has("azure.platformlogs.properties.result")
                && event
                    .get("azure.platformlogs.properties.result")
                    .is_some_and(|v| v.is_string())
                && ["'success'", "'failure'", "'unknown'"].contains(
                    &event
                        .get_str("azure.platformlogs.properties.result")
                        .unwrap_or(""),
                )
        };
        if cond {
            if let Some(val) = event.get("azure.platformlogs.properties.result") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.outcome", converted)?;
            }
        }

        let cond = {
            !event.has("event.outcome")
                && event.has("azure.platformlogs.Status")
                && event
                    .get("azure.platformlogs.Status")
                    .is_some_and(|v| v.is_string())
                && [
                    "'success'",
                    "'failure'",
                    "'unknown'",
                    "'Succeeded'",
                    "'Failed'",
                ]
                .contains(&event.get_str("azure.platformlogs.Status").unwrap_or(""))
        };
        if cond {
            if let Some(val) = event.get("azure.platformlogs.Status") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.outcome", converted)?;
            }
        }

        if event.has("azure.platformlogs.operationName") {
            event.rename(
                "azure.platformlogs.operationName",
                "azure.platformlogs.operation_name",
            )?;
        }

        if event.has("azure.platformlogs.operation_name") {
            if let Some(val) = event.get("azure.platformlogs.operation_name") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("event.action", converted)?;
            }
        }

        if event.has("azure.platformlogs.resultSignature") {
            event.rename(
                "azure.platformlogs.resultSignature",
                "azure.platformlogs.result_signature",
            )?;
        }

        if event.has("azure.platformlogs.correlationId") {
            event.rename("azure.platformlogs.correlationId", "azure.correlation_id")?;
        }

        if event.has("azure.platformlogs.properties.statusCode") {
            event.rename(
                "azure.platformlogs.properties.statusCode",
                "azure.platformlogs.properties.status_code",
            )?;
        }

        if event.has("azure.platformlogs.Status") {
            event.rename("azure.platformlogs.Status", "azure.platformlogs.status")?;
        }

        if event.has("source.ip") {
            if let Some(ip_str) = event.get_string("source.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("geo.location", v.clone())?;
                    }
                }
            }
        }

        // Painless script
        // Source: if (ctx?.azure?.platformlogs?.category == null) {\n  return;\n} def category = ctx.azure.platformlogs.category.toLowerCase(); if (params.get(category) == null) {\n  return;\n} def hm = new HashMap(params.get(category)); hm.forEach((k, v) -> ctx.event[k] = v);
        painless_exec(
            event,
            r#"if (ctx?.azure?.platformlogs?.category == null) {\n  return;\n} def category = ctx.azure.platformlogs.category.toLowerCase(); if (params.get(category) == null) {\n  return;\n} def hm = new HashMap(params.get(category)); hm.forEach((k, v) -> ctx.event[k] = v);"#,
        )?;

        if event.has("source.ip") {
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

        if event.has("source.ip") {
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

        if event.has("source.as.asn") {
            event.rename("source.as.asn", "source.as.number")?;
        }

        if event.has("source.as.organization_name") {
            event.rename("source.as.organization_name", "source.as.organization.name")?;
        }

        event.set("event.kind", json!("event"))?;

        // Begin nested pipeline: "azure-shared-pipeline"
        event.set("cloud.provider", json!("azure"))?;
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            // Pattern definitions for grok
            // PROVIDERNAME = .+
            // RULE = .+
            // NAMESPACE = .+
            // GROUPID = .+
            // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
            if let Some(input) = event.get_string("azure.resource_id") {
                // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/NAMESPACES/%{NAMESPACE:azure.resource.namespace}/AUTHORIZATIONRULES/%{RULE:azure.resource.authorization_rule}").extract_into(input, event)?;
                // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/namespaces/%{NAMESPACE:azure.resource.namespace}/authorizationRules/%{RULE:azure.resource.authorization_rule}
            }
            Ok(())
        })();
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // GROUPID = .+
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+/([A-Za-z])\w+.
                // NAME = ((?!AUTHORIZATIONRULES).)*$
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // GROUPID = .+
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+\/([A-Za-z][^\/])\w+
                // NAME = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}/providers/%{PROVIDERNAME:azure.resource.provider}/%{NAME:azure.resource.name}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // PROVIDER = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /providers/%{PROVIDER:azure.resource.provider}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/providers/%{PROVIDER:azure.resource.provider}")
                        .extract_into(input, event)?;
                    // Additional grok pattern 1: /PROVIDERS/%{PROVIDER:azure.resource.provider}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // PROVIDERNAME = ([A-Za-z])\w+.([A-Za-z])\w+\/([A-Za-z][^\/])\w+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/PROVIDERS/%{PROVIDERNAME:azure.resource.provider}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/providers/%{PROVIDERNAME:azure.resource.provider}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                // GROUPID = .+
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}/RESOURCEGROUPS/%{GROUPID:azure.resource.group}").extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}/resourceGroups/%{GROUPID:azure.resource.group}
                }
                Ok(())
            })();
        }
        let cond = { !event.has("azure.subscription_id") };
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Pattern definitions for grok
                // SUBID = (\{){0,1}[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}(\}){0,1}
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /SUBSCRIPTIONS/%{SUBID:azure.subscription_id}
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    cached_grok!("/SUBSCRIPTIONS/%{SUBID:azure.subscription_id}")
                        .extract_into(input, event)?;
                    // Additional grok pattern 1: /subscriptions/%{SUBID:azure.subscription_id}
                }
                Ok(())
            })();
        }
        if event.has("azure.resource_id") {
            event.rename("azure.resource_id", "azure.resource.id")?;
        }
        if event.has("event.outcome") {
            if let Some(s) = event.get_string("event.outcome") {
                let lowered = s.to_lowercase();
                event.set("event.outcome", lowered)?;
            }
        }
        // End nested pipeline: "azure-shared-pipeline"

        // tags is absent, or does not carry 'preserve_original_event'
        let cond = event.get_array("tags").is_none_or(|tags| {
            !tags
                .iter()
                .any(|t| t.as_str() == Some("preserve_original_event"))
        });
        if cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.original");
                Ok(())
            })();
        }

        // azure_log_forwarder.resource_type names Spring, in either casing
        let cond = event
            .get_str("azure_log_forwarder.resource_type")
            .is_some_and(|t| {
                t == "Microsoft.AppPlatform/Spring" || t == "MICROSOFT.APPPLATFORM/SPRING"
            });
        if cond {
            // Begin nested pipeline: "springcloudlogs-inner-pipeline"
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')
                painless_exec(
                    event,
                    r#"ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')"#,
                )?;
                Ok(())
            })();
            if event.has("azure.platformlogs") {
                event.rename("azure.platformlogs", "azure.springcloudlogs")?;
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.dataset", json!("azure.springcloudlogs"))?;
                Ok(())
            })();
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("data_stream.dataset", json!("azure.springcloudlogs"))?;
                Ok(())
            })();
            // ctx.azure.springcloudlogs.category != 'SystemLogs'
            //   && ctx.azure.springcloudlogs.category != 'ApplicationConsole'
            let cond = event
                .get_as_string("azure.springcloudlogs.category")
                .is_none_or(|c| c != "SystemLogs" && c != "ApplicationConsole");
            if cond {
                return Ok(TransformResult::Drop);
            }
            if event.has("azure.springcloudlogs.LogFormat") {
                event.rename(
                    "azure.springcloudlogs.LogFormat",
                    "azure.springcloudlogs.log_format",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.InstanceName") {
                event.rename(
                    "azure.springcloudlogs.properties.InstanceName",
                    "azure.springcloudlogs.properties.instance_name",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Log") {
                event.rename(
                    "azure.springcloudlogs.properties.Log",
                    "azure.springcloudlogs.properties.log",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.ServiceName") {
                event.rename(
                    "azure.springcloudlogs.properties.ServiceName",
                    "azure.springcloudlogs.properties.service_name",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Stream") {
                event.rename(
                    "azure.springcloudlogs.properties.Stream",
                    "azure.springcloudlogs.properties.stream",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.AppName") {
                event.rename(
                    "azure.springcloudlogs.properties.AppName",
                    "azure.springcloudlogs.properties.app_name",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.ServiceId") {
                event.rename(
                    "azure.springcloudlogs.properties.ServiceId",
                    "azure.springcloudlogs.properties.service_id",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Type") {
                event.rename(
                    "azure.springcloudlogs.properties.Type",
                    "azure.springcloudlogs.properties.type",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Level") {
                event.rename(
                    "azure.springcloudlogs.properties.Level",
                    "azure.springcloudlogs.level",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Logger") {
                event.rename(
                    "azure.springcloudlogs.properties.Logger",
                    "azure.springcloudlogs.properties.logger",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Stack") {
                event.rename(
                    "azure.springcloudlogs.properties.Stack",
                    "azure.springcloudlogs.properties.stack",
                )?;
            }
            if event.has("azure.springcloudlogs.properties.Thread") {
                event.rename(
                    "azure.springcloudlogs.properties.Thread",
                    "azure.springcloudlogs.properties.thread",
                )?;
            }
            if event.has("azure.springcloudlogs.level") {
                event.rename("azure.springcloudlogs.level", "log.level")?;
            }
            if event.has("azure.springcloudlogs.operationName") {
                event.rename(
                    "azure.springcloudlogs.operationName",
                    "azure.springcloudlogs.operation_name",
                )?;
            }
            if event.has("azure.springcloudlogs.operation_name") {
                if let Some(val) = event.get("azure.springcloudlogs.operation_name") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.action", converted)?;
                }
            }
            // End nested pipeline: "springcloudlogs-inner-pipeline"
        }

        if event.has("azure.platformlogs.resultDescription") {
            event.rename(
                "azure.platformlogs.resultDescription",
                "azure.platformlogs.result_description",
            )?;
        }

        Ok(TransformResult::Continue)
    }
}
