// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over the synthetic samples: no panic, errors pinned, fields emitted.
//!
//! No parity here: the samples have no expected output, so what is asserted is
//! what holds regardless of it. The transform runs every sample without panic,
//! without new errors, and still extracts something.

use dfe_transforms::filebeat::{cisco_ios, cisco_meraki, cisco_nexus, fortinet, panw};

floor!(
    fortinet_default,
    fortinet::default::Default,
    "fortinet",
    "fortigate",
    0
);

floor!(
    fortinet_6_2,
    fortinet::default::Default,
    "fortinet",
    "fortigate-6-2",
    0
);

floor!(
    fortinet_7_4,
    fortinet::default::Default,
    "fortinet",
    "fortigate-7-4",
    0
);

floor!(
    cisco_ios_default,
    cisco_ios::default::Default,
    "cisco_ios",
    "ios",
    0
);

floor!(
    cisco_ios_syslog,
    cisco_ios::default::Default,
    "cisco_ios",
    "ios-syslog-header",
    0
);

floor!(
    cisco_nexus_default,
    cisco_nexus::default::Default,
    "cisco_nexus",
    "nexus",
    0
);

floor!(
    cisco_meraki_events,
    cisco_meraki::default::Default,
    "cisco_meraki",
    "events",
    0
);

// The sub-pipelines are INLINED into `default`, which is also where the
// syslog header is parsed -- a sample driven straight at `flows` or `urls`
// never sees the fields the router keys on and comes out untouched.
floor!(
    cisco_meraki_flows,
    cisco_meraki::default::Default,
    "cisco_meraki",
    "flows",
    0
);

floor!(
    cisco_meraki_urls,
    cisco_meraki::default::Default,
    "cisco_meraki",
    "urls",
    0
);

// o365 reads the `o365audit` object its input delivers, which this harness
// would wrap into `message`, so its samples run in `untested_sources.rs`.

// `panw/default` is the router AND where the shared work lives -- the CSV
// parse, every `convert`, and the `_temp_` removal. A sub-pipeline driven on
// its own sees none of it.
floor!(panw_traffic, panw::default::Default, "panw", "traffic", 0);

floor!(panw_threat, panw::default::Default, "panw", "threat", 0);

floor!(panw_userid, panw::default::Default, "panw", "userid", 0);

/// The router picked the sub-pipeline and the CSV landed in the right columns.
/// The values are the first line of `tests/fixtures/synthetic/panw/traffic.log`,
/// in the column order of the PAN-OS traffic log field descriptions.
#[test]
fn panw_routes_and_parses_its_csv() {
    use dfe_runtime::transform::Transform;

    let dir = dfe_runtime::testutil::fixtures().join("synthetic/panw");
    let mut events = super::common::load_fixture_events(&dir, "traffic");
    let event = events.first_mut().expect("the sample has events");

    panw::default::Default
        .transform(event)
        .expect("panw transforms without error");

    assert_eq!(event.get_str("panw.panos.type"), Some("TRAFFIC"));
    assert_eq!(event.get_str("source.ip"), Some("192.0.2.70"));
    assert_eq!(event.get_str("destination.ip"), Some("198.51.100.90"));
    assert_eq!(event.get_str("rule.name"), Some("allow-outbound-web"));
    assert_eq!(event.get_i64("event.duration"), Some(45_000_000_000));

    // The columns that used to land one field to the left, putting an
    // interface name on `destination.port`.
    assert_eq!(
        event.get_str("observer.egress.interface.name"),
        Some("ethernet1/1")
    );
    assert_eq!(
        event.get_str("observer.ingress.interface.name"),
        Some("ethernet1/2")
    );
    assert_eq!(event.get_i64("destination.port"), Some(443));
}
