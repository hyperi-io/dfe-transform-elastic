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

            if event.has_value("statsd") {
                event.rename("statsd", "envoy")?;
            }

            // Painless script
            // Source: for (entry in ctx.envoy.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (k.startsWith('envoy_')) {\n        def parts = k.splitOnToken('_');\n        def newKey = '';\n\n        def matchFound = false;\n        for (specialCase in params.special_cases) {\n            def prefix = 'envoy_' + specialCase;\n            if (k.startsWith(prefix)) {\n                def suffix = k.substring(prefix.length());\n                if (suffix.startsWith('_')) {\n                    suffix = suffix.substring(1);\n                }\n                newKey = specialCase + (suffix.length() > 0 ? '.' + suffix : '');\n                matchFound = true;\n                break;\n            }\n        }\n        if (!matchFound && parts.length >= 2) {\n            def remaining = '';\n            for (int i = 2; i < parts.length; i++) {\n                remaining += parts[i];\n                if (i < parts.length - 1) remaining += '_';\n            }\n            newKey = parts[1] + '.' + remaining;\n        }\n        if (newKey.length() > 0) {\n            ctx.envoy.remove(k);\n            ctx.envoy[newKey] = v;\n        }\n    }\n    if (!(v instanceof String)) {\n        v.keySet().stream().filter(s -> s.startsWith('1m_rate') || s.startsWith('5m_rate') ||s.startsWith('15m_rate') || s.startsWith('p99_9') || s.startsWith('p75') || s.startsWith('p99') || s.startsWith('p95'))\n        .collect(Collectors.toList()).forEach(m -> v.remove(m))\n    }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (entry in ctx.envoy.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n    if (k.startsWith('envoy_')) {\n        def parts = k.splitOnToken('_');\n        def newKey = '';\n\n        def matchFound = false;\n        for (specialCase in params.special_cases) {\n            def prefix = 'envoy_' + specialCase;\n            if (k.startsWith(prefix)) {\n                def suffix = k.substring(prefix.length());\n                if (suffix.startsWith('_')) {\n                    suffix = suffix.substring(1);\n                }\n                newKey = specialCase + (suffix.length() > 0 ? '.' + suffix : '');\n                matchFound = true;\n                break;\n            }\n        }\n        if (!matchFound && parts.length >= 2) {\n            def remaining = '';\n            for (int i = 2; i < parts.length; i++) {\n                remaining += parts[i];\n                if (i < parts.length - 1) remaining += '_';\n            }\n            newKey = parts[1] + '.' + remaining;\n        }\n        if (newKey.length() > 0) {\n            ctx.envoy.remove(k);\n            ctx.envoy[newKey] = v;\n        }\n    }\n    if (!(v instanceof String)) {\n        v.keySet().stream().filter(s -> s.startsWith('1m_rate') || s.startsWith('5m_rate') ||s.startsWith('15m_rate') || s.startsWith('p99_9') || s.startsWith('p75') || s.startsWith('p99') || s.startsWith('p95'))\n        .collect(Collectors.toList()).forEach(m -> v.remove(m))\n    }\n}"#
                ),
                cached_params!(
                    "{\"special_cases\":[\"thread_local_cluster_manager\",\"cluster_manager\",\"tls_inspector\"]}"
                ),
            )?;

            dot_expand(event, "labels", "*")?;

            if event.has_value("labels.envoy.cluster_name") {
                event.rename("labels.envoy.cluster_name", "envoy.cluster.name")?;
            }

            if event.has_value("labels.envoy.listener_address") {
                event.rename("labels.envoy.listener_address", "envoy.listener.address")?;
            }

            if event.has_value("labels.envoy.http_conn_manager_prefix") {
                event.rename(
                    "labels.envoy.http_conn_manager_prefix",
                    "envoy.http.conn_manager_prefix",
                )?;
            }

            if event.has_value("labels.envoy.http_user_agent") {
                event.rename("labels.envoy.http_user_agent", "envoy.http.user_agent")?;
            }

            if event.has_value("labels.envoy.ssl_cipher") {
                event.rename("labels.envoy.ssl_cipher", "envoy.listener.ssl.cipher")?;
            }

            if event.has_value("labels.envoy.ssl_curve") {
                event.rename("labels.envoy.ssl_curve", "envoy.listener.ssl.curve")?;
            }

            if event.has_value("labels.envoy.ssl_sigalg") {
                event.rename("labels.envoy.ssl_sigalg", "envoy.listener.ssl.sigalg")?;
            }

            if event.has_value("labels.envoy.ssl_version") {
                event.rename("labels.envoy.ssl_version", "envoy.listener.ssl.version")?;
            }

            if event.has_value("labels.envoy.cipher_suite") {
                event.rename(
                    "labels.envoy.cipher_suite",
                    "envoy.listener.ssl.cipher_suite",
                )?;
            }

            if event.has_value("labels.envoy.clientssl_prefix") {
                event.rename(
                    "labels.envoy.clientssl_prefix",
                    "envoy.listener.ssl.clientssl_prefix",
                )?;
            }

            if event.has_value("labels.envoy.mongo_prefix") {
                event.rename("labels.envoy.mongo_prefix", "envoy.mongo.prefix")?;
            }

            if event.has_value("labels.envoy.mongo_cmd") {
                event.rename("labels.envoy.mongo_cmd", "envoy.mongo.cmd")?;
            }

            if event.has_value("labels.envoy.mongo_collection") {
                event.rename("labels.envoy.mongo_collection", "envoy.mongo.collection")?;
            }

            if event.has_value("labels.envoy.mongo_callsite") {
                event.rename("labels.envoy.mongo_callsite", "envoy.mongo.callsite")?;
            }

            if event.has_value("labels.envoy.ratelimit_prefix") {
                event.rename(
                    "labels.envoy.ratelimit_prefix",
                    "envoy.cluster.ratelimit.prefix",
                )?;
            }

            if event.has_value("labels.envoy.local_http_ratelimit_prefix") {
                event.rename(
                    "labels.envoy.local_http_ratelimit_prefix",
                    "envoy.local_http_ratelimit.prefix",
                )?;
            }

            if event.has_value("labels.envoy.local_network_ratelimit_prefix") {
                event.rename(
                    "labels.envoy.local_network_ratelimit_prefix",
                    "envoy.local_network_ratelimit.prefix",
                )?;
            }

            if event.has_value("labels.envoy.local_listener_ratelimit_prefix") {
                event.rename(
                    "labels.envoy.local_listener_ratelimit_prefix",
                    "envoy.local_listener_ratelimit.prefix",
                )?;
            }

            if event.has_value("labels.envoy.dns_filter_prefix") {
                event.rename("labels.envoy.dns_filter_prefix", "envoy.dns_filter.prefix")?;
            }

            if event.has_value("labels.envoy.connection_limit_prefix") {
                event.rename(
                    "labels.envoy.connection_limit_prefix",
                    "envoy.connection_limit.prefix",
                )?;
            }

            if event.has_value("labels.envoy.rbac_prefix") {
                event.rename("labels.envoy.rbac_prefix", "envoy.http.rbac.prefix")?;
            }

            if event.has_value("labels.envoy.rbac_http_prefix") {
                event.rename(
                    "labels.envoy.rbac_http_prefix",
                    "envoy.http.rbac.http_prefix",
                )?;
            }

            if event.has_value("labels.envoy.rbac_policy_name") {
                event.rename(
                    "labels.envoy.rbac_policy_name",
                    "envoy.http.rbac.policy_name",
                )?;
            }

            if event.has_value("labels.envoy.tcp_prefix") {
                event.rename("labels.envoy.tcp_prefix", "envoy.tcp.prefix")?;
            }

            if event.has_value("labels.envoy.udp_prefix") {
                event.rename("labels.envoy.udp_prefix", "envoy.udp.prefix")?;
            }

            if event.has_value("labels.envoy.fault_downstream_cluster") {
                event.rename(
                    "labels.envoy.fault_downstream_cluster",
                    "envoy.http.fault_downstream_cluster",
                )?;
            }

            if event.has_value("labels.envoy.dynamo_operation") {
                event.rename(
                    "labels.envoy.dynamo_operation",
                    "envoy.http.dynamodb.operation",
                )?;
            }

            if event.has_value("labels.envoy.dynamo_table") {
                event.rename("labels.envoy.dynamo_table", "envoy.http.dynamodb.table")?;
            }

            if event.has_value("labels.envoy.dynamo_partition_id") {
                event.rename(
                    "labels.envoy.dynamo_partition_id",
                    "envoy.http.dynamodb.partition_id",
                )?;
            }

            if event.has_value("labels.envoy.grpc_bridge_service") {
                event.rename(
                    "labels.envoy.grpc_bridge_service",
                    "envoy.cluster.grpc.bridge_service",
                )?;
            }

            if event.has_value("labels.envoy.grpc_bridge_method") {
                event.rename(
                    "labels.envoy.grpc_bridge_method",
                    "envoy.cluster.grpc.bridge_method",
                )?;
            }

            if event.has_value("labels.envoy.virtual_host") {
                event.rename("labels.envoy.virtual_host", "envoy.vhost.host")?;
            }

            if event.has_value("labels.envoy.virtual_cluster") {
                event.rename("labels.envoy.virtual_cluster", "envoy.vhost.cluster")?;
            }

            if event.has_value("labels.envoy.response_code") {
                event.rename("labels.envoy.response_code", "envoy.cluster.response_code")?;
            }

            if event.has_value("labels.envoy.response_code_class") {
                event.rename(
                    "labels.envoy.response_code_class",
                    "envoy.http.response_code_class",
                )?;
            }

            if event.has_value("labels.envoy.rds_route_config") {
                event.rename(
                    "labels.envoy.rds_route_config",
                    "envoy.http.rds.route_config",
                )?;
            }

            if event.has_value("labels.envoy.scoped_rds_config") {
                event.rename(
                    "labels.envoy.scoped_rds_config",
                    "envoy.http.rds.scoped_config",
                )?;
            }

            if event.has_value("labels.envoy.route") {
                event.rename("labels.envoy.route", "envoy.route")?;
            }

            if event.has_value("labels.envoy.ext_authz_prefix") {
                event.rename(
                    "labels.envoy.ext_authz_prefix",
                    "envoy.cluster.ext_authz.prefix",
                )?;
            }

            if event.has_value("labels.envoy.worker_id") {
                event.rename("labels.envoy.worker_id", "envoy.listener.worker_id")?;
            }

            if event.has_value("labels.envoy.thrift_prefix") {
                event.rename("labels.envoy.thrift_prefix", "envoy.thrift.prefix")?;
            }

            if event.has_value("labels.envoy.redis_prefix") {
                event.rename("labels.envoy.redis_prefix", "envoy.redis.prefix")?;
            }

            if event.has_value("labels.envoy.proxy_protocol_version") {
                event.rename(
                    "labels.envoy.proxy_protocol_version",
                    "envoy.proxy_proto.version",
                )?;
            }

            if event.has_value("labels.envoy.proxy_protocol_prefix") {
                event.rename(
                    "labels.envoy.proxy_protocol_prefix",
                    "envoy.proxy_proto.prefix",
                )?;
            }

            event.remove("metricset.name");

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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
