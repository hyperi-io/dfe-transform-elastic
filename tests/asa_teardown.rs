// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One ASA 302021 teardown line through the whole transform, with its memory
//! held to the deployment contract.
//!
//! The faddr/gaddr/laddr pattern carried 967 capture groups over 58,031 states
//! (every `%{IP}` and `%{IPV6}` brought 77 unnamed ones), and the regex engine's
//! search cache is states times slots: 1.8 GB on the first match, which killed
//! a pod limited to 768 MB. This test is its own process, so the high-water
//! mark is the transform's alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_runtime::Event;
use dfe_transform_elastic::registry;
use serde_json::json;

#[test]
fn a_302021_teardown_line_transforms_in_bounded_memory() {
    let transform = registry::lookup("filebeat.cisco_asa.default").expect("registered");
    let mut event = Event::new(json!({
        "message": "%ASA-6-302021: Teardown ICMP connection for faddr 198.51.100.102/62845 gaddr 192.0.2.24/0 laddr 192.168.50.22/0"
    }));

    transform
        .transform(&mut event)
        .expect("the line transforms");

    assert_eq!(event.get_str("event.code"), Some("302021"));
    assert_eq!(event.get_str("destination.address"), Some("198.51.100.102"));
    assert_eq!(event.get_str("source.address"), Some("192.168.50.22"));
    // Fails over the contract's memory limit, warns over its request.
    scalo::deployment::test_support::assert_peak_memory(
        "a_302021_teardown_line_transforms_in_bounded_memory",
        &dfe_transform_elastic::deployment::contract().resources,
    );
}
