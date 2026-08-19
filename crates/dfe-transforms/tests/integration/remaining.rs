// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Baselines are the measured match count per fixture, and only ever go up.
//!
//! The six sources that had no parity test. Their expectations are committed
//! as a bare JSON array rather than `{"expected": [...]}`, which is the only
//! thing that had ever stopped them running.

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
    0
);

parity!(
    fortinet_6_2,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet-6-2",
    0
);

parity!(
    fortinet_7_4,
    fortinet::default::Default,
    "fortinet/fortigate",
    "test-fortinet-7-4",
    0
);

parity!(
    cisco_ios_default,
    cisco_ios::default::Default,
    "cisco/ios",
    "test-cisco-ios",
    0
);

parity!(
    cisco_ios_syslog,
    cisco_ios::default::Default,
    "cisco/ios",
    "test-syslog",
    0
);

parity!(
    cisco_nexus_default,
    cisco_nexus::default::Default,
    "cisco/nexus",
    "test-nexus",
    0
);

parity!(
    cisco_meraki_events,
    cisco_meraki::default::Default,
    "cisco/meraki/logs",
    "test-events",
    0
);

parity!(
    cisco_meraki_flows,
    cisco_meraki::flows::Flows,
    "cisco/meraki/logs",
    "test-flows",
    0
);

parity!(
    cisco_meraki_urls,
    cisco_meraki::urls::Urls,
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

parity!(
    panw_traffic,
    panw::traffic::Traffic,
    "panw/panos",
    "traffic",
    0
);

parity!(
    panw_threat,
    panw::traffic::Traffic,
    "panw/panos",
    "threat",
    0
);

parity!(panw_userid, panw::userid::Userid, "panw/panos", "userid", 0);
