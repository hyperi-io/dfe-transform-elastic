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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: \\[%{POSINT:process.pid}\\]( (?P<nats_log_timestamp>(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})))? \\[(?P<log_level>(?:(INF|DBG|WRN|ERR|FTL|TRC)))\\] %{GREEDYDATA:nats.log.info}
                    if !cached_grok_mapped!("\\[%{POSINT:process.pid}\\]( (?P<nats_log_timestamp>(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})))? \\[(?P<log_level>(?:(INF|DBG|WRN|ERR|FTL|TRC)))\\] %{GREEDYDATA:nats.log.info}", [("nats_log_timestamp", "nats.log.timestamp"), ("log_level", "log.level")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("nats.log.info") {
                if let Some(input) = event.get_string("nats.log.info") {
                    // Grok pattern: %{IPV4:client.ip}:%{POSINT:client.port} - cid:%{POSINT:nats.log.client.id} - %{GREEDYDATA:nats.log.msg.info}
                    // Grok pattern: %{GREEDYDATA:nats.log.msg.data}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{IPV4:client.ip}:%{POSINT:client.port} - cid:%{POSINT:nats.log.client.id} - %{GREEDYDATA:nats.log.msg.info}"
                            ),
                            cached_grok!("%{GREEDYDATA:nats.log.msg.data}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("nats.log.msg.info") {
                if let Some(input) = event.get_string("nats.log.msg.info") {
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) (?P<nats_log_msg_type>(?:MSG_PAYLOAD)): \\[%{GREEDYDATA:nats.log.msg.payload}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:(?:(?:PING)|(?:PONG)|(?:OK))))\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:UNSUB))\\s+%{POSINT:nats.log.msg.sid}(\\s+%{POSINT:nats.log.msg.max_messages})?\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:PUB))\\s+%{NOTSPACE:nats.log.msg.subject}(\\s+%{NOTSPACE:nats.log.msg.reply_to})?\\s+%{POSINT:nats.log.msg.bytes}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:SUB))\\s+%{NOTSPACE:nats.log.msg.subject}(\\s+%{NOTSPACE:nats.log.msg.queue_group})?\\s+%{POSINT:nats.log.msg.sid}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:MSG))\\s+%{NOTSPACE:nats.log.msg.subject}\\s+%{POSINT:nats.log.msg.sid}(\\s+%{NOTSPACE:nats.log.msg.reply_to})?\\s+%{POSINT:nats.log.msg.bytes}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:(?:(?:CONNECT)|(?:INFO))))\\s+%{GREEDYDATA:nats.log.msg.data}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:-ERROR))\\s+\\s+(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:-ERROR))\\s+\\s+%{GREEDYDATA:nats.log.msg.error\\]
                    // Grok pattern: %{GREEDYDATA:nats.log.msg.data}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) (?P<nats_log_msg_type>(?:MSG_PAYLOAD)): \\[%{GREEDYDATA:nats.log.msg.payload}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:(?:(?:PING)|(?:PONG)|(?:OK))))\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:UNSUB))\\s+%{POSINT:nats.log.msg.sid}(\\s+%{POSINT:nats.log.msg.max_messages})?\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:PUB))\\s+%{NOTSPACE:nats.log.msg.subject}(\\s+%{NOTSPACE:nats.log.msg.reply_to})?\\s+%{POSINT:nats.log.msg.bytes}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:SUB))\\s+%{NOTSPACE:nats.log.msg.subject}(\\s+%{NOTSPACE:nats.log.msg.queue_group})?\\s+%{POSINT:nats.log.msg.sid}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:MSG))\\s+%{NOTSPACE:nats.log.msg.subject}\\s+%{POSINT:nats.log.msg.sid}(\\s+%{NOTSPACE:nats.log.msg.reply_to})?\\s+%{POSINT:nats.log.msg.bytes}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:(?:(?:CONNECT)|(?:INFO))))\\s+%{GREEDYDATA:nats.log.msg.data}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:-ERROR))\\s+\\s+(?P<network_direction>(?:(<<-|->>))) \\[(?P<nats_log_msg_type>(?:-ERROR))\\s+\\s+%{GREEDYDATA:nats.log.msg.error\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("nats_log_msg_type", "nats.log.msg.type")
                                ]
                            ),
                            cached_grok!("%{GREEDYDATA:nats.log.msg.data}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.remove("nats.log.info").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "nats.log.info".into(),
                });
            }

            event.remove("nats.log.msg.info");

            event.remove("nats.log.msg.payload");

            if event.has_value("nats.log.msg.data") {
                event.rename("nats.log.msg.data", "message")?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "7e350f473009430482b62e4dafe08427563f7fbe748ae3326d141fc3a013b05e",
                    r#"{"params":{"ParamRenameLadder":{"path":"log.level","arms":[["inf","info"],["dbg","debug"],["wrn","warning"],["err","error"],["ftl","fatal"],["trc","trace"]]}},"known":[]}"#
                ),
                cached_params!(
                    "{\"inf\":\"INF\",\"info\":\"info\",\"dbg\":\"DBG\",\"debug\":\"debug\",\"wrn\":\"WRN\",\"warning\":\"warning\",\"err\":\"ERR\",\"error\":\"error\",\"ftl\":\"FTL\",\"fatal\":\"fatal\",\"trc\":\"TRC\",\"trace\":\"trace\"}"
                ),
            )?;

            let _cond = { event.has_value("nats.log.msg.type") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "ff5e686a6418967f3651316772b467442cd308dc28d63377cf799123db1fa51e",
                        r#"{"params":{"ParamRenameLadder":{"path":"nats.log.msg.type","arms":[["msg","message"],["pub","publish"],["sub","subscribe"],["unsub","unsubscribe"],["msg_payload","payload"],["err","error"],["pi","ping"],["po","pong"],["ok","acknowledge"],["connect","connection"],["info","information"]]}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"msg\":\"MSG\",\"message\":\"message\",\"pub\":\"PUB\",\"publish\":\"publish\",\"sub\":\"SUB\",\"subscribe\":\"subscribe\",\"unsub\":\"UNSUB\",\"unsubscribe\":\"unsubscribe\",\"msg_payload\":\"MSG_PAYLOAD\",\"payload\":\"payload\",\"err\":\"-ERROR\",\"error\":\"error\",\"pi\":\"PING\",\"ping\":\"ping\",\"po\":\"PONG\",\"pong\":\"pong\",\"ok\":\"OK\",\"acknowledge\":\"acknowledge\",\"connect\":\"CONNECT\",\"connection\":\"connection\",\"info\":\"INFO\",\"information\":\"information\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("network.direction") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "c929f62b5fac5100a95eb2378e0ecf7bb3b06192ed7b25d653e6f8ba6f0c67f3",
                        r#"{"params":{"ParamRenameLadder":{"path":"network.direction","arms":[["in","inbound"],["out","outbound"]]}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"in\":\"<<-\",\"inbound\":\"inbound\",\"out\":\"->>\",\"outbound\":\"outbound\"}"
                    ),
                )?;
            }

            event.rename("@timestamp", "event.created")?;

            if let Some(date_str) = event.get_as_string("nats.log.timestamp") {
                match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss.SSSSSS"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "nats.log.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("nats.log.timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "nats.log.timestamp".into(),
                });
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            let _cond = {
                event.has_value("log.level")
                    && (event.get_str("log.level") == Some("error")
                        || event.get_str("log.level") == Some("fatal"))
            };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("process.pid") {
                if let Some(val) = event.get("process.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
            }

            if event.has_value("client.port") {
                if let Some(val) = event.get("client.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "client.port".into(),
                            message,
                        }
                    })?;
                    event.set("client.port", converted)?;
                }
            }

            if event.has_value("nats.log.msg.bytes") {
                if let Some(val) = event.get("nats.log.msg.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "nats.log.msg.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("nats.log.msg.bytes", converted)?;
                }
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
