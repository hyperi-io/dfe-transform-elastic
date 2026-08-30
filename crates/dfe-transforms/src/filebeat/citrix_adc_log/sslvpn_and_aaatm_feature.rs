// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `sslvpn_and_aaatm_feature` pipeline.
pub struct SslvpnAndAaatmFeature;

impl Transform for SslvpnAndAaatmFeature {
    fn name(&self) -> &str {
        "sslvpn_and_aaatm_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("citrix.name") == Some("LOGIN") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - SSLVPN_client_type %{DATA:citrix_adc.log.sslvpn_client_type} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    // Grok pattern: ^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - (SSLVPN_client_type %{WORD:citrix_adc.log.sslvpn_client_type} - )?Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                            cached_grok!("^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - SSLVPN_client_type %{DATA:citrix_adc.log.sslvpn_client_type} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                            cached_grok!("^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Browser_type \"%{DATA:citrix_adc.log.browser_type}\" - (SSLVPN_client_type %{WORD:citrix_adc.log.sslvpn_client_type} - )?Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("LOGOUT") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|%{DATA}))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \"%{DATA:_tmp.start_time}\"(?:%{SPACE}-%{SPACE})End_time \"%{DATA:_tmp.end_time}\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})?(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \"%{DATA:citrix_adc.log.logout_method}\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\") ?$
                    // Grok pattern: ^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP}(?:%{SPACE}-%{SPACE})SessionId: %{NUMBER:citrix_adc.log.session_id}(?:%{SPACE}-%{SPACE})(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|\\\\?\"%{DATA}\\\\?\"))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \\\\?\"%{DATA:_tmp.start_time}\\\\?\"(?:%{SPACE}-%{SPACE})End_time \\\\?\"%{DATA:_tmp.end_time}\\\\?\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \\\\?\"%{DATA:citrix_adc.log.logout_method}\\\\?\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \\\\?\"%{DATA:citrix_adc.log.groups}\\\\?\") ?$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|%{DATA}))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \"%{DATA:_tmp.start_time}\"(?:%{SPACE}-%{SPACE})End_time \"%{DATA:_tmp.end_time}\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})?(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \"%{DATA:citrix_adc.log.logout_method}\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\") ?$"),
                            cached_grok!("^(Logout handler : )?Context %{DATA:citrix_adc.log.username}@%{IP}(?:%{SPACE}-%{SPACE})SessionId: %{NUMBER:citrix_adc.log.session_id}(?:%{SPACE}-%{SPACE})(?:User %{DATA:citrix_adc.log.user})(?:%{SPACE}-%{SPACE})(?:Client_ip (%{IP:citrix_adc.log.client_ip})?)(?:%{SPACE}-%{SPACE})(?:Nat_ip (%{IP:citrix_adc.log.nat.ip}|\\\\?\"%{DATA}\\\\?\"))(?:%{SPACE}-%{SPACE})(?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port})(?:%{SPACE}-%{SPACE})(?:Start_time \\\\?\"%{DATA:_tmp.start_time}\\\\?\"(?:%{SPACE}-%{SPACE})End_time \\\\?\"%{DATA:_tmp.end_time}\\\\?\"(?:%{SPACE}-%{SPACE})Duration %{NOTSPACE:citrix_adc.log.duration})(?:%{SPACE}-%{SPACE})(?:Http_resources_accessed %{INT:citrix_adc.log.http_resources_accessed})(?:%{SPACE}-%{SPACE})(?:(?:NonHttp_services_accessed %{INT:citrix_adc.log.non_http_services_accessed}(?:%{SPACE}-%{SPACE}))?)(?:Total_TCP_connections %{INT:citrix_adc.log.total_tcp_connections}(?:(?:%{SPACE}-%{SPACE})Total_UDP_flows %{INT:citrix_adc.log.total_udp_flows})?)(?:%{SPACE}-%{SPACE})(?:Total_policies_allowed %{INT:citrix_adc.log.total_policies_allowed}(?:%{SPACE}-%{SPACE})Total_policies_denied %{INT:citrix_adc.log.total_policies_denied})(?:%{SPACE}-%{SPACE})(?:Total_bytes_send %{INT:citrix_adc.log.total_bytes_send}(?:%{SPACE}-%{SPACE})Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received})(?:%{SPACE}-%{SPACE})(?:Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send}(?:%{SPACE}-%{SPACE})Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved})(?:%{SPACE}-%{SPACE})(?:Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send}%(?:%{SPACE}-%{SPACE})Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved}%)(?:%{SPACE}-%{SPACE})(?:LogoutMethod \\\\?\"%{DATA:citrix_adc.log.logout_method}\\\\?\"(?:%{SPACE}-%{SPACE})Group\\(s\\) \\\\?\"%{DATA:citrix_adc.log.groups}\\\\?\") ?$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("ICASTART") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$
                    // Grok pattern: ^%{DATA} Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$"),
                            cached_grok!("^%{DATA} Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - customername(?:%{SPACE}%{WORD:citrix_adc.log.customer_name})?(?:%{SPACE}-%{SPACE})username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - applicationName %{DATA:citrix_adc.log.application_name} - startTime \"%{DATA:_tmp.start_time}\" - connectionId %{WORD:citrix_adc.log.connection_id}%{SPACE}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("ICAEND_CONNSTAT") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? - username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} ?$
                    // Grok pattern: ^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? ?- username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} - Total_bytes_wire_send %{INT:citrix_adc.log.total_bytes_wire_send} - Total_bytes_wire_recv %{INT:citrix_adc.log.total_bytes_wire_recieved} ?$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? - username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} ?$"),
                            cached_grok!("^%{DATA} ?Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - (SSLRelayAddress %{IP:citrix_adc.log.ssl_relay.address}:%{INT:citrix_adc.log.ssl_relay.port} - )?customername (%{WORD:citrix_adc.log.customer_name})? ?- username:domainname %{DATA:citrix_adc.log.username}:%{DATA:citrix_adc.log.domain_name} - startTime \"%{DATA:_tmp.start_time}\" - endTime \"%{DATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} ? - Total_bytes_send %{INT:citrix_adc.log.total_bytes_send} - Total_bytes_recv %{INT:citrix_adc.log.total_bytes_received} - Total_compressedbytes_send %{INT:citrix_adc.log.total_compressed_bytes_send} - Total_compressedbytes_recv %{INT:citrix_adc.log.total_compressed_bytes_recieved} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - connectionId %{WORD:citrix_adc.log.connection_id} - Total_bytes_wire_send %{INT:citrix_adc.log.total_bytes_wire_send} - Total_bytes_wire_recv %{INT:citrix_adc.log.total_bytes_wire_recieved} ?$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("TCPCONNSTAT") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\"$
                    // Grok pattern: ^Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\"$"),
                            cached_grok!("^Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Total_compressedbytes_send %{NUMBER:citrix_adc.log.total_compressed_bytes_send:int} - Total_compressedbytes_recv %{NUMBER:citrix_adc.log.total_compressed_bytes_recieved:int} - Compression_ratio_send %{NUMBER:citrix_adc.log.compression_ratio_send:float}% - Compression_ratio_recv %{NUMBER:citrix_adc.log.compression_ratio_recieved:float}% - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("TCPCONN_TIMEDOUT") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    // Grok pattern: ^Context %{DATA} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                            cached_grok!("^Context %{DATA} - SessionId: %{NUMBER:citrix_adc.log.session_id} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Last_contact \"%{DATA:citrix_adc.log.last_contact}\" - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("UDPFLOWSTAT") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    // Grok pattern: ^(Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - )?(\\[%{DATA}\\] )?User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                            cached_grok!("^(Context %{DATA:citrix_adc.log.username}@%{IP} - SessionId: %{NUMBER:citrix_adc.log.session_id} - )?(\\[%{DATA}\\] )?User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"${DATA}\") - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Source %{IP:citrix_adc.log.source.ip}:%{INT:citrix_adc.log.source.port} - Destination %{IP:citrix_adc.log.destination.ip}:%{INT:citrix_adc.log.destination.port} - Start_time \"%{DATA:_tmp.start_time}\" - End_time \"%{GREEDYDATA:_tmp.end_time}\" - Duration %{DATA:citrix_adc.log.duration} - Total_bytes_send %{NUMBER:citrix_adc.log.total_bytes_send:int} - Total_bytes_recv %{NUMBER:citrix_adc.log.total_bytes_received:int} - Access %{WORD:citrix_adc.log.access} - Group\\(s\\) \"%{DATA:citrix_adc.log.groups}\" ?$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("HTTPREQUEST") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^(?:(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) )?(?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - %{DATA:_tmp.timestamp}(?: %{WORD:citrix_adc.log.timezone})?(?: : (?:Message = )?SSO is %{WORD:citrix_adc.log.sso_status})? : (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$
                    // Grok pattern: ^(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) (?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - (?:%{DATA:_tmp.timestamp} %{DATA:citrix_adc.log.timezone}) (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^(?:(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) )?(?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - %{DATA:_tmp.timestamp}(?: %{WORD:citrix_adc.log.timezone})?(?: : (?:Message = )?SSO is %{WORD:citrix_adc.log.sso_status})? : (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$"),
                            cached_grok!("^(?:Context (?:(%{USERNAME:citrix_adc.log.username}|%{EMAILADDRESS:citrix_adc.log.username}|%{DATA:citrix_adc.log.username}))@%{IP:citrix_adc.log.client_ip} ?- SessionId: %{NUMBER:citrix_adc.log.session_id} ?-) (?:(?:\\[TECHSUPPORT\\]\\[ENUMERATION\\] )?)(?:%{HOSTNAME:citrix_adc.log.hostname} User (?:(%{USERNAME:citrix_adc.log.user}|%{EMAILADDRESS:citrix_adc.log.user}|%{DATA:citrix_adc.log.user})) ?: Group\\(s\\) %{DATA:citrix_adc.log.groups}) : (?:Vserver %{IP:citrix_adc.log.vserver.ip}:%{NUMBER:citrix_adc.log.vserver.port}) - (?:%{DATA:_tmp.timestamp} %{DATA:citrix_adc.log.timezone}) (?:%{WORD:citrix_adc.log.method} %{DATA:citrix_adc.log.request.path} - -) ?$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("NONHTTP_RESOURCEACCESS_DENIED") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$
                    let _ = cached_grok!("^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$").extract_into(&input, event)?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("HTTP_RESOURCEACCESS_DENIED") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$
                    let _ = cached_grok!("^- Denied_by_policy \"%{DATA:citrix_adc.log.policy_violation}\" ?$").extract_into(&input, event)?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("LICLMT_REACHED") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - License_limit %{NUMBER:citrix_adc.log.license_limit:int} ?$
                    let _ = cached_grok!("^Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - License_limit %{NUMBER:citrix_adc.log.license_limit:int} ?$").extract_into(&input, event)?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("CLISEC_CHECK") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^%{WORD:citrix_adc.log.alert_type} ?: %{WORD:citrix_adc.log.alert_level} - ClientIP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{DATA:citrix_adc.log.client_security_expression}\" - ?$
                    // Grok pattern: ^CaseID: %{WORD} - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{GREEDYDATA:citrix_adc.log.client_security_expression}\" - Client_security_check (?:\"%{GREEDYDATA:citrix_adc.log.client_security_check_status}\"|%{WORD:citrix_adc.log.client_security_check_status})$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^%{WORD:citrix_adc.log.alert_type} ?: %{WORD:citrix_adc.log.alert_level} - ClientIP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{DATA:citrix_adc.log.client_security_expression}\" - ?$"),
                            cached_grok!("^CaseID: %{WORD} - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client_security_expression \"%{GREEDYDATA:citrix_adc.log.client_security_expression}\" - Client_security_check (?:\"%{GREEDYDATA:citrix_adc.log.client_security_check_status}\"|%{WORD:citrix_adc.log.client_security_check_status})$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("STA_VALIDATE_RESP") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Xdatalen %{NUMBER:citrix_adc.log.data_length:int} - Xdata %{GREEDYDATA:citrix_adc.log.data} ?$
                    let _ = cached_grok!("^Xdatalen %{NUMBER:citrix_adc.log.data_length:int} - Xdata %{GREEDYDATA:citrix_adc.log.data} ?$").extract_into(&input, event)?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("REMOVE_SESSION_DEBUG") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^(Sessionid|Session id) %{NUMBER:citrix_adc.log.session_id:int} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver_ip %{IP:citrix_adc.log.vserver.ip} - Errmsg \"%{DATA:citrix_adc.log.errmsg}\" ?$
                    let _ = cached_grok!("^(Sessionid|Session id) %{NUMBER:citrix_adc.log.session_id:int} - User %{DATA:citrix_adc.log.user} - Client_ip %{IP:citrix_adc.log.client_ip} - Nat_ip (%{IP:citrix_adc.log.nat.ip}|\"%{DATA}\") - Vserver_ip %{IP:citrix_adc.log.vserver.ip} - Errmsg \"%{DATA:citrix_adc.log.errmsg}\" ?$").extract_into(&input, event)?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("CLISEC_EXP_EVAL") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^User %{USER:citrix_adc.log.user}%{SPACE}:%{SPACE}- Client%{SPACE}IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client%{SPACE}security%{SPACE}check%{SPACE}Passed\\(%{NUMBER:citrix_adc.log.client_security_check_status:int}\\)%{SPACE}on%{SPACE}the%{SPACE}client%{SPACE}machine$
                    // Grok pattern: ^CaseID %{WORD}: - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client security check %{GREEDYDATA} EXISTS %{GREEDYDATA:citrix_adc.log.client_security_check_status} on the client machine$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^User %{USER:citrix_adc.log.user}%{SPACE}:%{SPACE}- Client%{SPACE}IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client%{SPACE}security%{SPACE}check%{SPACE}Passed\\(%{NUMBER:citrix_adc.log.client_security_check_status:int}\\)%{SPACE}on%{SPACE}the%{SPACE}client%{SPACE}machine$"),
                            cached_grok!("^CaseID %{WORD}: - Client IP %{IP:citrix_adc.log.client_ip} - Vserver %{IP:citrix_adc.log.vserver.ip}:%{INT:citrix_adc.log.vserver.port} - Client security check %{GREEDYDATA} EXISTS %{GREEDYDATA:citrix_adc.log.client_security_check_status} on the client machine$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.get_str("citrix.name") == Some("Message") };
            if _cond {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Logout handler : %{DATA}, for user <%{USERNAME|EMAILADDRESS:citrix_adc.log.username}>$
                    // Grok pattern: ^aaatm_handler successfully parsed assertion client ip is %{IP:citrix_adx.log.client_ip}, username is %{DATA:citrix_adc.log.user}$
                    // Grok pattern: %{DATA}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Logout handler : %{DATA}, for user <%{USERNAME|EMAILADDRESS:citrix_adc.log.username}>$"),
                            cached_grok!("^aaatm_handler successfully parsed assertion client ip is %{IP:citrix_adx.log.client_ip}, username is %{DATA:citrix_adc.log.user}$"),
                            cached_grok!("%{DATA}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.has_value("citrix_adc.log.client_ip") && event.get_str("citrix_adc.log.client_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.client_ip") {
                if let Some(val) = event.get("citrix_adc.log.client_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.client_ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.client_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.client_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.ip", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.domain", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_bytes_received") {
                if let Some(val) = event.get("citrix_adc.log.total_bytes_received") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_bytes_received".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_bytes_received", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_received_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.total_bytes_received").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.bytes", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.destination.ip") && event.get_str("citrix_adc.log.destination.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.destination.ip") {
                if let Some(val) = event.get("citrix_adc.log.destination.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.destination.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.destination.port") {
                if let Some(val) = event.get("citrix_adc.log.destination.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.destination.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.groups").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("group.name", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.vserver.ip") && event.get_str("citrix_adc.log.vserver.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.vserver.ip") {
                if let Some(val) = event.get("citrix_adc.log.vserver.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.vserver.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.vserver.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vserver_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.vserver.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.vserver.port") {
                if let Some(val) = event.get("citrix_adc.log.vserver.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.vserver.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.vserver.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_vserver_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.vserver.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.port", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_bytes_send") {
                if let Some(val) = event.get("citrix_adc.log.total_bytes_send") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_bytes_send".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_bytes_send", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_send_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.total_bytes_send").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.bytes", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.source.ip") && event.get_str("citrix_adc.log.source.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.source.ip") {
                if let Some(val) = event.get("citrix_adc.log.source.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.source.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.nat.ip") && event.get_str("citrix_adc.log.nat.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.nat.ip") {
                if let Some(val) = event.get("citrix_adc.log.nat.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.nat.ip".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.nat.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_nat_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("citrix_adc.log.nat.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.nat.ip", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.source.port") {
                if let Some(val) = event.get("citrix_adc.log.source.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.source.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.source.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("citrix_adc.log.ssl_relay.address") && event.get_str("citrix_adc.log.ssl_relay.address") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.ssl_relay.address") {
                if let Some(val) = event.get("citrix_adc.log.ssl_relay.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.ssl_relay.address".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.ssl_relay.address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ssl_relay_address_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.ssl_relay.port") {
                if let Some(val) = event.get("citrix_adc.log.ssl_relay.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.ssl_relay.port".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.ssl_relay.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ssl_relay_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.session_id") {
                if let Some(val) = event.get("citrix_adc.log.session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.session_id".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.session_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_tcp_connections") {
                if let Some(val) = event.get("citrix_adc.log.total_tcp_connections") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_tcp_connections".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_tcp_connections", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_tcp_connections_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_udp_flows") {
                if let Some(val) = event.get("citrix_adc.log.total_udp_flows") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_udp_flows".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_udp_flows", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_udp_flows_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_policies_allowed") {
                if let Some(val) = event.get("citrix_adc.log.total_policies_allowed") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_policies_allowed".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_policies_allowed", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_policies_allowed_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_bytes_wire_send") {
                if let Some(val) = event.get("citrix_adc.log.total_bytes_wire_send") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_bytes_wire_send".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_bytes_wire_send", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_wire_send_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_bytes_wire_recieved") {
                if let Some(val) = event.get("citrix_adc.log.total_bytes_wire_recieved") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_bytes_wire_recieved".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_bytes_wire_recieved", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_bytes_wire_recieved_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_policies_denied") {
                if let Some(val) = event.get("citrix_adc.log.total_policies_denied") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_policies_denied".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_policies_denied", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_policies_denied_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_compressed_bytes_send") {
                if let Some(val) = event.get("citrix_adc.log.total_compressed_bytes_send") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_compressed_bytes_send".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_compressed_bytes_send", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_compressed_bytes_send_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.total_compressed_bytes_recieved") {
                if let Some(val) = event.get("citrix_adc.log.total_compressed_bytes_recieved") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.total_compressed_bytes_recieved".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.total_compressed_bytes_recieved", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_compressed_bytes_recieved_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.compression_ratio_send") {
                if let Some(val) = event.get("citrix_adc.log.compression_ratio_send") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.compression_ratio_send".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.compression_ratio_send", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_compression_ratio_send_to_double")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.compression_ratio_recieved") {
                if let Some(val) = event.get("citrix_adc.log.compression_ratio_recieved") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.compression_ratio_recieved".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.compression_ratio_recieved", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_compression_ratio_recieved_to_double")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.license_limit") {
                if let Some(val) = event.get("citrix_adc.log.license_limit") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.license_limit".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.license_limit", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_license_limit_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.client_security_check_status") {
                if let Some(val) = event.get("citrix_adc.log.client_security_check_status") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.client_security_check_status".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.client_security_check_status", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_security_check_status_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.data_length") {
                if let Some(val) = event.get("citrix_adc.log.data_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.data_length".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.data_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_data_length_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("citrix_adc.log.domain_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.domain", v)?;
            }

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
