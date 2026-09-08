// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! An optional grok capture that matched nothing must write nothing.

use dfe_runtime::Event;

const PFSENSE_FILTERLOG: &str = "(?:(?:%{INT},%{INT}?,,%{DATA:rule.id},%{DATA:observer.ingress.interface.name},(?P<event_reason>(?:[a-zA-Z-]+)),%{WORD:event.action},%{WORD:network.direction},)(?:(?:(?P<network_type>(4)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.ecn}?,%{NONNEGINT:pfsense.ip.ttl:long},%{NONNEGINT:pfsense.ip.id:long},%{NONNEGINT:pfsense.ip.offset:long},(?:%{WORD:pfsense.ip.flags}|(?P<pfsense_ip_flags>(?:[+]))),%{INT:network.iana_number},%{WORD:network.transport},)|(?:(?P<network_type>(6)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.flow_label},%{WORD:pfsense.ip.flags},(?P<network_transport>(?:[0-9a-zA-Z-]+)),%{INT:network.iana_number},))(?:%{NONNEGINT:network.bytes:long},%{IP:source.address},%{IP:destination.address},)(?:(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.tcp.length:long},%{WORD:pfsense.tcp.flags}?,%{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT},%{NONNEGINT:pfsense.tcp.ack:long}?,%{NONNEGINT:pfsense.tcp.window:long}?,%{WORD:pfsense.tcp.urg}?,%{GREEDYDATA:pfsense.tcp.options})|(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.udp.length:long}$)|(?:(?:(?P<pfsense_icmp_type>(request|reply|unreachproto|unreachport|unreach|timeexceed|paramprob|redirect|maskreply|needfrag|tstamp|tstampreply)),)(?:(?:%{NONNEGINT:pfsense.icmp.id:long},%{NONNEGINT:pfsense.icmp.seq:long})|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?,\\[?%{NONNEGINT:pfsense.icmp.unreachable.port:long}\\]?)|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?)|(?:%{GREEDYDATA:pfsense.icmp.unreachable.other})|(?:%{IP:pfsense.icmp.destination.ip},%{NONNEGINT:pfsense.icmp.mtu:long})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq},%{INT:pfsense.icmp.otime},%{INT:pfsense.icmp.rtime},%{INT:pfsense.icmp.ttime})))|(?:datalength=%{NONNEGINT:network.packets:long})|(?:%{GREEDYDATA})|(?:))?)%{GREEDYDATA}";

const PFSENSE_MAP: &[(&str, &str)] = &[
    ("event_reason", "event.reason"),
    ("pfsense_ip_flags", "pfsense.ip.flags"),
    ("network_transport", "network.transport"),
    ("network_type", "network.type"),
    ("pfsense_icmp_type", "pfsense.icmp.type"),
];

/// A real filterlog line: `ecn`, `ack` and `urg` sit on empty columns, and
/// `seq` is a bare number where the pattern wants a `seq:seq_end` range.
const TCP_LINE: &str = "146,,,1535324496,igb1.12,match,block,in,4,0x0,,63,12617,0,DF,6,tcp,60,10.170.12.50,175.16.199.1,49724,853,0,S,1891286705,,64240,,mss;sackOK;TS;nop;wscale";

#[test]
fn probe() {
    let compiled = dfe_runtime::grok_cache::grok_mapped(PFSENSE_FILTERLOG, PFSENSE_MAP);
    let mut event = Event::new(serde_json::json!({}));
    let matched = compiled.extract_into(TCP_LINE, &mut event).unwrap();
    println!("matched: {matched}");
    println!(
        "{}",
        serde_json::to_string_pretty(event.as_value()).unwrap()
    );
}
