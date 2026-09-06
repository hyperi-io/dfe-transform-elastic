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

            parse_json_field(event, "event.original", "gitlab.application")?;

            let _cond = { event.has_value("gitlab.application.time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gitlab.application.time") {
                        match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gitlab.application.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_event_created_time_epoch",
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
            }

            event.remove("gitlab.application.time");

            let _cond = { event.get_str("gitlab.application.severity") == Some("DEBUG") };
            if _cond {
                event.set("event.severity", json!(0))?;
            }

            let _cond = { event.get_str("gitlab.application.severity") == Some("INFO") };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("gitlab.application.severity") == Some("WARN") };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("gitlab.application.severity") == Some("ERROR") };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("gitlab.application.severity") == Some("FATAL") };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = { event.get_str("gitlab.application.severity") == Some("UNKNOWN") };
            if _cond {
                event.set("event.severity", json!(5))?;
            }

            event.remove("gitlab.application.severity");

            if event.has_value("gitlab.application.correlation_id") {
                event.rename("gitlab.application.correlation_id", "event.id")?;
            }

            if event.has_value("gitlab.application.pid") {
                event.rename("gitlab.application.pid", "process.pid")?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: if (ctx.gitlab?.application != null) {\n  def fieldsToRename = new ArrayList(ctx.gitlab.application.keySet());\n  for (fieldName in fieldsToRename) {\n    if (fieldName.endsWith('values')) {\n      def newField = fieldName.substring(0, fieldName.length() - 7);\n      def value = ctx.gitlab.application[fieldName];\n      if (value.size() > 1) {\n        ctx.gitlab.application[newField] = value;\n      } else {\n        ctx.gitlab.application[newField] = value[0]\n      }\n      ctx.gitlab.application.remove(fieldName);\n    }\n  }\n}\n
            unwrap_suffixed_keys(
                event,
                &UnwrapSuffixedKeys::new("gitlab.application".into(), "values".into(), 7),
            );

            dot_expand(event, "gitlab.application", "*")?;

            let _cond = { event.has_value("gitlab.application.params") };
            if _cond {
                // Painless script
                // Source: def keyValuePairs = [];\nfor (item in ctx.gitlab.application.params) {\n  def key = item.key;\n  def value = item.value;\n  def keyValueObject = [key: value];\n  keyValuePairs.add(keyValueObject)\n}\nctx.gitlab.application.params = keyValuePairs;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def keyValuePairs = [];\nfor (item in ctx.gitlab.application.params) {\n  def key = item.key;\n  def value = item.value;\n  def keyValueObject = [key: value];\n  keyValuePairs.add(keyValueObject)\n}\nctx.gitlab.application.params = keyValuePairs;\n"#
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.application.namespace_id") {
                    if let Some(val) = event.get("gitlab.application.namespace_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.application.namespace_id".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.application.namespace_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_application_namespace_id_to_long",
                )?;
                event.remove("gitlab.application.namespace_id");
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

            if event.has_value("gitlab.application.meta.user") {
                event.rename("gitlab.application.meta.user", "user.name")?;
            }

            if event.has_value("gitlab.application.meta.user_id") {
                if let Some(val) = event.get("gitlab.application.meta.user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "gitlab.application.meta.user_id".into(),
                            message,
                        }
                    })?;
                    event.set("gitlab.application.meta.user_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.application.user_id") {
                    if let Some(val) = event.get("gitlab.application.user_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.application.user_id".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.application.user_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_application_user_id_to_long",
                )?;
                event.remove("gitlab.application.user_id");
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

            if event.has_value("gitlab.application.meta.user_id") {
                event.rename("gitlab.application.meta.user_id", "user.id")?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            if event.has_value("gitlab.application.mergeability_merge_request_id") {
                event.rename(
                    "gitlab.application.mergeability_merge_request_id",
                    "gitlab.application.mergeability.merge_request_id",
                )?;
            }

            if event.has_value("gitlab.application.mergeability_project_id") {
                event.rename(
                    "gitlab.application.mergeability_project_id",
                    "gitlab.application.mergeability.project_id",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.application.duration_s") {
                    if let Some(val) = event.get("gitlab.application.duration_s") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.application.duration_s".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.application.duration_s", converted)?;
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
                event.remove("gitlab.application.duration_s");
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

            if event.has_value("gitlab.application.meta.gl_user_id") {
                if let Some(val) = event.get("gitlab.application.meta.gl_user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "gitlab.application.meta.gl_user_id".into(),
                            message,
                        }
                    })?;
                    event.set("gitlab.application.meta.gl_user_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("gitlab.application.meta.gl_user_id")
                    && event.get_str("gitlab.application.meta.gl_user_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("gitlab.application.meta.gl_user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("gitlab.application.meta.organization_id") {
                if let Some(val) = event.get("gitlab.application.meta.organization_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "gitlab.application.meta.organization_id".into(),
                            message,
                        }
                    })?;
                    event.set("gitlab.application.meta.organization_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("gitlab.application.meta.organization_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            let _cond = {
                event.has_value("gitlab.application.user_id")
                    && event.get_str("gitlab.application.user_id") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("gitlab.application.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("gitlab.application.message") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("gitlab.application.message") {
                        if let Some(input) = event.get_string("gitlab.application.message") {
                            // Grok pattern: ^Failed%{SPACE}Login:%{SPACE}username=%{USERNAME:user.name}%{SPACE}ip=%{IP:source.ip}$
                            // Grok pattern: ^%{USERNAME:user.name}%{SPACE}created%{SPACE}a%{SPACE}new%{SPACE}project%{SPACE}\"%{GREEDYDATA:gitlab.application.project_name}\"$
                            // Grok pattern: ^Project%{SPACE}\"%{GREEDYDATA:gitlab.application.project_name}\"%{SPACE}was%{SPACE}deleted$
                            // Grok pattern: ^User%{SPACE}Logout:%{SPACE}username=%{USERNAME:user.name}%{SPACE}ip=%{IP:source.ip}$
                            // Grok pattern: ^Successful%{SPACE}Login:%{SPACE}username=%{USERNAME:user.name}%{SPACE}ip=%{IP:source.ip}%{SPACE}method=%{WORD:gitlab.application.login_method}%{SPACE}admin=%{WORD:gitlab.application.user_admin}$
                            // Grok pattern: ^User%{SPACE}\"%{USERNAME:user.name}\"%{SPACE}\\(%{EMAILADDRESS:user.email}\\)%{SPACE}was%{SPACE}created$
                            // Grok pattern: ^User%{SPACE}\"%{USERNAME:user.name}\"%{SPACE}\\(%{EMAILADDRESS:user.email}\\)%{SPACE}was%{SPACE}removed$
                            // Grok pattern: ^Group%{SPACE}\"%{USERNAME:group.name}\"%{SPACE}was%{SPACE}created$
                            // Grok pattern: ^Group%{SPACE}\"%{USERNAME:group.name}\"%{SPACE}was%{SPACE}removed$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^Failed%{SPACE}Login:%{SPACE}username=%{USERNAME:user.name}%{SPACE}ip=%{IP:source.ip}$"
                                    ),
                                    cached_grok!(
                                        "^%{USERNAME:user.name}%{SPACE}created%{SPACE}a%{SPACE}new%{SPACE}project%{SPACE}\"%{GREEDYDATA:gitlab.application.project_name}\"$"
                                    ),
                                    cached_grok!(
                                        "^Project%{SPACE}\"%{GREEDYDATA:gitlab.application.project_name}\"%{SPACE}was%{SPACE}deleted$"
                                    ),
                                    cached_grok!(
                                        "^User%{SPACE}Logout:%{SPACE}username=%{USERNAME:user.name}%{SPACE}ip=%{IP:source.ip}$"
                                    ),
                                    cached_grok!(
                                        "^Successful%{SPACE}Login:%{SPACE}username=%{USERNAME:user.name}%{SPACE}ip=%{IP:source.ip}%{SPACE}method=%{WORD:gitlab.application.login_method}%{SPACE}admin=%{WORD:gitlab.application.user_admin}$"
                                    ),
                                    cached_grok!(
                                        "^User%{SPACE}\"%{USERNAME:user.name}\"%{SPACE}\\(%{EMAILADDRESS:user.email}\\)%{SPACE}was%{SPACE}created$"
                                    ),
                                    cached_grok!(
                                        "^User%{SPACE}\"%{USERNAME:user.name}\"%{SPACE}\\(%{EMAILADDRESS:user.email}\\)%{SPACE}was%{SPACE}removed$"
                                    ),
                                    cached_grok!(
                                        "^Group%{SPACE}\"%{USERNAME:group.name}\"%{SPACE}was%{SPACE}created$"
                                    ),
                                    cached_grok!(
                                        "^Group%{SPACE}\"%{USERNAME:group.name}\"%{SPACE}was%{SPACE}removed$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.application.user_admin") {
                    if let Some(val) = event.get("gitlab.application.user_admin") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.application.user_admin".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.application.user_admin", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_user_admin_to_boolean",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gitlab.application.meta.remote_ip") {
                    if let Some(val) = event.get("gitlab.application.meta.remote_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "gitlab.application.meta.remote_ip".into(),
                                message,
                            }
                        })?;
                        event.set("gitlab.application.meta.remote_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("gitlab.application.meta.remote_ip");
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

            if event.has_value("gitlab.application.meta.remote_ip") {
                event.rename("gitlab.application.meta.remote_ip", "client.ip")?;
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if let Some(v) = event.get("client.ip").cloned() {
                    event.set("client.address", v)?;
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if let Some(v) = event.get("client").cloned() {
                    event.set("source", v)?;
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = {
                event.has_value("gitlab.application.message")
                    && (event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Failed Login:"))
                            }
                            serde_json::Value::String(s) => s.contains("Failed Login:"),
                            _ => false,
                        })
                        || event
                            .get("gitlab.application.message")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Successful Login:"))
                                }
                                serde_json::Value::String(s) => s.contains("Successful Login:"),
                                _ => false,
                            }))
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.has_value("gitlab.application.message")
                    && event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Group"))
                            }
                            serde_json::Value::String(s) => s.contains("Group"),
                            _ => false,
                        })
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.has_value("gitlab.application.message")
                    && event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Failed Login:"))
                            }
                            serde_json::Value::String(s) => s.contains("Failed Login:"),
                            _ => false,
                        })
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.has_value("gitlab.application.message")
                    && event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Successful Login:"))
                            }
                            serde_json::Value::String(s) => s.contains("Successful Login:"),
                            _ => false,
                        })
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("gitlab.application.message")
                    && (event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("created a new project")),
                            serde_json::Value::String(s) => s.contains("created a new project"),
                            _ => false,
                        })
                        || event
                            .get("gitlab.application.message")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(" was created"))
                                }
                                serde_json::Value::String(s) => s.contains(" was created"),
                                _ => false,
                            }))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("gitlab.application.message")
                    && event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" was deleted"))
                            }
                            serde_json::Value::String(s) => s.contains(" was deleted"),
                            _ => false,
                        })
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            event.append("event.type", json!("info"))?;

            let _cond = {
                event.has_value("gitlab.application.message")
                    && !(event
                        .get("gitlab.application.message")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Failed Login:"))
                            }
                            serde_json::Value::String(s) => s.contains("Failed Login:"),
                            _ => false,
                        })
                        || event
                            .get("gitlab.application.message")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("Successful Login:"))
                                }
                                serde_json::Value::String(s) => s.contains("Successful Login:"),
                                _ => false,
                            }))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
