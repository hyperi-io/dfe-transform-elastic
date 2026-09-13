// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `audit` pipeline.
pub struct Audit;

impl Transform for Audit {
    fn name(&self) -> &str {
        "audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("backstage.is_audit_event", json!(true))?;

                event.append_unique("event.category", json!("api"))?;

            let _cond = { event.has_value("json.request") };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("json.status") == Some("initiated") };
            if _cond {
                event.append_unique("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("json.status") == Some("succeeded") || event.get_str("json.status") == Some("failed") };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            let _cond = { event.get_str("json.status") == Some("initiated") };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.get_str("json.status") == Some("succeeded") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("json.status") == Some("failed") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

                if event.has_value("json.severityLevel") {
                    event.rename("json.severityLevel", "backstage.severity_level")?;
                }

            let _cond = { event.get_str("backstage.severity_level") == Some("low") };
            if _cond {
            event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("backstage.severity_level") == Some("medium") };
            if _cond {
            event.set("event.severity", json!(5))?;
            }

            let _cond = { event.get_str("backstage.severity_level") == Some("high") };
            if _cond {
            event.set("event.severity", json!(7))?;
            }

            let _cond = { event.has_value("json.eventId") && event.has_value("json.status") };
            if _cond {
            event.set("event.action", json!(format!("{}:{}", event.get("json.eventId").map_or_else(String::new, template_to_string), event.get("json.status").map_or_else(String::new, template_to_string))))?;
            }

                if event.has_value("json.plugin") {
                    event.rename("json.plugin", "event.provider")?;
                }

                if event.has_value("json.eventId") {
                    event.rename("json.eventId", "event.code")?;
                }

                if event.has_value("json.name") {
                    event.rename("json.name", "error.type")?;
                }

                if event.has_value("json.stack") {
                    event.rename("json.stack", "error.stack_trace")?;
                }

                if event.has_value("json.actor.actorId") {
                    event.rename("json.actor.actorId", "user.id")?;
                }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("json.actor.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("json.actor.hostname").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.actor.ip") {
                    event.rename("json.actor.ip", "source.ip")?;
                }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }

                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.meta.query.filter") {
                    event.rename("json.meta.query.filter", "backstage.query.filter")?;
                }

                if event.has_value("json.meta.query.fields") {
                    event.rename("json.meta.query.fields", "backstage.query.fields")?;
                }

                if event.has_value("json.meta.queryType") {
                    event.rename("json.meta.queryType", "backstage.query.type")?;
                }

            let _cond = { !event.has_value("json.meta.query") || event.get("json.meta.query").is_some_and(|v| match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                event.remove("json.meta.query");
            }

                if event.has_value("json.meta.actionType") {
                    event.rename("json.meta.actionType", "backstage.action_type")?;
                }

                if event.has_value("json.meta.entityRef") {
                    event.rename("json.meta.entityRef", "backstage.entity.ref")?;
                }

                if event.has_value("json.meta.entityRefs") {
                    event.rename("json.meta.entityRefs", "backstage.entity.refs")?;
                }

                if event.has_value("json.meta.rootEntityRef") {
                    event.rename("json.meta.rootEntityRef", "backstage.entity.root_ref")?;
                }

                if event.has_value("json.meta.totalItems") {
                    event.rename("json.meta.totalItems", "backstage.total_items")?;
                }

            if event.has_value("backstage.total_items") {
                if let Some(val) = event.get("backstage.total_items") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "backstage.total_items".into(),
                            message,
                        })?;
                    event.set("backstage.total_items", converted)?;
                }
            }

                if event.has_value("json.meta.ancestry") {
                    event.rename("json.meta.ancestry", "backstage.ancestry")?;
                }

                if event.has_value("json.meta.createdBy") {
                    event.rename("json.meta.createdBy", "backstage.task.created_by")?;
                }

            let _cond = { event.has_value("backstage.task.created_by") };
            if _cond {
                event.append_unique("related.user", json!(event.get("backstage.task.created_by").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.meta.taskId") {
                    event.rename("json.meta.taskId", "backstage.task.id")?;
                }

                if event.has_value("json.meta.templateRef") {
                    event.rename("json.meta.templateRef", "backstage.template.ref")?;
                }

            let _cond = { event.has_value("json.meta.taskParameters") };
            if _cond {
                // Painless script
                // Source: ctx.backstage = ctx.backstage == null ? new HashMap() : ctx.backstage;\nctx.backstage.task = ctx.backstage.task == null ? new HashMap() : ctx.backstage.task;\nctx.backstage.task.parameters = ctx.json.meta.taskParameters.toString();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.backstage = ctx.backstage == null ? new HashMap() : ctx.backstage;\nctx.backstage.task = ctx.backstage.task == null ? new HashMap() : ctx.backstage.task;\nctx.backstage.task.parameters = ctx.json.meta.taskParameters.toString();\n"#))?;
            }

                event.remove("json.meta.taskParameters");

                event.remove("json.meta.pageInfo");

            let _cond = { event.has_value("json.meta") && event.get("json.meta").is_some_and(|v| !match v { serde_json::Value::String(s) => s.is_empty(), serde_json::Value::Array(a) => a.is_empty(), serde_json::Value::Object(o) => o.is_empty(), serde_json::Value::Null => true, _ => false }) };
            if _cond {
                // Painless script
                // Source: ctx.backstage = ctx.backstage == null ? new HashMap() : ctx.backstage;\nctx.backstage.meta = ctx.json.meta.toString();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.backstage = ctx.backstage == null ? new HashMap() : ctx.backstage;\nctx.backstage.meta = ctx.json.meta.toString();\n"#))?;
            }

                event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Pipeline '{}' failed at processor '{}' {}failed with message '{}'", event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
