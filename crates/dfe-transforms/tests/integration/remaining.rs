// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Baselines are the measured match count per fixture, and only ever go up.
//!
//! The six sources that had no parity test. Their expectations are committed
//! as a bare JSON array rather than `{"expected": [...]}`, which is the only
//! thing that had ever stopped them running.
//!
//! **o365 and panw sit at zero for a reason that is not the transform.** Their
//! `.log` files are the OUTPUT of an older pipeline generation, and the
//! expectation beside each one is that same generation's output. o365's
//! expectations are keyed `o365.audit.*` where `pipelines/o365/default.yml`
//! reads `o365audit.*` -- not one field can line up. panw's carry `client.*`
//! and `panw.panos.destination.nat.*`, which the current pipeline does not set
//! and explicitly removes. Raising these needs new fixtures generated against
//! the pipelines in `pipelines/`, not changes to the transforms.

use dfe_transforms::filebeat::{cisco_ios, cisco_meraki, cisco_nexus, fortinet, o365, panw};

const FIXTURE_BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures");

macro_rules! parity {
    ($name:ident, $transform:expr, $dir:literal, $fixture:literal, $baseline:expr) => {
        #[test]
        fn $name() {
            let dir = format!("{FIXTURE_BASE}/{}", $dir);
            super::common::run_fixture(&$transform, &dir, $fixture, $baseline);
        }
    };
}

parity!(
    fortinet_default,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet",
    54
);

parity!(
    fortinet_6_2,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet-6-2",
    56
);

parity!(
    fortinet_7_4,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet-7-4",
    72
);

parity!(
    cisco_ios_default,
    cisco_ios::default::Default,
    "cisco/ios",
    "test-cisco-ios",
    26
);

parity!(
    cisco_ios_syslog,
    cisco_ios::default::Default,
    "cisco/ios",
    "test-syslog",
    12
);

parity!(
    cisco_nexus_default,
    cisco_nexus::default::Default,
    "cisco/nexus",
    "test-nexus",
    68
);

parity!(
    cisco_meraki_events,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-events",
    32
);

// The sub-pipelines are INLINED into `default`, which is also where the
// syslog header is parsed -- a fixture driven straight at `flows` or `urls`
// never sees the fields the router keys on and comes out untouched.
parity!(
    cisco_meraki_flows,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-flows",
    16
);

parity!(
    cisco_meraki_urls,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-urls",
    0
);

parity!(
    o365_exchange_admin,
    o365::default::Default,
    "o365/audit",
    "01-exchange-admin",
    0
);

parity!(
    o365_sharepoint,
    o365::default::Default,
    "o365/audit",
    "04-sharepoint",
    0
);

parity!(
    o365_azuread,
    o365::default::Default,
    "o365/audit",
    "08-azuread",
    0
);

// `panw/default` is the router AND where the shared work lives -- the CSV
// parse, every `convert`, and the `_temp_` removal. A sub-pipeline driven on
// its own sees none of it.
parity!(
    panw_traffic,
    panw::default::Default,
    "panw/panos",
    "traffic",
    0
);

parity!(
    panw_threat,
    panw::default::Default,
    "panw/panos",
    "threat",
    0
);

parity!(
    panw_userid,
    panw::default::Default,
    "panw/panos",
    "userid",
    0
);

/// panw scores zero against a fixture from an older pipeline generation, so
/// this asserts the fields that generation agrees on -- the router picked the
/// sub-pipeline, and the CSV landed in the right columns.
#[test]
fn panw_routes_and_parses_its_csv() {
    use dfe_runtime::transform::Transform;

    let dir = format!("{FIXTURE_BASE}/panw/panos");
    let mut events = super::common::load_fixture_events(&dir, "traffic");
    let event = events.first_mut().expect("the fixture has events");

    panw::default::Default
        .transform(event)
        .expect("panw transforms without error");

    assert_eq!(event.get_str("panw.panos.type"), Some("TRAFFIC"));
    assert_eq!(event.get_str("source.ip"), Some("192.168.15.207"));
    assert_eq!(event.get_str("destination.ip"), Some("184.51.253.152"));
    assert_eq!(event.get_str("rule.name"), Some("new_outbound_from_trust"));
    assert_eq!(event.get_i64("event.duration"), Some(586_000_000_000));

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
