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

            event.set("event.kind", json!("event"))?;

            event.set("observer.vendor", json!("Fireeye"))?;

            event.set("observer.product", json!("NX"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "json.rawmsg", "rawmsg")?;
                Ok(())
            })();

            // Begin nested pipeline: "renaming-raws"
            if event.has_value("rawmsg.timestamp") {
                event.rename("rawmsg.timestamp", "temp_ts")?;
            }
            if event.has_value("rawmsg.proto") {
                event.rename("rawmsg.proto", "network.transport")?;
            }
            if event.has_value("rawmsg.app_proto") {
                event.rename("rawmsg.app_proto", "network.protocol")?;
            }
            if event.has_value("rawmsg.flow_id") {
                event.rename("rawmsg.flow_id", "fireeye.nx.flow_id")?;
            }
            if event.has_value("rawmsg.event_type") {
                event.rename("rawmsg.event_type", "event.type")?;
            }
            if event.has_value("rawmsg.src_ip") {
                event.rename("rawmsg.src_ip", "source.address")?;
            }
            if let Some(v) = event
                .get("source.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }
            if event.has_value("rawmsg.src_port") {
                event.rename("rawmsg.src_port", "source.port")?;
            }
            if event.has_value("rawmsg.dest_ip") {
                event.rename("rawmsg.dest_ip", "destination.address")?;
            }
            if let Some(v) = event
                .get("destination.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }
            if event.has_value("rawmsg.dest_port") {
                event.rename("rawmsg.dest_port", "destination.port")?;
            }
            let _cond = { event.has_value("json.meta_sip4") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("json.meta_sip4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            let _cond = { event.has_value("json.meta_sip4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.meta_sip4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            if event.has_value("json.meta_oml") {
                event.rename("json.meta_oml", "fireeye.nx.device_oml")?;
            }
            if event.has_value("json.deviceid") {
                event.rename("json.deviceid", "fireeye.nx.deviceid")?;
            }
            if event.has_value("json.meta_cbname") {
                event.rename("json.meta_cbname", "fireeye.nx.hostname")?;
            }
            if let Some(v) = event
                .get("fireeye.nx.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }
            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.proto_number") {
                    event.rename("rawmsg.proto_number", "network.iana_number")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.pkts_toserver") {
                    event.rename("rawmsg.flow.pkts_toserver", "source.packets")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.pkts_toclient") {
                    event.rename("rawmsg.flow.pkts_toclient", "destination.packets")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.bytes_toserver") {
                    event.rename("rawmsg.flow.bytes_toserver", "source.bytes")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.bytes_toclient") {
                    event.rename("rawmsg.flow.bytes_toclient", "destination.bytes")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.start") {
                    event.rename("rawmsg.flow.start", "fireeye.nx.flow.starttime")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.end") {
                    event.rename("rawmsg.flow.end", "fireeye.nx.flow.endtime")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.age") {
                    event.rename("rawmsg.flow.age", "fireeye.nx.flow.age")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.state") {
                    event.rename("rawmsg.flow.state", "fireeye.nx.flow.state")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.reason") {
                    event.rename("rawmsg.flow.reason", "fireeye.nx.flow.reason")?;
                }
            }
            if let Some(v) = event
                .get("fireeye.nx.flow.reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.flow.alerted") {
                    event.rename("rawmsg.flow.alerted", "fireeye.nx.flow.alerted")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.tcp") {
                    event.rename("rawmsg.tcp", "fireeye.nx.tcp")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.icmp_code") {
                    event.rename("rawmsg.icmp_code", "fireeye.nx.flow.icmp_code")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.icmp_type") {
                    event.rename("rawmsg.icmp_type", "fireeye.nx.flow.icmp_type")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.response_icmp_code") {
                    event.rename(
                        "rawmsg.response_icmp_code",
                        "fireeye.nx.flow.response_icmp_code",
                    )?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.response_icmp_type") {
                    event.rename(
                        "rawmsg.response_icmp_type",
                        "fireeye.nx.flow.response_icmp_type",
                    )?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.filename") {
                    event.rename("rawmsg.fileinfo.filename", "fireeye.nx.fileinfo.filename")?;
                }
            }
            if let Some(v) = event
                .get("fireeye.nx.fileinfo.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.magic") {
                    event.rename("rawmsg.fileinfo.magic", "fireeye.nx.fileinfo.magic")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.md5") {
                    event.rename("rawmsg.fileinfo.md5", "fireeye.nx.fileinfo.md5")?;
                }
            }
            if let Some(v) = event
                .get("fireeye.nx.fileinfo.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.size") {
                    event.rename("rawmsg.fileinfo.size", "fireeye.nx.fileinfo.size")?;
                }
            }
            if let Some(v) = event
                .get("fireeye.nx.fileinfo.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.state") {
                    event.rename("rawmsg.fileinfo.state", "fireeye.nx.fileinfo.state")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.stored") {
                    event.rename("rawmsg.fileinfo.stored", "fireeye.nx.fileinfo.stored")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.hostname") {
                    event.rename("rawmsg.http.hostname", "url.domain")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.http_content_type") {
                    event.rename("rawmsg.http.http_content_type", "http.request.mime_type")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.http_method") {
                    event.rename("rawmsg.http.http_method", "http.request.method")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.http_refer") {
                    event.rename("rawmsg.http.http_refer", "http.request.referrer")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.http_user_agent") {
                    event.rename("rawmsg.http.http_user_agent", "user_agent.original")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.length") {
                    event.rename("rawmsg.http.length", "http.response.bytes")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.protocol") {
                    event.rename("rawmsg.http.protocol", "http.version")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.status") {
                    event.rename("rawmsg.http.status", "http.response.status_code")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.http.url") {
                    event.rename("rawmsg.http.url", "url.path")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.iface") {
                    event.rename("rawmsg.iface", "observer.ingress.interface.name")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.hostname") {
                    event.rename("rawmsg.http.hostname", "url.domain")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_content_type") {
                    event.rename("rawmsg.http.http_content_type", "http.request.mime_type")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_method") {
                    event.rename("rawmsg.http.http_method", "http.request.method")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_refer") {
                    event.rename("rawmsg.http.http_refer", "http.request.referrer")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_user_agent") {
                    event.rename("rawmsg.http.http_user_agent", "user_agent.original")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.length") {
                    event.rename("rawmsg.http.length", "http.response.bytes")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.protocol") {
                    event.rename("rawmsg.http.protocol", "http.version")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.status") {
                    event.rename("rawmsg.http.status", "http.response.status_code")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.url") {
                    event.rename("rawmsg.http.url", "url.path")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.iface") {
                    event.rename("rawmsg.iface", "observer.ingress.interface.name")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.hostname") {
                    event.rename("rawmsg.http.hostname", "url.domain")?;
                }
            }
            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_content_type") {
                    event.rename("rawmsg.http.http_content_type", "http.request.mime_type")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_method") {
                    event.rename("rawmsg.http.http_method", "http.request.method")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_refer") {
                    event.rename("rawmsg.http.http_refer", "http.request.referrer")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.http_user_agent") {
                    event.rename("rawmsg.http.http_user_agent", "user_agent.original")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.length") {
                    event.rename("rawmsg.http.length", "http.response.bytes")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.protocol") {
                    event.rename("rawmsg.http.protocol", "http.version")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.status") {
                    event.rename("rawmsg.http.status", "http.response.status_code")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.http.url") {
                    event.rename("rawmsg.http.url", "url.path")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                if event.has_value("rawmsg.iface") {
                    event.rename("rawmsg.iface", "observer.ingress.interface.name")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.id") {
                    if let Some(val) = event.get("rawmsg.dns.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "rawmsg.dns.id".into(),
                                message,
                            }
                        })?;
                        event.set("dns.id", converted)?;
                    }
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.rcode") {
                    event.rename("rawmsg.dns.rcode", "dns.response_code")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.rdata") {
                    event.rename("rawmsg.dns.rdata", "dns.resolved_data")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.rrname") {
                    event.rename("rawmsg.dns.rrname", "dns.question.name")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.rrtype") {
                    event.rename("rawmsg.dns.rrtype", "dns.question.type")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.ttl") {
                    event.rename("rawmsg.dns.ttl", "dns.answers.ttl")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.dns.type") {
                    event.rename("rawmsg.dns.type", "dns.type")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("dns") };
            if _cond {
                if event.has_value("rawmsg.iface") {
                    event.rename("rawmsg.iface", "observer.ingress.interface.name")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.client_ciphersuites") {
                    event.rename("rawmsg.tls.client_ciphersuites", "tls.client.ciphersuites")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.client_tls_exts") {
                    event.rename("rawmsg.tls.client_tls_exts", "tls.client.tls_exts")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.fingerprint") {
                    event.rename("rawmsg.tls.fingerprint", "tls.client.fingerprint")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.issuerdn") {
                    event.rename("rawmsg.tls.issuerdn", "tls.client.issuer")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.ja3.hash") {
                    event.rename("rawmsg.tls.ja3.hash", "tls.client.ja3")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.ja3.string") {
                    event.rename("rawmsg.tls.ja3.string", "tls.client.ja3_string")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.ja3s.hash") {
                    event.rename("rawmsg.tls.ja3s.hash", "tls.server.ja3s")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.ja3s.string") {
                    event.rename("rawmsg.tls.ja3s.string", "tls.server.ja3s_string")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.notbefore") {
                    event.rename("rawmsg.tls.notbefore", "tls.client.not_before")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.notafter") {
                    event.rename("rawmsg.tls.notafter", "tls.client.not_after")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.pubkeylength") {
                    event.rename("rawmsg.tls.pubkeylength", "tls.public_keylength")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.server_ciphersuite") {
                    event.rename("rawmsg.tls.server_ciphersuite", "tls.server.ciphersuite")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.server_tls_exts") {
                    event.rename("rawmsg.tls.server_tls_exts", "tls.server.tls_exts")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.sni") {
                    event.rename("rawmsg.tls.sni", "tls.client.server_name")?;
                }
            }
            if let Some(v) = event
                .get("tls.client.server_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
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
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.subject") {
                    event.rename("rawmsg.tls.subject", "tls.client.subject")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.version") {
                    event.rename("rawmsg.tls.version", "tls.version")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.tls.fatal_alert") {
                    event.rename("rawmsg.tls.fatal_alert", "fireeye.nx.tls.fetal_alert")?;
                }
            }
            let _cond = { event.get_str("event.type") == Some("tls") };
            if _cond {
                if event.has_value("rawmsg.iface") {
                    event.rename("rawmsg.iface", "observer.ingress.interface.name")?;
                }
            }
            // End nested pipeline: "renaming-raws"

            if let Some(date_str) = event.get_as_string("temp_ts") {
                match parse_date_out(&date_str, &["strict_date_optional_time_nanos"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "temp_ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("temp_ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "temp_ts".into(),
                });
            }

            if event.has_value("destination.address") {
                if let Some(ip_str) = event.get_string("destination.address") {
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

            if event.has_value("source.address") {
                if let Some(ip_str) = event.get_string("source.address") {
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

            if event.has_value("source.address") {
                if let Some(ip_str) = event.get_string("source.address") {
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

            if event.has_value("destination.address") {
                if let Some(ip_str) = event.get_string("destination.address") {
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

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            let _cond =
                { ["dns", "flow", "tls"].contains(&event.get_str("event.type").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("event.type") == Some("http") };
            if _cond {
                event.append("event.category", json!("web"))?;
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                event.append("event.category", json!("file"))?;
                event.append("event.category", json!("network"))?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.direction") {
                map_strings(
                    event,
                    "network.direction",
                    "network.direction",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("network.type") {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
            }

            let _cond = { event.has_value("network.transport") };
            if _cond {
                // Painless script
                // Source: def net = ctx.network; def iana = params[net.transport]; if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} def reverse = new HashMap(); def[] arr = new def[] { null }; for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  reverse.put(String.format(\"%d\", arr), entry.getKey());\n} def trans = reverse[net.transport]; if (trans != null) {\n  net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def net = ctx.network; def iana = params[net.transport]; if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} def reverse = new HashMap(); def[] arr = new def[] { null }; for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  reverse.put(String.format(\"%d\", arr), entry.getKey());\n} def trans = reverse[net.transport]; if (trans != null) {\n  net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n"#
                    ),
                    cached_params!(
                        "{\"icmp\":\"1\",\"igmp\":\"2\",\"ipv4\":\"4\",\"tcp\":\"6\",\"egp\":\"8\",\"igp\":\"9\",\"pup\":\"12\",\"udp\":\"17\",\"rdp\":\"27\",\"irtp\":\"28\",\"dccp\":\"33\",\"idpr\":\"35\",\"ipv6\":\"41\",\"ipv6-route\":\"43\",\"ipv6-frag\":\"44\",\"rsvp\":\"46\",\"gre\":\"47\",\"esp\":\"50\",\"ipv6-icmp\":\"58\",\"ipv6-nonxt\":\"59\",\"ipv6-opts\":\"60\"}"
                    ),
                )?;
            }

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
                let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
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

            let _cond = { event.has_value("tls.server.ja3s") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("tls.server.ja3s")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.client.ja3") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("tls.client.ja3")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("rawmsg");
            event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
