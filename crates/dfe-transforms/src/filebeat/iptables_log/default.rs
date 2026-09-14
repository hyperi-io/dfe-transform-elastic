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

            if let Some(v) = event.get("message").cloned() {
                if !event.has("event.original") {
                    event.set("event.original", v)?;
                }
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                if !event.has("event.created") {
                    event.set("event.created", v)?;
                }
            }

            event.remove("syslog.priority");
            event.remove("syslog.facility");
            event.remove("journald.custom.seqnum");
            event.remove("journald.custom.seqnum_id");

            if event.has_value("syslog.pid") {
                event.rename("syslog.pid", "log.syslog.procid")?;
            }

            if event.has_value("syslog.identifier") {
                event.rename("syslog.identifier", "log.syslog.appname")?;
            }

            let _cond = {
                event.has_value("syslog")
                    && event.get("syslog").is_some_and(|v| v.is_object())
                    && event.get("syslog").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                if event.remove("syslog").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "syslog".into(),
                    });
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) %{GREEDYDATA:message}
                    // Grok pattern: ^(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) %{GREEDYDATA:message}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) %{GREEDYDATA:message}"
                            ),
                            cached_grok!(
                                "^(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) %{GREEDYDATA:message}"
                            ),
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
                    "grok_event_original_037fcb4e",
                )?;
                if let Some(v) = event.get("event.original").cloned() {
                    event.set("message", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(input) = event.get_string("message") {
                // Grok pattern: (?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)%{GREEDYDATA}\\[(?:(?P<iptables_ubiquiti_rule_set>(?:[^\\]]*))-(?P<iptables_ubiquiti_rule_number>(?:[^-\\]]*))-(?P<event_action>(?:[^-\\]]*)))\\](?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))
                // Grok pattern: (?:(:?%{WORD:event.action}:|(?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)%{SPACE}iptables%{SPACE}%{WORD:event.action}|(?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)))%{GREEDYDATA}(?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))
                // Grok pattern: (?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)%{SPACE}(?:((?:[0-9]+)%{SPACE})?(TTL|TL|L)=((?P<iptables_ttl>(?:[0-9]+)))%{SPACE}(ID=((?P<iptables_id>(?:[0-9]+)))%{SPACE})?(DF%{SPACE})?)(?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?)
                // Grok pattern: %{GREEDYDATA}\\[(?:(?P<iptables_ubiquiti_rule_set>(?:[^\\]]*))-(?P<iptables_ubiquiti_rule_number>(?:[^-\\]]*))-(?P<event_action>(?:[^-\\]]*)))\\](%{SPACE})?(?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))
                // Grok pattern: %{GREEDYDATA}(?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))
                if !extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "(?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)%{GREEDYDATA}\\[(?:(?P<iptables_ubiquiti_rule_set>(?:[^\\]]*))-(?P<iptables_ubiquiti_rule_number>(?:[^-\\]]*))-(?P<event_action>(?:[^-\\]]*)))\\](?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))",
                            [
                                ("iptables_ubiquiti_rule_set", "iptables.ubiquiti.rule_set"),
                                (
                                    "iptables_ubiquiti_rule_number",
                                    "iptables.ubiquiti.rule_number"
                                ),
                                ("event_action", "event.action"),
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_id", "iptables.id:int"),
                                ("iptables_fragment_flags", "iptables.fragment_flags"),
                                ("iptables_fragment_offset", "iptables.fragment_offset:int"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_tos", "iptables.tos"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_flow_label", "iptables.flow_label:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("network_transport", "network.transport"),
                                ("network_transport", "network.transport")
                            ]
                        ),
                        cached_grok_mapped!(
                            "(?:(:?%{WORD:event.action}:|(?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)%{SPACE}iptables%{SPACE}%{WORD:event.action}|(?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)))%{GREEDYDATA}(?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))",
                            [
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_id", "iptables.id:int"),
                                ("iptables_fragment_flags", "iptables.fragment_flags"),
                                ("iptables_fragment_offset", "iptables.fragment_offset:int"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_tos", "iptables.tos"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_flow_label", "iptables.flow_label:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("network_transport", "network.transport"),
                                ("network_transport", "network.transport")
                            ]
                        ),
                        cached_grok_mapped!(
                            "(?:%{HOSTNAME:observer.name}%{SPACE}(%{NOTSPACE}%{SPACE})?kernel:)%{SPACE}(?:((?:[0-9]+)%{SPACE})?(TTL|TL|L)=((?P<iptables_ttl>(?:[0-9]+)))%{SPACE}(ID=((?P<iptables_id>(?:[0-9]+)))%{SPACE})?(DF%{SPACE})?)(?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?)",
                            [
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_id", "iptables.id:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("network_transport", "network.transport")
                            ]
                        ),
                        cached_grok_mapped!(
                            "%{GREEDYDATA}\\[(?:(?P<iptables_ubiquiti_rule_set>(?:[^\\]]*))-(?P<iptables_ubiquiti_rule_number>(?:[^-\\]]*))-(?P<event_action>(?:[^-\\]]*)))\\](%{SPACE})?(?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))",
                            [
                                ("iptables_ubiquiti_rule_set", "iptables.ubiquiti.rule_set"),
                                (
                                    "iptables_ubiquiti_rule_number",
                                    "iptables.ubiquiti.rule_number"
                                ),
                                ("event_action", "event.action"),
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_id", "iptables.id:int"),
                                ("iptables_fragment_flags", "iptables.fragment_flags"),
                                ("iptables_fragment_offset", "iptables.fragment_offset:int"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_tos", "iptables.tos"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_flow_label", "iptables.flow_label:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("network_transport", "network.transport"),
                                ("network_transport", "network.transport")
                            ]
                        ),
                        cached_grok_mapped!(
                            "%{GREEDYDATA}(?:(?:IN=%{DATA:iptables.input_device} OUT=%{DATA:iptables.output_device}?(?: MAC=(?:(?:%{MAC:destination.mac}:%{MAC:source.mac}:(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?(?:(?::[A-Fa-f0-9]{2})*)|%{MAC:destination.mac}(?:(?::[A-Fa-f0-9]{2})*):(?P<iptables_ether_type>(?:(?:[A-Fa-f0-9]{2}):(?:[A-Fa-f0-9]{2})))?)))?) (:?(?:(?:SRC=%{IPV4:source.ip} DST=%{IPV4:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TOS=(?:0x)?%{BASE16NUM:iptables.tos} PREC=0x%{BASE16NUM:iptables.precedence_bits} TTL=(?P<iptables_ttl>(?:[0-9]+)) ID=(?P<iptables_id>(?:[0-9]+))(?: (?P<iptables_fragment_flags>(?:((?<= )(CE|DF|MF))*)))?(?: FRAG: (?P<iptables_fragment_offset>(?:[0-9]+)))?) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))|(?:(?:SRC=%{IPV6:source.ip} DST=%{IPV6:destination.ip} LEN=(?P<iptables_length>(?:[0-9]+)) TC=(?P<iptables_tos>(?:[0-9]+)) HOPLIMIT=(?P<iptables_ttl>(?:[0-9]+)) FLOWLBL=(?P<iptables_flow_label>(?:[0-9]+))) (?:(?:PROTO=(?P<network_transport>[a-zA-Z0-9]+))( (?:SPT=(?P<source_port>(?:[0-9]+)) DPT=(?P<destination_port>(?:[0-9]+))))?( ((?:(?:(?:SEQ=(?P<iptables_tcp_seq>(?:[0-9]+)) ACK=(?P<iptables_tcp_ack>(?:[0-9]+))) )?WINDOW=(?P<iptables_tcp_window>(?:[0-9]+)) RES=0x%{BASE16NUM:iptables.tcp_reserved_bits} (?P<iptables_tcp_flags>(?:(CWR |ECE |URG |ACK |PSH |RST |SYN |FIN )*)))|(?:LEN=(?P<iptables_udp_length>(?:[0-9]+)))|(?:TYPE=(?P<iptables_icmp_type>(?:[0-9]+)) CODE=(?P<iptables_icmp_code>(?:[0-9]+))(( (?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\]))|(?:( (?:(?:ID=(?P<iptables_icmp_id>(?:[0-9]+)) SEQ=(?P<iptables_icmp_seq>(?:[0-9]+)))|(?:PARAMETER=(?P<iptables_icmp_parameter>(?:[0-9]+)))|(?:GATEWAY=%{IP:iptables.icmp.redirect})))*)))|(?:INCOMPLETE \\[(?P<iptables_incomplete_bytes>(?:[0-9]+)) bytes\\])))?))))",
                            [
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_ether_type", "iptables.ether_type"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_id", "iptables.id:int"),
                                ("iptables_fragment_flags", "iptables.fragment_flags"),
                                ("iptables_fragment_offset", "iptables.fragment_offset:int"),
                                ("iptables_length", "iptables.length:int"),
                                ("iptables_tos", "iptables.tos"),
                                ("iptables_ttl", "iptables.ttl:int"),
                                ("iptables_flow_label", "iptables.flow_label:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("source_port", "source.port:int"),
                                ("destination_port", "destination.port:int"),
                                ("iptables_tcp_window", "iptables.tcp.window:int"),
                                ("iptables_tcp_flags", "iptables.tcp.flags"),
                                ("iptables_udp_length", "iptables.udp.length:int"),
                                ("iptables_icmp_type", "iptables.icmp.type:int"),
                                ("iptables_icmp_code", "iptables.icmp.code:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_tcp_seq", "iptables.tcp.seq:int"),
                                ("iptables_tcp_ack", "iptables.tcp.ack:int"),
                                ("iptables_incomplete_bytes", "iptables.incomplete_bytes:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("iptables_icmp_id", "iptables.icmp.id:int"),
                                ("iptables_icmp_seq", "iptables.icmp.seq:int"),
                                ("iptables_icmp_parameter", "iptables.icmp.parameter:int"),
                                ("network_transport", "network.transport"),
                                ("network_transport", "network.transport")
                            ]
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("iptables.ubiquiti.rule_set") {
                    if let Some(input) = event.get_string("iptables.ubiquiti.rule_set") {
                        // Grok pattern: (?P<iptables_ubiquiti_input_zone>(?:[^-]*))-(?P<iptables_ubiquiti_output_zone>(?:[^-]*))
                        if !cached_grok_mapped!("(?P<iptables_ubiquiti_input_zone>(?:[^-]*))-(?P<iptables_ubiquiti_output_zone>(?:[^-]*))", [("iptables_ubiquiti_input_zone", "iptables.ubiquiti.input_zone"), ("iptables_ubiquiti_output_zone", "iptables.ubiquiti.output_zone")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.timestamp8601") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp8601") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp8601".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("event.timezone") && event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp".into(),
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
                        "date__tmp_timestamp_8a0fd800",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "date processor error: {}",
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

            let _cond = { event.has_value("event.timezone") && event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp".into(),
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
                        "date__tmp_timestamp_245edf42",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "date processor error: {}",
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

            let _cond = { !event.has_value("observer.name") && event.has_value("hostname") };
            if _cond {
                if let Some(v) = event.get("hostname").cloned() {
                    event.set("observer.name", v)?;
                }
            }

            let _cond = { !event.has_value("observer.name") && event.has_value("hostname") };
            if _cond {
                if let Some(v) = event.get("hostname").cloned() {
                    event.set("observer.hostname", v)?;
                }
            }

            let _cond = {
                !event.has_value("network.iana_number")
                    && event.has_value("network.transport")
                    && event
                        .get_str("network.transport")
                        .is_some_and(|s| s.bytes().all(|b| b.is_ascii_digit()))
            };
            if _cond {
                if event.has_value("network.transport") {
                    event.rename("network.transport", "network.iana_number")?;
                }
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
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

            // Painless script
            // Source: for (action in params.mappings) {\n  def src = ctx[action.source.object];\n  if (src != null) {\n    Map map = action.map;\n    String key = src[action.source.key];\n    String mapping = map[key];\n    if (mapping != null) {\n      Map dst = ctx[action.destination.object];\n      if (dst == null) {\n          dst = new HashMap();\n          ctx[action.destination.object] = dst;\n      }\n      dst[action.destination.key] = mapping;\n    }\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (action in params.mappings) {\n  def src = ctx[action.source.object];\n  if (src != null) {\n    Map map = action.map;\n    String key = src[action.source.key];\n    String mapping = map[key];\n    if (mapping != null) {\n      Map dst = ctx[action.destination.object];\n      if (dst == null) {\n          dst = new HashMap();\n          ctx[action.destination.object] = dst;\n      }\n      dst[action.destination.key] = mapping;\n    }\n  }\n}"#
                ),
                cached_params!(
                    "{\"mappings\":[{\"source\":{\"object\":\"iptables\",\"key\":\"ether_type\"},\"destination\":{\"object\":\"network\",\"key\":\"type\"},\"map\":{\"08:00\":\"ipv4\",\"86:dd\":\"ipv6\"}},{\"source\":{\"object\":\"event\",\"key\":\"action\"},\"destination\":{\"object\":\"event\",\"key\":\"action\"},\"map\":{\"d\":\"drop\",\"a\":\"accept\"}},{\"source\":{\"object\":\"event\",\"key\":\"action\"},\"destination\":{\"object\":\"event\",\"key\":\"type\"},\"map\":{\"drop\":\"denied\",\"accept\":\"allowed\",\"deny\":\"denied\",\"drop_input\":\"denied\"}},{\"source\":{\"object\":\"network\",\"key\":\"transport\"},\"destination\":{\"object\":\"network\",\"key\":\"transport\"},\"map\":{\"icmpv6\":\"ipv6-icmp\"}}]}"
                ),
            )?;

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
                            ("iptables.icmp.type", "iptables.icmp.code")
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

            // Painless script
            // Source: def iptables = ctx['iptables']; if (iptables != null) {\n  for (key in params.hex_fields_to_convert) {\n    long value = 0;\n    def field = iptables[key];\n    if (field == null) continue;\n    char[] hex = field.toLowerCase().toCharArray();\n    for (chr in hex) {\n      long v = -1;\n      if (chr >= (char) 'a' && chr <= (char) 'f') v = (long) chr - (char) 'a' + 10;\n      else if (chr >= (char) '0' && chr <= (char) '9') v = (long) chr - (char) '0';\n      if (v >= 0) {\n        value = value * 16 + v;\n      }\n      iptables[key] = value;\n    }\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"def iptables = ctx['iptables']; if (iptables != null) {\n  for (key in params.hex_fields_to_convert) {\n    long value = 0;\n    def field = iptables[key];\n    if (field == null) continue;\n    char[] hex = field.toLowerCase().toCharArray();\n    for (chr in hex) {\n      long v = -1;\n      if (chr >= (char) 'a' && chr <= (char) 'f') v = (long) chr - (char) 'a' + 10;\n      else if (chr >= (char) '0' && chr <= (char) '9') v = (long) chr - (char) '0';\n      if (v >= 0) {\n        value = value * 16 + v;\n      }\n      iptables[key] = value;\n    }\n  }\n}"#
                ),
                cached_params!(
                    "{\"hex_fields_to_convert\":[\"ether_type\",\"tos\",\"precedence_bits\",\"tcp_reserved_bits\"]}"
                ),
            )?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.type", json!("connection"))?;
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

            if event.has_value("iptables.tcp_reserved_bits") {
                event.rename("iptables.tcp_reserved_bits", "iptables.tcp.reserved_bits")?;
            }

            if event.has_value("iptables.tcp.flags") {
                if let Some(s) = event.get_string("iptables.tcp.flags") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("iptables.tcp.flags", Value::Array(parts))?;
                }
            }

            if event.has_value("iptables.fragment_flags") {
                if let Some(s) = event.get_string("iptables.fragment_flags") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("iptables.fragment_flags", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("iptables.ubiquiti.output_zone") };
            if _cond {
                if let Some(v) = event.get("iptables.ubiquiti.output_zone").cloned() {
                    event.set("observer.egress.zone", v)?;
                }
            }

            let _cond = { event.has_value("iptables.ubiquiti.input_zone") };
            if _cond {
                if let Some(v) = event.get("iptables.ubiquiti.input_zone").cloned() {
                    event.set("observer.ingress.zone", v)?;
                }
            }

            let _cond = { event.has_value("iptables.ubiquiti.rule_number") };
            if _cond {
                if let Some(v) = event.get("iptables.ubiquiti.rule_number").cloned() {
                    event.set("rule.id", v)?;
                }
            }

            let _cond = { event.has_value("iptables.ubiquiti.rule_set") };
            if _cond {
                if let Some(v) = event.get("iptables.ubiquiti.rule_set").cloned() {
                    event.set("rule.name", v)?;
                }
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("_tmp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp".into(),
                    });
                }
                Ok(())
            })();

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("_tmp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp".into(),
                        });
                    }
                    Ok(())
                })();
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
