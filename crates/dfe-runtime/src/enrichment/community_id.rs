// SPDX-License-Identifier: FSL-1.1-ALv2
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
        "ICMP" | "1" => Some(1),
        "TCP" | "6" => Some(6),
        "UDP" | "17" => Some(17),
        "SCTP" | "132" => Some(132),
        "ICMPV6" | "ICMP6" | "58" => Some(58),
        _ => transport.parse::<u8>().ok(),
    }
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

    // Determine ordering: lower IP first, then lower port for tie-breaking
    let (ordered_src, ordered_dst, ordered_src_port, ordered_dst_port) =
        if src < dst || (src == dst && src_port < dst_port) {
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

    // Source port (2 bytes, big-endian)
    hasher.update(ordered_src_port.to_be_bytes());

    // Destination port (2 bytes, big-endian)
    hasher.update(ordered_dst_port.to_be_bytes());

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

    // A port outside 0-65535 is malformed input, not a value to wrap around.
    let src_port = u16::try_from(event.get_i64(src_port_field).unwrap_or(0)).unwrap_or(0);
    let dst_port = u16::try_from(event.get_i64(dst_port_field).unwrap_or(0)).unwrap_or(0);

    let transport = event
        .get_str("network.transport")
        .or_else(|| event.get_str("network.iana_number"))
        .unwrap_or("tcp")
        .to_string();

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
