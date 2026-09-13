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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("doppler.session.ip") {
                    if let Some(val) = event.get("doppler.session.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "doppler.session.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("doppler.session.method") {
                event.rename("doppler.session.method", "doppler.secret_read.method")?;
            }

            if event.has_value("doppler.session.browser") {
                event.rename("doppler.session.browser", "user_agent.name")?;
            }

            if event.has_value("doppler.session.os") {
                event.rename("doppler.session.os", "user_agent.os.name")?;
            }

            event.remove("doppler.session");

            let _cond = { !event.has_value("source.geo") && event.has_value("source.ip") };
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

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { !event.has_value("network.type") && event.has_value("source.ip") };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
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

            let _cond = { event.has_value("doppler.metadata.secrets") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: // Doppler normally sends secrets as a list, but tolerate a single object\n// so the read is never silently dropped before doppler.metadata is removed.\ndef raw = ctx.doppler.metadata.secrets;\nArrayList secrets = new ArrayList();\nif (raw instanceof List) { secrets.addAll(raw); }\nelse if (raw instanceof Map) { secrets.add(raw); }\nArrayList out = new ArrayList();\nArrayList names = new ArrayList();\nArrayList projects = new ArrayList();\nArrayList envs = new ArrayList();\nArrayList configs = new ArrayList();\nfor (def s : secrets) {\n  if (!(s instanceof Map)) { continue; }\n  HashMap m = new HashMap();\n  if (s.p != null) { m.project = s.p; if (!projects.contains(s.p)) { projects.add(s.p); } }\n  if (s.e != null) { m.environment = s.e; if (!envs.contains(s.e)) { envs.add(s.e); } }\n  if (s.c != null) { m.config = s.c; if (!configs.contains(s.c)) { configs.add(s.c); } }\n  if (s.s != null) { m.name = s.s; if (!names.contains(s.s)) { names.add(s.s); } }\n  if (s.v != null) { m.version = s.v; }\n  if (s.r instanceof Map) {\n    HashMap r = new HashMap();\n    if (s.r.p != null) { r.project = s.r.p; }\n    if (s.r.e != null) { r.environment = s.r.e; }\n    if (s.r.c != null) { r.config = s.r.c; }\n    m.inherited_from = r;\n  }\n  out.add(m);\n}\nif (ctx.doppler.secret_read == null) { ctx.doppler.secret_read = new HashMap(); }\nctx.doppler.secret_read.secrets = out;\nctx.doppler.secret_read.secret_names = names;\nctx.doppler.secret_read.projects = projects;\nctx.doppler.secret_read.environments = envs;\nctx.doppler.secret_read.configs = configs;\n// Count the secrets actually reshaped (well-formed entries), so the count\n// matches doppler.secret_read.secrets rather than any malformed input.\nctx.doppler.secret_read.secret_count = out.size();
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// Doppler normally sends secrets as a list, but tolerate a single object\n// so the read is never silently dropped before doppler.metadata is removed.\ndef raw = ctx.doppler.metadata.secrets;\nArrayList secrets = new ArrayList();\nif (raw instanceof List) { secrets.addAll(raw); }\nelse if (raw instanceof Map) { secrets.add(raw); }\nArrayList out = new ArrayList();\nArrayList names = new ArrayList();\nArrayList projects = new ArrayList();\nArrayList envs = new ArrayList();\nArrayList configs = new ArrayList();\nfor (def s : secrets) {\n  if (!(s instanceof Map)) { continue; }\n  HashMap m = new HashMap();\n  if (s.p != null) { m.project = s.p; if (!projects.contains(s.p)) { projects.add(s.p); } }\n  if (s.e != null) { m.environment = s.e; if (!envs.contains(s.e)) { envs.add(s.e); } }\n  if (s.c != null) { m.config = s.c; if (!configs.contains(s.c)) { configs.add(s.c); } }\n  if (s.s != null) { m.name = s.s; if (!names.contains(s.s)) { names.add(s.s); } }\n  if (s.v != null) { m.version = s.v; }\n  if (s.r instanceof Map) {\n    HashMap r = new HashMap();\n    if (s.r.p != null) { r.project = s.r.p; }\n    if (s.r.e != null) { r.environment = s.r.e; }\n    if (s.r.c != null) { r.config = s.r.c; }\n    m.inherited_from = r;\n  }\n  out.add(m);\n}\nif (ctx.doppler.secret_read == null) { ctx.doppler.secret_read = new HashMap(); }\nctx.doppler.secret_read.secrets = out;\nctx.doppler.secret_read.secret_names = names;\nctx.doppler.secret_read.projects = projects;\nctx.doppler.secret_read.environments = envs;\nctx.doppler.secret_read.configs = configs;\n// Count the secrets actually reshaped (well-formed entries), so the count\n// matches doppler.secret_read.secrets rather than any malformed input.\nctx.doppler.secret_read.secret_count = out.size();"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "reshape_secrets")?;
                    event.append("error.message", json!(format!("Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("doppler.metadata");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "drop_null_values")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'",
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

            // SKIPPED: nested pipeline "logs-doppler.secret_read@custom" is not in this pipeline set

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                        "Processor '{}' with tag '{}' in pipeline '{}' failed with message '{}'",
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
