// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cef_pipeline` pipeline.
pub struct CefPipeline;

impl Transform for CefPipeline {
    fn name(&self) -> &str {
        "cef_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("cef.extensions") };
            if _cond {
                event.rename("cef.extensions", "gigamon.ami")?;
            }

            let _cond = { event.has_value("cef.device") };
            if _cond {
                event.rename("cef.device", "gigamon.ami.device")?;
            }

                if event.has_value("gigamon.ami.GigamonMdata_smb_version") {
                    event.rename("gigamon.ami.GigamonMdata_smb_version", "gigamon.ami.smb_version")?;
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
                    event.rename("gigamon.ami.GigamonMdataFlowStartMsec", "gigamon.ami.start_time")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataFlowEndMsec") {
                    event.rename("gigamon.ami.GigamonMdataFlowEndMsec", "gigamon.ami.end_time")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataIntfName") {
                    event.rename("gigamon.ami.GigamonMdataIntfName", "gigamon.ami.intf_name")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataEgressIntfID") {
                    event.rename("gigamon.ami.GigamonMdataEgressIntfID", "gigamon.ami.egress_intf_id")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataSysUpTimeFirst") {
                    event.rename("gigamon.ami.GigamonMdataSysUpTimeFirst", "gigamon.ami.sys_up_time_first")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataSysUpTimeLast") {
                    event.rename("gigamon.ami.GigamonMdataSysUpTimeLast", "gigamon.ami.sys_up_time_last")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataFlowEndReason") {
                    event.rename("gigamon.ami.GigamonMdataFlowEndReason", "gigamon.ami.end_reason")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_smb_command_string") {
                    event.rename("gigamon.ami.GigamonMdata_smb_command_string", "gigamon.ami.smb_command_string")?;
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
                    event.rename("gigamon.ami.GigamonResponderOctets", "gigamon.ami.dst_bytes")?;
                }

                if event.has_value("gigamon.ami.GigamonResponderPackets") {
                    event.rename("gigamon.ami.GigamonResponderPackets", "gigamon.ami.dst_packets")?;
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
                    event.rename("gigamon.ami.GigamonInitiatorOctets", "gigamon.ami.src_bytes")?;
                }

                if event.has_value("gigamon.ami.GigamonInitiatorPackets") {
                    event.rename("gigamon.ami.GigamonInitiatorPackets", "gigamon.ami.src_packets")?;
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
                    event.rename("gigamon.ami.GigamonMdata_dns_qdcount", "gigamon.ami.dns_qdcount")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_transaction_id") {
                    event.rename("gigamon.ami.GigamonMdata_dns_transaction_id", "gigamon.ami.dns_transaction_id")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_name") {
                    event.rename("gigamon.ami.GigamonMdata_dns_name", "gigamon.ami.dns_name")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_host") {
                    event.rename("gigamon.ami.GigamonMdata_dns_host", "gigamon.ami.dns_host")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_host_addr") {
                    event.rename("gigamon.ami.GigamonMdata_dns_host_addr", "gigamon.ami.dns_host_addr")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_host_type") {
                    event.rename("gigamon.ami.GigamonMdata_dns_host_type", "gigamon.ami.dns_host_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_ttl") {
                    event.rename("gigamon.ami.GigamonMdata_dns_ttl", "gigamon.ami.dns_ttl")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_flags") {
                    event.rename("gigamon.ami.GigamonMdata_dns_flags", "gigamon.ami.dns_flags")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_opcode") {
                    event.rename("gigamon.ami.GigamonMdata_dns_opcode", "gigamon.ami.dns_opcode")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_class") {
                    event.rename("gigamon.ami.GigamonMdata_dns_class", "gigamon.ami.dns_class")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_host_class") {
                    event.rename("gigamon.ami.GigamonMdata_dns_host_class", "gigamon.ami.dns_host_class")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_host_raw") {
                    event.rename("gigamon.ami.GigamonMdata_dns_host_raw", "gigamon.ami.dns_host_raw")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_query") {
                    event.rename("gigamon.ami.GigamonMdata_dns_query", "gigamon.ami.dns_query")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_query_type") {
                    event.rename("gigamon.ami.GigamonMdata_dns_query_type", "gigamon.ami.dns_query_type")?;
                }

                if event.has_value("gigamon.ami.deviceInboundInterface") {
                    event.rename("gigamon.ami.deviceInboundInterface", "gigamon.ami.device_inbound_interface")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_ancount") {
                    event.rename("gigamon.ami.GigamonMdata_dns_ancount", "gigamon.ami.dns_ancount")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_arcount") {
                    event.rename("gigamon.ami.GigamonMdata_dns_arcount", "gigamon.ami.dns_arcount")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_reply_code") {
                    event.rename("gigamon.ami.GigamonMdata_dns_reply_code", "gigamon.ami.dns_reply_code")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_response_time") {
                    event.rename("gigamon.ami.GigamonMdata_dns_response_time", "gigamon.ami.dns_response_time")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_reverse_addr") {
                    event.rename("gigamon.ami.GigamonMdata_dns_reverse_addr", "gigamon.ami.dns_reverse_addr")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_dns_tunneling") {
                    event.rename("gigamon.ami.GigamonMdata_dns_tunneling", "gigamon.ami.dns_tunneling")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ip_wrong_crc") {
                    event.rename("gigamon.ami.GigamonMdata_ip_wrong_crc", "gigamon.ami.ip_wrong_crc")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_krb5_login") {
                    event.rename("gigamon.ami.GigamonMdata_krb5_login", "gigamon.ami.krb5_login")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_server") {
                    event.rename("gigamon.ami.GigamonMdata_http_server", "gigamon.ami.http_server")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_uri") {
                    event.rename("gigamon.ami.GigamonMdata_http_uri", "gigamon.ami.http_uri")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_uri_full") {
                    event.rename("gigamon.ami.GigamonMdata_http_uri_full", "gigamon.ami.http_uri_full")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_mime_type") {
                    event.rename("gigamon.ami.GigamonMdata_http_mime_type", "gigamon.ami.http_mime_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_server_agent") {
                    event.rename("gigamon.ami.GigamonMdata_http_server_agent", "gigamon.ami.http_server_agent")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_rtt") {
                    event.rename("gigamon.ami.GigamonMdata_http_rtt", "gigamon.ami.http_rtt")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_code") {
                    event.rename("gigamon.ami.GigamonMdata_http_code", "gigamon.ami.http_code")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_content_len") {
                    event.rename("gigamon.ami.GigamonMdata_http_content_len", "gigamon.ami.http_content_len")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_uri_path") {
                    event.rename("gigamon.ami.GigamonMdata_http_uri_path", "gigamon.ami.http_uri_path")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_request_size") {
                    event.rename("gigamon.ami.GigamonMdata_http_request_size", "gigamon.ami.http_request_size")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_host") {
                    event.rename("gigamon.ami.GigamonMdata_http_host", "gigamon.ami.http_host")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_uri_decoded") {
                    event.rename("gigamon.ami.GigamonMdata_http_uri_decoded", "gigamon.ami.http_uri_decoded")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_uri_raw") {
                    event.rename("gigamon.ami.GigamonMdata_http_uri_raw", "gigamon.ami.http_uri_raw")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_content_type") {
                    event.rename("gigamon.ami.GigamonMdata_http_content_type", "gigamon.ami.http_content_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_method") {
                    event.rename("gigamon.ami.GigamonMdata_http_method", "gigamon.ami.http_method")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_version") {
                    event.rename("gigamon.ami.GigamonMdata_http_version", "gigamon.ami.http_version")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_user_agent") {
                    event.rename("gigamon.ami.GigamonMdata_http_user_agent", "gigamon.ami.http_user_agent")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_response_ts") {
                    event.rename("gigamon.ami.GigamonMdata_http_response_ts", "gigamon.ami.http_response_ts")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_content_encoding") {
                    event.rename("gigamon.ami.GigamonMdata_http_content_encoding", "gigamon.ami.http_content_encoding")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_http_referer") {
                    event.rename("gigamon.ami.GigamonMdata_http_referer", "gigamon.ami.http_referer")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataFlowStartSec") {
                    event.rename("gigamon.ami.GigamonMdataFlowStartSec", "gigamon.ami.flow_start_sec")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpFlags") {
                    event.rename("gigamon.ami.GigamonMdataTcpFlags", "gigamon.ami.tcp_flags")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_tcp_rtt_app") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_rtt_app", "gigamon.ami.tcp_rtt_app")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_tcp_loss_count") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_loss_count", "gigamon.ami.tcp_loss_count")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_tcp_rtt") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_rtt", "gigamon.ami.tcp_rtt")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_tcp_retransmission_bytes") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_retransmission_bytes", "gigamon.ami.tcp_retransmission_bytes")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_tcp_wrong_crc") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_wrong_crc", "gigamon.ami.tcp_wrong_crc")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_tcp_flag_reset") {
                    event.rename("gigamon.ami.GigamonMdata_tcp_flag_reset", "gigamon.ami.tcp_flag_reset")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certif_md5") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certif_md5", "gigamon.ami.ssl_certif_md5")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_common_name") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_common_name", "gigamon.ami.ssl_common_name")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_validity_not_before") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_validity_not_before", "gigamon.ami.ssl_validity_not_before")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_validity_not_after") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_validity_not_after", "gigamon.ami.ssl_validity_not_after")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_serial_number") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_serial_number", "gigamon.ami.ssl_serial_number")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_handshake_type") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_handshake_type", "gigamon.ami.ssl_handshake_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_organization_name") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_organization_name", "gigamon.ami.ssl_organization_name")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_request_size") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_request_size", "gigamon.ami.ssl_request_size")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_cipher_suite_id") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_cipher_suite_id", "gigamon.ami.ssl_cipher_suite_id")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_cipher_suite_list") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_cipher_suite_list", "gigamon.ami.ssl_cipher_suite_list")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certif_sha1") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certif_sha1", "gigamon.ami.ssl_certif_sha1")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_content_type") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_content_type", "gigamon.ami.ssl_content_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_protocol_version") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_protocol_version", "gigamon.ami.ssl_protocol_version")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_client_hello_extension_type") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_client_hello_extension_type", "gigamon.ami.ssl_client_hello_extension_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_server_hello_extension_type") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_server_hello_extension_type", "gigamon.ami.ssl_server_hello_extension_type")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_dn_subject") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_dn_subject", "gigamon.ami.ssl_certificate_dn_subject")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_cn") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_cn", "gigamon.ami.ssl_certificate_subject_cn")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_l") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_l", "gigamon.ami.ssl_certificate_subject_l")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_st") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_st", "gigamon.ami.ssl_certificate_subject_st")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_o") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_o", "gigamon.ami.ssl_certificate_subject_o")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_ou") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_ou", "gigamon.ami.ssl_certificate_subject_ou")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_c") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_c", "gigamon.ami.ssl_certificate_subject_c")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_dn_issuer") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_dn_issuer", "gigamon.ami.ssl_certificate_dn_issuer")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_cn") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_issuer_cn", "gigamon.ami.ssl_certificate_issuer_cn")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_l") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_issuer_l", "gigamon.ami.ssl_certificate_issuer_l")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_st") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_issuer_st", "gigamon.ami.ssl_certificate_issuer_st")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_o") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_issuer_o", "gigamon.ami.ssl_certificate_issuer_o")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_ou") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_issuer_ou", "gigamon.ami.ssl_certificate_issuer_ou")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_issuer_c") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_issuer_c", "gigamon.ami.ssl_certificate_issuer_c")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_client_hello_extension_len") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_client_hello_extension_len", "gigamon.ami.ssl_client_hello_extension_len")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_server_hello_extension_len") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_server_hello_extension_len", "gigamon.ami.ssl_server_hello_extension_len")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_nb_compression_methods") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_nb_compression_methods", "gigamon.ami.ssl_nb_compression_methods")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_compression_method") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_compression_method", "gigamon.ami.ssl_compression_method")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithms_len") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithms_len", "gigamon.ami.ssl_ext_sig_algorithms_len")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_scheme") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_scheme", "gigamon.ami.ssl_ext_sig_algorithm_scheme")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_hash") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_hash", "gigamon.ami.ssl_ext_sig_algorithm_hash")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_sig") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_ext_sig_algorithm_sig", "gigamon.ami.ssl_ext_sig_algorithm_sig")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_key_algo_oid") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_key_algo_oid", "gigamon.ami.ssl_certificate_subject_key_algo_oid")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_certificate_subject_key_size") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_certificate_subject_key_size", "gigamon.ami.ssl_certificate_subject_key_size")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_cert_extension_oid") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_cert_extension_oid", "gigamon.ami.ssl_cert_extension_oid")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_cert_ext_authority_key_id") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_cert_ext_authority_key_id", "gigamon.ami.ssl_cert_ext_authority_key_id")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_cert_ext_subject_key_id") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_cert_ext_subject_key_id", "gigamon.ami.ssl_cert_ext_subject_key_id")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_fingerprint_ja3") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_fingerprint_ja3", "gigamon.ami.ssl_fingerprint_ja3")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_index") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_index", "gigamon.ami.ssl_index")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_issuer") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_issuer", "gigamon.ami.ssl_issuer")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_session_id") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_session_id", "gigamon.ami.ssl_session_id")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_declassify_override") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_declassify_override", "gigamon.ami.ssl_declassify_override")?;
                }

                if event.has_value("gigamon.ami.GigamonMdata_ssl_signalization_override") {
                    event.rename("gigamon.ami.GigamonMdata_ssl_signalization_override", "gigamon.ami.ssl_signalization_override")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpRttMin") {
                    event.rename("gigamon.ami.GigamonMdataTcpRttMin", "gigamon.ami.tcp_rtt_min")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpRttMax") {
                    event.rename("gigamon.ami.GigamonMdataTcpRttMax", "gigamon.ami.tcp_rtt_max")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpRttMean") {
                    event.rename("gigamon.ami.GigamonMdataTcpRttMean", "gigamon.ami.tcp_rtt_mean")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpRttAppMin") {
                    event.rename("gigamon.ami.GigamonMdataTcpRttAppMin", "gigamon.ami.tcp_rtt_app_min")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpRttAppMax") {
                    event.rename("gigamon.ami.GigamonMdataTcpRttAppMax", "gigamon.ami.tcp_rtt_app_max")?;
                }

                if event.has_value("gigamon.ami.GigamonMdataTcpRttAppMean") {
                    event.rename("gigamon.ami.GigamonMdataTcpRttAppMean", "gigamon.ami.tcp_rtt_app_mean")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("cef");
                event.remove("_tmp");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
