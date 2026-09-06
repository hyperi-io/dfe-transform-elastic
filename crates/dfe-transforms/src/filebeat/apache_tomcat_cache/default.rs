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

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("apache_tomcat"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            if event.has_value("prometheus.labels.context") {
                event.rename(
                    "prometheus.labels.context",
                    "apache_tomcat.cache.application_name",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_WebResourceRoot_objectMaxSize") {
                event.rename(
                    "prometheus.metrics.Catalina_WebResourceRoot_objectMaxSize",
                    "apache_tomcat.cache.object.size.max.kb",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_WebResourceRoot_hitCount") {
                event.rename(
                    "prometheus.metrics.Catalina_WebResourceRoot_hitCount",
                    "apache_tomcat.cache.hit.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_WebResourceRoot_lookupCount") {
                event.rename(
                    "prometheus.metrics.Catalina_WebResourceRoot_lookupCount",
                    "apache_tomcat.cache.lookup.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_WebResourceRoot_ttl") {
                event.rename(
                    "prometheus.metrics.Catalina_WebResourceRoot_ttl",
                    "apache_tomcat.cache.ttl.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_WebResourceRoot_maxSize") {
                event.rename(
                    "prometheus.metrics.Catalina_WebResourceRoot_maxSize",
                    "apache_tomcat.cache.size.max.kb",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_WebResourceRoot_size") {
                event.rename(
                    "prometheus.metrics.Catalina_WebResourceRoot_size",
                    "apache_tomcat.cache.size.current.kb",
                )?;
            }

            event.remove("prometheus");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
