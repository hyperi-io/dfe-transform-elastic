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

            let _cond = { event.has_value("json") };
            if _cond {
                event.rename("json", "gigamon.ami")?;
            }

            let _cond = { event.has_value("cef") };
            if _cond {
                // Begin nested pipeline: "cef-pipeline"
                let _cond = { event.has_value("cef.extensions") };
                if _cond {
                    event.rename("cef.extensions", "gigamon.ami")?;
                }
                let _cond = { event.has_value("cef.device") };
                if _cond {
                    event.rename("cef.device", "gigamon.ami.device")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_smb_version") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_smb_version",
                        "gigamon.ami.smb_version",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataSeqNum") {
                    event.rename("gigamon.ami.GigamonMdataSeqNum", "gigamon.ami.seq_num")?;
                }
                if event.has_value("gigamon.ami.GigamonApplicationID") {
                    event.rename("gigamon.ami.GigamonApplicationID", "gigamon.ami.app_id")?;
                }
                if event.has_value("gigamon.ami.GigamonApplicationName") {
                    event.rename("gigamon.ami.GigamonApplicationName", "gigamon.ami.app_name")?;
                }
                if event.has_value("gigamon.ami.GigamonMdataAppTags") {
                    event.rename("gigamon.ami.GigamonMdataAppTags", "gigamon.ami.app_tags")?;
                }
                if event.has_value("gigamon.ami.GigamonMdataIpVer") {
                    event.rename("gigamon.ami.GigamonMdataIpVer", "gigamon.ami.ip_version")?;
                }
                if event.has_value("gigamon.ami.GigamonMdataFlowStartMsec") {
                    event.rename(
                        "gigamon.ami.GigamonMdataFlowStartMsec",
                        "gigamon.ami.start_time",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataFlowEndMsec") {
                    event.rename(
                        "gigamon.ami.GigamonMdataFlowEndMsec",
                        "gigamon.ami.end_time",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataIntfName") {
                    event.rename("gigamon.ami.GigamonMdataIntfName", "gigamon.ami.intf_name")?;
                }
                if event.has_value("gigamon.ami.GigamonMdataEgressIntfID") {
                    event.rename(
                        "gigamon.ami.GigamonMdataEgressIntfID",
                        "gigamon.ami.egress_intf_id",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataSysUpTimeFirst") {
                    event.rename(
                        "gigamon.ami.GigamonMdataSysUpTimeFirst",
                        "gigamon.ami.sys_up_time_first",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataSysUpTimeLast") {
                    event.rename(
                        "gigamon.ami.GigamonMdataSysUpTimeLast",
                        "gigamon.ami.sys_up_time_last",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataFlowEndReason") {
                    event.rename(
                        "gigamon.ami.GigamonMdataFlowEndReason",
                        "gigamon.ami.end_reason",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_smb_command_string") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_smb_command_string",
                        "gigamon.ami.smb_command_string",
                    )?;
                }
                if event.has_value("gigamon.ami.dMac") {
                    event.rename("gigamon.ami.dMac", "gigamon.ami.dst_mac")?;
                }
                if event.has_value("gigamon.ami.dst") {
                    event.rename("gigamon.ami.dst", "gigamon.ami.dst_ip")?;
                }
                if event.has_value("gigamon.ami.dpt") {
                    event.rename("gigamon.ami.dpt", "gigamon.ami.dst_port")?;
                }
                if event.has_value("gigamon.ami.GigamonResponderOctets") {
                    event.rename(
                        "gigamon.ami.GigamonResponderOctets",
                        "gigamon.ami.dst_bytes",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonResponderPackets") {
                    event.rename(
                        "gigamon.ami.GigamonResponderPackets",
                        "gigamon.ami.dst_packets",
                    )?;
                }
                if event.has_value("gigamon.ami.sMac") {
                    event.rename("gigamon.ami.sMac", "gigamon.ami.src_mac")?;
                }
                if event.has_value("gigamon.ami.src") {
                    event.rename("gigamon.ami.src", "gigamon.ami.src_ip")?;
                }
                if event.has_value("gigamon.ami.spt") {
                    event.rename("gigamon.ami.spt", "gigamon.ami.src_port")?;
                }
                if event.has_value("gigamon.ami.GigamonInitiatorOctets") {
                    event.rename(
                        "gigamon.ami.GigamonInitiatorOctets",
                        "gigamon.ami.src_bytes",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonInitiatorPackets") {
                    event.rename(
                        "gigamon.ami.GigamonInitiatorPackets",
                        "gigamon.ami.src_packets",
                    )?;
                }
                let _cond = { !event.has_value("gigamon.ami.dst_port") };
                if _cond {
                    if event.has_value("gigamon.ami.destinationPort") {
                        event.rename("gigamon.ami.destinationPort", "gigamon.ami.dst_port")?;
                    }
                }
                let _cond = { !event.has_value("gigamon.ami.src_port") };
                if _cond {
                    if event.has_value("gigamon.ami.sourcePort") {
                        event.rename("gigamon.ami.sourcePort", "gigamon.ami.src_port")?;
                    }
                }
                let _cond = { !event.has_value("gigamon.ami.dst_ip") };
                if _cond {
                    if event.has_value("gigamon.ami.destinationAddress") {
                        event.rename("gigamon.ami.destinationAddress", "gigamon.ami.dst_ip")?;
                    }
                }
                let _cond = { !event.has_value("gigamon.ami.src_ip") };
                if _cond {
                    if event.has_value("gigamon.ami.sourceAddress") {
                        event.rename("gigamon.ami.sourceAddress", "gigamon.ami.src_ip")?;
                    }
                }
                let _cond = { !event.has_value("gigamon.ami.src_mac") };
                if _cond {
                    if event.has_value("gigamon.ami.sourceMacAddress") {
                        event.rename("gigamon.ami.sourceMacAddress", "gigamon.ami.src_mac")?;
                    }
                }
                let _cond = { !event.has_value("gigamon.ami.dst_mac") };
                if _cond {
                    if event.has_value("gigamon.ami.destinationMacAddress") {
                        event.rename("gigamon.ami.destinationMacAddress", "gigamon.ami.dst_mac")?;
                    }
                }
                if event.has_value("gigamon.ami.transportProtocol") {
                    event.rename("gigamon.ami.transportProtocol", "gigamon.ami.protocol")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_qdcount") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_qdcount",
                        "gigamon.ami.dns_qdcount",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_transaction_id") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_transaction_id",
                        "gigamon.ami.dns_transaction_id",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_name") {
                    event.rename("gigamon.ami.GigamonMdata_dns_name", "gigamon.ami.dns_name")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_host") {
                    event.rename("gigamon.ami.GigamonMdata_dns_host", "gigamon.ami.dns_host")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_host_addr") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_host_addr",
                        "gigamon.ami.dns_host_addr",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_host_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_host_type",
                        "gigamon.ami.dns_host_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_ttl") {
                    event.rename("gigamon.ami.GigamonMdata_dns_ttl", "gigamon.ami.dns_ttl")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_flags") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_flags",
                        "gigamon.ami.dns_flags",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_opcode") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_opcode",
                        "gigamon.ami.dns_opcode",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_class") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_class",
                        "gigamon.ami.dns_class",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_host_class") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_host_class",
                        "gigamon.ami.dns_host_class",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_host_raw") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_host_raw",
                        "gigamon.ami.dns_host_raw",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_query") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_query",
                        "gigamon.ami.dns_query",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_query_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_query_type",
                        "gigamon.ami.dns_query_type",
                    )?;
                }
                if event.has_value("gigamon.ami.deviceInboundInterface") {
                    event.rename(
                        "gigamon.ami.deviceInboundInterface",
                        "gigamon.ami.device_inbound_interface",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_ancount") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_ancount",
                        "gigamon.ami.dns_ancount",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_arcount") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_arcount",
                        "gigamon.ami.dns_arcount",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_reply_code") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_reply_code",
                        "gigamon.ami.dns_reply_code",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_response_time") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_response_time",
                        "gigamon.ami.dns_response_time",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_reverse_addr") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_reverse_addr",
                        "gigamon.ami.dns_reverse_addr",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_dns_tunneling") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_dns_tunneling",
                        "gigamon.ami.dns_tunneling",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ip_wrong_crc") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ip_wrong_crc",
                        "gigamon.ami.ip_wrong_crc",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_krb5_login") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_krb5_login",
                        "gigamon.ami.krb5_login",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_server") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_server",
                        "gigamon.ami.http_server",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_uri") {
                    event.rename("gigamon.ami.GigamonMdata_http_uri", "gigamon.ami.http_uri")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_uri_full") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_uri_full",
                        "gigamon.ami.http_uri_full",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_mime_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_mime_type",
                        "gigamon.ami.http_mime_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_server_agent") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_server_agent",
                        "gigamon.ami.http_server_agent",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_rtt") {
                    event.rename("gigamon.ami.GigamonMdata_http_rtt", "gigamon.ami.http_rtt")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_code") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_code",
                        "gigamon.ami.http_code",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_content_len") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_content_len",
                        "gigamon.ami.http_content_len",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_uri_path") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_uri_path",
                        "gigamon.ami.http_uri_path",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_request_size") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_request_size",
                        "gigamon.ami.http_request_size",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_host") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_host",
                        "gigamon.ami.http_host",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_uri_decoded") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_uri_decoded",
                        "gigamon.ami.http_uri_decoded",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_uri_raw") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_uri_raw",
                        "gigamon.ami.http_uri_raw",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_content_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_content_type",
                        "gigamon.ami.http_content_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_method") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_method",
                        "gigamon.ami.http_method",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_version") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_version",
                        "gigamon.ami.http_version",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_user_agent") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_user_agent",
                        "gigamon.ami.http_user_agent",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_response_ts") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_response_ts",
                        "gigamon.ami.http_response_ts",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_content_encoding") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_content_encoding",
                        "gigamon.ami.http_content_encoding",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_http_referer") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_http_referer",
                        "gigamon.ami.http_referer",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataFlowStartSec") {
                    event.rename(
                        "gigamon.ami.GigamonMdataFlowStartSec",
                        "gigamon.ami.flow_start_sec",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpFlags") {
                    event.rename("gigamon.ami.GigamonMdataTcpFlags", "gigamon.ami.tcp_flags")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_tcp_rtt_app") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_tcp_rtt_app",
                        "gigamon.ami.tcp_rtt_app",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_tcp_loss_count") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_tcp_loss_count",
                        "gigamon.ami.tcp_loss_count",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_tcp_rtt") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_rtt", "gigamon.ami.tcp_rtt")?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_tcp_retransmission_bytes") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_tcp_retransmission_bytes",
                        "gigamon.ami.tcp_retransmission_bytes",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_tcp_wrong_crc") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_tcp_wrong_crc",
                        "gigamon.ami.tcp_wrong_crc",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_tcp_flag_reset") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_tcp_flag_reset",
                        "gigamon.ami.tcp_flag_reset",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certif_md5") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certif_md5",
                        "gigamon.ami.ssl_certif_md5",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_common_name") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_common_name",
                        "gigamon.ami.ssl_common_name",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_validity_not_before") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_validity_not_before",
                        "gigamon.ami.ssl_validity_not_before",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_validity_not_after") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_validity_not_after",
                        "gigamon.ami.ssl_validity_not_after",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_serial_number") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_serial_number",
                        "gigamon.ami.ssl_serial_number",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_handshake_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_handshake_type",
                        "gigamon.ami.ssl_handshake_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_organization_name") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_organization_name",
                        "gigamon.ami.ssl_organization_name",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_request_size") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_request_size",
                        "gigamon.ami.ssl_request_size",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_cipher_suite_id") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_cipher_suite_id",
                        "gigamon.ami.ssl_cipher_suite_id",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_cipher_suite_list") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_cipher_suite_list",
                        "gigamon.ami.ssl_cipher_suite_list",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certif_sha1") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certif_sha1",
                        "gigamon.ami.ssl_certif_sha1",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_content_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_content_type",
                        "gigamon.ami.ssl_content_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_protocol_version") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_protocol_version",
                        "gigamon.ami.ssl_protocol_version",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_client_hello_extension_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_client_hello_extension_type",
                        "gigamon.ami.ssl_client_hello_extension_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_server_hello_extension_type") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_server_hello_extension_type",
                        "gigamon.ami.ssl_server_hello_extension_type",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_dn_subject") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_dn_subject",
                        "gigamon.ami.ssl_certificate_dn_subject",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_cn") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_cn",
                        "gigamon.ami.ssl_certificate_subject_cn",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_l") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_l",
                        "gigamon.ami.ssl_certificate_subject_l",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_st") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_st",
                        "gigamon.ami.ssl_certificate_subject_st",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_o") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_o",
                        "gigamon.ami.ssl_certificate_subject_o",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_ou") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_ou",
                        "gigamon.ami.ssl_certificate_subject_ou",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_c") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_c",
                        "gigamon.ami.ssl_certificate_subject_c",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_dn_issuer") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_dn_issuer",
                        "gigamon.ami.ssl_certificate_dn_issuer",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_cn") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_issuer_cn",
                        "gigamon.ami.ssl_certificate_issuer_cn",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_l") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_issuer_l",
                        "gigamon.ami.ssl_certificate_issuer_l",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_st") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_issuer_st",
                        "gigamon.ami.ssl_certificate_issuer_st",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_o") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_issuer_o",
                        "gigamon.ami.ssl_certificate_issuer_o",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_ou") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_issuer_ou",
                        "gigamon.ami.ssl_certificate_issuer_ou",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_c") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_issuer_c",
                        "gigamon.ami.ssl_certificate_issuer_c",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_client_hello_extension_len") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_client_hello_extension_len",
                        "gigamon.ami.ssl_client_hello_extension_len",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_server_hello_extension_len") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_server_hello_extension_len",
                        "gigamon.ami.ssl_server_hello_extension_len",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_nb_compression_methods") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_nb_compression_methods",
                        "gigamon.ami.ssl_nb_compression_methods",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_compression_method") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_compression_method",
                        "gigamon.ami.ssl_compression_method",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithms_len") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_ext_sig_algorithms_len",
                        "gigamon.ami.ssl_ext_sig_algorithms_len",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_scheme") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_scheme",
                        "gigamon.ami.ssl_ext_sig_algorithm_scheme",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_hash") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_hash",
                        "gigamon.ami.ssl_ext_sig_algorithm_hash",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_sig") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_sig",
                        "gigamon.ami.ssl_ext_sig_algorithm_sig",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_key_algo_oid")
                {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_key_algo_oid",
                        "gigamon.ami.ssl_certificate_subject_key_algo_oid",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_key_size") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_certificate_subject_key_size",
                        "gigamon.ami.ssl_certificate_subject_key_size",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_cert_extension_oid") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_cert_extension_oid",
                        "gigamon.ami.ssl_cert_extension_oid",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_cert_ext_authority_key_id") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_cert_ext_authority_key_id",
                        "gigamon.ami.ssl_cert_ext_authority_key_id",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_cert_ext_subject_key_id") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_cert_ext_subject_key_id",
                        "gigamon.ami.ssl_cert_ext_subject_key_id",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_fingerprint_ja3") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_fingerprint_ja3",
                        "gigamon.ami.ssl_fingerprint_ja3",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_index") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_index",
                        "gigamon.ami.ssl_index",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_issuer") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_issuer",
                        "gigamon.ami.ssl_issuer",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_session_id") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_session_id",
                        "gigamon.ami.ssl_session_id",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_declassify_override") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_declassify_override",
                        "gigamon.ami.ssl_declassify_override",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdata_ssl_signalization_override") {
                    event.rename(
                        "gigamon.ami.GigamonMdata_ssl_signalization_override",
                        "gigamon.ami.ssl_signalization_override",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpRttMin") {
                    event.rename(
                        "gigamon.ami.GigamonMdataTcpRttMin",
                        "gigamon.ami.tcp_rtt_min",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpRttMax") {
                    event.rename(
                        "gigamon.ami.GigamonMdataTcpRttMax",
                        "gigamon.ami.tcp_rtt_max",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpRttMean") {
                    event.rename(
                        "gigamon.ami.GigamonMdataTcpRttMean",
                        "gigamon.ami.tcp_rtt_mean",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpRttAppMin") {
                    event.rename(
                        "gigamon.ami.GigamonMdataTcpRttAppMin",
                        "gigamon.ami.tcp_rtt_app_min",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpRttAppMax") {
                    event.rename(
                        "gigamon.ami.GigamonMdataTcpRttAppMax",
                        "gigamon.ami.tcp_rtt_app_max",
                    )?;
                }
                if event.has_value("gigamon.ami.GigamonMdataTcpRttAppMean") {
                    event.rename(
                        "gigamon.ami.GigamonMdataTcpRttAppMean",
                        "gigamon.ami.tcp_rtt_app_mean",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("cef");
                    event.remove("_tmp");
                    Ok(())
                })();
                // End nested pipeline: "cef-pipeline"
            }

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("info"))?;

            let _cond = { event.has_value("gigamon.ami.ts") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gigamon.ami.ts") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE MMM dd HH:mm:ss yyyy",
                                "EEE MMM  d HH:mm:ss yyyy",
                                "EEE MMM d HH:mm:ss yyyy",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("gigamon.ami.ts", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.ts".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_gigamon_ami_ts")?;
                    if event.remove("gigamon.ami.ts").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.ts".into(),
                        });
                    }
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

            let _cond = { event.has_value("gigamon.ami.start_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gigamon.ami.start_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy:MM:dd HH:mm:ss.SSS",
                                "EEE MMM dd HH:mm:ss yyyy",
                                "EEE MMM  d HH:mm:ss yyyy",
                                "EEE MMM d HH:mm:ss yyyy",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("gigamon.ami.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.start_time".into(),
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
                        "date_gigamon_ami_start_time",
                    )?;
                    if event.remove("gigamon.ami.start_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.start_time".into(),
                        });
                    }
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

            let _cond = { event.has_value("gigamon.ami.end_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gigamon.ami.end_time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy:MM:dd HH:mm:ss.SSS",
                                "EEE MMM dd HH:mm:ss yyyy",
                                "EEE MMM  d HH:mm:ss yyyy",
                                "EEE MMM d HH:mm:ss yyyy",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("gigamon.ami.end_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.end_time".into(),
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
                        "date_gigamon_ami_end_time",
                    )?;
                    if event.remove("gigamon.ami.end_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.end_time".into(),
                        });
                    }
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

            let _cond = { event.has_value("gigamon.ami.flow_start_sec") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gigamon.ami.flow_start_sec") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy:MM:dd HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("gigamon.ami.flow_start_sec", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.flow_start_sec".into(),
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
                        "date_gigamon_ami_flow_start_sec",
                    )?;
                    if event.remove("gigamon.ami.flow_start_sec").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.flow_start_sec".into(),
                        });
                    }
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

            let _cond = { event.has_value("gigamon.ami.flow_end_sec") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gigamon.ami.flow_end_sec") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy:MM:dd HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("gigamon.ami.flow_end_sec", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.flow_end_sec".into(),
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
                        "date_gigamon_ami_flow_end_sec",
                    )?;
                    if event.remove("gigamon.ami.flow_end_sec").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.flow_end_sec".into(),
                        });
                    }
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

            let _cond = { event.has_value("gigamon.ami.ssl_validity_not_before") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("gigamon.ami.ssl_validity_not_before")
                    {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("gigamon.ami.ssl_validity_not_before", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.ssl_validity_not_before".into(),
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
                        "date_gigamon_ami_ssl_validity_not_before",
                    )?;
                    if event
                        .remove("gigamon.ami.ssl_validity_not_before")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.ssl_validity_not_before".into(),
                        });
                    }
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

            let _cond = { event.has_value("gigamon.ami.ssl_validity_not_after") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("gigamon.ami.ssl_validity_not_after")
                    {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("gigamon.ami.ssl_validity_not_after", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gigamon.ami.ssl_validity_not_after".into(),
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
                        "date_gigamon_ami_ssl_validity_not_after",
                    )?;
                    if event.remove("gigamon.ami.ssl_validity_not_after").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "gigamon.ami.ssl_validity_not_after".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("gigamon.ami.ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.has_value("gigamon.ami.seq_num") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.seq_num") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.seq_num".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.seq_num", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_seq_num")?;
                    event.remove("gigamon.ami.seq_num");
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

            let _cond = { event.has_value("gigamon.ami.sys_up_time_first") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.sys_up_time_first") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.sys_up_time_first".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.sys_up_time_first", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sys_up_time_first",
                    )?;
                    event.remove("gigamon.ami.sys_up_time_first");
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

            let _cond = { event.has_value("gigamon.ami.sys_up_time_last") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.sys_up_time_last") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.sys_up_time_last".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.sys_up_time_last", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_sys_up_time_last",
                    )?;
                    event.remove("gigamon.ami.sys_up_time_last");
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

            let _cond = { event.has_value("gigamon.ami.dst_bytes") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dst_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dst_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dst_bytes", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dst_bytes")?;
                    event.remove("gigamon.ami.dst_bytes");
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

            let _cond = { event.has_value("gigamon.ami.dst_packets") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dst_packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dst_packets".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dst_packets", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dst_packets")?;
                    event.remove("gigamon.ami.dst_packets");
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

            let _cond = { event.has_value("gigamon.ami.dst_port") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dst_port", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dst_port")?;
                    event.remove("gigamon.ami.dst_port");
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

            let _cond = { event.has_value("gigamon.ami.src_bytes") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.src_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.src_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.src_bytes", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_src_bytes")?;
                    event.remove("gigamon.ami.src_bytes");
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

            let _cond = { event.has_value("gigamon.ami.src_packets") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.src_packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.src_packets".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.src_packets", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_src_packets")?;
                    event.remove("gigamon.ami.src_packets");
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

            let _cond = { event.has_value("gigamon.ami.src_port") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.src_port".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.src_port", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_src_port")?;
                    event.remove("gigamon.ami.src_port");
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

            let _cond = { event.has_value("gigamon.ami.dns_qdcount") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_qdcount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_qdcount".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_qdcount", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dns_qdcount")?;
                    event.remove("gigamon.ami.dns_qdcount");
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

            let _cond = { event.has_value("gigamon.ami.dns_transaction_id") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_transaction_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_transaction_id".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_transaction_id", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_dns_transaction_id",
                    )?;
                    event.remove("gigamon.ami.dns_transaction_id");
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

            let _cond = { event.has_value("gigamon.ami.dns_ttl") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_ttl") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_ttl".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_ttl", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dns_ttl")?;
                    event.remove("gigamon.ami.dns_ttl");
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

            let _cond = { event.has_value("gigamon.ami.dns_ancount") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_ancount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_ancount".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_ancount", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dns_ancount")?;
                    event.remove("gigamon.ami.dns_ancount");
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

            let _cond = { event.has_value("gigamon.ami.dns_arcount") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_arcount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_arcount".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_arcount", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dns_arcount")?;
                    event.remove("gigamon.ami.dns_arcount");
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

            let _cond = { event.has_value("gigamon.ami.dns_reverse_addr") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_reverse_addr") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_reverse_addr".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_reverse_addr", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_dns_reverse_addr",
                    )?;
                    event.remove("gigamon.ami.dns_reverse_addr");
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

            let _cond = { event.has_value("gigamon.ami.dns_response_time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.dns_response_time") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.dns_response_time".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.dns_response_time", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_dns_response_time",
                    )?;
                    event.remove("gigamon.ami.dns_response_time");
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

            let _cond = { event.has_value("gigamon.ami.http_code") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.http_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.http_code".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.http_code", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_http_code")?;
                    event.remove("gigamon.ami.http_code");
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

            let _cond = { event.has_value("gigamon.ami.http_content_len") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.http_content_len") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.http_content_len".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.http_content_len", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_content_len",
                    )?;
                    event.remove("gigamon.ami.http_content_len");
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

            let _cond = { event.has_value("gigamon.ami.http_request_size") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.http_request_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.http_request_size".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.http_request_size", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_request_size",
                    )?;
                    event.remove("gigamon.ami.http_request_size");
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

            let _cond = { event.has_value("gigamon.ami.ssl_request_size") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.ssl_request_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.ssl_request_size".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.ssl_request_size", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ssl_request_size",
                    )?;
                    event.remove("gigamon.ami.ssl_request_size");
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

            let _cond = { event.has_value("gigamon.ami.ssl_client_hello_extension_len") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.ssl_client_hello_extension_len") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.ssl_client_hello_extension_len".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.ssl_client_hello_extension_len", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ssl_client_hello_extension_len",
                    )?;
                    event.remove("gigamon.ami.ssl_client_hello_extension_len");
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

            let _cond = { event.has_value("gigamon.ami.ssl_server_hello_extension_len") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.ssl_server_hello_extension_len") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.ssl_server_hello_extension_len".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.ssl_server_hello_extension_len", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ssl_server_hello_extension_len",
                    )?;
                    event.remove("gigamon.ami.ssl_server_hello_extension_len");
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

            let _cond = { event.has_value("gigamon.ami.ssl_ext_sig_algorithms_len") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.ssl_ext_sig_algorithms_len") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.ssl_ext_sig_algorithms_len".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.ssl_ext_sig_algorithms_len", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ssl_ext_sig_algorithms_len",
                    )?;
                    event.remove("gigamon.ami.ssl_ext_sig_algorithms_len");
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

            let _cond = { event.has_value("gigamon.ami.ssl_certificate_subject_key_size") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.ssl_certificate_subject_key_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.ssl_certificate_subject_key_size".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.ssl_certificate_subject_key_size", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ssl_certificate_subject_key_size",
                    )?;
                    event.remove("gigamon.ami.ssl_certificate_subject_key_size");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_tcp_rtt")?;
                    event.remove("gigamon.ami.tcp_rtt");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_app") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_app") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_app".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_app", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_tcp_rtt_app")?;
                    event.remove("gigamon.ami.tcp_rtt_app");
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

            let _cond = { event.has_value("gigamon.ami.tcp_retransmission_bytes") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_retransmission_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_retransmission_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_retransmission_bytes", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_tcp_retransmission_bytes",
                    )?;
                    event.remove("gigamon.ami.tcp_retransmission_bytes");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_min") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_min") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_min".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_min", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_tcp_rtt_min")?;
                    event.remove("gigamon.ami.tcp_rtt_min");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_max") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_max") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_max".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_max", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_tcp_rtt_max")?;
                    event.remove("gigamon.ami.tcp_rtt_max");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_mean") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_mean") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_mean".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_mean", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_tcp_rtt_mean")?;
                    event.remove("gigamon.ami.tcp_rtt_mean");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_app_min") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_app_min") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_app_min".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_app_min", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_tcp_rtt_app_min",
                    )?;
                    event.remove("gigamon.ami.tcp_rtt_app_min");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_app_max") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_app_max") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_app_max".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_app_max", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_tcp_rtt_app_max",
                    )?;
                    event.remove("gigamon.ami.tcp_rtt_app_max");
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

            let _cond = { event.has_value("gigamon.ami.tcp_rtt_app_mean") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.tcp_rtt_app_mean") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.tcp_rtt_app_mean".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.tcp_rtt_app_mean", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_tcp_rtt_app_mean",
                    )?;
                    event.remove("gigamon.ami.tcp_rtt_app_mean");
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

            let _cond = { event.has_value("gigamon.ami") };
            if _cond {
                // Painless script
                // Source: // end_reason\nif (ctx.gigamon.ami.end_reason != null) {\n    ctx.gigamon.ami.end_reason_value = params['end_reason'][ctx.gigamon.ami.end_reason];\n}\n// http_uri_path\nif (ctx.gigamon.ami.http_uri_path != null) {\n    ctx.gigamon.ami.http_uri_path_value = params['http_uri_path'][ctx.gigamon.ami.http_uri_path];\n}\n// smb_version\nif (ctx.gigamon.ami.smb_version != null) {\n    ctx.gigamon.ami.smb_version_value = params['smb_version'][ctx.gigamon.ami.smb_version];\n}\n// ssl_cipher_suite_id\nif (ctx.gigamon.ami.ssl_cipher_suite_id != null) {\n  String s = ctx.gigamon.ami.ssl_cipher_suite_id;\n  if (Character.isDigit(s.charAt(0))) {\n    def mapping = params['ssl_cipher_suite_id'][s];\n    if (mapping != null && mapping.size() > 0) {\n      ctx.gigamon.ami.ssl_cipher_suite_id_value = mapping[0];\n      if (mapping.size() > 1) {\n        ctx.gigamon.ami.ssl_cipher_suite_id_protocol = mapping[1];\n        }\n      }\n    }\n    else {\n        ctx.gigamon.ami.ssl_cipher_suite_id_value = s;\n    }\n}\n// ssl_protocol_version\nif (ctx.gigamon.ami.ssl_protocol_version != null) {\n    ctx.gigamon.ami.ssl_protocol_version_value = params['ssl_protocol_version'][ctx.gigamon.ami.ssl_protocol_version];\n}\n// ssl_ext_sig_algorithm_hash\nif (ctx.gigamon.ami.ssl_ext_sig_algorithm_hash != null) {\n    ctx.gigamon.ami.ssl_ext_sig_algorithm_hash_value = params['ssl_ext_sig_algorithm_hash'][ctx.gigamon.ami.ssl_ext_sig_algorithm_hash];\n}\n// ssl_ext_sig_algorithm_scheme\nif (ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme != null) {\n    ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme_value = params['ssl_ext_sig_algorithm_scheme'][ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme];\n}\n// dns_query_type\nif (ctx.gigamon.ami.dns_query_type != null) {\n    ctx.gigamon.ami.dns_query_type_value = params['dns_query_type'][ctx.gigamon.ami.dns_query_type];\n}\n// dns_reply_code\nif (ctx.gigamon.ami.dns_reply_code != null) {\n    ctx.gigamon.ami.dns_reply_code_value = params['dns_reply_code'][ctx.gigamon.ami.dns_reply_code];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"// end_reason\nif (ctx.gigamon.ami.end_reason != null) {\n    ctx.gigamon.ami.end_reason_value = params['end_reason'][ctx.gigamon.ami.end_reason];\n}\n// http_uri_path\nif (ctx.gigamon.ami.http_uri_path != null) {\n    ctx.gigamon.ami.http_uri_path_value = params['http_uri_path'][ctx.gigamon.ami.http_uri_path];\n}\n// smb_version\nif (ctx.gigamon.ami.smb_version != null) {\n    ctx.gigamon.ami.smb_version_value = params['smb_version'][ctx.gigamon.ami.smb_version];\n}\n// ssl_cipher_suite_id\nif (ctx.gigamon.ami.ssl_cipher_suite_id != null) {\n  String s = ctx.gigamon.ami.ssl_cipher_suite_id;\n  if (Character.isDigit(s.charAt(0))) {\n    def mapping = params['ssl_cipher_suite_id'][s];\n    if (mapping != null && mapping.size() > 0) {\n      ctx.gigamon.ami.ssl_cipher_suite_id_value = mapping[0];\n      if (mapping.size() > 1) {\n        ctx.gigamon.ami.ssl_cipher_suite_id_protocol = mapping[1];\n        }\n      }\n    }\n    else {\n        ctx.gigamon.ami.ssl_cipher_suite_id_value = s;\n    }\n}\n// ssl_protocol_version\nif (ctx.gigamon.ami.ssl_protocol_version != null) {\n    ctx.gigamon.ami.ssl_protocol_version_value = params['ssl_protocol_version'][ctx.gigamon.ami.ssl_protocol_version];\n}\n// ssl_ext_sig_algorithm_hash\nif (ctx.gigamon.ami.ssl_ext_sig_algorithm_hash != null) {\n    ctx.gigamon.ami.ssl_ext_sig_algorithm_hash_value = params['ssl_ext_sig_algorithm_hash'][ctx.gigamon.ami.ssl_ext_sig_algorithm_hash];\n}\n// ssl_ext_sig_algorithm_scheme\nif (ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme != null) {\n    ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme_value = params['ssl_ext_sig_algorithm_scheme'][ctx.gigamon.ami.ssl_ext_sig_algorithm_scheme];\n}\n// dns_query_type\nif (ctx.gigamon.ami.dns_query_type != null) {\n    ctx.gigamon.ami.dns_query_type_value = params['dns_query_type'][ctx.gigamon.ami.dns_query_type];\n}\n// dns_reply_code\nif (ctx.gigamon.ami.dns_reply_code != null) {\n    ctx.gigamon.ami.dns_reply_code_value = params['dns_reply_code'][ctx.gigamon.ami.dns_reply_code];\n}"#
                    ),
                    cached_params!(
                        "{\"end_reason\":{\"1\":\"Idle Timeout\",\"2\":\"Active Timeout\",\"3\":\"End of Flow\",\"0\":\"None\"},\"http_uri_path\":{\"*v1*\":\"V1\"},\"smb_version\":{\"1\":\"SMB-V1\",\"2\":\"SMB-V2\"},\"ssl_cipher_suite_id\":{\"47\":[\"TLS_RSA_WITH_AES_128_CBC_SHA\",\"AES128-SHA\"],\"50\":[\"TLS_DHE_DSS_WITH_AES_128_CBC_SHA\",\"DHE-DSS-AES128-SHA\"],\"51\":[\"TLS_DHE_RSA_WITH_AES_128_CBC_SHA\",\"DHE-RSA-AES128-SHA\"],\"52\":[\"TLS_DH_anon_WITH_AES_128_CBC_SHA\",\"ADH-AES128-SHA\"],\"53\":[\"TLS_RSA_WITH_AES_256_CBC_SHA\",\"AES256-SHA\"],\"56\":[\"TLS_DHE_DSS_WITH_AES_256_CBC_SHA\",\"DHE-DSS-AES256-SHA\"],\"57\":[\"TLS_DHE_RSA_WITH_AES_256_CBC_SHA\",\"DHE-RSA-AES256-SHA\"],\"58\":[\"TLS_DH_anon_WITH_AES_256_CBC_SHA\",\"ADH-AES256-SHA\"],\"65\":[\"TLS_RSA_WITH_CAMELLIA_128_CBC_SHA\",\"CAMELLIA128-SHA\"],\"68\":[\"TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA\",\"DHE-DSS-CAMELLIA128-SHA\"],\"69\":[\"TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA\",\"DHE-RSA-CAMELLIA128-SHA\"],\"70\":[\"TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA\",\"ADH-CAMELLIA128-SHA\"],\"108\":[\"TLS_DH_anon_WITH_AES_128_CBC_SHA256\",\"ADH-AES128-SHA256\"],\"109\":[\"TLS_DH_anon_WITH_AES_256_CBC_SHA256\",\"ADH-AES256-SHA256\"],\"132\":[\"TLS_RSA_WITH_CAMELLIA_256_CBC_SHA\",\"CAMELLIA256-SHA\"],\"135\":[\"TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA\",\"DHE-DSS-CAMELLIA256-SHA\"],\"136\":[\"TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA\",\"DHE-RSA-CAMELLIA256-SHA\"],\"137\":[\"TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA\",\"ADH-CAMELLIA256-SHA\"],\"138\":[\"TLS_PSK_WITH_RC4_128_SHA\",\"PSK-RC4-SHA\"],\"139\":[\"TLS_PSK_WITH_3DES_EDE_CBC_SHA\",\"PSK-3DES-EDE-CBC-SHA\"],\"140\":[\"TLS_PSK_WITH_AES_128_CBC_SHA\",\"PSK-AES128-CBC-SHA\"],\"141\":[\"TLS_PSK_WITH_AES_256_CBC_SHA\",\"PSK-AES256-CBC-SHA\"],\"150\":[\"TLS_RSA_WITH_SEED_CBC_SHA\",\"SEED-SHA\"],\"153\":[\"TLS_DHE_DSS_WITH_SEED_CBC_SHA\",\"DHE-DSS-SEED-SHA\"],\"154\":[\"TLS_DHE_RSA_WITH_SEED_CBC_SHA\",\"DHE-RSA-SEED-SHA\"],\"155\":[\"TLS_DH_anon_WITH_SEED_CBC_SHA\",\"ADH-SEED-SHA\"],\"156\":[\"TLS_RSA_WITH_AES_256_CBC_SHA\"],\"166\":[\"TLS_DH_anon_WITH_AES_128_GCM_SHA256\",\"ADH-AES128-GCM-SHA256\"],\"167\":[\"TLS_DH_anon_WITH_AES_256_GCM_SHA384\",\"ADH-AES256-GCM-SHA384\"],\"4865\":[\"TLS_AES_128_GCM_SHA256\"],\"4866\":[\"TLS_AES_256_GCM_SHA384\"],\"19171\":[\"TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA\"],\"49153\":[\"TLS_ECDH_ECDSA_WITH_NULL_SHA\",\"ECDH-ECDSA-NULL-SHA\"],\"49154\":[\"TLS_ECDH_ECDSA_WITH_RC4_128_SHA\",\"ECDH-ECDSA-RC4-SHA\"],\"49155\":[\"TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA\",\"ECDH-ECDSA-DES-CBC3-SHA\"],\"49156\":[\"TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA\",\"ECDH-ECDSA-AES128-SHA\"],\"49157\":[\"TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA\",\"ECDH-ECDSA-AES256-SHA\"],\"49158\":[\"TLS_ECDHE_ECDSA_WITH_NULL_SHA\",\"ECDHE-ECDSA-NULL-SHA\"],\"49159\":[\"TLS_ECDHE_ECDSA_WITH_RC4_128_SHA\",\"ECDHE-ECDSA-RC4-SHA\"],\"49160\":[\"TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA\",\"ECDHE-ECDSA-DES-CBC3-SHA\"],\"49161\":[\"TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA\",\"ECDHE-ECDSA-AES128-SHA\"],\"49162\":[\"TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA\",\"ECDHE-ECDSA-AES256-SHA\"],\"49163\":[\"TLS_ECDH_RSA_WITH_NULL_SHA\",\"ECDH-RSA-NULL-SHA\"],\"49164\":[\"TLS_ECDH_RSA_WITH_RC4_128_SHA\",\"ECDH-RSA-RC4-SHA\"],\"49165\":[\"TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA\",\"ECDH-RSA-DES-CBC3-SHA\"],\"49166\":[\"TLS_ECDH_RSA_WITH_AES_128_CBC_SHA\",\"ECDH-RSA-AES128-SHA\"],\"49167\":[\"TLS_ECDH_RSA_WITH_AES_256_CBC_SHA\",\"ECDH-RSA-AES256-SHA\"],\"49168\":[\"TLS_ECDHE_RSA_WITH_NULL_SHA\",\"ECDHE-RSA-NULL-SHA\"],\"49169\":[\"TLS_ECDHE_RSA_WITH_RC4_128_SHA\",\"ECDHE-RSA-RC4-SHA\"],\"49170\":[\"TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA\",\"ECDHE-RSA-DES-CBC3-SHA\"],\"49171\":[\"TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA\",\"ECDHE-RSA-AES128-SHA\"],\"49172\":[\"TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA\",\"ECDHE-RSA-AES256-SHA\"],\"49173\":[\"TLS_ECDH_anon_WITH_NULL_SHA\",\"AECDH-NULL-SHA\"],\"49174\":[\"TLS_ECDH_anon_WITH_RC4_128_SHA\",\"AECDH-RC4-SHA\"],\"49175\":[\"TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA\",\"AECDH-DES-CBC3-SHA\"],\"49176\":[\"TLS_ECDH_anon_WITH_AES_128_CBC_SHA\",\"AECDH-AES128-SHA\"],\"49177\":[\"TLS_ECDH_anon_WITH_AES_256_CBC_SHA\",\"AECDH-AES256-SHA\"],\"49178\":[\"TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA\",\"SRP-3DES-EDE-CBC-SHA\"],\"49179\":[\"TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA\",\"SRP-RSA-3DES-EDE-CBC-SHA\"],\"49180\":[\"TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA\",\"SRP-DSS-3DES-EDE-CBC-SHA\"],\"49181\":[\"TLS_SRP_SHA_WITH_AES_128_CBC_SHA\",\"SRP-AES-128-CBC-SHA\"],\"49182\":[\"TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA\",\"SRP-RSA-AES-128-CBC-SHA\"],\"49183\":[\"TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA\",\"SRP-DSS-AES-128-CBC-SHA\"],\"49184\":[\"TLS_SRP_SHA_WITH_AES_256_CBC_SHA\",\"SRP-AES-256-CBC-SHA\"],\"49185\":[\"TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA\",\"SRP-RSA-AES-256-CBC-SHA\"],\"49186\":[\"TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA\",\"SRP-DSS-AES-256-CBC-SHA\"],\"49191\":[\"TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256\"],\"49192\":[\"TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384\"],\"49195\":[\"TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256\"],\"49196\":[\"TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384\"],\"49199\":[\"TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256\"],\"49200\":[\"TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384\"],\"52392\":[\"TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256\"],\"52393\":[\"TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256\"]},\"ssl_protocol_version\":{\"2\":\"SSL_2_0\",\"768\":\"SSL_3_0\",\"769\":\"TLS_1_0\",\"770\":\"TLS_1_1\",\"771\":\"TLS_1_2\",\"772\":\"TLS_1_3\"},\"ssl_ext_sig_algorithm_hash\":{\"0\":\"None\",\"1\":\"MD5\",\"2\":\"SHA1\",\"3\":\"SHA224\",\"4\":\"SHA256\",\"5\":\"SHA384\",\"6\":\"SHA512\"},\"ssl_ext_sig_algorithm_scheme\":{\"1537\":\"rsa_pkcs1_sha512\",\"1027\":\"ecdsa_secp256r1_sha256\",\"257\":\"MD5 RSA\",\"514\":\"SHA1 DSA\",\"515\":\"ecdsa_sha1\",\"769\":\"SHA224 RSA\",\"770\":\"SHA224 DSA\",\"771\":\"SHA224 ECDSA\",\"1025\":\"rsa_pkcs1_sha256\",\"1026\":\"SHA256 DSA\",\"1281\":\"rsa_pkcs1_sha384\",\"1282\":\"SHA384 DSA\",\"1283\":\"ecdsa_secp384r1_sha384\",\"1538\":\"SHA512 DSA\",\"1539\":\"ecdsa_secp521r1_sha512\",\"2052\":\"rsa_pss_rsae_sha256\",\"2053\":\"rsa_pss_rsae_sha384\",\"2054\":\"rsa_pss_rsae_sha512\",\"2055\":\"ed25519\",\"2056\":\"ed448\",\"2057\":\"rsa_pss_pss_sha256\",\"2058\":\"rsa_pss_pss_sha384\",\"2059\":\"rsa_pss_pss_sha512\",\"2570\":\"GREASE\",\"0\":\"Anonymous\"},\"dns_query_type\":{\"1\":\"A\",\"2\":\"NS\",\"3\":\"MD\",\"4\":\"MF\",\"5\":\"CNAME\",\"6\":\"SOA\",\"7\":\"MB\",\"8\":\"MG\",\"9\":\"MR\",\"10\":\"NULL\",\"11\":\"WKS\",\"12\":\"PTR\",\"13\":\"HINFO\",\"14\":\"MINFO\",\"15\":\"MX\",\"16\":\"TXT\",\"17\":\"RP\",\"18\":\"AFSDB\",\"19\":\"X25\",\"20\":\"ISDN\",\"21\":\"RT\",\"22\":\"NSAP\",\"23\":\"NSAP-PTR\",\"24\":\"SIG\",\"25\":\"KEY\",\"26\":\"PX\",\"27\":\"GPOS\",\"28\":\"AAAA\",\"29\":\"LOC\",\"30\":\"NXT\",\"31\":\"EID\",\"32\":\"NIMLOC\",\"33\":\"SRV\",\"34\":\"ATMA\",\"35\":\"NAPTR\",\"36\":\"KX\",\"37\":\"CERT\",\"39\":\"DNAME\",\"40\":\"SINK\",\"41\":\"OPT\",\"42\":\"APL\",\"43\":\"DS\",\"44\":\"SSHFP\",\"45\":\"IPSECKEY\",\"46\":\"RRSIG\",\"47\":\"NSEC\",\"48\":\"DNSKEY\",\"49\":\"DHCID\",\"50\":\"NSEC3\",\"51\":\"NSEC3PARAM\",\"52\":\"TLSA\",\"53\":\"SMIMEA\",\"54\":\"Unassigned\",\"55\":\"HIP\",\"56\":\"NINFO\",\"57\":\"RKEY\",\"58\":\"TALINK\",\"59\":\"CDS\",\"60\":\"CDNSKEY\",\"61\":\"OPENPGPKEY\",\"62\":\"CSYNC\",\"63\":\"ZONEMD\",\"99\":\"SPF\",\"100\":\"UINFO\",\"101\":\"UID\",\"102\":\"GID\",\"103\":\"UNSPEC\",\"104\":\"NID\",\"105\":\"L32\",\"106\":\"L64\",\"107\":\"LP\",\"108\":\"EUI48\",\"109\":\"EUI64\",\"249\":\"TKEY\",\"250\":\"TSIG\",\"251\":\"IXFR\",\"252\":\"AXFR\",\"253\":\"MAILB\",\"254\":\"MAILA\",\"255\":\"*\",\"256\":\"URI\",\"257\":\"CAA\",\"258\":\"AVC\",\"259\":\"DOA\",\"260\":\"AMTRELAY\",\"32768\":\"TA\",\"32769\":\"DLV\",\"-1L\":\"unknown\"},\"dns_reply_code\":{\"0\":\"No Error\",\"1\":\"Format Error\",\"2\":\"Server Failure\",\"3\":\"Non-Existent Domain\",\"4\":\"Not Implemented\",\"5\":\"Query Refused\",\"6\":\"Name Exists when it should not\",\"7\":\"RR Set Exists when it should not\",\"8\":\"RR Set that should exist does not\",\"9\":\"Not Authorized\",\"10\":\"Name not contained in zone\",\"11\":\"DSO-TYPE Not Implemented\",\"16\":\"Bad OPT Version\",\"17\":\"Key not recognized\",\"18\":\"Signature out of time window\",\"19\":\"Bad TKEY Mode\",\"20\":\"Duplicate key name\",\"21\":\"Algorithm not supported\",\"22\":\"Bad Truncation\",\"23\":\"Bad/missing Server Cookie\",\"-1L\":\"unknown\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("gigamon.ami.dns_response_time") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event.duration = ctx.gigamon.ami.dns_response_time * 1000000000L;
                scale_field(
                    event,
                    &ScaleField::new(
                        "gigamon.ami.dns_response_time",
                        "event.duration",
                        Factor::Long(1000000000),
                    ),
                );
            }

            let _cond = { event.has_value("gigamon.ami.dns_query") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_query").cloned() {
                    event.set("dns.question.name", v)?;
                }
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
                // Painless script
                // Source: int idx = ctx.dns.question.name.lastIndexOf('.'); if (idx < 0) {\n  return;\n} ctx.dns.question.top_level_domain = ctx.dns.question.name.substring(idx + 1);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"int idx = ctx.dns.question.name.lastIndexOf('.'); if (idx < 0) {\n  return;\n} ctx.dns.question.top_level_domain = ctx.dns.question.name.substring(idx + 1);"#
                    ),
                )?;
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
                // Painless script
                // Source: def subdomain(String s) {\n  int n;\n  for (int i = s.length()-1; i >= 0; i--) {\n    if (s.charAt(i) == (char)'.') {\n      n++;\n      if (n == 2) {\n        return s.substring(0, i);\n      }\n    }\n  }\n  return null;\n} def sub = subdomain(ctx.dns.question.name); if (sub != null) {\n  ctx.dns.question.subdomain = sub;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def subdomain(String s) {\n  int n;\n  for (int i = s.length()-1; i >= 0; i--) {\n    if (s.charAt(i) == (char)'.') {\n      n++;\n      if (n == 2) {\n        return s.substring(0, i);\n      }\n    }\n  }\n  return null;\n} def sub = subdomain(ctx.dns.question.name); if (sub != null) {\n  ctx.dns.question.subdomain = sub;\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("gigamon.ami.dns_name") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_name").cloned() {
                    event.set("dns.question.registered_domain", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dns_host_addr") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("gigamon.ami.dns_host_addr")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("gigamon.ami.dns_ttl") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_ttl").cloned() {
                    event.set("dns.answers.ttl", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dns_query_type_value") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_query_type_value").cloned() {
                    event.set("dns.question.type", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dns_reply_code_value") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_reply_code_value").cloned() {
                    event.set("dns.response_code", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dns_host") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_host").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dns_host_type") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_host_type").cloned() {
                    event.set("host.type", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dns_reverse_addr") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("gigamon.ami.dns_reverse_addr")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("gigamon.ami.dns_message_type") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dns_message_type").cloned() {
                    event.set("dns.type", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_rtt") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("gigamon.ami.http_rtt") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "gigamon.ami.http_rtt".into(),
                                message,
                            }
                        })?;
                        event.set("gigamon.ami.http_rtt", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_http_rtt")?;
                    event.remove("gigamon.ami.http_rtt");
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

            let _cond = { event.has_value("gigamon.ami.http_rtt") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event.duration = ctx.gigamon.ami.http_rtt * 1000000000L;
                scale_field(
                    event,
                    &ScaleField::new(
                        "gigamon.ami.http_rtt",
                        "event.duration",
                        Factor::Long(1000000000),
                    ),
                );
            }

            let _cond = { event.has_value("gigamon.ami.http_user_agent") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_user_agent").cloned() {
                    event.set("user_agent.original", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_method") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_method").cloned() {
                    event.set("http.request.method", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_version") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_version").cloned() {
                    event.set("http.version", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_code") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_code").cloned() {
                    event.set("http.response.status_code", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_file_type") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_file_type").cloned() {
                    event.set("file.extension", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_referer") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_referer").cloned() {
                    event.set("http.request.referrer", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_server") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_server").cloned() {
                    event.set("server.domain", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_uri_path") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_uri_path").cloned() {
                    event.set("url.path", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_uri") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_uri").cloned() {
                    event.set("url.full", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_host") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_host").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.http_uri_raw") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.http_uri_raw").cloned() {
                    event.set("url.original", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_cipher_suite_id_value") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_cipher_suite_id_value").cloned() {
                    event.set("tls.cipher", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_validity_not_after") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_validity_not_after").cloned() {
                    event.set("tls.client.not_after", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_validity_not_before") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_validity_not_before").cloned() {
                    event.set("tls.client.not_before", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_protocol_version_value") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_protocol_version_value").cloned() {
                    event.set("tls.version_protocol", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_common_name") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_common_name").cloned() {
                    event.set("tls.client.server_name", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_issuer") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_issuer").cloned() {
                    event.set("tls.client.issuer", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.ssl_certificate_subject_cn") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.ssl_certificate_subject_cn").cloned() {
                    event.set("tls.client.subject", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.src_ip") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.src_ip").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dst_ip") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dst_ip").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.src_port") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.src_port").cloned() {
                    event.set("source.port", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dst_port") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dst_port").cloned() {
                    event.set("destination.port", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.app_name") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.app_name").cloned() {
                    event.set("network.protocol", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.start_time") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.start_time").cloned() {
                    event.set("event.start", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.end_time") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.end_time").cloned() {
                    event.set("event.end", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.src_ipv6") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.src_ipv6").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dst_ipv6") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dst_ipv6").cloned() {
                    event.set("destination.ip", v)?;
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

            let _cond = { event.has_value("gigamon.ami.src_bytes") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.src_bytes").cloned() {
                    event.set("source.bytes", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dst_bytes") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dst_bytes").cloned() {
                    event.set("destination.bytes", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.src_packets") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.src_packets").cloned() {
                    event.set("source.packets", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.dst_packets") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.dst_packets").cloned() {
                    event.set("destination.packets", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.snmp_version") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.snmp_version").cloned() {
                    event.set("service.version", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.smb_version_value") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.smb_version_value").cloned() {
                    event.set("service.version", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.id") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.app_id") };
            if _cond {
                if let Some(v) = event.get("gigamon.ami.app_id").cloned() {
                    event.set("service.id", v)?;
                }
            }

            let _cond = { event.has_value("gigamon.ami.src_mac") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.source.mac = ctx.gigamon.ami.src_mac.replace(\":\", \"-\").toUpperCase();
                guarded_replace(
                    event,
                    &GuardedReplace::new("gigamon.ami.src_mac", "source.mac", ":", "-"),
                );
            }

            let _cond = { event.has_value("gigamon.ami.dst_mac") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.destination.mac = ctx.gigamon.ami.dst_mac.replace(\":\", \"-\").toUpperCase();
                guarded_replace(
                    event,
                    &GuardedReplace::new("gigamon.ami.dst_mac", "destination.mac", ":", "-"),
                );
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("ts");
                event.remove("json");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
