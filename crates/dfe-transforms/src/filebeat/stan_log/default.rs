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
                    // Grok pattern: \\[%{POSINT:process.pid}\\]( (?P<stan_log_timestamp>(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})))? \\[(?P<log_level>(?:(INF|DBG|WRN|ERR|FTL|TRC)))\\] %{GREEDYDATA:stan.log.info}
                    if !cached_grok_mapped!("\\[%{POSINT:process.pid}\\]( (?P<stan_log_timestamp>(?:%{YEAR}/%{MONTHNUM}/%{MONTHDAY} %{TIME})))? \\[(?P<log_level>(?:(INF|DBG|WRN|ERR|FTL|TRC)))\\] %{GREEDYDATA:stan.log.info}", [("stan_log_timestamp", "stan.log.timestamp"), ("log_level", "log.level")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("stan.log.info") {
                if let Some(input) = event.get_string("stan.log.info") {
                    // Grok pattern: %{IPV4:client.ip}:%{POSINT:client.port} - cid:%{POSINT:stan.log.client.id} - %{GREEDYDATA:stan.log.msg.info}
                    // Grok pattern: %{GREEDYDATA:stan.log.msg.data}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{IPV4:client.ip}:%{POSINT:client.port} - cid:%{POSINT:stan.log.client.id} - %{GREEDYDATA:stan.log.msg.info}"
                            ),
                            cached_grok!("%{GREEDYDATA:stan.log.msg.data}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("stan.log.msg.info") {
                if let Some(input) = event.get_string("stan.log.msg.info") {
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) (?P<stan_log_msg_type>(?:MSG_PAYLOAD)): \\[%{GREEDYDATA:stan.log.msg.payload}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:(?:(?:PING)|(?:PONG)|(?:OK))))\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:UNSUB))\\s+%{POSINT:stan.log.msg.sid}(\\s+%{POSINT:stan.log.msg.max_messages})?\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:PUB))\\s+%{NOTSPACE:stan.log.msg.subject}(\\s+%{NOTSPACE:stan.log.msg.reply_to})?\\s+%{POSINT:stan.log.msg.bytes}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:SUB))\\s+%{NOTSPACE:stan.log.msg.subject}(\\s+%{NOTSPACE:stan.log.msg.queue_group})?\\s+%{POSINT:stan.log.msg.sid}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:MSG))\\s+%{NOTSPACE:stan.log.msg.subject}\\s+%{POSINT:stan.log.msg.sid}(\\s+%{NOTSPACE:stan.log.msg.reply_to})?\\s+%{POSINT:stan.log.msg.bytes}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:(?:(?:CONNECT)|(?:INFO))))\\s+%{GREEDYDATA:stan.log.msg.data}\\]
                    // Grok pattern: (?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:-ERROR))\\s+\\s+(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:-ERROR))\\s+\\s+%{GREEDYDATA:stan.log.msg.error\\]
                    // Grok pattern: %{GREEDYDATA:stan.log.msg.data}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) (?P<stan_log_msg_type>(?:MSG_PAYLOAD)): \\[%{GREEDYDATA:stan.log.msg.payload}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:(?:(?:PING)|(?:PONG)|(?:OK))))\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:UNSUB))\\s+%{POSINT:stan.log.msg.sid}(\\s+%{POSINT:stan.log.msg.max_messages})?\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:PUB))\\s+%{NOTSPACE:stan.log.msg.subject}(\\s+%{NOTSPACE:stan.log.msg.reply_to})?\\s+%{POSINT:stan.log.msg.bytes}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:SUB))\\s+%{NOTSPACE:stan.log.msg.subject}(\\s+%{NOTSPACE:stan.log.msg.queue_group})?\\s+%{POSINT:stan.log.msg.sid}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:MSG))\\s+%{NOTSPACE:stan.log.msg.subject}\\s+%{POSINT:stan.log.msg.sid}(\\s+%{NOTSPACE:stan.log.msg.reply_to})?\\s+%{POSINT:stan.log.msg.bytes}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:(?:(?:CONNECT)|(?:INFO))))\\s+%{GREEDYDATA:stan.log.msg.data}\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok_mapped!(
                                "(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:-ERROR))\\s+\\s+(?P<network_direction>(?:(<<-|->>))) \\[(?P<stan_log_msg_type>(?:-ERROR))\\s+\\s+%{GREEDYDATA:stan.log.msg.error\\]",
                                [
                                    ("network_direction", "network.direction"),
                                    ("stan_log_msg_type", "stan.log.msg.type")
                                ]
                            ),
                            cached_grok!("%{GREEDYDATA:stan.log.msg.data}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            event.remove("stan.log.info");
            event.remove("stan.log.msg.info");
            event.remove("stan.log.msg.payload");

            if event.has_value("stan.log.msg.data") {
                event.rename("stan.log.msg.data", "message")?;
            }

            // Painless script
            // Source: if (ctx.log.level == params.inf) {\n          ctx.log.level = params.info;\n        } else if (ctx.log.level == params.dbg) {\n          ctx.log.level = params.debug;\n        } else if (ctx.log.level == params.wrn) {\n          ctx.log.level = params.warning;\n        } else if (ctx.log.level == params.err) {\n          ctx.log.level = params.error;\n        } else if (ctx.log.level == params.ftl) {\n          ctx.log.level = params.fatal;\n        } else if (ctx.log.level == params.trc) {\n          ctx.log.level = params.trace;\n        }
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log.level == params.inf) {\n          ctx.log.level = params.info;\n        } else if (ctx.log.level == params.dbg) {\n          ctx.log.level = params.debug;\n        } else if (ctx.log.level == params.wrn) {\n          ctx.log.level = params.warning;\n        } else if (ctx.log.level == params.err) {\n          ctx.log.level = params.error;\n        } else if (ctx.log.level == params.ftl) {\n          ctx.log.level = params.fatal;\n        } else if (ctx.log.level == params.trc) {\n          ctx.log.level = params.trace;\n        }"#
                ),
                cached_params!(
                    "{\"inf\":\"INF\",\"info\":\"info\",\"dbg\":\"DBG\",\"debug\":\"debug\",\"wrn\":\"WRN\",\"warning\":\"warning\",\"err\":\"ERR\",\"error\":\"error\",\"ftl\":\"FTL\",\"fatal\":\"fatal\",\"trc\":\"TRC\",\"trace\":\"trace\"}"
                ),
            )?;

            let _cond = { event.has_value("stan.log.msg.type") };
            if _cond {
                // Painless script
                // Source: if (ctx.stan.log.msg.type == params.msg) {\n          ctx.stan.log.msg.type = params.message;\n        } else if (ctx.stan.log.msg.type == params.pub) {\n          ctx.stan.log.msg.type = params.publish;\n        } else if (ctx.stan.log.msg.type == params.sub) {\n          ctx.stan.log.msg.type = params.subscribe;\n        } else if (ctx.stan.log.msg.type == params.unsub) {\n          ctx.stan.log.msg.type = params.unsubscribe;\n        } else if (ctx.stan.log.msg.type == params.msg_payload) {\n          ctx.stan.log.msg.type = params.payload;\n        } else if (ctx.stan.log.msg.type == params.err) {\n          ctx.stan.log.msg.type = params.error;\n        } else if (ctx.stan.log.msg.type == params.pi) {\n          ctx.stan.log.msg.type = params.ping;\n        } else if (ctx.stan.log.msg.type == params.po) {\n          ctx.stan.log.msg.type = params.pong;\n        } else if (ctx.stan.log.msg.type == params.ok) {\n          ctx.stan.log.msg.type = params.acknowledge;\n        } else if (ctx.stan.log.msg.type == params.connect) {\n          ctx.stan.log.msg.type = params.connection;\n        } else if (ctx.stan.log.msg.type == params.info) {\n          ctx.stan.log.msg.type = params.information;\n        }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.stan.log.msg.type == params.msg) {\n          ctx.stan.log.msg.type = params.message;\n        } else if (ctx.stan.log.msg.type == params.pub) {\n          ctx.stan.log.msg.type = params.publish;\n        } else if (ctx.stan.log.msg.type == params.sub) {\n          ctx.stan.log.msg.type = params.subscribe;\n        } else if (ctx.stan.log.msg.type == params.unsub) {\n          ctx.stan.log.msg.type = params.unsubscribe;\n        } else if (ctx.stan.log.msg.type == params.msg_payload) {\n          ctx.stan.log.msg.type = params.payload;\n        } else if (ctx.stan.log.msg.type == params.err) {\n          ctx.stan.log.msg.type = params.error;\n        } else if (ctx.stan.log.msg.type == params.pi) {\n          ctx.stan.log.msg.type = params.ping;\n        } else if (ctx.stan.log.msg.type == params.po) {\n          ctx.stan.log.msg.type = params.pong;\n        } else if (ctx.stan.log.msg.type == params.ok) {\n          ctx.stan.log.msg.type = params.acknowledge;\n        } else if (ctx.stan.log.msg.type == params.connect) {\n          ctx.stan.log.msg.type = params.connection;\n        } else if (ctx.stan.log.msg.type == params.info) {\n          ctx.stan.log.msg.type = params.information;\n        }"#
                    ),
                    cached_params!(
                        "{\"msg\":\"MSG\",\"message\":\"message\",\"pub\":\"PUB\",\"publish\":\"publish\",\"sub\":\"SUB\",\"subscribe\":\"subscribe\",\"unsub\":\"UNSUB\",\"unsubscribe\":\"unsubscribe\",\"msg_payload\":\"MSG_PAYLOAD\",\"payload\":\"payload\",\"err\":\"-ERROR\",\"error\":\"error\",\"pi\":\"PING\",\"ping\":\"ping\",\"po\":\"PONG\",\"pong\":\"pong\",\"ok\":\"OK\",\"acknowledge\":\"acknowledge\",\"connect\":\"CONNECT\",\"connection\":\"connection\",\"info\":\"INFO\",\"information\":\"information\"}"
                    ),
                )?;
            }

            let _cond = { event.has_value("network.direction") };
            if _cond {
                // Painless script
                // Source: if (ctx.network.direction == params.in) {\n          ctx.network.direction = params.inbound;\n        } else if (ctx.network.direction == params.out) {\n          ctx.network.direction = params.outbound;\n        }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.network.direction == params.in) {\n          ctx.network.direction = params.inbound;\n        } else if (ctx.network.direction == params.out) {\n          ctx.network.direction = params.outbound;\n        }"#
                    ),
                    cached_params!(
                        "{\"in\":\"<<-\",\"inbound\":\"inbound\",\"out\":\"->>\",\"outbound\":\"outbound\"}"
                    ),
                )?;
            }

            event.rename("@timestamp", "event.created")?;

            if let Some(date_str) = event.get_as_string("stan.log.timestamp") {
                match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss.SSSSSS"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "stan.log.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("stan.log.timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "stan.log.timestamp".into(),
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

            if event.has_value("stan.log.msg.bytes") {
                if let Some(val) = event.get("stan.log.msg.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stan.log.msg.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("stan.log.msg.bytes", converted)?;
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
