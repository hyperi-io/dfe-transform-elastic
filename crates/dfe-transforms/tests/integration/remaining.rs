// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over committed fixtures: no panic, errors pinned, fields emitted.
//!
//! The parity half of these tests is RETIRED. Every committed expectation was
//! captured from an older generation of Elastic's pipelines and disagrees
//! with the current engine, so scoring against them punished correct output
//! -- `tests/compat_corpus.rs` against real Elasticsearch is the parity
//! measure, ratcheted by `tests/compat-baseline.json`. What survives here is
//! what is true regardless of expected output: the transform runs every
//! committed vendor payload without panic, without new errors, and still
//! extracts something. That floor runs on every build with no corpus on disk,
//! which the corpus test cannot.

use dfe_transforms::filebeat::{cisco_ios, cisco_meraki, cisco_nexus, fortinet, o365, panw};

floor!(
    fortinet_default,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet",
    0
);

floor!(
    fortinet_6_2,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet-6-2",
    0
);

floor!(
    fortinet_7_4,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet-7-4",
    0
);

floor!(
    cisco_ios_default,
    cisco_ios::default::Default,
    "cisco/ios",
    "test-cisco-ios",
    0
);

floor!(
    cisco_ios_syslog,
    cisco_ios::default::Default,
    "cisco/ios",
    "test-syslog",
    0
);

floor!(
    cisco_nexus_default,
    cisco_nexus::default::Default,
    "cisco/nexus",
    "test-nexus",
    0
);

floor!(
    cisco_meraki_events,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-events",
    0
);

// The sub-pipelines are INLINED into `default`, which is also where the
// syslog header is parsed -- a fixture driven straight at `flows` or `urls`
// never sees the fields the router keys on and comes out untouched.
floor!(
    cisco_meraki_flows,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-flows",
    0
);

floor!(
    cisco_meraki_urls,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-urls",
    0
);

floor!(
    o365_exchange_admin,
    o365::default::Default,
    "o365/audit",
    "01-exchange-admin",
    0
);

floor!(
    o365_sharepoint,
    o365::default::Default,
    "o365/audit",
    "04-sharepoint",
    0
);

floor!(
    o365_azuread,
    o365::default::Default,
    "o365/audit",
    "08-azuread",
    0
);

// `panw/default` is the router AND where the shared work lives -- the CSV
// parse, every `convert`, and the `_temp_` removal. A sub-pipeline driven on
// its own sees none of it.
floor!(
    panw_traffic,
    panw::default::Default,
    "panw/panos",
    "traffic",
    0
);

floor!(
    panw_threat,
    panw::default::Default,
    "panw/panos",
    "threat",
    0
);

floor!(
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
#[ignore = "reads Elastic-licensed test data kept outside this repository: set DFE_ELASTIC_FIXTURES to its fixtures/elastic directory and run with --ignored"]
fn panw_routes_and_parses_its_csv() {
    use dfe_runtime::transform::Transform;

    let dir = dfe_runtime::testutil::elastic_fixtures().join("panw/panos");
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
