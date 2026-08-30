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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            event.rename("json.author", "miniflux.author")?;

            if let Some(date_str) = event.get_as_string("json.changed_at") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("miniflux.changed_at", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.changed_at".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.rename("json.comments_url", "miniflux.comments_url")?;

            event.rename("json.content", "miniflux.content")?;

            if let Some(date_str) = event.get_as_string("json.created_at") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("miniflux.created_at", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.created_at".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.rename("json.enclosures", "miniflux.enclosures")?;

            event.rename(
                "json.feed.allow_self_signed_certificates",
                "miniflux.feed.allow_self_signed_certificates",
            )?;

            event.rename(
                "json.feed.apprise_service_urls",
                "miniflux.feed.apprise_service_urls",
            )?;

            event.rename("json.feed.blocklist_rules", "miniflux.feed.blocklist_rules")?;

            event.rename(
                "json.feed.category.hide_globally",
                "miniflux.feed.category.hide_globally",
            )?;

            event.rename("json.feed.category.id", "miniflux.feed.category.id")?;

            event.rename("json.feed.category.title", "miniflux.feed.category.title")?;

            event.rename(
                "json.feed.category.user_id",
                "miniflux.feed.category.user_id",
            )?;

            if let Some(date_str) = event.get_as_string("json.feed.checked_at") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("miniflux.feed.checked_at", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.feed.checked_at".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.rename("json.feed.cookie", "miniflux.feed.cookie")?;

            event.rename("json.feed.crawler", "miniflux.feed.crawler")?;

            event.rename("json.feed.description", "miniflux.feed.description")?;

            event.rename("json.feed.disable_http2", "miniflux.feed.disable_http2")?;

            event.rename("json.feed.disabled", "miniflux.feed.disabled")?;

            event.rename("json.feed.etag_header", "miniflux.feed.etag_header")?;

            event.rename("json.feed.feed_url", "miniflux.feed.feed_url")?;

            event.rename("json.feed.fetch_via_proxy", "miniflux.feed.fetch_via_proxy")?;

            event.rename("json.feed.hide_globally", "miniflux.feed.hide_globally")?;

            event.rename("json.feed.icon.feed_id", "miniflux.feed.icon.feed_id")?;

            event.rename("json.feed.icon.icon_id", "miniflux.feed.icon.icon_id")?;

            event.rename(
                "json.feed.icon.external_icon_id",
                "miniflux.feed.icon.external_icon_id",
            )?;

            event.rename("json.feed.id", "miniflux.feed.id")?;

            event.rename(
                "json.feed.ignore_http_cache",
                "miniflux.feed.ignore_http_cache",
            )?;

            event.rename("json.feed.keeplist_rules", "miniflux.feed.keeplist_rules")?;

            event.rename(
                "json.feed.last_modified_header",
                "miniflux.feed.last_modified_header",
            )?;

            if let Some(date_str) = event.get_as_string("json.feed.next_check_at") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("miniflux.feed.next_check_at", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.feed.next_check_at".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.rename("json.feed.no_media_player", "miniflux.feed.no_media_player")?;

            event.rename("json.feed.ntfy_enabled", "miniflux.feed.ntfy_enabled")?;

            event.rename("json.feed.ntfy_topic", "miniflux.feed.ntfy_topic")?;

            event.rename("json.feed.ntfy_priority", "miniflux.feed.ntfy_priority")?;

            event.rename(
                "json.feed.parsing_error_count",
                "miniflux.feed.parsing_error_count",
            )?;

            event.rename(
                "json.feed.parsing_error_message",
                "miniflux.feed.parsing_error_message",
            )?;

            event.rename("json.feed.password", "miniflux.feed.password")?;

            event.rename("json.feed.proxy_url", "miniflux.feed.proxy_url")?;

            event.rename(
                "json.feed.pushover_enabled",
                "miniflux.feed.pushover_enabled",
            )?;

            event.rename(
                "json.feed.pushover_priority",
                "miniflux.feed.pushover_priority",
            )?;

            event.rename("json.feed.rewrite_rules", "miniflux.feed.rewrite_rules")?;

            event.rename("json.feed.scraper_rules", "miniflux.feed.scraper_rules")?;

            event.rename("json.feed.site_url", "miniflux.feed.site_url")?;

            event.rename("json.feed.title", "miniflux.feed.title")?;

            event.rename(
                "json.feed.urlrewrite_rules",
                "miniflux.feed.urlrewrite_rules",
            )?;

            event.rename("json.feed.user_agent", "miniflux.feed.user_agent")?;

            event.rename("json.feed.user_id", "miniflux.feed.user_id")?;

            event.rename("json.feed.username", "miniflux.feed.username")?;

            event.rename("json.feed.webhook_url", "miniflux.feed.webhook_url")?;

            event.rename("json.feed_id", "miniflux.feed_id")?;

            event.rename("json.hash", "miniflux.hash")?;

            event.append(
                "related.hash",
                json!(
                    event
                        .get("miniflux.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.rename("json.id", "miniflux.id")?;

            if let Some(date_str) = event.get_as_string("json.published_at") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("miniflux.published_at", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.published_at".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.rename("json.reading_time", "miniflux.reading_time")?;

            event.rename("json.share_code", "miniflux.share_code")?;

            event.rename("json.starred", "miniflux.starred")?;

            event.rename("json.status", "miniflux.status")?;

            event.rename("json.tags", "miniflux.tags")?;

            event.rename("json.title", "miniflux.title")?;

            let _cond = { event.has_value("json.url") && event.get_str("json.url") != Some("") };
            if _cond {
                uri_parts(event, "json.url", "url", true, false)?;
            }

            let _cond =
                { event.has_value("url.original") && event.get_str("url.original") != Some("") };
            if _cond {
                if let Some(v) = event.get("url.original").cloned() {
                    event.set("url.full", v)?;
                }
            }

            event.rename("json.url", "miniflux.url")?;

            event.rename("json.user_id", "miniflux.user_id")?;

            if event.remove("json").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "json".into(),
                });
            }

            // Painless script
            // Source: boolean drop(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndrop(ctx);"#
                ),
            )?;

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
