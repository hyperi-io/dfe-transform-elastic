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
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("couchdb");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("database")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.ingested", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("http.server.error") == Some("unauthorized") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v =
                        json!("Unauthorized: You are not a server admin or read-only metrics user");
                    if !painless_is_empty_value(&v) {
                        event.set("error.message", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("http.server.error") == Some("not_found") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    let v = json!("Database does not exist");
                    if !painless_is_empty_value(&v) {
                        event.set("error.message", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.auth_cache_hits.value") {
                    event.rename(
                        "http.server.couchdb.auth_cache_hits.value",
                        "couchdb.server.auth_cache.hits",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.auth_cache_misses.value") {
                    event.rename(
                        "http.server.couchdb.auth_cache_misses.value",
                        "couchdb.server.auth_cache.misses",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.database_reads.value") {
                    event.rename(
                        "http.server.couchdb.database_reads.value",
                        "couchdb.server.database.reads",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.database_writes.value") {
                    event.rename(
                        "http.server.couchdb.database_writes.value",
                        "couchdb.server.database.writes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.open_databases.value") {
                    event.rename(
                        "http.server.couchdb.open_databases.value",
                        "couchdb.server.database.open",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.open_os_files.value") {
                    event.rename(
                        "http.server.couchdb.open_os_files.value",
                        "couchdb.server.open_os_files",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.request_time.value.arithmetic_mean") {
                    event.rename(
                        "http.server.couchdb.request_time.value.arithmetic_mean",
                        "couchdb.server.request_time.avg",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd.bulk_requests.value") {
                    event.rename(
                        "http.server.couchdb.httpd.bulk_requests.value",
                        "couchdb.server.httpd.requests.bulk",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd.requests.value") {
                    event.rename(
                        "http.server.couchdb.httpd.requests.value",
                        "couchdb.server.httpd.requests.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd.clients_requesting_changes.value") {
                    event.rename(
                        "http.server.couchdb.httpd.clients_requesting_changes.value",
                        "couchdb.server.httpd.clients_requesting_changes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd.temporary_view_reads.value") {
                    event.rename(
                        "http.server.couchdb.httpd.temporary_view_reads.value",
                        "couchdb.server.httpd.view_reads.temporary",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd.view_reads.value") {
                    event.rename(
                        "http.server.couchdb.httpd.view_reads.value",
                        "couchdb.server.httpd.view_reads.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_request_methods.COPY.value") {
                    event.rename(
                        "http.server.couchdb.httpd_request_methods.COPY.value",
                        "couchdb.server.httpd.request_methods.copy",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_request_methods.DELETE.value") {
                    event.rename(
                        "http.server.couchdb.httpd_request_methods.DELETE.value",
                        "couchdb.server.httpd.request_methods.delete",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_request_methods.GET.value") {
                    event.rename(
                        "http.server.couchdb.httpd_request_methods.GET.value",
                        "couchdb.server.httpd.request_methods.get",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_request_methods.HEAD.value") {
                    event.rename(
                        "http.server.couchdb.httpd_request_methods.HEAD.value",
                        "couchdb.server.httpd.request_methods.head",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_request_methods.POST.value") {
                    event.rename(
                        "http.server.couchdb.httpd_request_methods.POST.value",
                        "couchdb.server.httpd.request_methods.post",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_request_methods.PUT.value") {
                    event.rename(
                        "http.server.couchdb.httpd_request_methods.PUT.value",
                        "couchdb.server.httpd.request_methods.put",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.200.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.200.value",
                        "couchdb.server.httpd.status_codes.200",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.201.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.201.value",
                        "couchdb.server.httpd.status_codes.201",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.202.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.202.value",
                        "couchdb.server.httpd.status_codes.202",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.301.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.301.value",
                        "couchdb.server.httpd.status_codes.301",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.304.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.304.value",
                        "couchdb.server.httpd.status_codes.304",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.400.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.400.value",
                        "couchdb.server.httpd.status_codes.400",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.401.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.401.value",
                        "couchdb.server.httpd.status_codes.401",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.403.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.403.value",
                        "couchdb.server.httpd.status_codes.403",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.404.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.404.value",
                        "couchdb.server.httpd.status_codes.404",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.405.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.405.value",
                        "couchdb.server.httpd.status_codes.405",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.409.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.409.value",
                        "couchdb.server.httpd.status_codes.409",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.412.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.412.value",
                        "couchdb.server.httpd.status_codes.412",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.server.couchdb.httpd_status_codes.500.value") {
                    event.rename(
                        "http.server.couchdb.httpd_status_codes.500.value",
                        "couchdb.server.httpd.status_codes.500",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("http");
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) { if (object == null || object == \"\") { return true; } else if (object instanceof Map) { ((Map) object).values().removeIf(value -> dropEmptyFields(value)); return (((Map) object).size() == 0); } else if (object instanceof List) { ((List) object).removeIf(value -> dropEmptyFields(value)); return (((List) object).length == 0); } return false; } dropEmptyFields(ctx);
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
