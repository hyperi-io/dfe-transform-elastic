// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One ASA 302021 teardown line through the whole transform, with its memory
//! held to a bound.
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

/// The process's peak resident set, in MiB.
fn peak_mib() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").expect("linux /proc");
    let kib: u64 = status
        .lines()
        .find_map(|l| l.strip_prefix("VmHWM:"))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|n| n.parse().ok())
        .expect("VmHWM is reported");
    kib / 1024
}

/// Under the 768 MiB pod limit and a third of the 1,900 MiB the line took, with
/// room over the 320 MiB a dev build measures, so only a regression to per-group
/// slot growth trips it.
const PEAK_BUDGET_MIB: u64 = 640;

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
    let peak = peak_mib();
    assert!(
        peak < PEAK_BUDGET_MIB,
        "peak resident set {peak} MiB, budget {PEAK_BUDGET_MIB} MiB"
    );
}
