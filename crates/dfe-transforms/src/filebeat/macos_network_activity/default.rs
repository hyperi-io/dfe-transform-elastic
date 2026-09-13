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
            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

            // Begin nested pipeline: "common-pipeline"
            event.set("ecs.version", json!("9.2.0"))?;
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
            parse_json_field(event, "event.original", "json")?;
            event.set("event.kind", json!("event"))?;
            if event.has_value("json.activityIdentifier") {
                if let Some(val) = event.get("json.activityIdentifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.activityIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set("macos.activity_identifier", converted)?;
                }
            }
            let _cond = {
                event
                    .get("json.backtrace.frames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.backtrace.frames", |event| {
                    if event.has_value("_ingest._value.imageOffset") {
                        if let Some(val) = event.get("_ingest._value.imageOffset") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.imageOffset".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.image.offset", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }
            let _cond = {
                event
                    .get("json.backtrace.frames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.backtrace.frames", |event| {
                    event.remove("_ingest._value.imageOffset");
                    Ok(())
                })?;
            }
            let _cond = {
                event
                    .get("json.backtrace.frames")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.backtrace.frames", |event| {
                    if event.has_value("_ingest._value.imageUUID") {
                        event.rename("_ingest._value.imageUUID", "_ingest._value.image.uuid")?;
                    }
                    Ok(())
                })?;
            }
            if event.has_value("json.backtrace.frames") {
                event.rename("json.backtrace.frames", "macos.backtrace.frames")?;
            }
            if event.has_value("json.bootUUID") {
                event.rename("json.bootUUID", "macos.boot_uuid")?;
            }
            if event.has_value("json.category") {
                event.rename("json.category", "macos.category")?;
            }
            if event.has_value("json.eventMessage") {
                event.rename("json.eventMessage", "macos.event.message.description")?;
            }
            if let Some(v) = event
                .get("macos.event.message.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }
            if event.has_value("json.eventType") {
                event.rename("json.eventType", "macos.event.type")?;
            }
            if event.has_value("json.formatString") {
                event.rename("json.formatString", "macos.format_string")?;
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.machTimestamp") {
                    if let Some(val) = event.get("json.machTimestamp") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.machTimestamp".into(),
                                message,
                            }
                        })?;
                        event.set("macos.mach_timestamp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_machTimestamp_to_string",
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
            let _cond = { event.has_value("json.messageType") };
            if _cond {
                // Painless script
                // Source: ctx.log = ctx.log ?: [:];\nctx.log.put(\"level\", params.get(ctx.json.messageType.toLowerCase()));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.log = ctx.log ?: [:];\nctx.log.put(\"level\", params.get(ctx.json.messageType.toLowerCase()));"#
                    ),
                    cached_params!(
                        "{\"default\":\"info\",\"error\":\"error\",\"debug\":\"debug\",\"info\":\"info\",\"fault\":\"warning\"}"
                    ),
                )?;
            }
            if event.has_value("json.parentActivityIdentifier") {
                if let Some(val) = event.get("json.parentActivityIdentifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.parentActivityIdentifier".into(),
                            message,
                        }
                    })?;
                    event.set("macos.parent_activity_identifier", converted)?;
                }
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.processID") {
                    if let Some(val) = event.get("json.processID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.processID".into(),
                                message,
                            }
                        })?;
                        event.set("json.processID", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_processID_to_string",
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
            if let Some(v) = event
                .get("json.processID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }
            if event.has_value("json.processImagePath") {
                event.rename("json.processImagePath", "macos.process.image_path")?;
            }
            if event.has_value("json.processImageUUID") {
                event.rename("json.processImageUUID", "macos.process.image_uuid")?;
            }
            if event.has_value("json.senderImagePath") {
                event.rename("json.senderImagePath", "macos.sender.image_path")?;
            }
            if event.has_value("json.senderImageUUID") {
                event.rename("json.senderImageUUID", "macos.sender.image_uuid")?;
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.senderProgramCounter") {
                    if let Some(val) = event.get("json.senderProgramCounter") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.senderProgramCounter".into(),
                                message,
                            }
                        })?;
                        event.set("macos.sender.program_counter", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_senderProgramCounter_to_long",
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
            if event.has_value("json.source") {
                event.rename("json.source", "macos.source")?;
            }
            if event.has_value("json.subsystem") {
                event.rename("json.subsystem", "macos.subsystem")?;
            }
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threadID") {
                    if let Some(val) = event.get("json.threadID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threadID".into(),
                                message,
                            }
                        })?;
                        event.set("json.threadID", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_threadID_to_long",
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
            if let Some(v) = event
                .get("json.threadID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.thread.id", v)?;
            }
            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSZ",
                                "yyyy-MM-dd HH:mm:ss.SSSSSSZ",
                                "yyyy-MM-dd",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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
            if event.has_value("json.timezoneName") {
                event.rename("json.timezoneName", "macos.timezone_name")?;
            }
            if event.has_value("json.traceID") {
                if let Some(val) = event.get("json.traceID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.traceID".into(),
                            message,
                        }
                    })?;
                    event.set("macos.trace_id", converted)?;
                }
            }
            if event.has_value("json.userID") {
                if let Some(val) = event.get("json.userID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.userID".into(),
                            message,
                        }
                    })?;
                    event.set("json.userID", converted)?;
                }
            }
            if let Some(v) = event
                .get("json.userID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }
            let _cond = { event.has_value("json.userID") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.userID")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            event.remove("json");
            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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
            // End nested pipeline: "common-pipeline"

            if event.has_value("macos.event.message.description") {
                if let Some(input) = event.get_string("macos.event.message.description") {
                    // Grok pattern: ^\\[%{WORD}\\] %{DATA}\\:(?:%{SPACE}mach=%{WORD:macos.event.message.mach:boolean})?(?:%{SPACE}listener=%{WORD:macos.event.message.listener:boolean})?(?:%{SPACE}peer=%{WORD:macos.event.message.peer:boolean})?(?:%{SPACE}name=%{GREEDYDATA:macos.event.message.name})?
                    // Grok pattern: ^%{WORD} \\[%{DATA}\\](?:%{SPACE}flags=\\[%{DATA:macos.event.message.flags}\\])?(?:%{SPACE}seq=%{DATA:macos.event.message.seq},)?(?:%{SPACE}ack=%{DATA:macos.event.message.ack},)?(?:%{SPACE}win=%{DATA:macos.event.message.win})?(?:%{SPACE}state=%{DATA:macos.event.message.state})?(?:%{SPACE}rcv_nxt=%{DATA:macos.event.message.rcv_nxt},)?(?:snd_una=%{DATA:macos.event.message.snd_una})
                    // Grok pattern: ^nw_protocol_boringssl_signal_connected\\(%{NUMBER}\\) \\[%{DATA:macos.event.message.connection_identifier}\\]\\[%{DATA}\\] TLS connected \\[(?:version\\(%{DATA:macos.event.message.tls_version}\\))?(?:%{SPACE}ciphersuite\\(%{DATA:macos.event.message.cipher_suite}\\))?(?:%{SPACE}group\\(%{DATA:macos.event.message.group}\\))?(?:%{SPACE}signature_alg\\(%{DATA:process.code_signature.digest_algorithm}\\))?(?:%{SPACE}alpn\\(%{DATA:macos.event.message.alpn}\\))?(?:%{SPACE}resumed\\(%{DATA:macos.event.message.resumed}\\))?(?:%{SPACE}offered_ticket\\(%{DATA:macos.event.message.offered_ticket}\\))?(?:%{SPACE}false_started\\(%{DATA:macos.event.message.false_started}\\))?(?:%{SPACE}ocsp_received\\(%{DATA:macos.event.message.ocsp_received}\\))?(?:%{SPACE}sct_received\\(%{DATA:macos.event.message.sct_received}\\))?(?:%{SPACE}connect_time\\(%{DATA:macos.event.message.connection_time}\\))?(?:%{SPACE}flight_time\\(%{DATA:macos.event.message.flight_time}\\))?(?:%{SPACE}rtt\\(%{DATA:macos.event.message.rtt}\\))?(?:%{SPACE}write_stalls\\(%{DATA:macos.event.message.write_stalls:long}\\))?(?:%{SPACE}read_stalls\\(%{DATA:macos.event.message.read_stalls:long}\\))?(?:%{SPACE}pake\\(%{DATA:macos.event.message.pake}\\))?\\]
                    // Grok pattern: ^Task \\<%{DATA:macos.event.message.task_uid}\\>.\\<%{NUMBER}\\>%{SPACE}summary for %{DATA} \\{(?:transaction_duration_ms=%{NUMBER:macos.event.message.transaction_duration_ms:long},)?(?:%{SPACE}response_status=%{NUMBER:http.response.status_code:long},)?(?:%{SPACE}connection=%{NUMBER:macos.event.message.connection:long},)?(?:%{SPACE}protocol=%{DATA:macos.event.message.protocol},)?(?:%{SPACE}domain_lookup_duration_ms=%{NUMBER:macos.event.message.domain_lookup_duration_ms:long},)?(?:%{SPACE}connect_duration_ms=%{NUMBER:macos.event.message.connection_duration_ms:long},)?(?:%{SPACE}secure_connection_duration_ms=%{NUMBER:macos.event.message.secure_connection_duration_ms:long},)?(?:%{SPACE}private_relay=%{WORD:macos.event.message.private_relay:boolean},)?(?:%{SPACE}request_start_ms=%{NUMBER:macos.event.message.request_start_ms:long},)?(?:%{SPACE}request_duration_ms=%{NUMBER:macos.event.message.request_duration_ms:long},)?(?:%{SPACE}response_start_ms=%{NUMBER:macos.event.message.response_start_ms:long},)?(?:%{SPACE}response_duration_ms=%{NUMBER:macos.event.message.response_duration_ms:long},)?(?:%{SPACE}request_bytes=%{NUMBER:http.request.bytes:long},)?(?:%{SPACE}response_bytes=%{NUMBER:http.response.bytes:long},)?(?:%{SPACE}cache_hit=%{WORD:macos.event.message.cache_hit:boolean})?\\}
                    // Grok pattern: ^%{DATA} \\[%{DATA:macos.event.message.connection_identifier}\\]%{SPACE}\\[%{UUID:macos.event.message.connection_uuid} <private>:%{NUMBER:source.port:long}<-><private>:%{NUMBER:destination.port:long}\\]%{SPACE}Init: %{NUMBER:macos.event.message.init_flag:long}, Conn_Time: %{DATA:macos.event.message.connection_time}, SYNs: %{NUMBER:macos.event.message.syns:long}, WR_T: %{DATA:macos.event.message.wr_t_in_out}, RD_T: %{DATA:macos.event.message.rd_t_in_out}, TFO: %{DATA:macos.event.message.tfo_in_out_miss}, ECN: %{DATA:macos.event.message.ecn_in_out_miss}, Accurate ECN %{GREEDYDATA}: %{GREEDYDATA:macos.event.message.accurate_ecn}, TS: %{NUMBER:macos.event.message.timestamp_enabled:long}, TSO: %{NUMBER:macos.event.message.tso_enabled:long}%{SPACE}rtt_cache: %{DATA:macos.event.message.rtt_cache}, rtt_upd: %{NUMBER:macos.event.message.rtt_updates:long}, rtt: %{DATA:macos.event.message.rtt}, rtt_var: %{DATA:macos.event.message.rtt_var_ms} rtt_nc: %{DATA:macos.event.message.rtt_nc_ms}, rtt_var_nc: %{DATA:macos.event.message.rtt_var_nc_ms} base rtt: %{GREEDYDATA:macos.event.message.base_rtt_ms}%{SPACE}ACKs-compressed: %{NUMBER:macos.event.message.acks_compressed:long}, ACKs delayed: %{NUMBER:macos.event.message.acks_delayed:long} delayed ACKs sent: %{NUMBER:macos.event.message.delayed_acks_sent:long}
                    // Grok pattern: ^\\[C%{NUMBER:macos.event.message.connection_id} %{UUID:macos.event.message.session_uuid} (Hostname\\#)?%{DATA:host.id}:%{NUMBER:macos.event.message.hostname_port:long} %{DATA}(, bundle id: %{DATA:macos.event.message.bundle_id})?(, pid: %{DATA:process.pid:long})?(, account id: %{DATA:macos.event.message.account_id})?(, url: %{DATA:url.original})?(, url hash: %{BASE16NUM:macos.event.message.url_hash})?(, traffic class: %{NUMBER:macos.event.message.traffic_class})?(, expected workload: %{NUMBER:macos.event.message.expected_workload})?(, %{GREEDYDATA})?, attribution: %{DATA:macos.event.message.attribution}(, %{GREEDYDATA})?\\] cancelled\\n\\t\\[C%{DATA:macos.event.message.connection_detail} %{UUID:macos.event.message.connection_uuid} %{IP:source.ip}:%{NUMBER:source.port:long}<->(IPv4#)?%{DATA:macos.event.message.server_id}:%{NUMBER:destination.port:long}\\]\\n\\tConnected Path: %{DATA:macos.event.message.path_status}(, %{DATA})?(, interface: %{DATA:macos.event.message.interface})?(, %{GREEDYDATA})?\\n\\tPrivacy Stance: %{DATA:macos.event.message.privacy_stance}\\n\\tDuration: %{DATA:macos.event.message.duration}, DNS @%{DATA:macos.event.message.dns_start} took %{DATA:macos.event.message.dns_duration}, TCP @%{DATA:macos.event.message.tcp_start} took %{DATA:macos.event.message.tcp_duration}, TLS %{DATA:macos.event.message.tls_version} took %{DATA:macos.event.message.tls_duration}\\n\\tbytes in\\/out: %{NUMBER:source.bytes:long}\\/%{NUMBER:destination.bytes:long}, packets in\\/out: %{NUMBER:source.packets:long}\\/%{NUMBER:destination.packets:long}, rtt: %{DATA:macos.event.message.rtt}, retransmitted bytes: %{NUMBER:macos.event.message.retransmitted_bytes:long}, out-of-order bytes: %{NUMBER:macos.event.message.out_of_order_bytes:long}\\n\\tecn packets sent\\/acked\\/marked\\/lost: %{NUMBER:macos.event.message.ecn_sent:long}\\/%{NUMBER:macos.event.message.ecn_acked:long}\\/%{NUMBER:macos.event.message.ecn_marked:long}\\/%{NUMBER:macos.event.message.ecn_lost:long}$
                    // Grok pattern: %{GREEDYDATA:macos.event.message.original}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^\\[%{WORD}\\] %{DATA}\\:(?:%{SPACE}mach=%{WORD:macos.event.message.mach:boolean})?(?:%{SPACE}listener=%{WORD:macos.event.message.listener:boolean})?(?:%{SPACE}peer=%{WORD:macos.event.message.peer:boolean})?(?:%{SPACE}name=%{GREEDYDATA:macos.event.message.name})?"
                            ),
                            cached_grok!(
                                "^%{WORD} \\[%{DATA}\\](?:%{SPACE}flags=\\[%{DATA:macos.event.message.flags}\\])?(?:%{SPACE}seq=%{DATA:macos.event.message.seq},)?(?:%{SPACE}ack=%{DATA:macos.event.message.ack},)?(?:%{SPACE}win=%{DATA:macos.event.message.win})?(?:%{SPACE}state=%{DATA:macos.event.message.state})?(?:%{SPACE}rcv_nxt=%{DATA:macos.event.message.rcv_nxt},)?(?:snd_una=%{DATA:macos.event.message.snd_una})"
                            ),
                            cached_grok!(
                                "^nw_protocol_boringssl_signal_connected\\(%{NUMBER}\\) \\[%{DATA:macos.event.message.connection_identifier}\\]\\[%{DATA}\\] TLS connected \\[(?:version\\(%{DATA:macos.event.message.tls_version}\\))?(?:%{SPACE}ciphersuite\\(%{DATA:macos.event.message.cipher_suite}\\))?(?:%{SPACE}group\\(%{DATA:macos.event.message.group}\\))?(?:%{SPACE}signature_alg\\(%{DATA:process.code_signature.digest_algorithm}\\))?(?:%{SPACE}alpn\\(%{DATA:macos.event.message.alpn}\\))?(?:%{SPACE}resumed\\(%{DATA:macos.event.message.resumed}\\))?(?:%{SPACE}offered_ticket\\(%{DATA:macos.event.message.offered_ticket}\\))?(?:%{SPACE}false_started\\(%{DATA:macos.event.message.false_started}\\))?(?:%{SPACE}ocsp_received\\(%{DATA:macos.event.message.ocsp_received}\\))?(?:%{SPACE}sct_received\\(%{DATA:macos.event.message.sct_received}\\))?(?:%{SPACE}connect_time\\(%{DATA:macos.event.message.connection_time}\\))?(?:%{SPACE}flight_time\\(%{DATA:macos.event.message.flight_time}\\))?(?:%{SPACE}rtt\\(%{DATA:macos.event.message.rtt}\\))?(?:%{SPACE}write_stalls\\(%{DATA:macos.event.message.write_stalls:long}\\))?(?:%{SPACE}read_stalls\\(%{DATA:macos.event.message.read_stalls:long}\\))?(?:%{SPACE}pake\\(%{DATA:macos.event.message.pake}\\))?\\]"
                            ),
                            cached_grok!(
                                "^Task \\<%{DATA:macos.event.message.task_uid}\\>.\\<%{NUMBER}\\>%{SPACE}summary for %{DATA} \\{(?:transaction_duration_ms=%{NUMBER:macos.event.message.transaction_duration_ms:long},)?(?:%{SPACE}response_status=%{NUMBER:http.response.status_code:long},)?(?:%{SPACE}connection=%{NUMBER:macos.event.message.connection:long},)?(?:%{SPACE}protocol=%{DATA:macos.event.message.protocol},)?(?:%{SPACE}domain_lookup_duration_ms=%{NUMBER:macos.event.message.domain_lookup_duration_ms:long},)?(?:%{SPACE}connect_duration_ms=%{NUMBER:macos.event.message.connection_duration_ms:long},)?(?:%{SPACE}secure_connection_duration_ms=%{NUMBER:macos.event.message.secure_connection_duration_ms:long},)?(?:%{SPACE}private_relay=%{WORD:macos.event.message.private_relay:boolean},)?(?:%{SPACE}request_start_ms=%{NUMBER:macos.event.message.request_start_ms:long},)?(?:%{SPACE}request_duration_ms=%{NUMBER:macos.event.message.request_duration_ms:long},)?(?:%{SPACE}response_start_ms=%{NUMBER:macos.event.message.response_start_ms:long},)?(?:%{SPACE}response_duration_ms=%{NUMBER:macos.event.message.response_duration_ms:long},)?(?:%{SPACE}request_bytes=%{NUMBER:http.request.bytes:long},)?(?:%{SPACE}response_bytes=%{NUMBER:http.response.bytes:long},)?(?:%{SPACE}cache_hit=%{WORD:macos.event.message.cache_hit:boolean})?\\}"
                            ),
                            cached_grok!(
                                "^%{DATA} \\[%{DATA:macos.event.message.connection_identifier}\\]%{SPACE}\\[%{UUID:macos.event.message.connection_uuid} <private>:%{NUMBER:source.port:long}<-><private>:%{NUMBER:destination.port:long}\\]%{SPACE}Init: %{NUMBER:macos.event.message.init_flag:long}, Conn_Time: %{DATA:macos.event.message.connection_time}, SYNs: %{NUMBER:macos.event.message.syns:long}, WR_T: %{DATA:macos.event.message.wr_t_in_out}, RD_T: %{DATA:macos.event.message.rd_t_in_out}, TFO: %{DATA:macos.event.message.tfo_in_out_miss}, ECN: %{DATA:macos.event.message.ecn_in_out_miss}, Accurate ECN %{GREEDYDATA}: %{GREEDYDATA:macos.event.message.accurate_ecn}, TS: %{NUMBER:macos.event.message.timestamp_enabled:long}, TSO: %{NUMBER:macos.event.message.tso_enabled:long}%{SPACE}rtt_cache: %{DATA:macos.event.message.rtt_cache}, rtt_upd: %{NUMBER:macos.event.message.rtt_updates:long}, rtt: %{DATA:macos.event.message.rtt}, rtt_var: %{DATA:macos.event.message.rtt_var_ms} rtt_nc: %{DATA:macos.event.message.rtt_nc_ms}, rtt_var_nc: %{DATA:macos.event.message.rtt_var_nc_ms} base rtt: %{GREEDYDATA:macos.event.message.base_rtt_ms}%{SPACE}ACKs-compressed: %{NUMBER:macos.event.message.acks_compressed:long}, ACKs delayed: %{NUMBER:macos.event.message.acks_delayed:long} delayed ACKs sent: %{NUMBER:macos.event.message.delayed_acks_sent:long}"
                            ),
                            cached_grok!(
                                "^\\[C%{NUMBER:macos.event.message.connection_id} %{UUID:macos.event.message.session_uuid} (Hostname\\#)?%{DATA:host.id}:%{NUMBER:macos.event.message.hostname_port:long} %{DATA}(, bundle id: %{DATA:macos.event.message.bundle_id})?(, pid: %{DATA:process.pid:long})?(, account id: %{DATA:macos.event.message.account_id})?(, url: %{DATA:url.original})?(, url hash: %{BASE16NUM:macos.event.message.url_hash})?(, traffic class: %{NUMBER:macos.event.message.traffic_class})?(, expected workload: %{NUMBER:macos.event.message.expected_workload})?(, %{GREEDYDATA})?, attribution: %{DATA:macos.event.message.attribution}(, %{GREEDYDATA})?\\] cancelled\\n\\t\\[C%{DATA:macos.event.message.connection_detail} %{UUID:macos.event.message.connection_uuid} %{IP:source.ip}:%{NUMBER:source.port:long}<->(IPv4#)?%{DATA:macos.event.message.server_id}:%{NUMBER:destination.port:long}\\]\\n\\tConnected Path: %{DATA:macos.event.message.path_status}(, %{DATA})?(, interface: %{DATA:macos.event.message.interface})?(, %{GREEDYDATA})?\\n\\tPrivacy Stance: %{DATA:macos.event.message.privacy_stance}\\n\\tDuration: %{DATA:macos.event.message.duration}, DNS @%{DATA:macos.event.message.dns_start} took %{DATA:macos.event.message.dns_duration}, TCP @%{DATA:macos.event.message.tcp_start} took %{DATA:macos.event.message.tcp_duration}, TLS %{DATA:macos.event.message.tls_version} took %{DATA:macos.event.message.tls_duration}\\n\\tbytes in\\/out: %{NUMBER:source.bytes:long}\\/%{NUMBER:destination.bytes:long}, packets in\\/out: %{NUMBER:source.packets:long}\\/%{NUMBER:destination.packets:long}, rtt: %{DATA:macos.event.message.rtt}, retransmitted bytes: %{NUMBER:macos.event.message.retransmitted_bytes:long}, out-of-order bytes: %{NUMBER:macos.event.message.out_of_order_bytes:long}\\n\\tecn packets sent\\/acked\\/marked\\/lost: %{NUMBER:macos.event.message.ecn_sent:long}\\/%{NUMBER:macos.event.message.ecn_acked:long}\\/%{NUMBER:macos.event.message.ecn_marked:long}\\/%{NUMBER:macos.event.message.ecn_lost:long}$"
                            ),
                            cached_grok!("%{GREEDYDATA:macos.event.message.original}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("source.bytes") && event.has_value("destination.bytes") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.network = new HashMap();\nctx.network.bytes = ctx.source.bytes + ctx.destination.bytes\n
                sum_directions(event, &["bytes"]);
            }

            let _cond =
                { event.has_value("source.packets") && event.has_value("destination.packets") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: if (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nctx.network.packets = ctx.source.packets + ctx.destination.packets\n
                sum_directions(event, &["packets"]);
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("url.original") {
                    if !uri_parts(event, "url.original", "url", true, false)?
                        && event
                            .get_str("url.original")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "url.original".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.id")
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

            let _cond = { event.get_str("macos.event.message.resumed") == Some("1") };
            if _cond {
                event.set("macos.event.message.resumed", json!(true))?;
            }

            let _cond = { event.get_str("macos.event.message.resumed") == Some("0") };
            if _cond {
                event.set("macos.event.message.resumed", json!(false))?;
            }

            let _cond = { event.get_str("macos.event.message.offered_ticket") == Some("1") };
            if _cond {
                event.set("macos.event.message.offered_ticket", json!(true))?;
            }

            let _cond = { event.get_str("macos.event.message.offered_ticket") == Some("0") };
            if _cond {
                event.set("macos.event.message.offered_ticket", json!(false))?;
            }

            let _cond = { event.get_str("macos.event.message.false_started") == Some("1") };
            if _cond {
                event.set("macos.event.message.false_started", json!(true))?;
            }

            let _cond = { event.get_str("macos.event.message.false_started") == Some("0") };
            if _cond {
                event.set("macos.event.message.false_started", json!(false))?;
            }

            let _cond = { event.get_str("macos.event.message.ocsp_received") == Some("1") };
            if _cond {
                event.set("macos.event.message.ocsp_received", json!(true))?;
            }

            let _cond = { event.get_str("macos.event.message.ocsp_received") == Some("0") };
            if _cond {
                event.set("macos.event.message.ocsp_received", json!(false))?;
            }

            let _cond = { event.get_str("macos.event.message.sct_received") == Some("1") };
            if _cond {
                event.set("macos.event.message.sct_received", json!(true))?;
            }

            let _cond = { event.get_str("macos.event.message.sct_received") == Some("0") };
            if _cond {
                event.set("macos.event.message.sct_received", json!(false))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.ecn_in_out_miss") {
                    if let Some(input) = event.get_string("macos.event.message.ecn_in_out_miss") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("macos.event.message.ecn_in", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("macos.event.message.ecn_out", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("macos.event.message.ecn_miss", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "macos.event.message.ecn_in_out_miss".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "dissect_ecn_in_out_miss",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.ecn_in") {
                    if let Some(val) = event.get("macos.event.message.ecn_in") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.ecn_in".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.ecn_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_ecn_in_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.ecn_out") {
                    if let Some(val) = event.get("macos.event.message.ecn_out") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.ecn_out".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.ecn_out", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_ecn_out_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.ecn_miss") {
                    if let Some(val) = event.get("macos.event.message.ecn_miss") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.ecn_miss".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.ecn_miss", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_ecn_miss_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.rd_t_in_out") {
                    if let Some(input) = event.get_string("macos.event.message.rd_t_in_out") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("macos.event.message.rd_t_in", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("macos.event.message.rd_t_out", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "macos.event.message.rd_t_in_out".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_rd_t_in_out")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.rd_t_in") {
                    if let Some(val) = event.get("macos.event.message.rd_t_in") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.rd_t_in".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.rd_t_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_rd_t_in_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.rd_t_out") {
                    if let Some(val) = event.get("macos.event.message.rd_t_out") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.rd_t_out".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.rd_t_out", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_rd_t_out_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.tfo_in_out_miss") {
                    if let Some(input) = event.get_string("macos.event.message.tfo_in_out_miss") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("macos.event.message.tfo_in", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("macos.event.message.tfo_out", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("macos.event.message.tfo_miss", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "macos.event.message.tfo_in_out_miss".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "dissect_tfo_in_out_miss",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.tfo_in") {
                    if let Some(val) = event.get("macos.event.message.tfo_in") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.tfo_in".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.tfo_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_tfo_in_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.tfo_out") {
                    if let Some(val) = event.get("macos.event.message.tfo_out") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.tfo_out".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.tfo_out", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_tfo_out_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.tfo_miss") {
                    if let Some(val) = event.get("macos.event.message.tfo_miss") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.tfo_miss".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.tfo_miss", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_tfo_miss_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.wr_t_in_out") {
                    if let Some(input) = event.get_string("macos.event.message.wr_t_in_out") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("macos.event.message.wr_t_in", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("macos.event.message.wr_t_out", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "macos.event.message.wr_t_in_out".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_wr_t_in_out")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.wr_t_in") {
                    if let Some(val) = event.get("macos.event.message.wr_t_in") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.wr_t_in".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.wr_t_in", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_wr_t_in_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.wr_t_out") {
                    if let Some(val) = event.get("macos.event.message.wr_t_out") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "macos.event.message.wr_t_out".into(),
                                message,
                            }
                        })?;
                        event.set("macos.event.message.wr_t_out", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_macos_event_message_wr_t_out_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("macos.event.message.accurate_ecn") {
                    if let Some(input) = event.get_string("macos.event.message.accurate_ecn") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push((
                                "macos.event.message.accurate_ecn_client",
                                &remaining[..pos],
                            ));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("macos.event.message.accurate_ecn_server", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "macos.event.message.accurate_ecn".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_accurate_ecn")?;
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

            event.remove("macos.event.message.original");
            event.remove("macos.event.message.ecn_in_out_miss");
            event.remove("macos.event.message.rd_t_in_out");
            event.remove("macos.event.message.tfo_in_out_miss");
            event.remove("macos.event.message.wr_t_in_out");
            event.remove("macos.event.message.accurate_ecn");

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
