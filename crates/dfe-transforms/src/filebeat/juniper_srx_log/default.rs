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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?:<%{POSINT:syslog_pri}>(?:\\d{1,3})?%{SPACE})?%{TIMESTAMP_ISO8601:_temp_.raw_date}\\s%{SYSLOGHOST:syslog_hostname}\\s(?P<syslog_program>(?:RT_FLOW|RT_UTM|RT_IDP|RT_IDS|RT_AAMW|RT_SECINTEL))\\s(?:%{POSINT:syslog_pid}|-)?\\s%{WORD:tag}\\s\\[([^=]+?\\s)?%{GREEDYDATA:_temp_.traffic_structured}\\]\\s?$
                // Grok pattern: ^(?:<%{POSINT:syslog_pri}>(?:\\d{1,3})?%{SPACE})?(?P<_temp__raw_date>(?:%{TIMESTAMP_ISO8601}|(%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME})))\\s%{SYSLOGHOST:syslog_hostname}\\s%{PROG:syslog_program}\\s(?:%{POSINT:syslog_pid}|-)?\\s%{WORD:tag}\\s\\[([^=]+?\\s)?%{GREEDYDATA:_temp_.system_structured}\\](?!=)\\s?%{DATA:_temp_.unparsed.message}\\s?$
                // Grok pattern: ^(?:<%{POSINT:syslog_pri}>(?:\\d{1,3})?%{SPACE})?(?P<_temp__raw_date>(?:%{TIMESTAMP_ISO8601}|(%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME})))\\s%{SYSLOGHOST:syslog_hostname}\\s%{GREEDYDATA:_temp_.unparsed.message}$
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^(?:<%{POSINT:syslog_pri}>(?:\\d{1,3})?%{SPACE})?%{TIMESTAMP_ISO8601:_temp_.raw_date}\\s%{SYSLOGHOST:syslog_hostname}\\s(?P<syslog_program>(?:RT_FLOW|RT_UTM|RT_IDP|RT_IDS|RT_AAMW|RT_SECINTEL))\\s(?:%{POSINT:syslog_pid}|-)?\\s%{WORD:tag}\\s\\[([^=]+?\\s)?%{GREEDYDATA:_temp_.traffic_structured}\\]\\s?$"
                        ),
                        cached_grok_mapped!(
                            "^(?:<%{POSINT:syslog_pri}>(?:\\d{1,3})?%{SPACE})?(?P<_temp__raw_date>(?:%{TIMESTAMP_ISO8601}|(%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME})))\\s%{SYSLOGHOST:syslog_hostname}\\s%{PROG:syslog_program}\\s(?:%{POSINT:syslog_pid}|-)?\\s%{WORD:tag}\\s\\[([^=]+?\\s)?%{GREEDYDATA:_temp_.system_structured}\\](?!=)\\s?%{DATA:_temp_.unparsed.message}\\s?$",
                            [("_temp__raw_date", "_temp_.raw_date")]
                        ),
                        cached_grok_mapped!(
                            "^(?:<%{POSINT:syslog_pri}>(?:\\d{1,3})?%{SPACE})?(?P<_temp__raw_date>(?:%{TIMESTAMP_ISO8601}|(%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME})))\\s%{SYSLOGHOST:syslog_hostname}\\s%{GREEDYDATA:_temp_.unparsed.message}$",
                            [("_temp__raw_date", "_temp_.raw_date")]
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            if event.has_value("_temp_.traffic_structured") {
                if let Some(kv_str) = event.get_string("_temp_.traffic_structured") {
                    for pair in cached_regex!(" (?=[a-z0-9\\_\\-]+=)")
                        .split(&kv_str)
                        .into_iter()
                    {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.traffic_structured".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| "\"".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("juniper.srx.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("_temp_.system_structured") {
                if let Some(kv_str) = event.get_string("_temp_.system_structured") {
                    for pair in cached_regex!(" (?=[a-z0-9\\_\\-]+=)")
                        .split(&kv_str)
                        .into_iter()
                    {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.system_structured".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| "\"".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("juniper.srx.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("syslog_program") {
                event.rename("syslog_program", "juniper.srx.process")?;
            }

            let _cond = { event.has_value("juniper.srx") };
            if _cond {
                // Painless script
                // Source: ctx.juniper.srx = ctx?.juniper?.srx.entrySet().stream().collect(Collectors.toMap(e -> e.getKey().replace('-', '_'), e -> e.getValue()));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.juniper.srx = ctx?.juniper?.srx.entrySet().stream().collect(Collectors.toMap(e -> e.getKey().replace('-', '_'), e -> e.getValue()));"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss",
                            "yyyy-MM-dd HH:mm:ss z",
                            "yyyy-MM-dd HH:mm:ss Z",
                            "ISO8601",
                            "MMM d HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.raw_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss",
                            "yyyy-MM-dd HH:mm:ss z",
                            "yyyy-MM-dd HH:mm:ss Z",
                            "ISO8601",
                            "MMM d HH:mm:ss",
                        ],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.raw_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("juniper.srx.elapsed_time") };
            if _cond {
                event.rename("juniper.srx.elapsed_time", "juniper.srx.duration")?;
            }

            let _cond = { event.has_value("juniper.srx.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Integer.parseInt(ctx.juniper.srx.duration) * 1000000000L; ctx.event.start = ctx['@timestamp']; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Integer.parseInt(ctx.juniper.srx.duration) * 1000000000L; ctx.event.start = ctx['@timestamp']; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);"#
                    ),
                )?;
            }

            let _cond = { event.has_value("juniper.srx") };
            if _cond {
                // Painless script
                // Source: ctx?.juniper?.srx.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx?.juniper?.srx.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));"#
                    ),
                    cached_params!("{\"values\":[\"None\",\"UNKNOWN\",\"N/A\",\"-\"]}"),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("syslog_pri") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "syslog_pri".into(),
                            message,
                        }
                    })?;
                    event.set("event.severity", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_syslog_pri_to_long",
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

            let _cond = {
                [
                    "0", "8", "16", "24", "32", "40", "48", "56", "64", "72", "80", "88", "96",
                    "104", "112", "128", "136", "144", "152", "160", "168", "176", "184",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("emergency"))?;
            }

            let _cond = {
                [
                    "1", "9", "17", "25", "33", "41", "49", "57", "65", "73", "81", "89", "97",
                    "105", "113", "129", "137", "145", "153", "161", "169", "177", "185",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("alert"))?;
            }

            let _cond = {
                [
                    "2", "10", "18", "26", "34", "42", "50", "58", "66", "74", "82", "90", "98",
                    "106", "114", "130", "138", "146", "154", "162", "170", "178", "186",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("critical"))?;
            }

            let _cond = {
                [
                    "3", "11", "19", "27", "35", "43", "51", "59", "67", "75", "83", "91", "99",
                    "107", "115", "131", "139", "147", "155", "163", "171", "179", "187",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("error"))?;
            }

            let _cond = {
                [
                    "4", "12", "20", "28", "36", "44", "52", "60", "68", "76", "84", "92", "100",
                    "108", "116", "132", "140", "148", "156", "164", "172", "180", "188",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("warning"))?;
            }

            let _cond = {
                [
                    "5", "13", "21", "29", "37", "45", "53", "61", "69", "77", "85", "93", "101",
                    "109", "117", "133", "141", "149", "157", "165", "173", "181", "189",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("notification"))?;
            }

            let _cond = {
                [
                    "6", "14", "22", "30", "38", "46", "54", "62", "70", "78", "86", "94", "102",
                    "110", "118", "134", "142", "150", "158", "166", "174", "182", "190",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("informational"))?;
            }

            let _cond = {
                [
                    "7", "15", "23", "31", "39", "47", "55", "63", "71", "79", "87", "95", "103",
                    "111", "119", "135", "143", "151", "159", "167", "175", "183", "191",
                ]
                .contains(&event.get_str("syslog_pri").unwrap_or(""))
            };
            if _cond {
                event.set("log.level", json!("debug"))?;
            }

            event.set("observer.vendor", json!("Juniper"))?;

            event.set("observer.product", json!("SRX"))?;

            event.set("observer.type", json!("firewall"))?;

            if event.has_value("syslog_hostname") {
                event.rename("syslog_hostname", "observer.name")?;
            }

            if event.has_value("juniper.srx.packet_incoming_interface") {
                event.rename(
                    "juniper.srx.packet_incoming_interface",
                    "observer.ingress.interface.name",
                )?;
            }

            if event.has_value("juniper.srx.destination_interface_name") {
                event.rename(
                    "juniper.srx.destination_interface_name",
                    "observer.egress.interface.name",
                )?;
            }

            if event.has_value("juniper.srx.source_interface_name") {
                event.rename(
                    "juniper.srx.source_interface_name",
                    "observer.ingress.interface.name",
                )?;
            }

            if event.has_value("juniper.srx.interface_name") {
                event.rename(
                    "juniper.srx.interface_name",
                    "observer.ingress.interface.name",
                )?;
            }

            if event.has_value("juniper.srx.source_zone_name") {
                event.rename("juniper.srx.source_zone_name", "observer.ingress.zone")?;
            }

            if event.has_value("juniper.srx.source_zone") {
                event.rename("juniper.srx.source_zone", "observer.ingress.zone")?;
            }

            if event.has_value("juniper.srx.destination_zone_name") {
                event.rename("juniper.srx.destination_zone_name", "observer.egress.zone")?;
            }

            if event.has_value("juniper.srx.destination_zone") {
                event.rename("juniper.srx.destination_zone", "observer.egress.zone")?;
            }

            if event.has_value("tag") {
                event.rename("tag", "juniper.srx.tag")?;
            }

            event.remove("message");

            let _cond = { event.get_str("juniper.srx.process") == Some("RT_FLOW") };
            if _cond {
                // Begin nested pipeline: "flow"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("juniper.srx.tag") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                event.append("event.category", json!("network"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.application_risk") {
                        if let Some(val) = event.get("juniper.srx.application_risk") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.application_risk".into(),
                                    message,
                                }
                            })?;
                            event.set("event.risk_score", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event
                        .get_str("juniper.srx.tag")
                        .is_some_and(|s| s.ends_with("CREATE"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("UPDATE"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("CREATE_LS"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("UPDATE_LS"))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    event
                        .get_str("juniper.srx.tag")
                        .is_some_and(|s| s.ends_with("CLOSE"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("CLOSE_LS"))
                };
                if _cond {
                    event.append("event.type", json!("end"))?;
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    event
                        .get_str("juniper.srx.tag")
                        .is_some_and(|s| s.ends_with("DENY"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("DENY_LS"))
                };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    event
                        .get_str("juniper.srx.tag")
                        .is_some_and(|s| s.ends_with("CREATE"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("UPDATE"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("CREATE_LS"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("UPDATE_LS"))
                };
                if _cond {
                    event.set("event.action", json!("flow_started"))?;
                }
                let _cond = {
                    event
                        .get_str("juniper.srx.tag")
                        .is_some_and(|s| s.ends_with("CLOSE"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("CLOSE_LS"))
                };
                if _cond {
                    event.set("event.action", json!("flow_close"))?;
                }
                let _cond = {
                    event
                        .get_str("juniper.srx.tag")
                        .is_some_and(|s| s.ends_with("DENY"))
                        || event
                            .get_str("juniper.srx.tag")
                            .is_some_and(|s| s.ends_with("DENY_LS"))
                };
                if _cond {
                    event.set("event.action", json!("flow_deny"))?;
                }
                let _cond = { event.has_value("juniper.srx.destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_server") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_server") {
                            if let Some(val) = event.get("juniper.srx.packets_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.source_address") };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_source_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_client") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_client") {
                            if let Some(val) = event.get("juniper.srx.packets_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.policy_name") };
                if _cond {
                    if event.has_value("juniper.srx.policy_name") {
                        event.rename("juniper.srx.policy_name", "rule.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_id") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_id") {
                        event.rename("juniper.srx.protocol_id", "network.iana_number")?;
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.ip") {
                        if let Some(ip_str) = event.get_string("source.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.as") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("source.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("source.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.as") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                let _cond =
                    { event.has_value("source.bytes") && event.has_value("destination.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script, resolved to its runners at generation time
                        // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                        sum_directions(event, &["bytes"]);
                        Ok(())
                    })();
                }
                let _cond =
                    { event.has_value("client.packets") && event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script
                        // Source: ctx.network.packets = ctx.client.packets + ctx.server.packets
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"ctx.network.packets = ctx.client.packets + ctx.server.packets"#
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("juniper.srx.application_risk");
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.bytes_from_client");
                event.remove("juniper.srx.packets_from_client");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.bytes_from_server");
                event.remove("juniper.srx.packets_from_server");
                // End nested pipeline: "flow"
            }

            let _cond = { event.get_str("juniper.srx.process") == Some("RT_UTM") };
            if _cond {
                // Begin nested pipeline: "utm"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("juniper.srx.tag") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                event.append("event.category", json!("network"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.urlcategory_risk") {
                        if let Some(val) = event.get("juniper.srx.urlcategory_risk") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "juniper.srx.urlcategory_risk".into(),
                                    message,
                                }
                            })?;
                            event.set("event.risk_score", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    [
                        "AV_VIRUS_DETECTED_MT",
                        "WEBFILTER_URL_BLOCKED",
                        "ANTISPAM_SPAM_DETECTED_MT",
                        "CONTENT_FILTERING_BLOCKED_MT",
                        "AV_VIRUS_DETECTED_MT_LS",
                        "WEBFILTER_URL_BLOCKED_LS",
                        "ANTISPAM_SPAM_DETECTED_MT_LS",
                        "CONTENT_FILTERING_BLOCKED_MT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    [
                        "AV_VIRUS_DETECTED_MT",
                        "WEBFILTER_URL_BLOCKED",
                        "ANTISPAM_SPAM_DETECTED_MT",
                        "CONTENT_FILTERING_BLOCKED_MT",
                        "AV_VIRUS_DETECTED_MT_LS",
                        "WEBFILTER_URL_BLOCKED_LS",
                        "ANTISPAM_SPAM_DETECTED_MT_LS",
                        "CONTENT_FILTERING_BLOCKED_MT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = {
                    [
                        "AV_VIRUS_DETECTED_MT",
                        "WEBFILTER_URL_BLOCKED",
                        "ANTISPAM_SPAM_DETECTED_MT",
                        "CONTENT_FILTERING_BLOCKED_MT",
                        "AV_VIRUS_DETECTED_MT_LS",
                        "WEBFILTER_URL_BLOCKED_LS",
                        "ANTISPAM_SPAM_DETECTED_MT_LS",
                        "CONTENT_FILTERING_BLOCKED_MT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    !([
                        "AV_VIRUS_DETECTED_MT",
                        "WEBFILTER_URL_BLOCKED",
                        "ANTISPAM_SPAM_DETECTED_MT",
                        "CONTENT_FILTERING_BLOCKED_MT",
                        "AV_VIRUS_DETECTED_MT_LS",
                        "WEBFILTER_URL_BLOCKED_LS",
                        "ANTISPAM_SPAM_DETECTED_MT_LS",
                        "CONTENT_FILTERING_BLOCKED_MT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or("")))
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    ["WEBFILTER_URL_BLOCKED", "WEBFILTER_URL_BLOCKED_LS"]
                        .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("web_filter"))?;
                }
                let _cond = {
                    [
                        "CONTENT_FILTERING_BLOCKED_MT",
                        "CONTENT_FILTERING_BLOCKED_MT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("content_filter"))?;
                }
                let _cond = {
                    ["ANTISPAM_SPAM_DETECTED_MT", "ANTISPAM_SPAM_DETECTED_MT_LS"]
                        .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("antispam_filter"))?;
                }
                let _cond = {
                    ["AV_VIRUS_DETECTED_MT", "AV_VIRUS_DETECTED_MT_LS"]
                        .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("virus_detected"))?;
                }
                let _cond = { event.has_value("juniper.srx.destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_server") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_server") {
                            if let Some(val) = event.get("juniper.srx.packets_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.source_address") };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_source_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_client") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_client") {
                            if let Some(val) = event.get("juniper.srx.packets_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.policy_name") };
                if _cond {
                    if event.has_value("juniper.srx.policy_name") {
                        event.rename("juniper.srx.policy_name", "rule.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.url") };
                if _cond {
                    if event.has_value("juniper.srx.url") {
                        event.rename("juniper.srx.url", "url.domain")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.obj") };
                if _cond {
                    if event.has_value("juniper.srx.obj") {
                        event.rename("juniper.srx.obj", "url.path")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.filename") };
                if _cond {
                    if event.has_value("juniper.srx.filename") {
                        event.rename("juniper.srx.filename", "file.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol") };
                if _cond {
                    if event.has_value("juniper.srx.protocol") {
                        event.rename("juniper.srx.protocol", "network.protocol")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_id") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_id") {
                        event.rename("juniper.srx.protocol_id", "network.iana_number")?;
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.ip") {
                        if let Some(ip_str) = event.get_string("source.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.as") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("source.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("source.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.as") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.bytes_from_client");
                event.remove("juniper.srx.packets_from_client");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.bytes_from_server");
                event.remove("juniper.srx.packets_from_server");
                event.remove("juniper.srx.urlcategory_risk");
                // End nested pipeline: "utm"
            }

            let _cond = { event.get_str("juniper.srx.process") == Some("RT_IDP") };
            if _cond {
                // Begin nested pipeline: "idp"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("juniper.srx.tag") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                event.append("event.category", json!("network"))?;
                let _cond = {
                    [
                        "IDP_ATTACK_LOG_EVENT",
                        "IDP_APPDDOS_APP_STATE_EVENT",
                        "IDP_APPDDOS_APP_ATTACK_EVENT",
                        "IDP_ATTACK_LOG_EVENT_LS",
                        "IDP_APPDDOS_APP_STATE_EVENT_LS",
                        "IDP_APPDDOS_APP_ATTACK_EVENT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    [
                        "IDP_ATTACK_LOG_EVENT",
                        "IDP_APPDDOS_APP_STATE_EVENT",
                        "IDP_APPDDOS_APP_ATTACK_EVENT",
                        "IDP_ATTACK_LOG_EVENT_LS",
                        "IDP_APPDDOS_APP_STATE_EVENT_LS",
                        "IDP_APPDDOS_APP_ATTACK_EVENT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                }
                let _cond = {
                    [
                        "IDP_ATTACK_LOG_EVENT",
                        "IDP_APPDDOS_APP_STATE_EVENT",
                        "IDP_APPDDOS_APP_ATTACK_EVENT",
                        "IDP_ATTACK_LOG_EVENT_LS",
                        "IDP_APPDDOS_APP_STATE_EVENT_LS",
                        "IDP_APPDDOS_APP_ATTACK_EVENT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    !([
                        "IDP_ATTACK_LOG_EVENT",
                        "IDP_APPDDOS_APP_STATE_EVENT",
                        "IDP_APPDDOS_APP_ATTACK_EVENT",
                        "IDP_ATTACK_LOG_EVENT_LS",
                        "IDP_APPDDOS_APP_STATE_EVENT_LS",
                        "IDP_APPDDOS_APP_ATTACK_EVENT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or("")))
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    [
                        "IDP_APPDDOS_APP_STATE_EVENT",
                        "IDP_APPDDOS_APP_ATTACK_EVENT",
                        "IDP_APPDDOS_APP_STATE_EVENT_LS",
                        "IDP_APPDDOS_APP_ATTACK_EVENT_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("application_ddos"))?;
                }
                let _cond = {
                    ["IDP_ATTACK_LOG_EVENT", "IDP_ATTACK_LOG_EVENT_LS"]
                        .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("security_threat"))?;
                }
                let _cond = { event.has_value("juniper.srx.destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.inbound_bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.inbound_bytes") {
                            if let Some(val) = event.get("juniper.srx.inbound_bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.inbound_bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.inbound_packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.inbound_packets") {
                            if let Some(val) = event.get("juniper.srx.inbound_packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.inbound_packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.source_address") };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_source_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.outbound_bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.outbound_bytes") {
                            if let Some(val) = event.get("juniper.srx.outbound_bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.outbound_bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.outbound_packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.outbound_packets") {
                            if let Some(val) = event.get("juniper.srx.outbound_packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.outbound_packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.rulebase_name") };
                if _cond {
                    if event.has_value("juniper.srx.rulebase_name") {
                        event.rename("juniper.srx.rulebase_name", "rule.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.rule_name") };
                if _cond {
                    if event.has_value("juniper.srx.rule_name") {
                        event.rename("juniper.srx.rule_name", "rule.id")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_name") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_name") {
                        event.rename("juniper.srx.protocol_name", "network.protocol")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.message") };
                if _cond {
                    if event.has_value("juniper.srx.message") {
                        event.rename("juniper.srx.message", "message")?;
                    }
                }
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.outbound_bytes");
                event.remove("juniper.srx.outbound_packets");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.inbound_bytes");
                event.remove("juniper.srx.inbound_packets");
                // End nested pipeline: "idp"
            }

            let _cond = { event.get_str("juniper.srx.process") == Some("RT_IDS") };
            if _cond {
                // Begin nested pipeline: "ids"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("juniper.srx.tag") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                event.append("event.category", json!("network"))?;
                let _cond = {
                    [
                        "RT_SCREEN_TCP",
                        "RT_SCREEN_UDP",
                        "RT_SCREEN_ICMP",
                        "RT_SCREEN_IP",
                        "RT_SCREEN_TCP_DST_IP",
                        "RT_SCREEN_TCP_SRC_IP",
                        "RT_SCREEN_TCP_LS",
                        "RT_SCREEN_UDP_LS",
                        "RT_SCREEN_ICMP_LS",
                        "RT_SCREEN_IP_LS",
                        "RT_SCREEN_TCP_DST_IP_LS",
                        "RT_SCREEN_TCP_SRC_IP_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    [
                        "RT_SCREEN_TCP",
                        "RT_SCREEN_UDP",
                        "RT_SCREEN_ICMP",
                        "RT_SCREEN_IP",
                        "RT_SCREEN_TCP_DST_IP",
                        "RT_SCREEN_TCP_SRC_IP",
                        "RT_SCREEN_TCP_LS",
                        "RT_SCREEN_UDP_LS",
                        "RT_SCREEN_ICMP_LS",
                        "RT_SCREEN_IP_LS",
                        "RT_SCREEN_TCP_DST_IP_LS",
                        "RT_SCREEN_TCP_SRC_IP_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                }
                let _cond = {
                    [
                        "RT_SCREEN_TCP",
                        "RT_SCREEN_UDP",
                        "RT_SCREEN_ICMP",
                        "RT_SCREEN_IP",
                        "RT_SCREEN_TCP_DST_IP",
                        "RT_SCREEN_TCP_SRC_IP",
                        "RT_SCREEN_TCP_LS",
                        "RT_SCREEN_UDP_LS",
                        "RT_SCREEN_ICMP_LS",
                        "RT_SCREEN_IP_LS",
                        "RT_SCREEN_TCP_DST_IP_LS",
                        "RT_SCREEN_TCP_SRC_IP_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    !([
                        "RT_SCREEN_TCP",
                        "RT_SCREEN_UDP",
                        "RT_SCREEN_ICMP",
                        "RT_SCREEN_IP",
                        "RT_SCREEN_TCP_DST_IP",
                        "RT_SCREEN_TCP_SRC_IP",
                        "RT_SCREEN_TCP_LS",
                        "RT_SCREEN_UDP_LS",
                        "RT_SCREEN_ICMP_LS",
                        "RT_SCREEN_IP_LS",
                        "RT_SCREEN_TCP_DST_IP_LS",
                        "RT_SCREEN_TCP_SRC_IP_LS",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or("")))
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    [
                        "ICMP flood!",
                        "UDP flood!",
                        "SYN flood!",
                        "SYN flood Src-IP based!",
                        "SYN flood Dst-IP based!",
                    ]
                    .contains(&event.get_str("juniper.srx.attack_name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("flood_detected"))?;
                }
                let _cond = { event.get_str("juniper.srx.attack_name") == Some("TCP port scan!") };
                if _cond {
                    event.set("event.action", json!("scan_detected"))?;
                }
                let _cond = {
                    ["TCP sweep!", "IP sweep!", "UDP sweep!", "Address sweep!"]
                        .contains(&event.get_str("juniper.srx.attack_name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("sweep_detected"))?;
                }
                let _cond = {
                    ["ICMP fragment!", "SYN fragment!"]
                        .contains(&event.get_str("juniper.srx.attack_name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("fragment_detected"))?;
                }
                let _cond = { event.get_str("juniper.srx.attack_name") == Some("IP spoofing!") };
                if _cond {
                    event.set("event.action", json!("spoofing_detected"))?;
                }
                let _cond = {
                    ["Src IP session limit!", "Dst IP session limit!"]
                        .contains(&event.get_str("juniper.srx.attack_name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("session_limit_detected"))?;
                }
                let _cond = {
                    ["Land attack!", "WinNuke attack!"]
                        .contains(&event.get_str("juniper.srx.attack_name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("attack_detected"))?;
                }
                let _cond = {
                    ["No TCP flag!", "SYN and FIN bits!", "FIN but no ACK bit!"]
                        .contains(&event.get_str("juniper.srx.attack_name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("illegal_tcp_flag_detected"))?;
                }
                let _cond = {
                    event
                        .get_str("juniper.srx.attack_name")
                        .is_some_and(|s| s.starts_with("Tunnel"))
                };
                if _cond {
                    event.set("event.action", json!("tunneling_screen"))?;
                }
                let _cond = { event.has_value("juniper.srx.destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_server") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_server") {
                            if let Some(val) = event.get("juniper.srx.packets_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.source_address") };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_source_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_client") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_client") {
                            if let Some(val) = event.get("juniper.srx.packets_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_id") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_id") {
                        event.rename("juniper.srx.protocol_id", "network.iana_number")?;
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.ip") {
                        if let Some(ip_str) = event.get_string("source.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.as") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("source.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("source.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.as") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.bytes_from_client");
                event.remove("juniper.srx.packets_from_client");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.bytes_from_server");
                event.remove("juniper.srx.packets_from_server");
                // End nested pipeline: "ids"
            }

            let _cond = { event.get_str("juniper.srx.process") == Some("RT_AAMW") };
            if _cond {
                // Begin nested pipeline: "atp"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("juniper.srx.tag") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                event.append("event.category", json!("network"))?;
                let _cond = {
                    [
                        "SRX_AAMW_ACTION_LOG",
                        "AAMW_MALWARE_EVENT_LOG",
                        "AAMW_HOST_INFECTED_EVENT_LOG",
                        "AAMW_ACTION_LOG",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                        && event.get_str("juniper.srx.action") != Some("PERMIT")
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    [
                        "SRX_AAMW_ACTION_LOG",
                        "AAMW_MALWARE_EVENT_LOG",
                        "AAMW_HOST_INFECTED_EVENT_LOG",
                        "AAMW_ACTION_LOG",
                    ]
                    .contains(&event.get_str("juniper.srx.tag").unwrap_or(""))
                        && event.get_str("juniper.srx.action") != Some("PERMIT")
                };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = {
                    event.get_str("juniper.srx.action") == Some("BLOCK")
                        || event.get_str("juniper.srx.tag") == Some("AAMW_MALWARE_EVENT_LOG")
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    event.get_str("juniper.srx.action") != Some("BLOCK")
                        && event.get_str("juniper.srx.tag") != Some("AAMW_MALWARE_EVENT_LOG")
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = {
                    event.get_str("juniper.srx.action") == Some("BLOCK")
                        || event.get_str("juniper.srx.tag") == Some("AAMW_MALWARE_EVENT_LOG")
                };
                if _cond {
                    event.set("event.action", json!("malware_detected"))?;
                }
                let _cond = { event.has_value("juniper.srx.destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_server") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_server") {
                            if let Some(val) = event.get("juniper.srx.packets_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.source_address") };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_source_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_client") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_client") {
                            if let Some(val) = event.get("juniper.srx.packets_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.hostname") };
                if _cond {
                    if event.has_value("juniper.srx.hostname") {
                        event.rename("juniper.srx.hostname", "source.domain")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.client_ip") };
                if _cond {
                    if event.has_value("juniper.srx.client_ip") {
                        event.rename("juniper.srx.client_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.http_host") };
                if _cond {
                    if event.has_value("juniper.srx.http_host") {
                        event.rename("juniper.srx.http_host", "url.domain")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_id") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_id") {
                        event.rename("juniper.srx.protocol_id", "network.iana_number")?;
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.ip") {
                        if let Some(ip_str) = event.get_string("source.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.as") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("source.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("source.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.as") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.timestamp") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("juniper.srx.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["EEE MMM dd HH:mm:ss yyyy", "EEE MMM  d HH:mm:ss yyyy"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("juniper.srx.timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "juniper.srx.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_juniper_srx_timestamp_to_juniper_srx_timestamp_bc1c29bf",
                        )?;
                        if event.remove("juniper.srx.timestamp").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "juniper.srx.timestamp".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.bytes_from_client");
                event.remove("juniper.srx.packets_from_client");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.bytes_from_server");
                event.remove("juniper.srx.packets_from_server");
                // End nested pipeline: "atp"
            }

            let _cond = { event.get_str("juniper.srx.process") == Some("RT_SECINTEL") };
            if _cond {
                // Begin nested pipeline: "secintel"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("juniper.srx.tag") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                event.append("event.category", json!("network"))?;
                let _cond = {
                    event.get_str("juniper.srx.tag") == Some("SECINTEL_ACTION_LOG")
                        && event.get_str("juniper.srx.action") != Some("PERMIT")
                };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = {
                    event.get_str("juniper.srx.tag") == Some("SECINTEL_ACTION_LOG")
                        && event.get_str("juniper.srx.action") != Some("PERMIT")
                };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = { event.get_str("juniper.srx.action") == Some("BLOCK") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                    event.append("event.type", json!("denied"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("juniper.srx.action") != Some("BLOCK") };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.get_str("juniper.srx.action") == Some("BLOCK") };
                if _cond {
                    event.set("event.action", json!("malware_detected"))?;
                }
                let _cond = { event.has_value("juniper.srx.destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_server") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_server") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_server") {
                            if let Some(val) = event.get("juniper.srx.packets_from_server") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_server".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.source_address") };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_source_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.bytes_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.bytes_from_client") {
                            if let Some(val) = event.get("juniper.srx.bytes_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.bytes_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.packets_from_client") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("juniper.srx.packets_from_client") {
                            if let Some(val) = event.get("juniper.srx.packets_from_client") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.packets_from_client".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.hostname") };
                if _cond {
                    if event.has_value("juniper.srx.hostname") {
                        event.rename("juniper.srx.hostname", "source.address")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.client_ip") };
                if _cond {
                    if event.has_value("juniper.srx.client_ip") {
                        event.rename("juniper.srx.client_ip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.http_host") };
                if _cond {
                    if event.has_value("juniper.srx.http_host") {
                        event.rename("juniper.srx.http_host", "url.domain")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_id") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_id") {
                        event.rename("juniper.srx.protocol_id", "network.iana_number")?;
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.ip") {
                        if let Some(ip_str) = event.get_string("source.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.ip") {
                        if let Some(ip_str) = event.get_string("destination.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("destination.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("destination.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.geo") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("source.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("source.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("source.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("source.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("source.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("source.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("source.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("source.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.geo") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-City.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                                if let Some(v) = geo.get("country_iso_code") {
                                    event.set("destination.geo.country_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("country_name") {
                                    event.set("destination.geo.country_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("continent_name") {
                                    event.set("destination.geo.continent_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_iso_code") {
                                    event.set("destination.geo.region_iso_code", v.clone())?;
                                }
                                if let Some(v) = geo.get("region_name") {
                                    event.set("destination.geo.region_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("city_name") {
                                    event.set("destination.geo.city_name", v.clone())?;
                                }
                                if let Some(v) = geo.get("timezone") {
                                    event.set("destination.geo.timezone", v.clone())?;
                                }
                                if let Some(v) = geo.get("location") {
                                    event.set("destination.geo.location", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("source.as") };
                if _cond {
                    if event.has_value("source.nat.ip") {
                        if let Some(ip_str) = event.get_string("source.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("source.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("source.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                let _cond = { !event.has_value("destination.as") };
                if _cond {
                    if event.has_value("destination.nat.ip") {
                        if let Some(ip_str) = event.get_string("destination.nat.ip") {
                            let ip_str = ip_str.to_string();
                            // GeoIP enrichment (GeoLite2-ASN.mmdb)
                            if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                                if let Some(v) = geo.get("asn") {
                                    event.set("destination.as.asn", v.clone())?;
                                }
                                if let Some(v) = geo.get("organization_name") {
                                    event.set("destination.as.organization_name", v.clone())?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.bytes_from_client");
                event.remove("juniper.srx.packets_from_client");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.bytes_from_server");
                event.remove("juniper.srx.packets_from_server");
                // End nested pipeline: "secintel"
            }

            let _cond = {
                !event.has_value("juniper.srx.process")
                    || !([
                        "RT_FLOW",
                        "RT_UTM",
                        "RT_IDP",
                        "RT_IDS",
                        "RT_AAMW",
                        "RT_SECINTEL",
                    ]
                    .contains(&event.get_str("juniper.srx.process").unwrap_or("")))
            };
            if _cond {
                // Begin nested pipeline: "system"
                let _cond = {
                    event.has_value("_temp_.unparsed.message")
                        && event.get_str("_temp_.unparsed.message") != Some("")
                };
                if _cond {
                    if let Some(input) = event.get_string("_temp_.unparsed.message") {
                        // Grok pattern: ^(?:%{PROG:syslog_program}|-)?\\s(?:%{POSINT:syslog_pid}|-)?\\s(?:%{WORD:tag}|-)?\\s([-]+\\s)?%{GREEDYDATA:_temp_.unparsed.system_structured_brief}\\s?$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^(?:%{PROG:syslog_program}|-)?\\s(?:%{POSINT:syslog_pid}|-)?\\s(?:%{WORD:tag}|-)?\\s([-]+\\s)?%{GREEDYDATA:_temp_.unparsed.system_structured_brief}\\s?$"
                                ),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("_temp_.unparsed.system_structured_brief")
                        && event.get_str("_temp_.unparsed.system_structured_brief") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("_temp_.unparsed.system_structured_brief")
                        {
                            // Grok pattern: ^%{WORD:_temp_.negotiation.type} negotiation %{GREEDYDATA:_temp_.negotiation.message}$
                            // Grok pattern: ^(%{SYSLOGHOST:syslog_hostname}\\s)?((?P<_temp__tag_brief>(?:(?!FW)[A-Za-z_]+))(\\s\\(pid=%{DATA:syslog_pid}\\))?(:\\s))?%{GREEDYDATA:_temp_.message_brief}$
                            // Grok pattern: ^%{GREEDYDATA:message}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{WORD:_temp_.negotiation.type} negotiation %{GREEDYDATA:_temp_.negotiation.message}$"
                                    ),
                                    cached_grok_mapped!(
                                        "^(%{SYSLOGHOST:syslog_hostname}\\s)?((?P<_temp__tag_brief>(?:(?!FW)[A-Za-z_]+))(\\s\\(pid=%{DATA:syslog_pid}\\))?(:\\s))?%{GREEDYDATA:_temp_.message_brief}$",
                                        [("_temp__tag_brief", "_temp_.tag_brief")]
                                    ),
                                    cached_grok!("^%{GREEDYDATA:message}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "grok_system_structured_brief",
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
                }
                let _cond = {
                    event.has_value("syslog_program")
                        && (!event.has_value("juniper.srx.process")
                            || event.get_str("juniper.srx.process") == Some("-"))
                };
                if _cond {
                    event.set(
                        "juniper.srx.process",
                        json!(
                            event
                                .get("syslog_program")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("_temp_.tag_brief")
                        && (!event.has_value("juniper.srx.tag")
                            || event.get_str("juniper.srx.tag") == Some("-"))
                };
                if _cond {
                    event.set(
                        "juniper.srx.tag",
                        json!(
                            event
                                .get("_temp_.tag_brief")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("_temp_.negotiation.message")
                        && event
                            .get_str("_temp_.negotiation.message")
                            .is_some_and(|s| s.starts_with("failed"))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_temp_.negotiation.message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("failed with error: ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(". ") else {
                                    break 'dissect false;
                                };
                                captured.push(("_temp_.negotiation.err_msg", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(". ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("message", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.negotiation.message".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set("_ingest.on_failure_processor_tag", "dissect_neg_failed")?;
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
                let _cond = {
                    event.has_value("_temp_.negotiation.message")
                        && event
                            .get_str("_temp_.negotiation.message")
                            .is_some_and(|s| s.starts_with("success"))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_temp_.negotiation.message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("successfully completed. ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("message", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.negotiation.message".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set("_ingest.on_failure_processor_tag", "dissect_neg_success")?;
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
                let _cond = { event.has_value("_temp_.negotiation") };
                if _cond {
                    event.rename("_temp_.negotiation", "juniper.srx.negotiation")?;
                }
                let _cond = {
                    event.has_value("_temp_.message_brief")
                        && event
                            .get_str("_temp_.message_brief")
                            .is_some_and(|s| s.starts_with("FW:"))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_temp_.message_brief") {
                            // Grok pattern: ^FW:\\s%{NOTSPACE:_temp_.fw.interface_name}\\s%{NOTSPACE:_temp_.fw.filter_action}\\s%{NOTSPACE:_temp_.fw.packet_protocol}\\s%{NOTSPACE:_temp_.fw.src_addr}\\s%{NOTSPACE:_temp_.fw.dst_addr}\\s%{NOTSPACE:_temp_.fw.src_port}\\s%{NOTSPACE:_temp_.fw.dst_port}\\s(\\(%{NOTSPACE:_temp_.fw.packets_num} packets\\))?\\s?$
                            if !cached_grok!("^FW:\\s%{NOTSPACE:_temp_.fw.interface_name}\\s%{NOTSPACE:_temp_.fw.filter_action}\\s%{NOTSPACE:_temp_.fw.packet_protocol}\\s%{NOTSPACE:_temp_.fw.src_addr}\\s%{NOTSPACE:_temp_.fw.dst_addr}\\s%{NOTSPACE:_temp_.fw.src_port}\\s%{NOTSPACE:_temp_.fw.dst_port}\\s(\\(%{NOTSPACE:_temp_.fw.packets_num} packets\\))?\\s?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set("_ingest.on_failure_processor_tag", "grok_message_brief")?;
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
                let _cond = { event.has_value("_temp_.fw") };
                if _cond {
                    event.rename("_temp_.fw", "juniper.srx.firewall")?;
                }
                let _cond = { event.has_value("juniper.srx.firewall.interface_name") };
                if _cond {
                    event.rename(
                        "juniper.srx.firewall.interface_name",
                        "juniper.srx.interface_name",
                    )?;
                }
                let _cond = {
                    event.has_value("_temp_.message_brief")
                        && event
                            .get_str("_temp_.message_brief")
                            .is_some_and(|s| s.starts_with("rtslib_dfwsm_get_async_cb:"))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_temp_.message_brief") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) =
                                    remaining.strip_prefix("rtslib_dfwsm_get_async_cb:u_data:")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" k_usr_d:") else {
                                    break 'dissect false;
                                };
                                captured.push(("_temp_.rtslib_dfwsm.u_data", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" k_usr_d:") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("_temp_.rtslib_dfwsm.k_usr_d", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.message_brief".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set("_ingest.on_failure_processor_tag", "dissect_rtslib_dfwsmr")?;
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
                let _cond = { event.has_value("_temp_.rtslib_dfwsm") };
                if _cond {
                    event.rename("_temp_.rtslib_dfwsm", "juniper.srx.rtslib_dfwsm")?;
                }
                let _cond = {
                    event.has_value("_temp_.tag_brief")
                        && event.get_str("_temp_.tag_brief") == Some("ip_mon_reth_scan")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_temp_.message_brief") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("interface ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" trigger ") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "_temp_.ip_mon_reth_scan.interface_name",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" trigger ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("_temp_.ip_mon_reth_scan.trigger", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.message_brief".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "dissect_ip_mon_reth_scan",
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
                }
                let _cond = { event.has_value("_temp_.ip_mon_reth_scan") };
                if _cond {
                    event.rename("_temp_.ip_mon_reth_scan", "juniper.srx.ip_mon_reth_scan")?;
                }
                let _cond = { event.has_value("juniper.srx.ip_mon_reth_scan.interface_name") };
                if _cond {
                    event.rename(
                        "juniper.srx.ip_mon_reth_scan.interface_name",
                        "juniper.srx.interface_name",
                    )?;
                }
                let _cond = {
                    event.has_value("_temp_.tag_brief")
                        && event.get_str("_temp_.tag_brief") == Some("dpdk_eth_devstart")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("_temp_.message_brief") {
                            // Grok pattern: ^port %{POSINT:_temp_.dpdk.port_number} (has already been started|ifd %{DATA:_temp_.dpdk.interface_name}), (new\\s)?dpdk_port_state=%{POSINT:_temp_.dpdk.port_state} dpdk_swt_port_state %{POSINT:_temp_.dpdk.swt_port_state}$
                            if !cached_grok!("^port %{POSINT:_temp_.dpdk.port_number} (has already been started|ifd %{DATA:_temp_.dpdk.interface_name}), (new\\s)?dpdk_port_state=%{POSINT:_temp_.dpdk.port_state} dpdk_swt_port_state %{POSINT:_temp_.dpdk.swt_port_state}$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set("_ingest.on_failure_processor_tag", "grok_dpdk_eth_devstart")?;
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
                let _cond = { event.has_value("_temp_.dpdk.port_number") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_temp_.dpdk.port_number") {
                            if let Some(val) = event.get("_temp_.dpdk.port_number") {
                                let converted =
                                    convert_value(val, "integer").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_temp_.dpdk.port_number".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_temp_.dpdk.port_number", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dpdk_port_number_to_int",
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
                }
                let _cond = { event.has_value("_temp_.dpdk.port_state") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_temp_.dpdk.port_state") {
                            if let Some(val) = event.get("_temp_.dpdk.port_state") {
                                let converted =
                                    convert_value(val, "integer").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_temp_.dpdk.port_state".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_temp_.dpdk.port_state", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dpdk_port_state_to_int",
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
                }
                let _cond = { event.has_value("_temp_.dpdk.swt_port_state") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_temp_.dpdk.swt_port_state") {
                            if let Some(val) = event.get("_temp_.dpdk.swt_port_state") {
                                let converted =
                                    convert_value(val, "integer").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_temp_.dpdk.swt_port_state".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_temp_.dpdk.swt_port_state", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_swt_port_state_to_int",
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
                }
                let _cond = { event.has_value("_temp_.dpdk") };
                if _cond {
                    event.rename("_temp_.dpdk", "juniper.srx.dpdk")?;
                }
                let _cond = { event.has_value("juniper.srx.dpdk.interface_name") };
                if _cond {
                    event.rename(
                        "juniper.srx.dpdk.interface_name",
                        "juniper.srx.interface_name",
                    )?;
                }
                let _cond = {
                    event.has_value("_temp_.unparsed.system_structured_brief")
                        && event.get_str("juniper.srx.tag") == Some("RTLOG_CONN_ERROR")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("_temp_.unparsed.system_structured_brief")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix(" Connection error ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "_temp_.rtlog_conn_error.stream_name",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("_temp_.rtlog_conn_error.err_msg", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.unparsed.system_structured_brief".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "dissect_tag_rtlog_conn_err",
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
                }
                let _cond = {
                    event.has_value("_temp_.rtlog_conn_error")
                        && event.get_str("juniper.srx.tag") == Some("RTLOG_CONN_ERROR")
                };
                if _cond {
                    event.rename("_temp_.rtlog_conn_error", "juniper.srx.rtlog_conn_error")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("juniper.srx.rtlog_conn_error.err_msg") {
                        if let Some(input) =
                            event.get_string("juniper.srx.rtlog_conn_error.err_msg")
                        {
                            // Grok pattern: ^(status: %{DATA:juniper.srx.rtlog_conn_error.status}, )?Error code: major %{NUMBER:juniper.srx.rtlog_conn_error.major} minor %{NUMBER:juniper.srx.rtlog_conn_error.minor} code %{NUMBER:juniper.srx.rtlog_conn_error.code}, description:%{DATA:juniper.srx.rtlog_conn_error.description}$
                            if !cached_grok!("^(status: %{DATA:juniper.srx.rtlog_conn_error.status}, )?Error code: major %{NUMBER:juniper.srx.rtlog_conn_error.major} minor %{NUMBER:juniper.srx.rtlog_conn_error.minor} code %{NUMBER:juniper.srx.rtlog_conn_error.code}, description:%{DATA:juniper.srx.rtlog_conn_error.description}$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("juniper.srx.rtlog_conn_error.status") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.rtlog_conn_error.status") {
                            if let Some(val) = event.get("juniper.srx.rtlog_conn_error.status") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.rtlog_conn_error.status".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.rtlog_conn_error.status", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rtlog_conn_error_status_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.rtlog_conn_error.major") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.rtlog_conn_error.major") {
                            if let Some(val) = event.get("juniper.srx.rtlog_conn_error.major") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.rtlog_conn_error.major".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.rtlog_conn_error.major", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rtlog_conn_error_major_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.rtlog_conn_error.minor") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.rtlog_conn_error.minor") {
                            if let Some(val) = event.get("juniper.srx.rtlog_conn_error.minor") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.rtlog_conn_error.minor".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.rtlog_conn_error.minor", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rtlog_conn_error_minor_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.rtlog_conn_error.code") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.rtlog_conn_error.code") {
                            if let Some(val) = event.get("juniper.srx.rtlog_conn_error.code") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.rtlog_conn_error.code".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.rtlog_conn_error.code", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_rtlog_conn_error_code_to_long",
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
                }
                let _cond = {
                    event.has_value("_temp_.unparsed.system_structured_brief")
                        && event.get_str("juniper.srx.tag") == Some("PING_TEST_COMPLETED")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("_temp_.unparsed.system_structured_brief")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix(" pingCtlOwnerIndex = ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", pingCtlTestName = ") else {
                                    break 'dissect false;
                                };
                                captured.push(("_temp_.ping_test.owner", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", pingCtlTestName = ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("_temp_.ping_test.name", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.unparsed.system_structured_brief".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set("_ingest.on_failure_processor_tag", "dissect_tag_ping_test")?;
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
                let _cond = {
                    event.has_value("_temp_.ping_test")
                        && event.get_str("juniper.srx.tag") == Some("PING_TEST_COMPLETED")
                };
                if _cond {
                    event.rename("_temp_.ping_test", "juniper.srx.ping_test")?;
                }
                let _cond = {
                    event.has_value("_temp_.unparsed.system_structured_brief")
                        && event.get_str("juniper.srx.tag") == Some("KERN_ARP_ADDR_CHANGE")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("_temp_.unparsed.system_structured_brief")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) =
                                    remaining.strip_prefix(" arp info overwritten for ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" from ") else {
                                    break 'dissect false;
                                };
                                captured
                                    .push(("_temp_.kern_arp_addr_change.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" from ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" to ") else {
                                    break 'dissect false;
                                };
                                captured
                                    .push(("_temp_.kern_arp_addr_change.mac1", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" to ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("_temp_.kern_arp_addr_change.mac2", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.unparsed.system_structured_brief".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "dissect")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "dissect_tag_kern_arp_addr",
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
                }
                let _cond = {
                    event.has_value("_temp_.kern_arp_addr_change")
                        && event.get_str("juniper.srx.tag") == Some("KERN_ARP_ADDR_CHANGE")
                };
                if _cond {
                    event.rename(
                        "_temp_.kern_arp_addr_change",
                        "juniper.srx.kern_arp_addr_change",
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.kern_arp_addr_change.ip") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.kern_arp_addr_change.ip") {
                            if let Some(val) = event.get("juniper.srx.kern_arp_addr_change.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.kern_arp_addr_change.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.kern_arp_addr_change.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_kern_arp_ip_to_ip",
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
                }
                let _cond = { event.has_value("message") && event.get_str("message") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(kv_str) = event.get_string("message") {
                                for pair in cached_regex!(",\\s(?=[a-zA-Z0-9\\_\\-\\s]+:)")
                                    .split(&kv_str)
                                    .into_iter()
                                {
                                    if pair.trim().is_empty() {
                                        continue;
                                    }
                                    let Some((key, value)) = pair.split_once(":") else {
                                        return Err(TransformError::ParseError {
                                            path: "message".into(),
                                            message: format!(
                                                "does not contain value_split: {pair}"
                                            ),
                                        });
                                    };
                                    {
                                        let value = value.trim_matches(|c| "\"".contains(c));
                                        if !key.is_empty() {
                                            kv_put(
                                                event,
                                                &format!("juniper.srx.system.{}", key),
                                                value,
                                            )?;
                                        }
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("juniper.srx.system") };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.juniper.srx.system = ctx.juniper.srx.system.entrySet().stream().collect(Collectors.toMap(e -> e.getKey().replace(' ', '_').replace('-', '_').toLowerCase(), e -> e.getValue().trim()));
                    guarded_replace(
                        event,
                        &GuardedReplace::new(
                            "juniper.srx.system.entrySet().stream().collect(Collectors.toMap(e -> e.getKey()",
                            "juniper.srx.system",
                            " ",
                            "_",
                        ),
                    );
                }
                let _cond = { event.has_value("juniper.srx.system.aux_spi") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.system.aux_spi") {
                            if let Some(val) = event.get("juniper.srx.system.aux_spi") {
                                let converted =
                                    convert_value(val, "integer").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "juniper.srx.system.aux_spi".into(),
                                            message,
                                        }
                                    })?;
                                event.set("juniper.srx.system.aux_spi", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_aux_spi_to_int")?;
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
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("juniper.srx.system.ike_version") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.system.ike_version") {
                            if let Some(val) = event.get("juniper.srx.system.ike_version") {
                                let converted =
                                    convert_value(val, "integer").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "juniper.srx.system.ike_version".into(),
                                            message,
                                        }
                                    })?;
                                event.set("juniper.srx.system.ike_version", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_ike_version_to_int",
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
                }
                let _cond = { event.has_value("juniper.srx.system.local_gateway") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.system.local_gateway") {
                            if let Some(val) = event.get("juniper.srx.system.local_gateway") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.system.local_gateway".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.system.local_gateway", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_local_gateway_to_ip",
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
                }
                let _cond = { event.has_value("juniper.srx.system.remote_gateway") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.system.remote_gateway") {
                            if let Some(val) = event.get("juniper.srx.system.remote_gateway") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.system.remote_gateway".into(),
                                        message,
                                    }
                                })?;
                                event.set("juniper.srx.system.remote_gateway", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_remote_gateway_to_ip",
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
                }
                let _cond = { event.has_value("juniper.srx.system") };
                if _cond {
                    // Painless script
                    // Source: ctx?.juniper?.srx?.system.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx?.juniper?.srx?.system.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));"#
                        ),
                        cached_params!(
                            "{\"values\":[\"None\",\"UNKNOWN\",\"N/A\",\"-\",\"Not-Available\"]}"
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("_temp_.message_brief") && !event.has_value("message") };
                if _cond {
                    if let Some(v) = event.get("_temp_.message_brief").cloned() {
                        event.set("message", v)?;
                    }
                }
                event.set("juniper.srx.log_type", json!("system"))?;
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                let _cond = { event.get_str("juniper.srx.tag") == Some("SSHD_LOGIN_FAILED") };
                if _cond {
                    event.append("event.category", json!("authentication"))?;
                }
                let _cond = { event.get_str("juniper.srx.firewall.filter_action") == Some("A") };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                }
                let _cond = { event.get_str("juniper.srx.firewall.filter_action") == Some("D") };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = { event.get_str("juniper.srx.firewall.filter_action") == Some("R") };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                }
                let _cond = { event.has_value("juniper.srx.ike_negotiation") };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                let _cond = { event.has_value("juniper.srx.rtslib_dfwsm") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { event.has_value("juniper.srx.rtlog_conn_error") };
                if _cond {
                    event.append("event.type", json!("error"))?;
                    event.append("event.type", json!("connection"))?;
                }
                let _cond = { event.has_value("juniper.srx.ping_test") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { event.has_value("juniper.srx.ping_test") };
                if _cond {
                    event.append("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("juniper.srx.tag") == Some("SSHD_LOGIN_FAILED") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { event.has_value("juniper.srx.remote_address") };
                if _cond {
                    if event.has_value("juniper.srx.remote_address") {
                        event.rename("juniper.srx.remote_address", "destination.ip")?;
                    }
                }
                let _cond = {
                    !event.has_value("destination.ip")
                        && event.has_value("juniper.srx.destination_address")
                };
                if _cond {
                    if event.has_value("juniper.srx.destination_address") {
                        event.rename("juniper.srx.destination_address", "destination.ip")?;
                    }
                }
                let _cond = {
                    !event.has_value("destination.ip")
                        && event.has_value("juniper.srx.firewall.dst_addr")
                };
                if _cond {
                    if event.has_value("juniper.srx.firewall.dst_addr") {
                        event.rename("juniper.srx.firewall.dst_addr", "destination.ip")?;
                    }
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    event.set(
                        "server.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_remote_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_remote_address") {
                        event.rename("juniper.srx.nat_remote_address", "destination.nat.ip")?;
                    }
                }
                let _cond = {
                    !event.has_value("destination.nat.ip")
                        && event.has_value("juniper.srx.nat_destination_address")
                };
                if _cond {
                    if event.has_value("juniper.srx.nat_destination_address") {
                        event
                            .rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.destination_port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.destination_port") {
                            if let Some(val) = event.get("juniper.srx.destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_port_to_long",
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
                }
                let _cond = {
                    !event.has_value("destination.port")
                        && event.has_value("juniper.srx.firewall.dst_port")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.firewall.dst_port") {
                            if let Some(val) = event.get("juniper.srx.firewall.dst_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.firewall.dst_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_firewall_destination_port_to_long",
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
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    event.set(
                        "server.port",
                        json!(
                            event
                                .get("destination.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("server.port") {
                            if let Some(val) = event.get("server.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_port_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.nat_destination_port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_destination_port") {
                            if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_destination_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nat_destination_port_to_long",
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
                }
                let _cond = { event.has_value("destination.nat.port") };
                if _cond {
                    event.set(
                        "server.nat.port",
                        json!(
                            event
                                .get("destination.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.nat.port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("server.nat.port") {
                            if let Some(val) = event.get("server.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_nat_port_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.inbound_bytes") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.inbound_bytes") {
                            if let Some(val) = event.get("juniper.srx.inbound_bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.inbound_bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_inbound_bytes_to_long",
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
                }
                let _cond = { event.has_value("destination.bytes") };
                if _cond {
                    event.set(
                        "server.bytes",
                        json!(
                            event
                                .get("destination.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.bytes") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("server.bytes") {
                            if let Some(val) = event.get("server.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_bytes_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.inbound_packets") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.inbound_packets") {
                            if let Some(val) = event.get("juniper.srx.inbound_packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.inbound_packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.packets", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_inbound_packets_to_long",
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
                }
                let _cond = { event.has_value("destination.packets") };
                if _cond {
                    event.set(
                        "server.packets",
                        json!(
                            event
                                .get("destination.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("server.packets") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("server.packets") {
                            if let Some(val) = event.get("server.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.packets", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_server_packets_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.local_address") };
                if _cond {
                    if event.has_value("juniper.srx.local_address") {
                        event.rename("juniper.srx.local_address", "source.ip")?;
                    }
                }
                let _cond = {
                    !event.has_value("source.ip") && event.has_value("juniper.srx.source_address")
                };
                if _cond {
                    if event.has_value("juniper.srx.source_address") {
                        event.rename("juniper.srx.source_address", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.firewall.src_addr") };
                if _cond {
                    if event.has_value("juniper.srx.firewall.src_addr") {
                        event.rename("juniper.srx.firewall.src_addr", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.set(
                        "client.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.nat_local_address") };
                if _cond {
                    if event.has_value("juniper.srx.nat_local_address") {
                        event.rename("juniper.srx.nat_local_address", "source.nat.ip")?;
                    }
                }
                let _cond = {
                    !event.has_value("source.nat.ip")
                        && event.has_value("juniper.srx.nat_source_address")
                };
                if _cond {
                    if event.has_value("juniper.srx.nat_source_address") {
                        event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.sourceip") };
                if _cond {
                    if event.has_value("juniper.srx.sourceip") {
                        event.rename("juniper.srx.sourceip", "source.ip")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.source_port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.source_port") {
                            if let Some(val) = event.get("juniper.srx.source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_source_port_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.firewall.src_port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.firewall.src_port") {
                            if let Some(val) = event.get("juniper.srx.firewall.src_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.firewall.src_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_firewall_src_port_to_long",
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
                }
                let _cond = { event.has_value("source.port") };
                if _cond {
                    event.set(
                        "client.port",
                        json!(
                            event
                                .get("source.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
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
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_port_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.nat_source_port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.nat_source_port") {
                            if let Some(val) = event.get("juniper.srx.nat_source_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.nat_source_port".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nat_source_port_to_long",
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
                }
                let _cond = { event.has_value("source.nat.port") };
                if _cond {
                    event.set(
                        "client.nat.port",
                        json!(
                            event
                                .get("source.nat.port")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.nat.port") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("client.nat.port") {
                            if let Some(val) = event.get("client.nat.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.nat.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.nat.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_nat_port",
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
                }
                let _cond = { event.has_value("juniper.srx.outbound_bytes") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.outbound_bytes") {
                            if let Some(val) = event.get("juniper.srx.outbound_bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.outbound_bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_outbounds_bytes_to_long",
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
                }
                let _cond = { event.has_value("source.bytes") };
                if _cond {
                    event.set(
                        "client.bytes",
                        json!(
                            event
                                .get("source.bytes")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.bytes") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("client.bytes") {
                            if let Some(val) = event.get("client.bytes") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.bytes".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.bytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_bytes_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.outbound_packets") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.outbound_packets") {
                            if let Some(val) = event.get("juniper.srx.outbound_packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.outbound_packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_outbound_packets_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.firewall.packets_num") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("juniper.srx.firewall.packets_num") {
                            if let Some(val) = event.get("juniper.srx.firewall.packets_num") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "juniper.srx.firewall.packets_num".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.packets", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_firewall_packets_to_long",
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
                }
                let _cond = { event.has_value("source.packets") };
                if _cond {
                    event.set(
                        "client.packets",
                        json!(
                            event
                                .get("source.packets")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("client.packets") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("client.packets") {
                            if let Some(val) = event.get("client.packets") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.packets".into(),
                                        message,
                                    }
                                })?;
                                event.set("client.packets", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_packets_to_long",
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
                }
                let _cond = { event.has_value("juniper.srx.username") };
                if _cond {
                    if event.has_value("juniper.srx.username") {
                        event.rename("juniper.srx.username", "source.user.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.system.local_gateway") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("juniper.srx.system.local_gateway")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("juniper.srx.system.remote_gateway") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("juniper.srx.system.remote_gateway")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("juniper.srx.interface_name") {
                    event.rename(
                        "juniper.srx.interface_name",
                        "observer.ingress.interface.name",
                    )?;
                }
                let _cond =
                    { event.has_value("syslog_hostname") && !event.has_value("observer.name") };
                if _cond {
                    if event.has_value("syslog_hostname") {
                        event.rename("syslog_hostname", "observer.name")?;
                    }
                }
                if let Some(v) = event
                    .get("observer.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                let _cond = { event.has_value("juniper.srx.rulebase_name") };
                if _cond {
                    if event.has_value("juniper.srx.rulebase_name") {
                        event.rename("juniper.srx.rulebase_name", "rule.name")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.rule_name") };
                if _cond {
                    if event.has_value("juniper.srx.rule_name") {
                        event.rename("juniper.srx.rule_name", "rule.id")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.protocol_name") };
                if _cond {
                    if event.has_value("juniper.srx.protocol_name") {
                        event.rename("juniper.srx.protocol_name", "network.protocol")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.firewall.packet_protocol") };
                if _cond {
                    if event.has_value("juniper.srx.firewall.packet_protocol") {
                        event
                            .rename("juniper.srx.firewall.packet_protocol", "network.transport")?;
                    }
                }
                let _cond = { event.has_value("juniper.srx.message") };
                if _cond {
                    if event.has_value("juniper.srx.message") {
                        event.rename("juniper.srx.message", "message")?;
                    }
                }
                let _cond = {
                    event.has_value("juniper.srx.process")
                        && ["-", "N/A", "UNKNOWN", "None"]
                            .contains(&event.get_str("juniper.srx.process").unwrap_or(""))
                };
                if _cond {
                    if event.remove("juniper.srx.process").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "juniper.srx.process".into(),
                        });
                    }
                }
                event.remove("syslog_program");
                event.remove("syslog_hostname");
                event.remove("tag");
                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.outbound_bytes");
                event.remove("juniper.srx.outbound_packets");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.inbound_bytes");
                event.remove("juniper.srx.inbound_packets");
                event.remove("juniper.srx.firewall");
                // End nested pipeline: "system"
            }

            let _cond = { event.has_value("network.iana_number") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def iana_number = ctx.network.iana_number;\nif (iana_number == '0') {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def iana_number = ctx.network.iana_number;\nif (iana_number == '0') {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("source.user.name")
                    && event
                        .get_str("source.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("source.user.name", "source.user.email")?;
            }

            let _cond =
                { !event.has_value("source.user.name") && !event.has_value("source.user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.user.email") {
                        if let Some(input) = event.get_string("source.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("source.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("source.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.nat.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.nat.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("url.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("source.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("destination.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("juniper.srx.process") };
            if _cond {
                event.set(
                    "process.name",
                    json!(
                        event
                            .get("juniper.srx.process")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("syslog_pid") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("syslog_pid") {
                        if let Some(val) = event.get("syslog_pid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "syslog_pid".into(),
                                    message,
                                }
                            })?;
                            event.set("syslog_pid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_syslog_pid_to_long",
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
            }

            if event.has_value("syslog_pid") {
                event.rename("syslog_pid", "process.pid")?;
            }

            event.remove("_temp_");
            event.remove("juniper.srx.duration");
            event.remove("juniper.srx.dir_disp");
            event.remove("juniper.srx.srczone");
            event.remove("juniper.srx.dstzone");
            event.remove("syslog_pri");

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
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
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
