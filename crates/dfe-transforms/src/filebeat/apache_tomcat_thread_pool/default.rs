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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("prometheus.labels.name") {
                    if let Some(input) = event.get_string("prometheus.labels.name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("\"") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("\"") else {
                                break 'dissect false;
                            };
                            captured.push((
                                "apache_tomcat.thread_pool.nio_connector",
                                &remaining[..pos],
                            ));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\"") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "prometheus.labels.name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_nio_connector")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
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

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_acceptCount") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_acceptCount",
                    "apache_tomcat.thread_pool.thread.accept.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_currentThreadsBusy") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_currentThreadsBusy",
                    "apache_tomcat.thread_pool.thread.current.busy",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_connectionCount") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_connectionCount",
                    "apache_tomcat.thread_pool.connection.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_connectionLinger") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_connectionLinger",
                    "apache_tomcat.thread_pool.connection.linger",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_maxConnections") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_maxConnections",
                    "apache_tomcat.thread_pool.connection.max",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_connectionTimeout") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_connectionTimeout",
                    "apache_tomcat.thread_pool.connection.timeout",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_currentThreadCount") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_currentThreadCount",
                    "apache_tomcat.thread_pool.thread.current.count",
                )?;
            }

            if event.has_value(
                "prometheus.metrics.Catalina_ThreadPool_executorTerminationTimeoutMillis",
            ) {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_executorTerminationTimeoutMillis",
                    "apache_tomcat.thread_pool.executor_termination.timeout.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_keepAliveCount") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_keepAliveCount",
                    "apache_tomcat.thread_pool.keep_alive.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_maxKeepAliveRequests") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_maxKeepAliveRequests",
                    "apache_tomcat.thread_pool.keep_alive.max_requests",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_keepAliveTimeout") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_keepAliveTimeout",
                    "apache_tomcat.thread_pool.keep_alive.timeout",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_maxThreads") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_maxThreads",
                    "apache_tomcat.thread_pool.thread.requests.max",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_localPort") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_localPort",
                    "apache_tomcat.thread_pool.thread.port.default",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_portOffset") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_portOffset",
                    "apache_tomcat.thread_pool.thread.port.offset",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_port") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_port",
                    "apache_tomcat.thread_pool.thread.port.value",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_portWithOffset") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_portWithOffset",
                    "apache_tomcat.thread_pool.thread.port.with_offset",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_acceptorThreadPriority") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_acceptorThreadPriority",
                    "apache_tomcat.thread_pool.thread.priority.acceptor",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_pollerThreadPriority") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_pollerThreadPriority",
                    "apache_tomcat.thread_pool.thread.priority.poller",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_threadPriority") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_threadPriority",
                    "apache_tomcat.thread_pool.thread.priority.count",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_selectorTimeout") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_selectorTimeout",
                    "apache_tomcat.thread_pool.thread.selector.timeout",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_sniParseLimit") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_sniParseLimit",
                    "apache_tomcat.thread_pool.thread.sni_parse_limit",
                )?;
            }

            if event.has_value("prometheus.metrics.Catalina_ThreadPool_minSpareThreads") {
                event.rename(
                    "prometheus.metrics.Catalina_ThreadPool_minSpareThreads",
                    "apache_tomcat.thread_pool.thread.running.min",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_CurrentThreadAllocatedBytes")
            {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_CurrentThreadAllocatedBytes",
                    "apache_tomcat.thread_pool.thread.current.allocated.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_CurrentThreadCpuTime") {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_CurrentThreadCpuTime",
                    "apache_tomcat.thread_pool.thread.current.cpu.time.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_CurrentThreadUserTime") {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_CurrentThreadUserTime",
                    "apache_tomcat.thread_pool.thread.current.user.time.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_DaemonThreadCount") {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_DaemonThreadCount",
                    "apache_tomcat.thread_pool.thread.daemon.count",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_PeakThreadCount") {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_PeakThreadCount",
                    "apache_tomcat.thread_pool.thread.peak.count",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_ThreadCount") {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_ThreadCount",
                    "apache_tomcat.thread_pool.thread.active.count",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Threading_TotalStartedThreadCount") {
                event.rename(
                    "prometheus.metrics.java_lang_Threading_TotalStartedThreadCount",
                    "apache_tomcat.thread_pool.thread.total",
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_running") == Some(1) };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.running.value",
                    json!(true),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_running") == Some(0) };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.running.value",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_sSLEnabled") == Some(1) };
            if _cond {
                event.set("apache_tomcat.thread_pool.ssl_enabled", json!(true))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_sSLEnabled") == Some(0) };
            if _cond {
                event.set("apache_tomcat.thread_pool.ssl_enabled", json!(false))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_tcpNoDelay") == Some(1) };
            if _cond {
                event.set("apache_tomcat.thread_pool.tcp_no_delay", json!(true))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_tcpNoDelay") == Some(0) };
            if _cond {
                event.set("apache_tomcat.thread_pool.tcp_no_delay", json!(false))?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_ThreadPool_useInheritedChannel")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.use_inherited_channel",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_ThreadPool_useInheritedChannel")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.use_inherited_channel",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_useSendfile") == Some(1) };
            if _cond {
                event.set("apache_tomcat.thread_pool.use_send_file", json!(true))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_useSendfile") == Some(0) };
            if _cond {
                event.set("apache_tomcat.thread_pool.use_send_file", json!(false))?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_ThreadAllocatedMemoryEnabled")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.allocated_memory.enabled",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_ThreadAllocatedMemoryEnabled")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.allocated_memory.enabled",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.java_lang_Threading_ThreadAllocatedMemorySupported",
                ) == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.allocated_memory.supported",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.java_lang_Threading_ThreadAllocatedMemorySupported",
                ) == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.allocated_memory.supported",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.java_lang_Threading_ThreadContentionMonitoringEnabled",
                ) == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.contention.monitoring_enabled",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.java_lang_Threading_ThreadContentionMonitoringEnabled",
                ) == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.contention.monitoring_enabled",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_bindOnInit") == Some(1) };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.initiated_connector.state",
                    json!(true),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_bindOnInit") == Some(0) };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.initiated_connector.state",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_daemon") == Some(1) };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.daemon.status",
                    json!(true),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_daemon") == Some(0) };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.daemon.status",
                    json!(false),
                )?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_paused") == Some(1) };
            if _cond {
                event.set("apache_tomcat.thread_pool.thread.paused", json!(true))?;
            }

            let _cond =
                { event.get_i64("prometheus.metrics.Catalina_ThreadPool_paused") == Some(0) };
            if _cond {
                event.set("apache_tomcat.thread_pool.thread.paused", json!(false))?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_ThreadCpuTimeEnabled")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.current.cpu.time.enabled",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_ThreadCpuTimeEnabled")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.current.cpu.time.enabled",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.java_lang_Threading_ThreadContentionMonitoringSupported",
                ) == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.contention_monitoring",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64(
                    "prometheus.metrics.java_lang_Threading_ThreadContentionMonitoringSupported",
                ) == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.contention_monitoring",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_ObjectMonitorUsageSupported")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.usage.object_monitor",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_ObjectMonitorUsageSupported")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.usage.object_monitor",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_SynchronizerUsageSupported")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.usage.synchronizer",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.java_lang_Threading_SynchronizerUsageSupported")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.usage.synchronizer",
                    json!(false),
                )?;
            }

            let _cond = {
                event
                    .get_i64("prometheus.metrics.java_lang_Threading_CurrentThreadCpuTimeSupported")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.cpu.current.time",
                    json!(true),
                )?;
            }

            let _cond = {
                event
                    .get_i64("prometheus.metrics.java_lang_Threading_CurrentThreadCpuTimeSupported")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.thread.supported.cpu.current.time",
                    json!(false),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_Manager_persistAuthentication")
                    == Some(1)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.persist_authentication",
                    json!(true),
                )?;
            }

            let _cond = {
                event.get_i64("prometheus.metrics.Catalina_Manager_persistAuthentication")
                    == Some(0)
            };
            if _cond {
                event.set(
                    "apache_tomcat.thread_pool.persist_authentication",
                    json!(false),
                )?;
            }

            event.remove("prometheus");

            // Painless script
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
