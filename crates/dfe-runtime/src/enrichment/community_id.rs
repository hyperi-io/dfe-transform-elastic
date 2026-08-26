// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Community ID hash algorithm for network flow identification.
//!
//! Implements the Community ID v1 spec for deterministic network flow hashing.
//! Reference: <https://github.com/corelight/community-id-spec>

use std::net::IpAddr;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use sha1::{Digest, Sha1};

use crate::error::{Result, TransformError};
use crate::event::Event;

/// IANA protocol number mapping from transport name.
fn protocol_number(transport: &str) -> Option<u8> {
    match transport.to_uppercase().as_str() {
        "HOPOPT" | "0" => Some(0),
        "ICMP" | "1" => Some(1),
        "IGMP" | "2" => Some(2),
        "TCP" | "6" => Some(6),
        "UDP" | "17" => Some(17),
        "RSVP" | "46" => Some(46),
        "GRE" | "47" => Some(47),
        "ESP" | "50" => Some(50),
        "AH" | "51" => Some(51),
        "ICMPV6" | "ICMP6" | "IPV6-ICMP" | "58" => Some(58),
        "EIGRP" | "88" => Some(88),
        "OSPF" | "89" => Some(89),
        "VRRP" | "112" => Some(112),
        "SCTP" | "132" => Some(132),
        _ => transport.parse::<u8>().ok(),
    }
}

const PROTO_ICMP: u8 = 1;
const PROTO_TCP: u8 = 6;
const PROTO_UDP: u8 = 17;
const PROTO_ICMPV6: u8 = 58;
const PROTO_SCTP: u8 = 132;

/// The ICMP message type that ANSWERS this one, where there is one.
///
/// The spec substitutes type and code for the ports, but a request and its
/// reply must hash to the SAME id -- so a type with a counterpart is stored
/// as `(type, counterpart)` and the pair is ordered like a port pair. A type
/// with no counterpart has no reply to match, so the flow is one-way and is
/// hashed in the direction it arrived.
fn icmp_counterpart(proto: u8, icmp_type: u16) -> Option<u16> {
    let pairs: &[(u16, u16)] = match proto {
        PROTO_ICMP => &[
            (0, 8),   // echo reply / echo
            (9, 10),  // router advertisement / solicitation
            (13, 14), // timestamp / timestamp reply
            (15, 16), // information request / reply
            (17, 18), // address mask request / reply
            (33, 34), // IPv6 where-are-you / i-am-here
            (35, 36), // mobile registration request / reply
            (37, 38), // domain name request / reply
        ],
        PROTO_ICMPV6 => &[
            (128, 129), // echo request / reply
            (130, 131), // multicast listener query / report
            (133, 134), // router solicitation / advertisement
            (135, 136), // neighbor solicitation / advertisement
            (144, 145), // home agent address discovery request / reply
        ],
        _ => return None,
    };
    pairs.iter().find_map(|&(a, b)| match icmp_type {
        t if t == a => Some(b),
        t if t == b => Some(a),
        _ => None,
    })
}

/// Compute Community ID v1 hash.
///
/// Follows the spec: `1:<base64(sha1(seed + src_ip + dst_ip + proto + 0 + src_port + dst_port))>`
pub fn community_id_v1(
    src_ip: &str,
    dst_ip: &str,
    src_port: u16,
    dst_port: u16,
    transport: &str,
    seed: u16,
) -> std::result::Result<String, String> {
    let src: IpAddr = src_ip
        .parse()
        .map_err(|e| format!("invalid source IP '{src_ip}': {e}"))?;
    let dst: IpAddr = dst_ip
        .parse()
        .map_err(|e| format!("invalid destination IP '{dst_ip}': {e}"))?;
    let proto =
        protocol_number(transport).ok_or_else(|| format!("unknown transport: {transport}"))?;

    // Elastic's processor demands a real port pair on the transport protocols
    // -- `if (flow.sourcePort < 1 || flow.sourcePort > 65535) throw` -- and an
    // absent port parses to 0 and hits the same check. So a udp flow logged
    // with `(0)` for both ports gets NO id, where hashing the zeroes would
    // have invented one. A `u16` cannot exceed 65535, so 0 is the whole test.
    if matches!(proto, PROTO_TCP | PROTO_UDP | PROTO_SCTP) {
        if src_port == 0 {
            return Err("invalid source port [0]".to_string());
        }
        if dst_port == 0 {
            return Err("invalid destination port [0]".to_string());
        }
    }

    // Only five protocols carry a port pair; for the rest the spec omits the
    // four port bytes rather than hashing zeroes. ICMP substitutes the
    // message type and code, pairing a type with its counterpart so a reply
    // hashes to the same id, and leaving an unpaired type as a one-way flow.
    let ports = match proto {
        PROTO_ICMP | PROTO_ICMPV6 => Some(match icmp_counterpart(proto, src_port) {
            Some(counterpart) => (src_port, counterpart, true),
            None => (src_port, dst_port, false),
        }),
        PROTO_TCP | PROTO_UDP | PROTO_SCTP => Some((src_port, dst_port, true)),
        _ => None,
    };
    let (src_port, dst_port, orderable) = ports.unwrap_or((0, 0, true));

    // Determine ordering: lower IP first, then lower port for tie-breaking
    let (ordered_src, ordered_dst, ordered_src_port, ordered_dst_port) =
        if !orderable || src < dst || (src == dst && src_port < dst_port) {
            (src, dst, src_port, dst_port)
        } else {
            (dst, src, dst_port, src_port)
        };

    let mut hasher = Sha1::new();

    // Seed (2 bytes, big-endian)
    hasher.update(seed.to_be_bytes());

    // Source IP (4 bytes for IPv4, 16 for IPv6)
    match ordered_src {
        IpAddr::V4(ip) => hasher.update(ip.octets()),
        IpAddr::V6(ip) => hasher.update(ip.octets()),
    }

    // Destination IP
    match ordered_dst {
        IpAddr::V4(ip) => hasher.update(ip.octets()),
        IpAddr::V6(ip) => hasher.update(ip.octets()),
    }

    // Protocol (1 byte)
    hasher.update([proto]);

    // Padding (1 byte)
    hasher.update([0u8]);

    // Port pair (2 bytes each, big-endian), only where the protocol has one.
    if ports.is_some() {
        hasher.update(ordered_src_port.to_be_bytes());
        hasher.update(ordered_dst_port.to_be_bytes());
    }

    let hash = hasher.finalize();
    Ok(format!("1:{}", BASE64.encode(hash)))
}

