// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_traffic` pipeline.
pub struct PipelineTraffic;

impl Transform for PipelineTraffic {
    fn name(&self) -> &str {
        "pipeline_traffic"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.body") {
                if let Some(input) = event.get_string("watchguard_firebox.log.body") {
                    // Grok pattern: ^%{WORD:watchguard_firebox.log.disposition} %{DATA:watchguard_firebox.log.in_interface_name} %{DATA:watchguard_firebox.log.out_interface_name}(?: %{NUMBER:watchguard_firebox.log.ip_packet_length:long})? %{WORD:watchguard_firebox.log.transport}(?: %{NUMBER:watchguard_firebox.log.iph_length:long} %{NUMBER:watchguard_firebox.log.ttl:long})? %{IP:watchguard_firebox.log.source_ip} %{IP:watchguard_firebox.log.destination_ip}(?: %{NUMBER:watchguard_firebox.log.source_port:long} %{NUMBER:watchguard_firebox.log.destination_port:long})?(?: offset %{NUMBER:watchguard_firebox.log.offset:long} %{DATA:watchguard_firebox.log.protocol_flags} %{NUMBER:watchguard_firebox.log.sequence_number:long} win %{NUMBER:watchguard_firebox.log.window_size:long})?(?: %{GREEDYDATA:_temp})? \\(%{DATA:watchguard_firebox.log.policy_name}\\)$
                    // Grok pattern: ^%{WORD:watchguard_firebox.log.disposition} %{DATA:watchguard_firebox.log.in_interface_name} %{DATA:watchguard_firebox.log.out_interface_name}(?: %{NUMBER:watchguard_firebox.log.ip_packet_length:long})? %{WORD:watchguard_firebox.log.transport}(?: %{NUMBER:watchguard_firebox.log.iph_length:long} %{NUMBER:watchguard_firebox.log.ttl:long})? %{IP:watchguard_firebox.log.source_ip} %{IP:watchguard_firebox.log.destination_ip}(?: %{NUMBER:watchguard_firebox.log.source_port:long} %{NUMBER:watchguard_firebox.log.destination_port:long})?(?: offset %{NUMBER:watchguard_firebox.log.offset:long} %{DATA:watchguard_firebox.log.protocol_flags} %{NUMBER:watchguard_firebox.log.sequence_number:long} win %{NUMBER:watchguard_firebox.log.window_size:long})?(?: %{GREEDYDATA:_temp})?$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{WORD:watchguard_firebox.log.disposition} %{DATA:watchguard_firebox.log.in_interface_name} %{DATA:watchguard_firebox.log.out_interface_name}(?: %{NUMBER:watchguard_firebox.log.ip_packet_length:long})? %{WORD:watchguard_firebox.log.transport}(?: %{NUMBER:watchguard_firebox.log.iph_length:long} %{NUMBER:watchguard_firebox.log.ttl:long})? %{IP:watchguard_firebox.log.source_ip} %{IP:watchguard_firebox.log.destination_ip}(?: %{NUMBER:watchguard_firebox.log.source_port:long} %{NUMBER:watchguard_firebox.log.destination_port:long})?(?: offset %{NUMBER:watchguard_firebox.log.offset:long} %{DATA:watchguard_firebox.log.protocol_flags} %{NUMBER:watchguard_firebox.log.sequence_number:long} win %{NUMBER:watchguard_firebox.log.window_size:long})?(?: %{GREEDYDATA:_temp})? \\(%{DATA:watchguard_firebox.log.policy_name}\\)$"),
                            cached_grok!("^%{WORD:watchguard_firebox.log.disposition} %{DATA:watchguard_firebox.log.in_interface_name} %{DATA:watchguard_firebox.log.out_interface_name}(?: %{NUMBER:watchguard_firebox.log.ip_packet_length:long})? %{WORD:watchguard_firebox.log.transport}(?: %{NUMBER:watchguard_firebox.log.iph_length:long} %{NUMBER:watchguard_firebox.log.ttl:long})? %{IP:watchguard_firebox.log.source_ip} %{IP:watchguard_firebox.log.destination_ip}(?: %{NUMBER:watchguard_firebox.log.source_port:long} %{NUMBER:watchguard_firebox.log.destination_port:long})?(?: offset %{NUMBER:watchguard_firebox.log.offset:long} %{DATA:watchguard_firebox.log.protocol_flags} %{NUMBER:watchguard_firebox.log.sequence_number:long} win %{NUMBER:watchguard_firebox.log.window_size:long})?(?: %{GREEDYDATA:_temp})?$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            let _cond = { event.get("_temp").is_some_and(|v| v.is_string()) };
            if _cond {
                // Painless script
                // Source: def kvStart = 0; def kvSplit = 0; def inQuote = false;\nPattern quotePattern = /^\\\"|\\\"$/;\nfor (int i = 0, n = ctx[\"_temp\"].length(); i < n; ++i) {\n  char c = ctx[\"_temp\"].charAt(i);\n  char c2 = i < n - 1 ? ctx[\"_temp\"].charAt(i + 1) : 0;\n\n  if (c == (char)'\"') {\n    if (inQuote && (c2 == 0 || c2 == (char)' ' || c2 == (char)':')) {\n      inQuote = false;\n    } else {\n      inQuote = true;\n    }\n  }\n  if (inQuote) {\n    continue;\n  }\n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)'\"' || c == (char)' ' || c2 == 0) {\n    if (i != kvStart) {\n      def endIndex = i == n - 1 ? i + 1 : i;\n      def key = ctx[\"_temp\"].substring(kvStart, kvSplit);\n      def value = quotePattern.matcher(ctx[\"_temp\"].substring(kvSplit + 1, endIndex)).replaceAll(\"\");\n\n      if (key != '') {\n        ctx.watchguard_firebox.log.put(key, value);\n      }\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def kvStart = 0; def kvSplit = 0; def inQuote = false;\nPattern quotePattern = /^\\\"|\\\"$/;\nfor (int i = 0, n = ctx[\"_temp\"].length(); i < n; ++i) {\n  char c = ctx[\"_temp\"].charAt(i);\n  char c2 = i < n - 1 ? ctx[\"_temp\"].charAt(i + 1) : 0;\n\n  if (c == (char)'\"') {\n    if (inQuote && (c2 == 0 || c2 == (char)' ' || c2 == (char)':')) {\n      inQuote = false;\n    } else {\n      inQuote = true;\n    }\n  }\n  if (inQuote) {\n    continue;\n  }\n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)'\"' || c == (char)' ' || c2 == 0) {\n    if (i != kvStart) {\n      def endIndex = i == n - 1 ? i + 1 : i;\n      def key = ctx[\"_temp\"].substring(kvStart, kvSplit);\n      def value = quotePattern.matcher(ctx[\"_temp\"].substring(kvSplit + 1, endIndex)).replaceAll(\"\");\n\n      if (key != '') {\n        ctx.watchguard_firebox.log.put(key, value);\n      }\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}"#))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.in_interface_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.ingress.interface.alias", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.out_interface_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.egress.interface.alias", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.transport").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.source_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.destination_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.address") && event.has_value("watchguard_firebox.log.msg_id") && ["1BFF-0004", "1BFF-0022"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
                event.append_unique("email.from.address", json!(event.get("watchguard_firebox.log.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.address") && event.has_value("watchguard_firebox.log.msg_id") && ["1BFF-0005"].contains(&event.get_str("watchguard_firebox.log.msg_id").unwrap_or("")) };
            if _cond {
                event.append_unique("email.to.address", json!(event.get("watchguard_firebox.log.address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.address") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.address").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.app_ctl_disp") {
                    event.rename("watchguard_firebox.log.app_ctl_disp", "watchguard_firebox.log.app_control_disposition")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.app_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.application", v)?;
            }

            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.arg").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.path", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.authenticated_user") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.authenticated_user") {
                if let Some(input) = event.get_string("watchguard_firebox.log.authenticated_user") {
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.authenticated_user_domain}\\\\%{USERNAME:watchguard_firebox.log.authenticated_user}$
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.authenticated_user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.authenticated_user}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.authenticated_user}@%{HOSTNAME:watchguard_firebox.log.authenticated_user_domain}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.authenticated_user}$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.authenticated_user_domain}\\\\%{USERNAME:watchguard_firebox.log.authenticated_user}$"),
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.authenticated_user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.authenticated_user}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.authenticated_user}@%{HOSTNAME:watchguard_firebox.log.authenticated_user_domain}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.authenticated_user}$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            let _cond = { event.has_value("watchguard_firebox.log.authenticated_user") };
            if _cond {
                event.append_unique("user.name", json!(event.get("watchguard_firebox.log.authenticated_user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.authenticated_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.authenticated_user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.authenticated_user_domain") };
            if _cond {
                event.append_unique("user.domain", json!(event.get("watchguard_firebox.log.authenticated_user_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.authenticated_user_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.authenticated_user_domain").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.auth_method") {
                    event.rename("watchguard_firebox.log.auth_method", "watchguard_firebox.log.authentication_method")?;
                }

                if event.has_value("watchguard_firebox.log.authtype") {
                    event.rename("watchguard_firebox.log.authtype", "watchguard_firebox.log.authentication_type")?;
                }

            let _cond = { event.has_value("watchguard_firebox.log.bounce_ip") && event.get_str("watchguard_firebox.log.bounce_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.bounce_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.bounce_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.bounce_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.bounce_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bounce_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.bounce_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.bounce_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.bounce_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.bounce_ip").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.rcvd_bytes") {
                if let Some(val) = event.get("watchguard_firebox.log.rcvd_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.rcvd_bytes".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.bytes_in", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_rcvd_bytes_to_long")?;
                        if event.remove("watchguard_firebox.log.rcvd_bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.rcvd_bytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.bytes_in").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.bytes", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.sent_bytes") {
                if let Some(val) = event.get("watchguard_firebox.log.sent_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.sent_bytes".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.bytes_out", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sent_bytes_to_long")?;
                        if event.remove("watchguard_firebox.log.sent_bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.sent_bytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.bytes_out").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.bytes", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: if (ctx.network == null) {\n  ctx.network = new HashMap();\n} if (ctx.source.bytes != null && ctx.destination.bytes != null) {\n  ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes\n} else if (ctx.source.bytes == null && ctx.destination.bytes != null) {\n  ctx.network.bytes = ctx.destination.bytes\n} else if (ctx.source.bytes != null && ctx.destination.bytes == null) {\n  ctx.network.bytes = ctx.source.bytes\n}
                sum_directions(event, &["bytes"]);
                Ok(())
            })();

            let _cond = { event.has_value("watchguard_firebox.log.call_from") && event.get_str("watchguard_firebox.log.call_from") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.call_from") {
                if let Some(val) = event.get("watchguard_firebox.log.call_from") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.call_from".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.call_from", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_call_from_to_ip")?;
                        if event.remove("watchguard_firebox.log.call_from").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.call_from".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.call_from") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.call_from").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.call_to") && event.get_str("watchguard_firebox.log.call_to") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.call_to") {
                if let Some(val) = event.get("watchguard_firebox.log.call_to") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.call_to".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.call_to", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_call_to_to_ip")?;
                        if event.remove("watchguard_firebox.log.call_to").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.call_to".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.call_to") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.call_to").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.cat_name") {
                    event.rename("watchguard_firebox.log.cat_name", "watchguard_firebox.log.category_name")?;
                }

