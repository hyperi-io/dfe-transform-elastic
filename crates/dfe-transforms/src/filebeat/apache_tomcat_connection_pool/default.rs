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
                    "apache_tomcat.connection_pool.application_name",
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.Catalina_DataSource_accessToUnderlyingConnectionAllowed",
                ) == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.access_to_underlying_connection_allowed",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.Catalina_DataSource_accessToUnderlyingConnectionAllowed",
                ) == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.access_to_underlying_connection_allowed",
                    json!(false),
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_cacheState") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_cacheState",
                    "apache_tomcat.connection_pool.cache.state",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_removeAbandonedOnBorrow")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.remove_abandoned_on_borrow",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_removeAbandonedOnBorrow")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.remove_abandoned_on_borrow",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_abandonedUsageTracking")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.abandoned_usage_tracking",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_abandonedUsageTracking")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.abandoned_usage_tracking",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_autoCommitOnReturn")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.autocommit_on_return",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_autoCommitOnReturn")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.autocommit_on_return",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_closed") == Some(1) };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.closed",
                    json!(true),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_closed") == Some(0) };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.closed",
                    json!(false),
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_numActive") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_numActive",
                    "apache_tomcat.connection_pool.connection.active.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_defaultTransactionIsolation")
            {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_defaultTransactionIsolation",
                    "apache_tomcat.connection_pool.connection.default_transaction_isolation",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_enableAutoCommitOnReturn")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.enable_autocommit_on_return",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_enableAutoCommitOnReturn")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.enable_autocommit_on_return",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_fastFailValidation")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.fast_fail_validation",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_fastFailValidation")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.fast_fail_validation",
                    json!(false),
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_numIdle") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_numIdle",
                    "apache_tomcat.connection_pool.connection.idle.count",
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_logAbandoned") == Some(1) };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.idle.exists",
                    json!(true),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_logAbandoned") == Some(0) };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.idle.exists",
                    json!(false),
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_maxOpenPreparedStatements") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_maxOpenPreparedStatements",
                    "apache_tomcat.connection_pool.connection.idle.max.size",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_maxIdle") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_maxIdle",
                    "apache_tomcat.connection_pool.connection.idle.max.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_minIdle") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_minIdle",
                    "apache_tomcat.connection_pool.connection.idle.min.size",
                )?;
            }

            if event
                .has_value("prometheus.metrics.Catalina_DataSource_softMinEvictableIdleTimeMillis")
            {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_softMinEvictableIdleTimeMillis",
                    "apache_tomcat.connection_pool.connection.idle.min.time.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_numTestsPerEvictionRun") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_numTestsPerEvictionRun",
                    "apache_tomcat.connection_pool.connection.idle.max.time.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_initialSize") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_initialSize",
                    "apache_tomcat.connection_pool.connection.initial_size.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_maxConnLifetimeMillis") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_maxConnLifetimeMillis",
                    "apache_tomcat.connection_pool.connection.lifetime.max.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_maxWaitMillis") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_maxWaitMillis",
                    "apache_tomcat.connection_pool.connection.database.time.max.ms",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_logExpiredConnections")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.log_expired",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_logExpiredConnections")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.log_expired",
                    json!(false),
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_minEvictableIdleTimeMillis")
            {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_minEvictableIdleTimeMillis",
                    "apache_tomcat.connection_pool.connection.min_evictable_idle.time",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_removeAbandonedOnMaintenance")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.remove_abandoned_on_maintenance",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_removeAbandonedOnMaintenance")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.remove_abandoned_on_maintenance",
                    json!(false),
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_removeAbandonedTimeout") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_removeAbandonedTimeout",
                    "apache_tomcat.connection_pool.connection.remove_abandoned_timeout",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_rollbackOnReturn") == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.rollback_on_return",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_rollbackOnReturn") == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.rollback_on_return",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_testOnReturn") == Some(1) };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.test_on_return",
                    json!(true),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_testOnReturn") == Some(0) };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.test_on_return",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_testWhileIdle") == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.test_while_idle",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_testWhileIdle") == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.test_while_idle",
                    json!(false),
                )?;
            }

            if event
                .has_value("prometheus.metrics.Catalina_DataSource_timeBetweenEvictionRunsMillis")
            {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_timeBetweenEvictionRunsMillis",
                    "apache_tomcat.connection_pool.connection.time_betwen_eviction_run.time.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_validationQueryTimeout") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_validationQueryTimeout",
                    "apache_tomcat.connection_pool.connection.validate",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_clearStatementPoolOnReturn")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.clear_statement_pool_on_return",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_clearStatementPoolOnReturn")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.connection.clear_statement_pool_on_return",
                    json!(false),
                )?;
            }

            let _cond = { event.get_i64("prometheus.metrics.Catalina_DataSource_lifo") == Some(1) };
            if _cond {
                event.set("apache_tomcat.connection_pool.lifo", json!(true))?;
            }

            let _cond = { event.get_i64("prometheus.metrics.Catalina_DataSource_lifo") == Some(0) };
            if _cond {
                event.set("apache_tomcat.connection_pool.lifo", json!(false))?;
            }

            if event.has_value("prometheus.metrics.Catalina_DataSource_maxTotal") {
                event.rename(
                    "prometheus.metrics.Catalina_DataSource_maxTotal",
                    "apache_tomcat.connection_pool.max.total",
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_poolPreparedStatements")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.prepared_statements",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_DataSource_poolPreparedStatements")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.connection_pool.prepared_statements",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_testOnBorrow") == Some(1) };
            if _cond {
                event.set("apache_tomcat.connection_pool.test_on_borrow", json!(true))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_testOnBorrow") == Some(0) };
            if _cond {
                event.set("apache_tomcat.connection_pool.test_on_borrow", json!(false))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_testOnCreate") == Some(1) };
            if _cond {
                event.set("apache_tomcat.connection_pool.test_on_create", json!(true))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_DataSource_testOnCreate") == Some(0) };
            if _cond {
                event.set("apache_tomcat.connection_pool.test_on_create", json!(false))?;
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
