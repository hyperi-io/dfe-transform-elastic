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
            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    || event.get("division").is_some_and(|v| v.is_string())
                    || event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            if event.has_value("message") {
                event.rename("message", "raw_message")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("raw_message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.original", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field_to_root(event, "raw_message", false)?;
                Ok(())
            })();

            let _cond = { event.get("tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def flatTags = []; for (def t : ctx.tags) {\n  if (t instanceof Map && t.containsKey('name')) {\n    flatTags.add(t.name);\n  } else {\n    flatTags.add(t.toString());\n  }\n} ctx.tags = flatTags;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def flatTags = []; for (def t : ctx.tags) {\n  if (t instanceof Map && t.containsKey('name')) {\n    flatTags.add(t.name);\n  } else {\n    flatTags.add(t.toString());\n  }\n} ctx.tags = flatTags;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get("severity").is_some_and(|v| v.is_string()) };
            if _cond {
                // Painless script
                // Source: def s = ctx.severity.toLowerCase(); if (ctx.event == null) { ctx.event = [:]; } if (ctx.labels == null) { ctx.labels = [:]; }\nif (s == 'high') { \n  ctx.event.severity = 1; \n  ctx.labels.severity = 'High';\n} else if (s == 'medium') { \n  ctx.event.severity = 3; \n  ctx.labels.severity = 'Medium';\n} else if (s == 'low') { \n  ctx.event.severity = 5; \n  ctx.labels.severity = 'Low';\n} else { \n  ctx.event.severity = 5; \n  ctx.labels.severity = 'Unknown';\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def s = ctx.severity.toLowerCase(); if (ctx.event == null) { ctx.event = [:]; } if (ctx.labels == null) { ctx.labels = [:]; }\nif (s == 'high') { \n  ctx.event.severity = 1; \n  ctx.labels.severity = 'High';\n} else if (s == 'medium') { \n  ctx.event.severity = 3; \n  ctx.labels.severity = 'Medium';\n} else if (s == 'low') { \n  ctx.event.severity = 5; \n  ctx.labels.severity = 'Low';\n} else { \n  ctx.event.severity = 5; \n  ctx.labels.severity = 'Unknown';\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("last_activity_timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("last_activity_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "last_activity_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("last_activity_timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("last_activity_timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("doppel.alert_updated_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "last_activity_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("created_at") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("created_at") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSS",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("notes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("notes")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppel.notes", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("product") == Some("telco") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("entity")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("product") != Some("telco") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("entity")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.url.full", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("brand")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("organization.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("queue_state")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.queue_state", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_state")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.entity_state", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("product")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.product", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.platform", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("source")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.provider", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("screenshot_url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reference", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("score")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.risk_score", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("assignee")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.assignee", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("doppel_link")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.url", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("uploaded_by")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.uploaded_by", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("audit_logs")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("labels.audit_logs", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("labels.entity_content", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.root_domain.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.url.domain", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.root_domain.ip_address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.ip", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.root_domain.registrar")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.domain.registrar", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.root_domain.hosting_provider")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.domain.hosting_provider", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.telco.country_code")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.telco.country_code", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.telco.telco_provider")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.telco.provider", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.social_media_user.slug")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.social_media_user.profile_display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.full_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.social_media_user.profile_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.social_media_user.profile_display_url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.social.profile_url", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.social_media_user.profile_image_url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.social.profile_image_url", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.social_media_user.num_followers")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.social.num_followers", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.mobile_app.title")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.app.title", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.mobile_app.platform_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.app.platform", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.mobile_app.bundle_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.app.bundle_id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.mobile_app.developer_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.app.developer_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.ecommerce.title")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.ecommerce.title", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.ecommerce.seller_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.ecommerce.seller_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.ecommerce.price")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.ecommerce.price", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.ecommerce.num_units")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.ecommerce.num_units", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.cred_leaks.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.cred_leaks.credential_url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.darkweb.credential_url", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.cred_leaks.leak_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("doppel.darkweb.leak_name", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("entity_content.cred_leaks.password") };
            if _cond {
                event.set("doppel.darkweb.cred_leaks_password", json!("[REDACTED]"))?;
            }

            event.set("event.kind", json!("alert"))?;

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.dataset", json!("doppel.alerts"))?;

            event.set("event.module", json!("doppel"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.original", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("entity_content.root_domain.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.domain", v)?;
                }
                Ok(())
            })();

            event.append_unique("event.category", json!("threat"))?;

            event.append_unique("event.type", json!("indicator"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.id") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "event.id".into(),
                    });
                }
                if let Some(v) = event.get("last_activity_timestamp") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "last_activity_timestamp".into(),
                    });
                }
                if !values.is_empty() {
                    event.set(
                        "_id",
                        json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                            TransformError::ParseError {
                                path: "_id".into(),
                                message,
                            }
                        })?),
                    )?;
                }
            }

            // SKIPPED: condition not transpiled: ctx.containsKey('tags') && ctx.tags != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("tags")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("doppel.tags", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("tags", Value::Array(vec![json!("preserve_original_event")]))?;
                Ok(())
            })();

            event.remove("raw_message");
            event.remove("id");
            event.remove("entity");
            event.remove("brand");
            event.remove("queue_state");
            event.remove("entity_state");
            event.remove("severity");
            event.remove("product");
            event.remove("platform");
            event.remove("source");
            event.remove("notes");
            event.remove("screenshot_url");
            event.remove("score");
            event.remove("assignee");
            event.remove("doppel_link");
            event.remove("uploaded_by");
            event.remove("entity_content");
            event.remove("audit_logs");
            event.remove("last_activity_timestamp");
            event.remove("created_at");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

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
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
