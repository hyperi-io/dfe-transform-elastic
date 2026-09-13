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

            event.set("event.kind", json!("event"))?;

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

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("#"))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            parse_json_field(event, "event.original", "gitlab.production")?;

            if event.has_value("gitlab.production.method") {
                event.rename("gitlab.production.method", "http.request.method")?;
            }

            if event.has_value("gitlab.production.path") {
                event.rename("gitlab.production.path", "url.path")?;
            }

            if event.has_value("gitlab.production.location") {
                event.rename("gitlab.production.location", "url.full")?;
            }

            dot_expand(event, "gitlab.production", "meta.caller_id")?;

            dot_expand(event, "gitlab.production", "meta.client_id")?;

            dot_expand(event, "gitlab.production", "meta.feature_category")?;

            dot_expand(event, "gitlab.production", "meta.organization_id")?;

            dot_expand(event, "gitlab.production", "meta.remote_ip")?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.meta.remote_ip") {
                    if let Some(val) = event.get("gitlab.production.meta.remote_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.meta.remote_ip".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.meta.remote_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("gitlab.production.meta.remote_ip");
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            dot_expand(event, "gitlab.production", "meta.user")?;

            dot_expand(event, "gitlab.production", "meta.user_id")?;

            dot_expand(event, "gitlab.production", "meta.search.page")?;

            if event.has_value("gitlab.production.action") {
                event.rename("gitlab.production.action", "event.action")?;
            }

            if event.has_value("gitlab.production.meta.caller_id") {
                event.rename("gitlab.production.meta.caller_id", "event.provider")?;
            }

            if event.has_value("gitlab.production.status") {
                event.rename("gitlab.production.status", "http.response.status_code")?;
            }

            let _cond = { event.has_value("gitlab.production.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("gitlab.production.time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSSX"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "gitlab.production.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("gitlab.production.pid") {
                event.rename("gitlab.production.pid", "process.pid")?;
            }

            if event.has_value("gitlab.production.worker_id") {
                event.rename("gitlab.production.worker_id", "process.name")?;
            }

            if event.has_value("gitlab.production.correlation_id") {
                event.rename("gitlab.production.correlation_id", "event.id")?;
            }

            if event.has_value("gitlab.production.duration_s") {
                event.rename("gitlab.production.duration_s", "event.duration")?;
            }

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event['duration'] = ctx.event.duration * 1e9;\n
                scale_field(
                    event,
                    &ScaleField::new(
                        "event.duration",
                        "event.duration",
                        Factor::Double(1000000000.0),
                    ),
                );
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("gitlab.production.remote_ip") {
                    if let Some(val) = event.get("gitlab.production.remote_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.remote_ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("gitlab.production.remote_ip");
                Ok(())
            })();

            if event.has_value("gitlab.production.user_id") {
                event.rename("gitlab.production.user_id", "user.id")?;
            }

            if event.has_value("user.id") {
                if let Some(val) = event.get("user.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "user.id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            if event.has_value("gitlab.production.username") {
                event.rename("gitlab.production.username", "user.name")?;
            }

            if event.has_value("gitlab.production.ua") {
                event.rename("gitlab.production.ua", "user_agent.original")?;
            }

            let _cond = { event.has_value("gitlab.production.params") };
            if _cond {
                // Painless script
                // Source: Map map = [:];\nfor (item in ctx.gitlab.production.params) {\n  def key = item.key;\n  def value = item.value;\n  if (key == \"variables\" && value instanceof Map) {\n    map[key] = Json.dump(value);\n  } else {\n    map[key] = value;\n  }\n}\nctx.gitlab.production.params = map;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map map = [:];\nfor (item in ctx.gitlab.production.params) {\n  def key = item.key;\n  def value = item.value;\n  if (key == \"variables\" && value instanceof Map) {\n    map[key] = Json.dump(value);\n  } else {\n    map[key] = value;\n  }\n}\nctx.gitlab.production.params = map;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("gitlab.production.graphql") };
            if _cond {
                // Painless script
                // Source: if (ctx.gitlab?.production?.graphql instanceof List) {\n  for (int i = 0; i < ctx.gitlab.production.graphql.size(); i++) {\n    def item = ctx.gitlab.production.graphql[i];\n    if (item.containsKey(\"variables\") && item.variables instanceof Map) {\n      item.variables = Json.dump(item.variables);\n    }\n  }\n} else if (ctx.gitlab.production.graphql instanceof Map) {\n  def vars = ctx.gitlab.production.graphql.variables;\n  if (vars instanceof Map) {\n    ctx.gitlab.production.graphql.variables = Json.dump(vars);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.gitlab?.production?.graphql instanceof List) {\n  for (int i = 0; i < ctx.gitlab.production.graphql.size(); i++) {\n    def item = ctx.gitlab.production.graphql[i];\n    if (item.containsKey(\"variables\") && item.variables instanceof Map) {\n      item.variables = Json.dump(item.variables);\n    }\n  }\n} else if (ctx.gitlab.production.graphql instanceof Map) {\n  def vars = ctx.gitlab.production.graphql.variables;\n  if (vars instanceof Map) {\n    ctx.gitlab.production.graphql.variables = Json.dump(vars);\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("gitlab.production.params.graphql.variables")
                    && event
                        .get("gitlab.production.params.graphql.variables")
                        .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: def vars = ctx.gitlab.production.params.graphql.variables;\nctx.gitlab.production.params.graphql.variables = Json.dump(vars);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def vars = ctx.gitlab.production.params.graphql.variables;\nctx.gitlab.production.params.graphql.variables = Json.dump(vars);\n"#
                    ),
                )?;
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
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            event.append_unique("event.type", json!("info"))?;

            let _cond = { event.get_str("url.path") == Some("/-/metrics") };
            if _cond {
                event.append_unique("event.category", json!("database"))?;
            }

            let _cond = { event.get_str("url.path") == Some("/dashboard/activity") };
            if _cond {
                event.append_unique("event.category", json!("database"))?;
            }

            let _cond = { event.get_str("url.path") == Some("/api/graphql") };
            if _cond {
                event.append_unique("event.category", json!("database"))?;
            }

            let _cond = { event.get_str("url.path") == Some("/-/manifest.json") };
            if _cond {
                event.append_unique("event.category", json!("web"))?;
            }

            let _cond = { event.get_str("url.path") == Some("/users/sign_in") };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("url.path")
                    && event.get("url.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("/search/"))
                        }
                        serde_json::Value::String(s) => s.contains("/search/"),
                        _ => false,
                    })
            };
            if _cond {
                event.append_unique("event.category", json!("web"))?;
            }

            let _cond = { event.get_str("url.path") == Some("/dashboard/groups") };
            if _cond {
                event.append_unique("event.category", json!("database"))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.path_traversal_check_duration_s") {
                    if let Some(val) =
                        event.get("gitlab.production.path_traversal_check_duration_s")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.path_traversal_check_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gitlab.production.path_traversal_check_duration_s",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_path_traversal_check_duration_s _to_double",
                )?;
                event.remove("gitlab.production.path_traversal_check_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_ci_replica_txn_max_duration_s") {
                    if let Some(val) =
                        event.get("gitlab.production.db_ci_replica_txn_max_duration_s")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_ci_replica_txn_max_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gitlab.production.db_ci_replica_txn_max_duration_s",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_ci_replica_txn_max_duration_s_to_double",
                )?;
                event.remove("gitlab.production.db_ci_replica_txn_max_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_ci_replica_write_count") {
                    if let Some(val) = event.get("gitlab.production.db_ci_replica_write_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_ci_replica_write_count".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_ci_replica_write_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_ci_replica_write_count_to_long",
                )?;
                event.remove("gitlab.production.db_ci_replica_write_count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_ci_txn_max_duration_s") {
                    if let Some(val) = event.get("gitlab.production.db_ci_txn_max_duration_s") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_ci_txn_max_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_ci_txn_max_duration_s", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_ci_txn_max_duration_s_to_double",
                )?;
                event.remove("gitlab.production.db_ci_txn_max_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_ci_write_count") {
                    if let Some(val) = event.get("gitlab.production.db_ci_write_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_ci_write_count".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_ci_write_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_ci_write_count_to_long",
                )?;
                event.remove("gitlab.production.db_ci_write_count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_main_replica_txn_max_duration_s") {
                    if let Some(val) =
                        event.get("gitlab.production.db_main_replica_txn_max_duration_s")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_main_replica_txn_max_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gitlab.production.db_main_replica_txn_max_duration_s",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_main_replica_txn_max_duration_s_to_double",
                )?;
                event.remove("gitlab.production.db_main_replica_txn_max_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_main_replica_write_count") {
                    if let Some(val) = event.get("gitlab.production.db_main_replica_write_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_main_replica_write_count".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_main_replica_write_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_main_replica_write_count_to_long",
                )?;
                event.remove("gitlab.production.db_main_replica_write_count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_main_txn_max_duration_s") {
                    if let Some(val) = event.get("gitlab.production.db_main_txn_max_duration_s") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_main_txn_max_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_main_txn_max_duration_s", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_main_txn_max_duration_s_to_double",
                )?;
                event.remove("gitlab.production.db_main_txn_max_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_main_write_count") {
                    if let Some(val) = event.get("gitlab.production.db_main_write_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_main_write_count".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_main_write_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_main_write_count_to_long",
                )?;
                event.remove("gitlab.production.db_main_write_count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_primary_txn_max_duration_s") {
                    if let Some(val) = event.get("gitlab.production.db_primary_txn_max_duration_s")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_primary_txn_max_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_primary_txn_max_duration_s", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_primary_txn_max_duration_s_to_double",
                )?;
                event.remove("gitlab.production.db_primary_txn_max_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_primary_write_count") {
                    if let Some(val) = event.get("gitlab.production.db_primary_write_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_primary_write_count".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_primary_write_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_primary_write_count_to_long",
                )?;
                event.remove("gitlab.production.db_primary_write_count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_replica_txn_max_duration_s") {
                    if let Some(val) = event.get("gitlab.production.db_replica_txn_max_duration_s")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_replica_txn_max_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_replica_txn_max_duration_s", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_replica_txn_max_duration_s_to_double",
                )?;
                event.remove("gitlab.production.db_replica_txn_max_duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.db_replica_write_count") {
                    if let Some(val) = event.get("gitlab.production.db_replica_write_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.db_replica_write_count".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.db_replica_write_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_db_replica_write_count_to_long",
                )?;
                event.remove("gitlab.production.db_replica_write_count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.production.duration_s") {
                    if let Some(val) = event.get("gitlab.production.duration_s") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.production.duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.production.duration_s", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_duration_s_to_double",
                )?;
                event.remove("gitlab.production.duration_s");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.api.path_traversal_check_duration_s") {
                    if let Some(val) = event.get("gitlab.api.path_traversal_check_duration_s") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.api.path_traversal_check_duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.api.path_traversal_check_duration_s", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_path_traversal_check_duration_s_to_double",
                )?;
                event.remove("gitlab.api.path_traversal_check_duration_s");
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

            if event.has_value("gitlab.production.meta.organization_id") {
                if let Some(val) = event.get("gitlab.production.meta.organization_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "gitlab.production.meta.organization_id".into(),
                            message,
                        }
                    })?;
                    event.set("gitlab.production.meta.organization_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("gitlab.production.meta.organization_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