                if event.has_value("watchguard_firebox.log.cert_issuer") {
                    event.rename("watchguard_firebox.log.cert_issuer", "watchguard_firebox.log.certificate_issuer")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.certificate_issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.issuer", v)?;
            }

                if event.has_value("watchguard_firebox.log.cert_subject") {
                    event.rename("watchguard_firebox.log.cert_subject", "watchguard_firebox.log.certificate_subject")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.certificate_subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.server.x509.subject.distinguished_name", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.client_ssl") };
            if _cond {
                event.append_unique("tls.client.supported_ciphers", json!(event.get("watchguard_firebox.log.client_ssl").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.cn") };
            if _cond {
                event.append_unique("tls.server.x509.subject.common_name", json!(event.get("watchguard_firebox.log.cn").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.cn") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.cn").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.content_src") {
                    event.rename("watchguard_firebox.log.content_src", "watchguard_firebox.log.content_source")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.ctl_dst") {
                if let Some(input) = event.get_string("watchguard_firebox.log.ctl_dst") {
                    // Grok pattern: ^%{IP:watchguard_firebox.log.ctl_dst_ip}:%{POSINT:watchguard_firebox.log.ctl_dst_port:long}$
                    // Grok pattern: ^%{IP:watchguard_firebox.log.ctl_dst_ip}$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{IP:watchguard_firebox.log.ctl_dst_ip}:%{POSINT:watchguard_firebox.log.ctl_dst_port:long}$"),
                            cached_grok!("^%{IP:watchguard_firebox.log.ctl_dst_ip}$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            let _cond = { event.has_value("watchguard_firebox.log.ctl_dst_ip") };
            if _cond {
                event.append_unique("destination.ip", json!(event.get("watchguard_firebox.log.ctl_dst_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ctl_dst_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.ctl_dst_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ctl_dst_port") };
            if _cond {
                event.append_unique("destination.port", json!(event.get("watchguard_firebox.log.ctl_dst_port").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.ctl_src") {
                if let Some(input) = event.get_string("watchguard_firebox.log.ctl_src") {
                    // Grok pattern: ^%{IP:watchguard_firebox.log.ctl_src_ip}:%{POSINT:watchguard_firebox.log.ctl_src_port:long}$
                    // Grok pattern: ^%{IP:watchguard_firebox.log.ctl_src_ip}$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{IP:watchguard_firebox.log.ctl_src_ip}:%{POSINT:watchguard_firebox.log.ctl_src_port:long}$"),
                            cached_grok!("^%{IP:watchguard_firebox.log.ctl_src_ip}$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            let _cond = { event.has_value("watchguard_firebox.log.ctl_src_ip") };
            if _cond {
                event.append_unique("source.ip", json!(event.get("watchguard_firebox.log.ctl_src_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ctl_src_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.ctl_src_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ctl_src_port") };
            if _cond {
                event.append_unique("source.port", json!(event.get("watchguard_firebox.log.ctl_src_port").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.dstname") {
                    event.rename("watchguard_firebox.log.dstname", "watchguard_firebox.log.destination_name")?;
                }

            let _cond = { event.has_value("watchguard_firebox.log.destination_name") };
            if _cond {
                event.append_unique("destination.domain", json!(event.get("watchguard_firebox.log.destination_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.destination_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dst_user") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.dst_user") {
                if let Some(input) = event.get_string("watchguard_firebox.log.dst_user") {
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.destination_user_domain}\\\\%{USERNAME:watchguard_firebox.log.destination_user}$
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.destination_user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.destination_user}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.destination_user}@%{HOSTNAME:watchguard_firebox.log.destination_user_domain}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.destination_user}$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.destination_user_domain}\\\\%{USERNAME:watchguard_firebox.log.destination_user}$"),
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.destination_user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.destination_user}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.destination_user}@%{HOSTNAME:watchguard_firebox.log.destination_user_domain}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.destination_user}$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            if let Some(v) = event.get("watchguard_firebox.log.destination_user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.user.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_user_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.user.domain", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.destination_user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.destination_user_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.destination_user_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dlp_rule") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("watchguard_firebox.log.dlp_rule").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.question") {
                    event.rename("watchguard_firebox.log.question", "watchguard_firebox.log.dns_question")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.dns_question").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.name", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.dns_question") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.dns_question").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.domain").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.duration") {
                if let Some(input) = event.get_string("watchguard_firebox.log.duration") {
                    // Grok pattern: %{NUMBER:watchguard_firebox.log.duration}
                    if !cached_grok!("%{NUMBER:watchguard_firebox.log.duration}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.duration") {
                if let Some(val) = event.get("watchguard_firebox.log.duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.duration".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_duration_to_long")?;
                        if event.remove("watchguard_firebox.log.duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.duration".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("watchguard_firebox.log.duration") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: if (ctx.event == null) {\n  HashMap hm = new HashMap();\n  ctx.put('event', hm);\n} ctx.event.duration = ctx.watchguard_firebox.log.duration * 1000000000;
                scale_field(event, &ScaleField::new("watchguard_firebox.log.duration", "event.duration", Factor::Long(1000000000)));
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.email_len") {
                if let Some(val) = event.get("watchguard_firebox.log.email_len") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.email_len".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.email_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_email_len_to_long")?;
                        if event.remove("watchguard_firebox.log.email_len").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.email_len".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("watchguard_firebox.log.type") {
                    event.rename("watchguard_firebox.log.type", "watchguard_firebox.log.encoding_type")?;
                }

                if event.has_value("watchguard_firebox.log.file") {
                    event.rename("watchguard_firebox.log.file", "watchguard_firebox.log.file_name")?;
                }

                if event.has_value("watchguard_firebox.log.filename") {
                    event.rename("watchguard_firebox.log.filename", "watchguard_firebox.log.file_name")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.name", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.from") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.from").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("watchguard_firebox.log.geo_dst") {
                    event.rename("watchguard_firebox.log.geo_dst", "watchguard_firebox.log.geo_destination")?;
                }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.headers_size") {
                if let Some(val) = event.get("watchguard_firebox.log.headers_size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.headers_size".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.headers_size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_headers_size_to_long")?;
                        if event.remove("watchguard_firebox.log.headers_size").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.headers_size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("watchguard_firebox.log.host") && event.get_str("watchguard_firebox.log.host") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.host") {
                if let Some(val) = event.get("watchguard_firebox.log.host") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.host".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.host_dest_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_host_to_ip")?;
                        if event.has_value("watchguard_firebox.log.host") {
                            event.rename("watchguard_firebox.log.host", "watchguard_firebox.log.host_dest_domain")?;
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.host_dest_domain") };
            if _cond {
                event.append_unique("destination.domain", json!(event.get("watchguard_firebox.log.host_dest_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.host_dest_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.host_dest_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.host_dest_ip") };
            if _cond {
                event.append_unique("destination.ip", json!(event.get("watchguard_firebox.log.host_dest_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.host_dest_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.host_dest_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.hostname") };
            if _cond {
                event.append_unique("destination.domain", json!(event.get("watchguard_firebox.log.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ipaddress") && event.get_str("watchguard_firebox.log.ipaddress") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.ipaddress") {
                if let Some(val) = event.get("watchguard_firebox.log.ipaddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.ipaddress".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ipaddress_to_ip")?;
                        if event.remove("watchguard_firebox.log.ipaddress").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.ipaddress".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.ip_address") };
            if _cond {
                event.append_unique("destination.ip", json!(event.get("watchguard_firebox.log.ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.ip_address").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.length") {
                if let Some(val) = event.get("watchguard_firebox.log.length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.length".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_length_to_long")?;
                        if event.remove("watchguard_firebox.log.length").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.length".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.line_length") {
                if let Some(val) = event.get("watchguard_firebox.log.line_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.line_length".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.line_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_line_length_to_long")?;
                        if event.remove("watchguard_firebox.log.line_length").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.line_length".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("watchguard_firebox.log.md5").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.method").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.method", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.num_recipients") {
                if let Some(val) = event.get("watchguard_firebox.log.num_recipients") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.num_recipients".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.number_of_recipients", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_num_recipients_to_long")?;
                        if event.remove("watchguard_firebox.log.num_recipients").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.num_recipients".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.op").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.method", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.rcvd_pkts") {
                if let Some(val) = event.get("watchguard_firebox.log.rcvd_pkts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.rcvd_pkts".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.packets_in", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_rcvd_pkts_to_long")?;
                        if event.remove("watchguard_firebox.log.rcvd_pkts").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.rcvd_pkts".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.packets_in").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.packets", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.sent_pkts") {
                if let Some(val) = event.get("watchguard_firebox.log.sent_pkts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.sent_pkts".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.packets_out", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_watchguard_firebox_log_sent_pkts_to_watchguard_firebox_log_packets_out_50ffcf36")?;
                        if event.remove("watchguard_firebox.log.sent_pkts").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.sent_pkts".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.packets_out").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.packets", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.policy_name") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("watchguard_firebox.log.policy_name").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.port") {
                if let Some(val) = event.get("watchguard_firebox.log.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_port_to_long")?;
                        if event.remove("watchguard_firebox.log.port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.port".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("watchguard_firebox.log.port") };
            if _cond {
                event.append_unique("destination.port", json!(event.get("watchguard_firebox.log.port").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.query_class").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.class", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.query_opcode").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.op_code", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.query_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.type", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.record_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("dns.question.type", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.recipients") };
            if _cond {
                event.append_unique("email.to.address", json!(event.get("watchguard_firebox.log.recipients").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.recipients") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.recipients").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.reputation") {
                if let Some(val) = event.get("watchguard_firebox.log.reputation") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.reputation".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.reputation", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_reputation_to_long")?;
                        if event.remove("watchguard_firebox.log.reputation").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.reputation".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.response") {
                if let Some(val) = event.get("watchguard_firebox.log.response") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.response".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.response_code", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_response_to_long")?;
                        if event.remove("watchguard_firebox.log.response").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.response".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.response_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.status_code", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.response_size") {
                if let Some(val) = event.get("watchguard_firebox.log.response_size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.response_size".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.response_size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_response_size_to_long")?;
                        if event.remove("watchguard_firebox.log.response_size").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.response_size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.response_size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.response.body.bytes", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.rule_name") };
            if _cond {
                event.append_unique("rule.name", json!(event.get("watchguard_firebox.log.rule_name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.sender").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.sender.address", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.sender") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.sender").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.sensor") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.sensor").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.service").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("service.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.severity") {
                if let Some(val) = event.get("watchguard_firebox.log.severity") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.severity".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.severity", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_severity_to_long")?;
                        if event.remove("watchguard_firebox.log.severity").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.severity".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.severity", v)?;
            }

                if event.has_value("watchguard_firebox.log.signature_cat") {
                    event.rename("watchguard_firebox.log.signature_cat", "watchguard_firebox.log.signature_category")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.signature_category").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.category", v)?;
            }

                if event.has_value("watchguard_firebox.log.sig_vers") {
                    event.rename("watchguard_firebox.log.sig_vers", "watchguard_firebox.log.signature_version")?;
                }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.size") {
                if let Some(val) = event.get("watchguard_firebox.log.size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.size".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_size_to_long")?;
                        if event.remove("watchguard_firebox.log.size").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.sni").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("tls.client.server_name", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.sni") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.sni").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.src_user") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.src_user") {
                if let Some(input) = event.get_string("watchguard_firebox.log.src_user") {
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.source_user_domain}\\\\%{USERNAME:watchguard_firebox.log.source_user}$
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.source_user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.source_user}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.source_user}@%{HOSTNAME:watchguard_firebox.log.source_user_domain}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.source_user}$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.source_user_domain}\\\\%{USERNAME:watchguard_firebox.log.source_user}$"),
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.source_user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.source_user}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.source_user}@%{HOSTNAME:watchguard_firebox.log.source_user_domain}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.source_user}$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            if let Some(v) = event.get("watchguard_firebox.log.source_user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.user.name", v)?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_user_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.user.domain", v)?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.source_user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_user_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.source_user_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.srv_ip") && event.get_str("watchguard_firebox.log.srv_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.srv_ip") {
                if let Some(val) = event.get("watchguard_firebox.log.srv_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.srv_ip".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.srv_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_srv_ip_to_ip")?;
                        if event.remove("watchguard_firebox.log.srv_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.srv_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.srv_ip") };
            if _cond {
            if let Some(v) = event.get("watchguard_firebox.log.srv_ip").cloned() {
                event.set("destination.ip", v)?;
            }
            }

            let _cond = { event.has_value("watchguard_firebox.log.srv_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("watchguard_firebox.log.srv_ip").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.srv_port") {
                if let Some(val) = event.get("watchguard_firebox.log.srv_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.srv_port".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.srv_port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_srv_port_to_long")?;
                        if event.remove("watchguard_firebox.log.srv_port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.srv_port".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("watchguard_firebox.log.srv_port") };
            if _cond {
            if let Some(v) = event.get("watchguard_firebox.log.srv_port").cloned() {
                event.set("destination.port", v)?;
            }
            }

                if event.has_value("watchguard_firebox.log.subj_tag") {
                    event.rename("watchguard_firebox.log.subj_tag", "watchguard_firebox.log.tag")?;
                }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.timeout") {
                if let Some(val) = event.get("watchguard_firebox.log.timeout") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "watchguard_firebox.log.timeout".into(),
                            message,
                        })?;
                    event.set("watchguard_firebox.log.timeout", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_timeout_to_long")?;
                        if event.remove("watchguard_firebox.log.timeout").is_none() {
                            return Err(TransformError::FieldNotFound { path: "watchguard_firebox.log.timeout".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("watchguard_firebox.log.to") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.to").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("watchguard_firebox.log.user") {
                if let Some(input) = event.get_string("watchguard_firebox.log.user") {
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\%{USERNAME:watchguard_firebox.log.user_name}$
                    // Grok pattern: ^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.user_name}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.user_name}@%{HOSTNAME:watchguard_firebox.log.user_domain}$
                    // Grok pattern: ^%{USERNAME:watchguard_firebox.log.user_name}$
                    // Grok pattern: ^%{GREEDYDATA:watchguard_firebox.log.body}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{HOSTNAME:watchguard_firebox.log.user_domain}\\\\\\\\%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.user_name}@%{HOSTNAME:watchguard_firebox.log.user_domain}$"),
                            cached_grok!("^%{USERNAME:watchguard_firebox.log.user_name}$"),
                            cached_grok!("^%{GREEDYDATA:watchguard_firebox.log.body}$"),
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

            let _cond = { event.has_value("watchguard_firebox.log.user_name") };
            if _cond {
                event.append_unique("user.name", json!(event.get("watchguard_firebox.log.user_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("watchguard_firebox.log.user_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_domain") };
            if _cond {
                event.append_unique("user.domain", json!(event.get("watchguard_firebox.log.user_domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("watchguard_firebox.log.user_domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("watchguard_firebox.log.user_domain").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("watchguard_firebox.log.virus").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.name", v)?;
            }

                if event.has_value("watchguard_firebox.log.From-header") {
                    event.rename("watchguard_firebox.log.From-header", "watchguard_firebox.log.from_header")?;
                }

                if event.has_value("watchguard_firebox.log.To-header") {
                    event.rename("watchguard_firebox.log.To-header", "watchguard_firebox.log.to_header")?;
                }

                if event.has_value("watchguard_firebox.log.Protocol") {
                    event.rename("watchguard_firebox.log.Protocol", "watchguard_firebox.log.protocol")?;
                }

            if let Some(v) = event.get("watchguard_firebox.log.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("source.port") {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("destination.port") {
                if let Some(val) = event.get("destination.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "destination.port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("watchguard_firebox.log.source_ip") && event.has_value("watchguard_firebox.log.destination_ip") && event.has_value("watchguard_firebox.log.source_port") && event.get_i64("watchguard_firebox.log.source_port") != Some(0) && event.has_value("watchguard_firebox.log.destination_port") && event.get_i64("watchguard_firebox.log.destination_port") != Some(0) };
            if _cond {
            if event.has_value("watchguard_firebox.log.source_ip") {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("watchguard_firebox.log.source_ip"),
                    event.get_string("watchguard_firebox.log.destination_ip"),
                    event.get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("watchguard_firebox.log.source_port", "watchguard_firebox.log.destination_port")
                    };
                    let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => return Err(TransformError::ParseError {
                            path: "network.community_id".into(),
                            message,
                        }),
                    }
                }
            }
            }

            if event.has_value("watchguard_firebox.log.source_ip") {
                if let Some(ip_str) = event.get_string("watchguard_firebox.log.source_ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("watchguard_firebox.log.source_ip_geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("watchguard_firebox.log.source_ip_geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("watchguard_firebox.log.source_ip_geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("watchguard_firebox.log.source_ip_geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("watchguard_firebox.log.source_ip_geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("watchguard_firebox.log.source_ip_geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("watchguard_firebox.log.source_ip_geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("watchguard_firebox.log.source_ip_geo.location", v.clone())?;
                        }
                    }
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.source_ip_geo").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo", v)?;
            }

            if event.has_value("watchguard_firebox.log.destination_ip") {
                if let Some(ip_str) = event.get_string("watchguard_firebox.log.destination_ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("watchguard_firebox.log.destination_ip_geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("watchguard_firebox.log.destination_ip_geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("watchguard_firebox.log.destination_ip_geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("watchguard_firebox.log.destination_ip_geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("watchguard_firebox.log.destination_ip_geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("watchguard_firebox.log.destination_ip_geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("watchguard_firebox.log.destination_ip_geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("watchguard_firebox.log.destination_ip_geo.location", v.clone())?;
                        }
                    }
                }
            }

            if let Some(v) = event.get("watchguard_firebox.log.destination_ip_geo").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo", v)?;
            }

                event.remove("_temp");
                event.remove("watchguard_firebox.log.rcvd_bytes");
                event.remove("watchguard_firebox.log.sent_bytes");
                event.remove("watchguard_firebox.log.email_len");
                event.remove("watchguard_firebox.log.host");
                event.remove("watchguard_firebox.log.ipaddress");
                event.remove("watchguard_firebox.log.num_recipients");
                event.remove("watchguard_firebox.log.rcvd_pkts");
                event.remove("watchguard_firebox.log.sent_pkts");
                event.remove("watchguard_firebox.log.ctl_src");
                event.remove("watchguard_firebox.log.ctl_dst");
                event.remove("watchguard_firebox.log.user");
                event.remove("watchguard_firebox.log.src_user");
                event.remove("watchguard_firebox.log.dst_user");
                event.remove("watchguard_firebox.log.response");

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("watchguard_firebox.log.action");
                event.remove("watchguard_firebox.log.app_name");
                event.remove("watchguard_firebox.log.arg");
                event.remove("watchguard_firebox.log.authenticated_user");
                event.remove("watchguard_firebox.log.authenticated_user_domain");
                event.remove("watchguard_firebox.log.bytes_in");
                event.remove("watchguard_firebox.log.bytes_out");
                event.remove("watchguard_firebox.log.certificate_issuer");
                event.remove("watchguard_firebox.log.certificate_subject");
                event.remove("watchguard_firebox.log.client_ssl");
                event.remove("watchguard_firebox.log.cn");
                event.remove("watchguard_firebox.log.destination_name");
                event.remove("watchguard_firebox.log.destination_user");
                event.remove("watchguard_firebox.log.destination_user_domain");
                event.remove("watchguard_firebox.log.dlp_rule");
                event.remove("watchguard_firebox.log.dns_question");
                event.remove("watchguard_firebox.log.file_name");
                event.remove("watchguard_firebox.log.hostname");
                event.remove("watchguard_firebox.log.in_interface_name");
                event.remove("watchguard_firebox.log.md5");
                event.remove("watchguard_firebox.log.method");
                event.remove("watchguard_firebox.log.op");
                event.remove("watchguard_firebox.log.out_interface_name");
                event.remove("watchguard_firebox.log.packets_in");
                event.remove("watchguard_firebox.log.packets_out");
                event.remove("watchguard_firebox.log.path");
                event.remove("watchguard_firebox.log.protocol");
                event.remove("watchguard_firebox.log.query_class");
                event.remove("watchguard_firebox.log.query_opcode");
                event.remove("watchguard_firebox.log.query_type");
                event.remove("watchguard_firebox.log.reason");
                event.remove("watchguard_firebox.log.recipients");
                event.remove("watchguard_firebox.log.record_type");
                event.remove("watchguard_firebox.log.response_code");
                event.remove("watchguard_firebox.log.response_size");
                event.remove("watchguard_firebox.log.sender");
                event.remove("watchguard_firebox.log.service");
                event.remove("watchguard_firebox.log.severity");
                event.remove("watchguard_firebox.log.signature_category");
                event.remove("watchguard_firebox.log.size");
                event.remove("watchguard_firebox.log.sni");
                event.remove("watchguard_firebox.log.source_user");
                event.remove("watchguard_firebox.log.source_user_domain");
                event.remove("watchguard_firebox.log.user_domain");
                event.remove("watchguard_firebox.log.user_name");
                event.remove("watchguard_firebox.log.virus");
                event.remove("watchguard_firebox.log.source_ip_geo.city_name");
                event.remove("watchguard_firebox.log.source_ip_geo.continent_name");
                event.remove("watchguard_firebox.log.source_ip_geo.country_iso_code");
                event.remove("watchguard_firebox.log.source_ip_geo.country_name");
                event.remove("watchguard_firebox.log.source_ip_geo.location");
                event.remove("watchguard_firebox.log.source_ip_geo.region_iso_code");
                event.remove("watchguard_firebox.log.source_ip_geo.region_name");
                event.remove("watchguard_firebox.log.destination_ip_geo.city_name");
                event.remove("watchguard_firebox.log.destination_ip_geo.continent_name");
                event.remove("watchguard_firebox.log.destination_ip_geo.country_iso_code");
                event.remove("watchguard_firebox.log.destination_ip_geo.country_name");
                event.remove("watchguard_firebox.log.destination_ip_geo.location");
                event.remove("watchguard_firebox.log.destination_ip_geo.region_iso_code");
                event.remove("watchguard_firebox.log.destination_ip_geo.region_name");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
