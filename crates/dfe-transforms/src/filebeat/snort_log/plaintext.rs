// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `plaintext` pipeline.
pub struct Plaintext;

impl Transform for Plaintext {
    fn name(&self) -> &str {
        "plaintext"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>))?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{SYSLOGFACILITY} )?(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name})) (?:%{PROG:process.name}(?:\\[%{POSINT:process.pid:int}\\])?):(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))
                    // Grok pattern: (?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),%{NONNEGINT:snort.ip.id:long},(%{DATA:rule.category}|),%{NONNEGINT:event.severity:long},%{WORD},%{WORD:_tmp.action}
                    // Grok pattern: (?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),(%{MAC:source.mac}|),(%{MAC:destination.mac}|),(%{DATA:snort.eth.length}|),(%{DATA:snort.tcp.flags}|),(%{BASE16NUM:snort.tcp.seq}|),(%{BASE16NUM:snort.tcp.ack}|),(|%{DATA:snort.tcp.length}),(%{BASE16NUM:snort.tcp.window}|),(%{NONNEGINT:snort.ip.ttl:long}|),(%{NONNEGINT:snort.ip.tos:long}|),(%{NONNEGINT:snort.ip.id:long}|),(%{NONNEGINT:snort.dgm.length:long}|),(%{NONNEGINT:snort.ip.length:long}|),(%{NONNEGINT:snort.icmp.type:long}|),(%{NONNEGINT:snort.icmp.code:long}|),(%{NONNEGINT:snort.icmp.id:long}|),(%{NONNEGINT:snort.icmp.seq:long}|)
                    // Grok pattern: (?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))%{SPACE}(?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))
                    // Grok pattern: (?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))\\n((?:(\\[Classification: %{DATA:rule.category}\\])?) )?(?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\n(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME})) %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|)\\n%{WORD:network.transport} (TTL:%{NONNEGINT:snort.ip.ttl:long}|) (TOS:%{BASE16NUM:snort.ip.tos}|) (ID:%{NONNEGINT:snort.ip.id:long}|) (IpLen:%{NONNEGINT:snort.ip.length:long}|) (DgmLen:%{NONNEGINT:snort.dgm.length:long}|)(%{SPACE}%{NOTSPACE:snort.ip.flags})?\\n((?:(Len: %{NONNEGINT:snort.udp.length:long}))|(?:(Type:%{NONNEGINT:snort.icmp.type:long})%{SPACE}(Code:%{NONNEGINT:snort.icmp.code:long})%{SPACE}(ID:%{NONNEGINT:snort.icmp.id:long})%{SPACE}(Seq:%{NONNEGINT:snort.icmp.seq:long})%{GREEDYDATA})|(?:(%{NOTSPACE:snort.tcp.flags})%{SPACE}(Seq: %{BASE16NUM:snort.tcp.seq})%{SPACE}(Ack: %{BASE16NUM:snort.tcp.ack})%{SPACE}(Win: %{BASE16NUM:snort.tcp.window})%{SPACE}(TcpLen: %{NONNEGINT:snort.tcp.length:long})))
                    if !extract_first_match(
                        &[
                            cached_grok!("^((?:<%{NONNEGINT:log.syslog.priority:long}>))?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{SYSLOGFACILITY} )?(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name})) (?:%{PROG:process.name}(?:\\[%{POSINT:process.pid:int}\\])?):(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))"),
                            cached_grok_mapped!("(?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),%{NONNEGINT:snort.ip.id:long},(%{DATA:rule.category}|),%{NONNEGINT:event.severity:long},%{WORD},%{WORD:_tmp.action}", [("_tmp_timestamp", "_tmp.timestamp")]),
                            cached_grok_mapped!("(?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),(%{MAC:source.mac}|),(%{MAC:destination.mac}|),(%{DATA:snort.eth.length}|),(%{DATA:snort.tcp.flags}|),(%{BASE16NUM:snort.tcp.seq}|),(%{BASE16NUM:snort.tcp.ack}|),(|%{DATA:snort.tcp.length}),(%{BASE16NUM:snort.tcp.window}|),(%{NONNEGINT:snort.ip.ttl:long}|),(%{NONNEGINT:snort.ip.tos:long}|),(%{NONNEGINT:snort.ip.id:long}|),(%{NONNEGINT:snort.dgm.length:long}|),(%{NONNEGINT:snort.ip.length:long}|),(%{NONNEGINT:snort.icmp.type:long}|),(%{NONNEGINT:snort.icmp.code:long}|),(%{NONNEGINT:snort.icmp.id:long}|),(%{NONNEGINT:snort.icmp.seq:long}|)", [("_tmp_timestamp", "_tmp.timestamp")]),
                            cached_grok_mapped!("(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))%{SPACE}(?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))", [("_tmp_timestamp", "_tmp.timestamp")]),
                            cached_grok_mapped!("(?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))\\n((?:(\\[Classification: %{DATA:rule.category}\\])?) )?(?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\n(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME})) %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|)\\n%{WORD:network.transport} (TTL:%{NONNEGINT:snort.ip.ttl:long}|) (TOS:%{BASE16NUM:snort.ip.tos}|) (ID:%{NONNEGINT:snort.ip.id:long}|) (IpLen:%{NONNEGINT:snort.ip.length:long}|) (DgmLen:%{NONNEGINT:snort.dgm.length:long}|)(%{SPACE}%{NOTSPACE:snort.ip.flags})?\\n((?:(Len: %{NONNEGINT:snort.udp.length:long}))|(?:(Type:%{NONNEGINT:snort.icmp.type:long})%{SPACE}(Code:%{NONNEGINT:snort.icmp.code:long})%{SPACE}(ID:%{NONNEGINT:snort.icmp.id:long})%{SPACE}(Seq:%{NONNEGINT:snort.icmp.seq:long})%{GREEDYDATA})|(?:(%{NOTSPACE:snort.tcp.flags})%{SPACE}(Seq: %{BASE16NUM:snort.tcp.seq})%{SPACE}(Ack: %{BASE16NUM:snort.tcp.ack})%{SPACE}(Win: %{BASE16NUM:snort.tcp.window})%{SPACE}(TcpLen: %{NONNEGINT:snort.tcp.length:long})))", [("_tmp_timestamp", "_tmp.timestamp")]),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

                // Painless script
                // Source: if (ctx.snort?.ip?.tos != null && ctx.snort.ip.tos instanceof String) {\n    ctx.snort.ip.tos = Long.decode(ctx.snort.ip.tos);\n} if (ctx.snort?.eth?.length != null && ctx.snort.eth.length instanceof String) {\n    ctx.snort.eth.length = Long.decode(ctx.snort.eth.length);\n} if (ctx.snort?.tcp?.ack != null && ctx.snort.tcp.ack instanceof String) {\n    ctx.snort.tcp.ack = Long.decode(ctx.snort.tcp.ack);\n} if (ctx.snort?.tcp?.seq != null && ctx.snort.tcp.seq instanceof String) {\n    ctx.snort.tcp.seq = Long.decode(ctx.snort.tcp.seq);\n} if (ctx.snort?.tcp?.window != null && ctx.snort.tcp.window instanceof String) {\n    ctx.snort.tcp.window = Long.decode(ctx.snort.tcp.window);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.snort?.ip?.tos != null && ctx.snort.ip.tos instanceof String) {\n    ctx.snort.ip.tos = Long.decode(ctx.snort.ip.tos);\n} if (ctx.snort?.eth?.length != null && ctx.snort.eth.length instanceof String) {\n    ctx.snort.eth.length = Long.decode(ctx.snort.eth.length);\n} if (ctx.snort?.tcp?.ack != null && ctx.snort.tcp.ack instanceof String) {\n    ctx.snort.tcp.ack = Long.decode(ctx.snort.tcp.ack);\n} if (ctx.snort?.tcp?.seq != null && ctx.snort.tcp.seq instanceof String) {\n    ctx.snort.tcp.seq = Long.decode(ctx.snort.tcp.seq);\n} if (ctx.snort?.tcp?.window != null && ctx.snort.tcp.window instanceof String) {\n    ctx.snort.tcp.window = Long.decode(ctx.snort.tcp.window);\n}"#))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
