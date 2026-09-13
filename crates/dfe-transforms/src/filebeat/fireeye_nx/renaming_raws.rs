// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `renaming_raws` pipeline.
pub struct RenamingRaws;

impl Transform for RenamingRaws {
    fn name(&self) -> &str {
        "renaming_raws"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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

            if let Some(v) = event.get("source.address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

                if event.has_value("rawmsg.src_port") {
                    event.rename("rawmsg.src_port", "source.port")?;
                }

                if event.has_value("rawmsg.dest_ip") {
                    event.rename("rawmsg.dest_ip", "destination.address")?;
                }

            if let Some(v) = event.get("destination.address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

                if event.has_value("rawmsg.dest_port") {
                    event.rename("rawmsg.dest_port", "destination.port")?;
                }

            let _cond = { event.has_value("json.meta_sip4") };
            if _cond {
                event.append_unique("observer.ip", json!(event.get("json.meta_sip4").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("json.meta_sip4") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("json.meta_sip4").map_or_else(String::new, template_to_string)))?;
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

            if let Some(v) = event.get("fireeye.nx.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("observer.hostname").map_or_else(String::new, template_to_string)))?;
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

            if let Some(v) = event.get("fireeye.nx.flow.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
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
                    event.rename("rawmsg.response_icmp_code", "fireeye.nx.flow.response_icmp_code")?;
                }
            }

            let _cond = { event.get_str("event.type") == Some("flow") };
            if _cond {
                if event.has_value("rawmsg.response_icmp_type") {
                    event.rename("rawmsg.response_icmp_type", "fireeye.nx.flow.response_icmp_type")?;
                }
            }

            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.filename") {
                    event.rename("rawmsg.fileinfo.filename", "fireeye.nx.fileinfo.filename")?;
                }
            }

            if let Some(v) = event.get("fireeye.nx.fileinfo.filename").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            if let Some(v) = event.get("fireeye.nx.fileinfo.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.get_str("event.type") == Some("fileinfo") };
            if _cond {
                if event.has_value("rawmsg.fileinfo.size") {
                    event.rename("rawmsg.fileinfo.size", "fireeye.nx.fileinfo.size")?;
                }
            }

            if let Some(v) = event.get("fireeye.nx.fileinfo.size").filter(|v| !painless_is_empty_value(v)).cloned() {
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
                event.append_unique("related.hosts", json!(event.get("url.domain").map_or_else(String::new, template_to_string)))?;
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
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "rawmsg.dns.id".into(),
                            message,
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

            if let Some(v) = event.get("tls.client.server_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("destination.domain").map_or_else(String::new, template_to_string)))?;
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
