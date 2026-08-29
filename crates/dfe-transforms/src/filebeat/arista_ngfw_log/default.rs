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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: <%{NONNEGINT:log.syslog.priority:int}>%{SYSLOGTIMESTAMP:_temp_.raw_date} %{WORD}  %{NOTSPACE}\\:[\\s]+%{GREEDYDATA:_temp_.full_message}
                let _ = cached_grok!("<%{NONNEGINT:log.syslog.priority:int}>%{SYSLOGTIMESTAMP:_temp_.raw_date} %{WORD}  %{NOTSPACE}\\:[\\s]+%{GREEDYDATA:_temp_.full_message}").extract_into(&input, event)?;
            }

            // Painless script
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx.log?.syslog?.facility?.code == null || !params.containsKey((ctx.log.syslog.facility.code).toString())) {\n  return;\n}\nctx.log.syslog.facility.name = params[(ctx.log.syslog.facility.code).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.facility?.code == null || !params.containsKey((ctx.log.syslog.facility.code).toString())) {\n  return;\n}\nctx.log.syslog.facility.name = params[(ctx.log.syslog.facility.code).toString()];"#
                ),
                cached_params!(
                    "{\"0\":\"Kernel\",\"1\":\"User\",\"2\":\"Mail\",\"3\":\"System\",\"4\":\"Security\",\"5\":\"Syslog\",\"6\":\"Line printer\",\"7\":\"Network news\",\"8\":\"UUCP\",\"9\":\"Clock\",\"10\":\"Security\",\"11\":\"FTPd\",\"12\":\"NTPd\",\"13\":\"Log audit\",\"14\":\"Log alert\",\"15\":\"Clock daemon\",\"16\":\"Local 0\",\"17\":\"Local 1\",\"18\":\"Local 2\",\"19\":\"Local 3\",\"20\":\"Local 4\",\"21\":\"Local 5\",\"22\":\"Local 6\",\"23\":\"Local 7\"}"
                ),
            )?;

            // Painless script
            // Source: if (ctx.log?.syslog?.severity?.code == null || !params.containsKey((ctx.log.syslog.severity.code).toString())) {\n  return;\n}\nctx.log.syslog.severity.name = params[(ctx.log.syslog.severity.code).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.severity?.code == null || !params.containsKey((ctx.log.syslog.severity.code).toString())) {\n  return;\n}\nctx.log.syslog.severity.name = params[(ctx.log.syslog.severity.code).toString()];"#
                ),
                cached_params!(
                    "{\"0\":\"Emergency\",\"1\":\"Alert\",\"2\":\"Critical\",\"3\":\"Error\",\"4\":\"Warning\",\"5\":\"Notice\",\"6\":\"Informational\",\"7\":\"Debug\"}"
                ),
            )?;

            let _cond = {
                event.has_value("log.source.address")
                    && event.get_str("log.source.address") != Some("")
            };
            if _cond {
                gsub_field(
                    event,
                    "log.source.address",
                    "log.syslog.hostname",
                    cached_regex!(":.*"),
                    "",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "_temp_.full_message", "arista")?;
                Ok(())
            })();

            // SKIPPED: condition not transpiled: Collection supportedClasses = [ 'class com.untangle.uvm.event.AdminLoginEvent', 'class com.untangle.app.firewall.FirewallEvent', 'class com.untangle.app.http.HttpRequestEvent', 'class com.untangle.app ...
            #[allow(unreachable_code, unused_variables)]
            if false {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some("class com.untangle.uvm.event.AdminLoginEvent")
            };
            if _cond {
                // Begin nested pipeline: "admin_login"
                if event.has_value("arista.reason") {
                    event.rename("arista.reason", "event.reason")?;
                }
                if event.has_value("arista.login") {
                    event.rename("arista.login", "user.name")?;
                }
                if event.has_value("arista.clientAddress") {
                    event.rename("arista.clientAddress", "source.ip")?;
                }
                // Painless script
                // Source: if (ctx?.event == null) {\n  Map map = new HashMap();\n  ctx.put('event', map);\n} if (ctx.arista?.succeeded == null || !params.containsKey((ctx.arista.succeeded).toString())) {\n  return;\n} ctx.event.category = params.get((ctx.arista.succeeded).toString()).get('category').clone(); ctx.event.kind = params.get((ctx.arista.succeeded).toString()).get('kind'); ctx.event.outcome = params.get((ctx.arista.succeeded).toString()).get('outcome'); ctx.event.type = params.get((ctx.arista.succeeded).toString()).get('type').clone(); ctx.event.provider = params.get((ctx.arista.succeeded).toString()).get('provider'); ctx.arista.remove('succeeded');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx?.event == null) {\n  Map map = new HashMap();\n  ctx.put('event', map);\n} if (ctx.arista?.succeeded == null || !params.containsKey((ctx.arista.succeeded).toString())) {\n  return;\n} ctx.event.category = params.get((ctx.arista.succeeded).toString()).get('category').clone(); ctx.event.kind = params.get((ctx.arista.succeeded).toString()).get('kind'); ctx.event.outcome = params.get((ctx.arista.succeeded).toString()).get('outcome'); ctx.event.type = params.get((ctx.arista.succeeded).toString()).get('type').clone(); ctx.event.provider = params.get((ctx.arista.succeeded).toString()).get('provider'); ctx.arista.remove('succeeded');"#
                    ),
                    cached_params!(
                        "{\"false\":{\"category\":[\"network\",\"authentication\",\"iam\"],\"kind\":\"event\",\"outcome\":\"failure\",\"type\":[\"denied\"],\"provider\":\"admin_login\"},\"true\":{\"category\":[\"network\",\"authentication\",\"iam\"],\"kind\":\"event\",\"outcome\":\"success\",\"type\":[\"allowed\"],\"provider\":\"admin_login\"}}"
                    ),
                )?;
                event.remove("arista.local");
                // End nested pipeline: "admin_login"
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some("class com.untangle.uvm.logging.InterfaceStatEvent")
            };
            if _cond {
                // Begin nested pipeline: "interface_stats"
                if event.has_value("arista.interfaceId") {
                    event.rename("arista.interfaceId", "arista.interface.id")?;
                }
                if event.has_value("arista.rxBytes") {
                    event.rename("arista.rxBytes", "arista.received.bytes")?;
                }
                if event.has_value("arista.rxRate") {
                    event.rename("arista.rxRate", "arista.received.rate")?;
                }
                if event.has_value("arista.received.rate") {
                    if let Some(val) = event.get("arista.received.rate") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "arista.received.rate".into(),
                                message,
                            }
                        })?;
                        event.set("arista.received.rate", converted)?;
                    }
                }
                if event.has_value("arista.txBytes") {
                    event.rename("arista.txBytes", "arista.transmitted.bytes")?;
                }
                if event.has_value("arista.txRate") {
                    event.rename("arista.txRate", "arista.transmitted.rate")?;
                }
                if event.has_value("arista.transmitted.rate") {
                    if let Some(val) = event.get("arista.transmitted.rate") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "arista.transmitted.rate".into(),
                                message,
                            }
                        })?;
                        event.set("arista.transmitted.rate", converted)?;
                    }
                }
                // End nested pipeline: "interface_stats"
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some("class com.untangle.uvm.logging.SystemStatEvent")
            };
            if _cond {
                // Begin nested pipeline: "system_stats"
                if event.has_value("arista.activeHosts") {
                    event.rename("arista.activeHosts", "arista.hosts.active")?;
                }
                if event.has_value("arista.diskTotal") {
                    event.rename("arista.diskTotal", "arista.disk.total.bytes")?;
                }
                if event.has_value("arista.diskUsed") {
                    event.rename("arista.diskUsed", "arista.disk.used.bytes")?;
                }
                if event.has_value("arista.diskFree") {
                    event.rename("arista.diskFree", "arista.disk.free.bytes")?;
                }
                if event.has_value("arista.diskUsedPercent") {
                    event.rename("arista.diskUsedPercent", "arista.disk.used.pct")?;
                }
                if event.has_value("arista.diskFreePercent") {
                    event.rename("arista.diskFreePercent", "arista.disk.free.pct")?;
                }
                if event.has_value("arista.cpuSystem") {
                    event.rename("arista.cpuSystem", "arista.cpu.system.pct")?;
                }
                if event.has_value("arista.cpuUser") {
                    event.rename("arista.cpuUser", "arista.cpu.user.pct")?;
                }
                let _cond = {
                    event.has_value("arista.cpu.system.pct")
                        && event.has_value("arista.cpu.user.pct")
                };
                if _cond {
                    // Painless script
                    // Source: if (ctx.arista?.cpu?.total == null) {\n  Map map = new HashMap();\n  ctx.arista.cpu.put('total', map);\n}\nif (ctx.arista?.cpu?.system?.pct != null && ctx.arista?.cpu?.user?.pct != null) {\n  ctx.arista.cpu.total.pct = (ctx.arista.cpu.system.pct + ctx.arista.cpu.user.pct);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.arista?.cpu?.total == null) {\n  Map map = new HashMap();\n  ctx.arista.cpu.put('total', map);\n}\nif (ctx.arista?.cpu?.system?.pct != null && ctx.arista?.cpu?.user?.pct != null) {\n  ctx.arista.cpu.total.pct = (ctx.arista.cpu.system.pct + ctx.arista.cpu.user.pct);\n}"#
                        ),
                    )?;
                }
                if event.has_value("arista.load1") {
                    event.rename("arista.load1", "arista.cpu.load.1")?;
                }
                if event.has_value("arista.load5") {
                    event.rename("arista.load5", "arista.cpu.load.5")?;
                }
                if event.has_value("arista.load15") {
                    event.rename("arista.load15", "arista.cpu.load.15")?;
                }
                if event.has_value("arista.memTotal") {
                    event.rename("arista.memTotal", "arista.memory.total.bytes")?;
                }
                if event.has_value("arista.memUsed") {
                    event.rename("arista.memUsed", "arista.memory.used.bytes")?;
                }
                if event.has_value("arista.memFree") {
                    event.rename("arista.memFree", "arista.memory.free.bytes")?;
                }
                if event.has_value("arista.memCache") {
                    event.rename("arista.memCache", "arista.memory.cache.bytes")?;
                }
                if event.has_value("arista.memUsedPercent") {
                    event.rename("arista.memUsedPercent", "arista.memory.used.pct")?;
                }
                if event.has_value("arista.memFreePercent") {
                    event.rename("arista.memFreePercent", "arista.memory.free.pct")?;
                }
                if event.has_value("arista.memBuffers") {
                    event.rename("arista.memBuffers", "arista.memory.buffers")?;
                }
                if event.has_value("arista.swapTotal") {
                    event.rename("arista.swapTotal", "arista.memory.swap.total.bytes")?;
                }
                if event.has_value("arista.swapUsed") {
                    event.rename("arista.swapUsed", "arista.memory.swap.used.bytes")?;
                }
                if event.has_value("arista.swapFree") {
                    event.rename("arista.swapFree", "arista.memory.swap.free.bytes")?;
                }
                if event.has_value("arista.swapUsedPercent") {
                    event.rename("arista.swapUsedPercent", "arista.memory.swap.used.pct")?;
                }
                if event.has_value("arista.swapFreePercent") {
                    event.rename("arista.swapFreePercent", "arista.memory.swap.free.pct")?;
                }
                // End nested pipeline: "system_stats"
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some("class com.untangle.app.web_filter.WebFilterEvent")
            };
            if _cond {
                // Begin nested pipeline: "web_filter"
                let _cond = {
                    event.has_value("arista.requestLine")
                        && event.get("arista.requestLine").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" "))
                            }
                            serde_json::Value::String(s) => s.contains(" "),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("arista.requestLine") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("http.request.method", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp.url_full", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "arista.requestLine".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = { event.has_value("_temp.url_full") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        uri_parts(event, "_temp.url_full", "url", true, false)?;
                        Ok(())
                    })();
                }
                if event.has_value("_temp.url_full") {
                    event.rename("_temp.url_full", "url.full")?;
                }
                event.remove("arista.appName");
                // End nested pipeline: "web_filter"
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some(
                        "class com.untangle.app.intrusion_prevention.IntrusionPreventionLogEvent",
                    )
            };
            if _cond {
                // Begin nested pipeline: "intrusion_prevention"
                if event.has_value("arista.msg") {
                    event.rename("arista.msg", "rule.name")?;
                }
                if event.has_value("arista.ipDestination") {
                    event.rename("arista.ipDestination", "destination.ip")?;
                }
                if event.has_value("arista.ipSource") {
                    event.rename("arista.ipSource", "source.ip")?;
                }
                if event.has_value("arista.classtype") {
                    event.rename("arista.classtype", "rule.ruleset")?;
                }
                if event.has_value("arista.category") {
                    event.rename("arista.category", "rule.category")?;
                }
                event.remove("rule.id");
                if event.has_value("arista.signatureId") {
                    event.rename("arista.signatureId", "rule.id")?;
                }
                if event.has_value("rule.id") {
                    if let Some(val) = event.get("rule.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "rule.id".into(),
                                message,
                            }
                        })?;
                        event.set("rule.id", converted)?;
                    }
                }
                event.remove("arista.dportIcode");
                event.remove("arista.generatorId");
                event.remove("arista.ruleId");
                event.remove("arista.signatureId");
                event.remove("arista.sportItype");
                event.remove("arista.timestamp");
                // End nested pipeline: "intrusion_prevention"
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some("class com.untangle.app.http.HttpRequestEvent")
                    || event.get_str("arista.class")
                        == Some("class com.untangle.app.http.HttpResponseEvent")
            };
            if _cond {
                // Begin nested pipeline: "http_event"
                let _cond = { event.has_value("arista.httpRequestEvent.timeStamp") };
                if _cond {
                    event.remove("arista.httpRequestEvent.timeStamp");
                }
                let _cond = { event.has_value("arista.httpRequestEvent.sessionEvent.timeStamp") };
                if _cond {
                    event.remove("arista.httpRequestEvent.sessionEvent.timeStamp");
                }
                let _cond = { event.has_value("arista.httpRequestEvent.sessionEvent") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject =
                                event.get("arista.httpRequestEvent.sessionEvent").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(from) = resolve_path(event, "_ingest._value")
                                            && let Some(to) =
                                                resolve_path(event, "arista.{{{_ingest._key}}}")
                                            && event.has(&from)
                                        {
                                            event.rename(&from, &to)?;
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "rename")?;
                                        if event.remove("_ingest._key").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._key".into(),
                                            });
                                        }
                                        event.append(
                                            "error.message",
                                            json!(
                                                event
                                                    .get("_ingest.on_failure_message")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "arista.httpRequestEvent.sessionEvent",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                        Ok(())
                    })();
                }
                event.remove("arista.httpRequestEvent.contentLength");
                if event.has_value("arista.httpRequestEvent.domain") {
                    event.rename("arista.httpRequestEvent.domain", "destination.domain")?;
                }
                if event.has_value("arista.httpRequestEvent.method") {
                    event.rename("arista.httpRequestEvent.method", "http.request.method")?;
                }
                if event.has_value("arista.httpRequestEvent.requestId") {
                    event.rename("arista.httpRequestEvent.requestId", "arista.requestId")?;
                }
                if event.has_value("arista.httpRequestEvent.requestUri") {
                    event.rename("arista.httpRequestEvent.requestUri", "arista.requestUri")?;
                }
                if event.has_value("arista.httpRequestEvent.timeStamp") {
                    event.rename("arista.httpRequestEvent.timeStamp", "arista.timeStamp")?;
                }
                if event.has_value("arista.domain") {
                    event.rename("arista.domain", "destination.domain")?;
                }
                if event.has_value("arista.method") {
                    event.rename("arista.method", "http.request.method")?;
                }
                let _cond = {
                    event.get_str("arista.class")
                        == Some("class com.untangle.app.http.HttpRequestEvent")
                };
                if _cond {
                    if event.has_value("arista.contentLength") {
                        event.rename("arista.contentLength", "http.request.bytes")?;
                    }
                }
                let _cond = {
                    event.get_str("arista.class")
                        == Some("class com.untangle.app.http.HttpResponseEvent")
                };
                if _cond {
                    if event.has_value("arista.contentLength") {
                        event.rename("arista.contentLength", "http.response.bytes")?;
                    }
                }
                if event.has_value("arista.requestUri") {
                    event.rename("arista.requestUri", "url.path")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("arista.contentFilename", "file.name")?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("arista.requestLine")
                        && event.get("arista.requestLine").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" "))
                            }
                            serde_json::Value::String(s) => s.contains(" "),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("arista.requestLine") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("http.request.method", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_temp.url_full", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "arista.requestLine".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = { event.has_value("_temp.url_full") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        uri_parts(event, "_temp.url_full", "url", true, false)?;
                        Ok(())
                    })();
                }
                if event.has_value("_temp.url_full") {
                    event.rename("_temp.url_full", "url.full")?;
                }
                event.remove("arista.contentType");
                event.remove("arista.host");
                event.remove("arista.httpRequestEvent.host");
                event.remove("arista.requestId");
                // End nested pipeline: "http_event"
            }

            let _cond = {
                event.get_str("arista.class")
                    == Some("class com.untangle.uvm.app.SessionStatsEvent")
            };
            if _cond {
                // Begin nested pipeline: "session_stats"
                if event.has_value("arista.endTime") {
                    event.rename("arista.endTime", "event.end")?;
                }
                if event.has_value("arista.sessionEvent.timeStamp") {
                    event.rename("arista.sessionEvent.timeStamp", "event.start")?;
                }
                event.remove("arista.p2cBytes");
                event.remove("arista.p2sBytes");
                event.remove("arista.sessionEvent.sessionId");
                // End nested pipeline: "session_stats"
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    event.set("_temp_.tz", v)?;
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(v) = event.get("event.timezone").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            if !event.has("_temp_.tz") {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            if let Some(v) = event.get("_temp_.tz").cloned() {
                if !event.has("event.timezone") {
                    event.set("event.timezone", v)?;
                }
            }

            let _cond = { event.has_value("arista.timeStamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("arista.timeStamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd HH:mm:ss.SSS",
                                "yyyy-MM-dd HH:mm:ss.SS",
                                "yyyy-MM-dd HH:mm:ss.S",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "arista.timeStamp".into(),
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
                        "date_arista_timeStamp_8317ee3e",
                    )?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("arista.timeStamp") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("arista.timeStamp") {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "ISO8601",
                                        "yyyy-MM-dd HH:mm:ss.SSS",
                                        "yyyy-MM-dd HH:mm:ss.SS",
                                        "yyyy-MM-dd HH:mm:ss.S",
                                    ],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set("@timestamp", parsed)?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "arista.timeStamp".into(),
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
                                "date_arista_timeStamp_24497be9",
                            )?;
                            event.append("error.message", json!(format!("Error parsing date from field `arista.timeStamp`. Value of field: {}: {}", event.get("arista.timeStamp").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond =
                { event.has_value("_temp_.raw_date") && !event.has_value("arista.timeStamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "MMMM dd HH:mm:ss"],
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__temp__raw_date_d69e2957",
                    )?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("_temp_.raw_date") };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "ISO8601",
                                        "yyyy-MM-dd HH:mm:ss.SSS",
                                        "yyyy-MM-dd HH:mm:ss.SS",
                                        "yyyy-MM-dd HH:mm:ss.S",
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
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "date__temp__raw_date_092b278a",
                            )?;
                            event.append("error.message", json!(format!("Error parsing date from syslog timestamp. Value of field: {}{}", event.get("_temp_.raw_date").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("arista.sessionEvent.timeStamp") };
            if _cond {
                event.remove("arista.sessionEvent.timeStamp");
            }

            let _cond = { event.has_value("arista.sessionEvent") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("arista.sessionEvent").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(from) = resolve_path(event, "_ingest._value")
                                        && let Some(to) =
                                            resolve_path(event, "arista.{{{_ingest._key}}}")
                                        && event.has(&from)
                                    {
                                        event.rename(&from, &to)?;
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "rename")?;
                                    if event.remove("_ingest._key").is_none() {
                                        return Err(TransformError::FieldNotFound {
                                            path: "_ingest._key".into(),
                                        });
                                    }
                                    event.append(
                                        "error.message",
                                        json!(
                                            event
                                                .get("_ingest.on_failure_message")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "arista.sessionEvent",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            if event.has_value("arista.CClientAddr") {
                event.rename("arista.CClientAddr", "source.ip")?;
            }

            if event.has_value("source.ip") {
                if let Some(val) = event.get("source.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("arista.CClientPort") {
                event.rename("arista.CClientPort", "source.port")?;
            }

            if event.has_value("arista.SClientAddr") {
                event.rename("arista.SClientAddr", "source.nat.ip")?;
            }

            if event.has_value("source.nat.ip") {
                if let Some(val) = event.get("source.nat.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.nat.ip".into(),
                            message,
                        })?;
                    event.set("source.nat.ip", converted)?;
                }
            }

            if event.has_value("arista.SClientPort") {
                event.rename("arista.SClientPort", "source.nat.port")?;
            }

            if event.has_value("arista.c2pBytes") {
                event.rename("arista.c2pBytes", "source.bytes")?;
            }

            if event.has_value("source.bytes") {
                if let Some(val) = event.get("source.bytes") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("source.bytes", converted)?;
                }
            }

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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("arista.CServerAddr", "destination.ip")?;
                Ok(())
            })();

            if event.has_value("destination.ip") {
                if let Some(val) = event.get("destination.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.ip".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("arista.CServerPort", "destination.port")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("arista.SServerAddr", "destination.nat.ip")?;
                Ok(())
            })();

            if event.has_value("destination.nat.ip") {
                if let Some(val) = event.get("destination.nat.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.nat.ip".into(),
                            message,
                        })?;
                    event.set("destination.nat.ip", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("arista.SServerPort", "destination.nat.port")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("arista.s2pBytes", "destination.bytes")?;
                Ok(())
            })();

            if event.has_value("destination.bytes") {
                if let Some(val) = event.get("destination.bytes") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("destination.bytes", converted)?;
                }
            }

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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            if event.has_value("arista.reason") {
                event.rename("arista.reason", "event.reason")?;
            }

            if event.has_value("arista.sessionId") {
                event.rename("arista.sessionId", "event.id")?;
            }

            if event.has_value("event.id") {
                if let Some(val) = event.get("event.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.id".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
            }

            // Painless script
            // Source: if (ctx?.event == null) {\n  Map map = new HashMap();\n  ctx.put('event', map);\n}\nif (ctx.arista?.blocked != null) {\n  if (ctx.arista.blocked) {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  } else {\n    ctx.event.outcome = 'success';\n    ctx.event.type = 'allowed';\n  }\n}\nif (ctx.arista?.filterPrefix != null) {\n  if (ctx.arista.filterPrefix == 'filter_blocked') {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  } else if (ctx.arista.filterPrefix == 'invalid_blocked') {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  } else if (ctx.arista.filterPrefix == 'shield_blocked') {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx?.event == null) {\n  Map map = new HashMap();\n  ctx.put('event', map);\n}\nif (ctx.arista?.blocked != null) {\n  if (ctx.arista.blocked) {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  } else {\n    ctx.event.outcome = 'success';\n    ctx.event.type = 'allowed';\n  }\n}\nif (ctx.arista?.filterPrefix != null) {\n  if (ctx.arista.filterPrefix == 'filter_blocked') {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  } else if (ctx.arista.filterPrefix == 'invalid_blocked') {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  } else if (ctx.arista.filterPrefix == 'shield_blocked') {\n    ctx.event.outcome = 'failure';\n    ctx.event.type = 'denied';\n  }\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("event.start") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.start") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd HH:mm:ss.SSS",
                                "yyyy-MM-dd HH:mm:ss.SS",
                                "yyyy-MM-dd HH:mm:ss.S",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.start".into(),
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
                        "date_event_start_to_event_start_4af1089d",
                    )?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("event.start") };
                    if _cond {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("event.start") {
                                match parse_date_out(
                                    &date_str,
                                    &[
                                        "ISO8601",
                                        "yyyy-MM-dd HH:mm:ss.SSS",
                                        "yyyy-MM-dd HH:mm:ss.SS",
                                        "yyyy-MM-dd HH:mm:ss.S",
                                    ],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set("event.start", parsed)?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "event.start".into(),
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
                                "date_event_start_to_event_start_0bb95121",
                            )?;
                            event.append("error.message", json!(format!("Error parsing date from field `event.start`. Value of field: {}: {}", event.get("event.start").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            if event.remove("event.end").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "event.end".into(),
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("event.end") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.end") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("event.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.end".into(),
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
                        "date_event_end_to_event_end_86ecdb91",
                    )?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("event.end") };
                    if _cond {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("event.end") {
                                match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                    Some(parsed) => event.set("event.end", parsed)?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "event.end".into(),
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
                                "date_event_end_to_event_end_12eb103f",
                            )?;
                            event.append("error.message", json!(format!("Error parsing date from field `event.end`. Value of field: {}: {}", event.get("event.end").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            if event.remove("event.end").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "event.end".into(),
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // Painless script
                // Source: Instant eventstart = ZonedDateTime.parse(ctx.event?.start).toInstant(); Instant eventend = ZonedDateTime.parse(ctx.event?.end).toInstant(); ctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Instant eventstart = ZonedDateTime.parse(ctx.event?.start).toInstant(); Instant eventend = ZonedDateTime.parse(ctx.event?.end).toInstant(); ctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);"#
                    ),
                )?;
            }

            // Painless script
            // Source: if (ctx?.arista?.class == null || !params.containsKey(ctx.arista.class)) {\n  return;\n} ctx.event.kind = params.get(ctx.arista.class).get('kind'); ctx.event.category = params.get(ctx.arista.class).get('category').clone(); ctx.event.type = params.get(ctx.arista.class).get('type').clone(); ctx.event.provider = params.get(ctx.arista.class).get('provider'); if (ctx?.event?.outcome == null) {\n  return;\n} if (ctx.event.category.contains('network') || ctx.event.category.contains('intrusion_detection')) {\n  if (ctx.event.outcome == 'success') {\n    if (ctx.event?.type == null || !ctx.event.type.contains('allowed')) {\n      ctx.event.type.add('allowed');\n    }\n  }\n  if (ctx.event.outcome == 'failure') {\n    if (ctx.event?.type == null || !ctx.event.type.contains('denied')) {\n      ctx.event.type.add('denied');\n    }\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx?.arista?.class == null || !params.containsKey(ctx.arista.class)) {\n  return;\n} ctx.event.kind = params.get(ctx.arista.class).get('kind'); ctx.event.category = params.get(ctx.arista.class).get('category').clone(); ctx.event.type = params.get(ctx.arista.class).get('type').clone(); ctx.event.provider = params.get(ctx.arista.class).get('provider'); if (ctx?.event?.outcome == null) {\n  return;\n} if (ctx.event.category.contains('network') || ctx.event.category.contains('intrusion_detection')) {\n  if (ctx.event.outcome == 'success') {\n    if (ctx.event?.type == null || !ctx.event.type.contains('allowed')) {\n      ctx.event.type.add('allowed');\n    }\n  }\n  if (ctx.event.outcome == 'failure') {\n    if (ctx.event?.type == null || !ctx.event.type.contains('denied')) {\n      ctx.event.type.add('denied');\n    }\n  }\n}"#
                ),
                cached_params!(
                    "{\"class com.untangle.app.ad_blocker.AdBlockerEvent\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\",\"denied\"],\"provider\":\"ad_blocker\"},\"class com.untangle.app.firewall.FirewallEvent\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\"],\"provider\":\"firewall\"},\"class com.untangle.app.http.HttpRequestEvent\":{\"kind\":\"event\",\"category\":[\"network\",\"web\"],\"type\":[\"connection\",\"start\"],\"provider\":\"http_request\"},\"class com.untangle.app.http.HttpResponseEvent\":{\"kind\":\"event\",\"category\":[\"network\",\"web\"],\"type\":[\"connection\",\"end\"],\"provider\":\"http_response\"},\"class com.untangle.app.intrusion_prevention.IntrusionPreventionLogEvent\":{\"kind\":\"event\",\"category\":[\"intrusion_detection\"],\"type\":[\"denied\"],\"provider\":\"intrusion_prevention\"},\"class com.untangle.app.web_filter.WebFilterEvent\":{\"kind\":\"event\",\"category\":[\"network\",\"web\"],\"type\":[\"connection\",\"end\"],\"provider\":\"web_filter\"},\"class com.untangle.uvm.app.SessionEvent\":{\"kind\":\"event\",\"category\":[\"network\",\"session\"],\"type\":[\"info\"],\"provider\":\"session_event\"},\"class com.untangle.uvm.app.SessionStatsEvent\":{\"kind\":\"event\",\"category\":[\"network\",\"session\"],\"type\":[\"info\"],\"provider\":\"session_stats\"},\"class com.untangle.uvm.DeviceTableEvent\":{\"kind\":\"event\",\"outcome\":\"failure\",\"category\":[\"database\"],\"type\":[\"change\"],\"provider\":\"device_table\"},\"class com.untangle.uvm.HostTableEvent\":{\"kind\":\"event\",\"category\":[\"authentication\"],\"type\":[\"change\"],\"provider\":\"host_table\"},\"class com.untangle.uvm.logging.InterfaceStatEvent\":{\"kind\":\"metric\",\"category\":[\"host\"],\"type\":[\"info\"],\"provider\":\"interface_stats\"},\"class com.untangle.uvm.logging.SystemStatEvent\":{\"kind\":\"metric\",\"category\":[\"host\"],\"type\":[\"info\"],\"provider\":\"system_stats\"}}"
                ),
            )?;

            if event.has_value("arista.protocol") {
                event.rename("arista.protocol", "network.iana_number")?;
            }

            if event.has_value("network.iana_number") {
                if let Some(val) = event.get("network.iana_number") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "network.iana_number".into(),
                            message,
                        }
                    })?;
                    event.set("network.iana_number", converted)?;
                }
            }

            if event.has_value("arista.protocolName") {
                event.rename("arista.protocolName", "network.transport")?;
            }

            let _cond = {
                (event.has_value("source.bytes") || event.has_value("destination.bytes"))
                    && !event.has_value("network.bytes")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nif (ctx.source.bytes != null && ctx.destination.bytes != null) {\n  ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes\n} else if (ctx.source.bytes == null && ctx.destination.bytes != null) {\n  ctx.network.bytes = ctx.destination.bytes\n} else if (ctx.source.bytes != null && ctx.destination.bytes == null) {\n  ctx.network.bytes = ctx.source.bytes\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nif (ctx.source.bytes != null && ctx.destination.bytes != null) {\n  ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes\n} else if (ctx.source.bytes == null && ctx.destination.bytes != null) {\n  ctx.network.bytes = ctx.destination.bytes\n} else if (ctx.source.bytes != null && ctx.destination.bytes == null) {\n  ctx.network.bytes = ctx.source.bytes\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                (event.has_value("source.packets") || event.has_value("destination.packets"))
                    && !event.has_value("network.packets")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nif (ctx.source.packets != null && ctx.destination.packets != null) {\n  ctx.network.packets = ctx.source.packets + ctx.destination.packets\n} else if (ctx.source.packets == null && ctx.destination.packets != null) {\n  ctx.network.packets = ctx.destination.packets\n} else if (ctx.source.packets != null && ctx.destination.packets == null) {\n  ctx.network.packets = ctx.source.packets\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nif (ctx.source.packets != null && ctx.destination.packets != null) {\n  ctx.network.packets = ctx.source.packets + ctx.destination.packets\n} else if (ctx.source.packets == null && ctx.destination.packets != null) {\n  ctx.network.packets = ctx.destination.packets\n} else if (ctx.source.packets != null && ctx.destination.packets == null) {\n  ctx.network.packets = ctx.source.packets\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                // Painless script
                // Source: boolean isPrivateCIDR(def ip) {\n  CIDR class_a_network = new CIDR('10.0.0.0/8');\n  CIDR class_b_network = new CIDR('172.16.0.0/12');\n  CIDR class_c_network = new CIDR('192.168.0.0/16');\n\n  try {\n    return class_a_network.contains(ip) || class_b_network.contains(ip) || class_c_network.contains(ip);\n  } catch (IllegalArgumentException e) {\n    return false;\n  }\n}\ntry {\n  if (ctx?.network == null) {\n    Map map = new HashMap();\n    ctx.put('network', map);\n  }\n\n  if (!isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'inbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'outbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'internal';\n  } else if (!isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'external';\n  } else {\n    ctx.network.direction = 'unknown';\n  }\n}\ncatch (Exception e) {\n  ctx.network.direction = null;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean isPrivateCIDR(def ip) {\n  CIDR class_a_network = new CIDR('10.0.0.0/8');\n  CIDR class_b_network = new CIDR('172.16.0.0/12');\n  CIDR class_c_network = new CIDR('192.168.0.0/16');\n\n  try {\n    return class_a_network.contains(ip) || class_b_network.contains(ip) || class_c_network.contains(ip);\n  } catch (IllegalArgumentException e) {\n    return false;\n  }\n}\ntry {\n  if (ctx?.network == null) {\n    Map map = new HashMap();\n    ctx.put('network', map);\n  }\n\n  if (!isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'inbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'outbound';\n  } else if (isPrivateCIDR(ctx.source.ip) && isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'internal';\n  } else if (!isPrivateCIDR(ctx.source.ip) && !isPrivateCIDR(ctx.destination.ip)) {\n    ctx.network.direction = 'external';\n  } else {\n    ctx.network.direction = 'unknown';\n  }\n}\ncatch (Exception e) {\n  ctx.network.direction = null;\n}\n"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.ip") {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                        event
                            .get_as_string("network.iana_number")
                            .or_else(|| event.get_as_string("network.transport")),
                    ) {
                        let icmp = matches!(
                            protocol.to_ascii_lowercase().as_str(),
                            "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                        );
                        let (src_field, dst_field) = if icmp {
                            ("icmp.type", "icmp.code")
                        } else {
                            ("source.port", "destination.port")
                        };
                        let src_port =
                            u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                        let dst_port =
                            u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                        match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                            Ok(cid) => event.set("network.community_id", cid)?,
                            Err(message) => {
                                return Err(TransformError::ParseError {
                                    path: "network.community_id".into(),
                                    message,
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.set("observer.product", json!("Arista NG Firewall"))?;

            event.set("observer.vendor", json!("Arista"))?;

            event.set("observer.type", json!("firewall"))?;

            if event.has_value("arista.serverIntf") {
                event.rename("arista.serverIntf", "observer.egress.interface.id")?;
            }

            if event.has_value("arista.clientIntf") {
                event.rename("arista.clientIntf", "observer.ingress.interface.id")?;
            }

            if event.has_value("observer.egress.interface.id") {
                if let Some(val) = event.get("observer.egress.interface.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "observer.egress.interface.id".into(),
                            message,
                        }
                    })?;
                    event.set("observer.egress.interface.id", converted)?;
                }
            }

            if event.has_value("observer.ingress.interface.id") {
                if let Some(val) = event.get("observer.ingress.interface.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "observer.ingress.interface.id".into(),
                            message,
                        }
                    })?;
                    event.set("observer.ingress.interface.id", converted)?;
                }
            }

            // Painless script
            // Source: if (ctx.observer?.egress?.interface?.id == \"1\") {\n  if (ctx._conf?.interface_id_1_alias != null) {\n    ctx.observer.egress.interface.alias = ctx._conf.interface_id_1_alias;\n  }\n  if (ctx._conf?.interface_id_1_name != null) {\n    ctx.observer.egress.interface.name = ctx._conf.interface_id_1_name;\n  }\n} else if (ctx?.observer?.egress?.interface?.id == \"2\") {\n  if (ctx._conf?.interface_id_2_alias != null) {\n    ctx.observer.egress.interface.alias = ctx._conf.interface_id_2_alias;\n  }\n  if (ctx._conf?.interface_id_2_name != null) {\n    ctx.observer.egress.interface.name = ctx._conf.interface_id_2_name;\n  }\n}\nif (ctx.observer?.ingress?.interface?.id == \"1\") {\n  if (ctx._conf?.interface_id_1_alias != null) {\n    ctx.observer.ingress.interface.alias = ctx._conf.interface_id_1_alias;\n  }\n  if (ctx._conf?.interface_id_1_name != null) {\n    ctx.observer.ingress.interface.name = ctx._conf.interface_id_1_name;\n  }\n} else if (ctx.observer?.ingress?.interface?.id == \"2\") {\n  if (ctx._conf?.interface_id_2_alias != null) {\n    ctx.observer.ingress.interface.alias = ctx._conf.interface_id_2_alias;\n  }\n  if (ctx._conf?.interface_id_2_name != null) {\n    ctx.observer.ingress.interface.name = ctx._conf.interface_id_2_name;\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.observer?.egress?.interface?.id == \"1\") {\n  if (ctx._conf?.interface_id_1_alias != null) {\n    ctx.observer.egress.interface.alias = ctx._conf.interface_id_1_alias;\n  }\n  if (ctx._conf?.interface_id_1_name != null) {\n    ctx.observer.egress.interface.name = ctx._conf.interface_id_1_name;\n  }\n} else if (ctx?.observer?.egress?.interface?.id == \"2\") {\n  if (ctx._conf?.interface_id_2_alias != null) {\n    ctx.observer.egress.interface.alias = ctx._conf.interface_id_2_alias;\n  }\n  if (ctx._conf?.interface_id_2_name != null) {\n    ctx.observer.egress.interface.name = ctx._conf.interface_id_2_name;\n  }\n}\nif (ctx.observer?.ingress?.interface?.id == \"1\") {\n  if (ctx._conf?.interface_id_1_alias != null) {\n    ctx.observer.ingress.interface.alias = ctx._conf.interface_id_1_alias;\n  }\n  if (ctx._conf?.interface_id_1_name != null) {\n    ctx.observer.ingress.interface.name = ctx._conf.interface_id_1_name;\n  }\n} else if (ctx.observer?.ingress?.interface?.id == \"2\") {\n  if (ctx._conf?.interface_id_2_alias != null) {\n    ctx.observer.ingress.interface.alias = ctx._conf.interface_id_2_alias;\n  }\n  if (ctx._conf?.interface_id_2_name != null) {\n    ctx.observer.ingress.interface.name = ctx._conf.interface_id_2_name;\n  }\n}"#
                ),
            )?;

            if event.has_value("arista.category") {
                event.rename("arista.category", "rule.category")?;
            }

            if event.has_value("arista.policyId") {
                event.rename("arista.policyId", "rule.ruleset")?;
            }

            if event.has_value("rule.ruleset") {
                if let Some(val) = event.get("rule.ruleset") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "rule.ruleset".into(),
                            message,
                        }
                    })?;
                    event.set("rule.ruleset", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("arista.ruleId") {
                    event.rename("arista.ruleId", "rule.id")?;
                }
                Ok(())
            })();

            if event.has_value("rule.id") {
                if let Some(val) = event.get("rule.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "rule.id".into(),
                            message,
                        }
                    })?;
                    event.set("rule.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("arista.tagsString")
                    && event.get_str("arista.tagsString") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "tags",
                        json!(
                            event
                                .get("arista.tagsString")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("arista.username") {
                event.rename("arista.username", "user.name")?;
            }

            if event.has_value("arista.hostname") {
                event.rename("arista.hostname", "host.name")?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("host.ip")
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

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("arista.policyId") {
                event.rename("arista.policyId", "arista.policy.id")?;
            }

            if event.has_value("arista.policyRuleId") {
                event.rename("arista.policyRuleId", "arista.policy.rule_id")?;
            }

            event.remove("arista.blocked");
            event.remove("arista.categoryId");
            event.remove("arista.class");
            event.remove("arista.clientCountry");
            event.remove("arista.clientLatitude");
            event.remove("arista.clientLongitude");
            event.remove("arista.filterPrefix");
            event.remove("arista.localAddr");
            event.remove("arista.remoteAddr");
            event.remove("arista.httpRequestEvent.sessionEvent");
            event.remove("arista.requestLine");
            event.remove("arista.serverCountry");
            event.remove("arista.serverLatitude");
            event.remove("arista.serverLongitude");
            event.remove("arista.sessionEvent");
            event.remove("arista.tagsString");
            event.remove("arista.timeStamp");

            let _cond =
                { event.has_value("arista") && event.get_bool("arista.empty") == Some(true) };
            if _cond {
                if event.remove("arista").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "arista".into(),
                    });
                }
            }

            event.remove("_temp");
            event.remove("_temp_");
            event.remove("_conf");
            event.remove("_ingest");

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
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