/// Configuration for Community ID enrichment.
pub struct CommunityIdConfig<'a> {
    pub src_ip_field: &'a str,
    pub src_port_field: &'a str,
    pub dst_ip_field: &'a str,
    pub dst_port_field: &'a str,
    pub target_field: &'a str,
    pub seed: u16,
    pub ignore_missing: bool,
}

/// Enrich an event with Community ID v1 hash.
///
/// Reads source/destination IP and port fields, computes the hash,
/// and sets the target field (typically `network.community_id`).
pub fn enrich(event: &mut Event, config: &CommunityIdConfig<'_>) -> Result<()> {
    let CommunityIdConfig {
        src_ip_field,
        src_port_field,
        dst_ip_field,
        dst_port_field,
        target_field,
        seed,
        ignore_missing,
    } = config;
    let src_ip = match event.get_str(src_ip_field) {
        Some(v) => v.to_string(),
        None if *ignore_missing => return Ok(()),
        None => {
            return Err(TransformError::FieldNotFound {
                path: (*src_ip_field).into(),
            });
        }
    };

    let dst_ip = match event.get_str(dst_ip_field) {
        Some(v) => v.to_string(),
        None if *ignore_missing => return Ok(()),
        None => {
            return Err(TransformError::FieldNotFound {
                path: (*dst_ip_field).into(),
            });
        }
    };

    // Without a transport there is nothing to hash -- Elastic's own processor
    // skips, so defaulting to tcp would invent an id for the flow.
    let transport = match event
        .get_as_string("network.transport")
        .or_else(|| event.get_as_string("network.iana_number"))
    {
        Some(v) => v,
        None if *ignore_missing => return Ok(()),
        None => {
            return Err(TransformError::FieldNotFound {
                path: "network.transport".into(),
            });
        }
    };

    // ICMP carries no ports; the spec hashes the message type and code.
    let (src_port_field, dst_port_field) = if matches!(
        transport.to_ascii_lowercase().as_str(),
        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58"
    ) {
        ("icmp.type", "icmp.code")
    } else {
        (*src_port_field, *dst_port_field)
    };

    // A port outside 0-65535 is malformed input, not a value to wrap around.
    let src_port = u16::try_from(event.get_as_i64(src_port_field).unwrap_or(0)).unwrap_or(0);
    let dst_port = u16::try_from(event.get_as_i64(dst_port_field).unwrap_or(0)).unwrap_or(0);

    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &transport, *seed) {
        Ok(cid) => {
            event.set(target_field, serde_json::json!(cid))?;
            Ok(())
        }
        Err(msg) => Err(TransformError::EnrichmentError {
            enrichment: "community_id".into(),
            message: msg,
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn tcp_basic() {
        let cid = community_id_v1("123.124.125.126", "55.56.57.58", 12345, 80, "TCP", 0).unwrap();
        // Known test vector from the community-id spec
        assert_eq!(cid, "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM=");
    }

    #[test]
    fn tcp_reversed() {
        // Reversing src/dst should give the same community ID
        let cid1 = community_id_v1("1.2.3.4", "5.6.7.8", 1000, 80, "TCP", 0).unwrap();
        let cid2 = community_id_v1("5.6.7.8", "1.2.3.4", 80, 1000, "TCP", 0).unwrap();
        assert_eq!(cid1, cid2);
    }

    #[test]
    fn udp_basic() {
        let cid = community_id_v1("1.2.3.4", "5.6.7.8", 1234, 5678, "UDP", 0).unwrap();
        assert!(cid.starts_with("1:"));
        assert!(cid.len() > 5);
    }

    #[test]
    fn iana_number_string() {
        // Protocol can be specified as IANA number string
        let cid1 = community_id_v1("1.2.3.4", "5.6.7.8", 80, 443, "TCP", 0).unwrap();
        let cid2 = community_id_v1("1.2.3.4", "5.6.7.8", 80, 443, "6", 0).unwrap();
        assert_eq!(cid1, cid2);
    }

    /// Verbatim from `tests/fixtures/cisco/ios`: a destination-unreachable
    /// (type 3) has no counterpart, so the flow is hashed one-way.
    #[test]
    fn icmp_without_a_counterpart_is_one_way() {
        let cid = community_id_v1("192.168.100.1", "192.168.100.2", 3, 4, "icmp", 0).unwrap();
        assert_eq!(cid, "1:qFmXhpjtK+/aneNSpMgRiI7dwi4=");
    }

    /// An echo and its reply are the same flow, so they hash the same.
    #[test]
    fn an_icmp_echo_and_its_reply_hash_alike() {
        let echo = community_id_v1("1.2.3.4", "5.6.7.8", 8, 0, "icmp", 0).unwrap();
        let reply = community_id_v1("5.6.7.8", "1.2.3.4", 0, 0, "icmp", 0).unwrap();
        assert_eq!(echo, reply);
    }

    /// Verbatim from `tests/fixtures/cisco/ios`. IGMP has no ports, and
    /// hashing a zero pair in their place yields a different id.
    #[test]
    fn a_protocol_without_ports_omits_the_port_bytes() {
        let cid = community_id_v1("192.168.100.197", "224.0.0.22", 0, 0, "igmp", 0).unwrap();
        assert_eq!(cid, "1:NCx7UOZoQUvxIB+uzqMmGnZTSzI=");
    }

    /// Verbatim from `testdata/compat/cisco_ios/log/test-asr920`: an ACL deny
    /// logging `81.2.69.192(0) -> 224.0.0.252(0)` over udp. Elasticsearch
    /// throws on the zero port and the pipeline's `ignore_failure` swallows
    /// it, so the event carries no community id at all.
    #[test]
    fn a_transport_flow_with_a_zero_port_gets_no_id() {
        let err = community_id_v1("81.2.69.192", "224.0.0.252", 0, 0, "udp", 0)
            .expect_err("a udp flow with no ports must not hash");
        assert!(err.contains("source port"), "{err}");
    }

    /// Its neighbour in the same fixture is VRRP, which is not a transport
    /// protocol, so it needs no ports and still hashes. The id is the one
    /// Elasticsearch 9.2.2 produced for that event.
    #[test]
    fn a_portless_protocol_still_hashes() {
        let cid = community_id_v1("89.160.20.112", "224.0.0.18", 0, 0, "112", 0).unwrap();
        assert_eq!(cid, "1:yTOnBBP4TTf0EyFmw0nUNwq2Tgo=");
    }

    #[test]
    fn invalid_ip() {
        let result = community_id_v1("not-an-ip", "5.6.7.8", 80, 443, "TCP", 0);
        assert!(result.is_err());
    }

    fn default_config() -> CommunityIdConfig<'static> {
        CommunityIdConfig {
            src_ip_field: "source.ip",
            src_port_field: "source.port",
            dst_ip_field: "destination.ip",
            dst_port_field: "destination.port",
            target_field: "network.community_id",
            seed: 0,
            ignore_missing: false,
        }
    }

    #[test]
    fn enrich_event() {
        let mut event = Event::new(serde_json::json!({
            "source": { "ip": "123.124.125.126", "port": 12345 },
            "destination": { "ip": "55.56.57.58", "port": 80 },
            "network": { "transport": "TCP" }
        }));

        enrich(&mut event, &default_config()).unwrap();

        assert_eq!(
            event.get_str("network.community_id").unwrap(),
            "1:9qr9Z1LViXcNwtLVOHZ3CL8MlyM="
        );
    }

    #[test]
    fn enrich_ignore_missing() {
        let mut event = Event::new(serde_json::json!({
            "source": { "port": 12345 },
            "destination": { "ip": "55.56.57.58", "port": 80 }
        }));

        let config = CommunityIdConfig {
            ignore_missing: true,
            ..default_config()
        };
        let result = enrich(&mut event, &config);
        assert!(result.is_ok());
        // Should not set community_id since source.ip is missing
        assert!(event.get_str("network.community_id").is_none());
    }

    #[test]
    fn enrich_missing_field_error() {
        let mut event = Event::new(serde_json::json!({
            "source": { "port": 12345 },
            "destination": { "ip": "55.56.57.58", "port": 80 }
        }));

        let result = enrich(&mut event, &default_config());
        assert!(result.is_err());
    }
}
