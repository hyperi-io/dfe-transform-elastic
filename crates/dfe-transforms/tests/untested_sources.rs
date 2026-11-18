// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over RAW-shaped fixtures, without the config wrapping.
//!
//! `tests/integration/` runs the same sources through the full harness
//! (config fields merged, envelope detected); these cases feed the bare
//! `.log` lines instead, which is the other intake shape, and panw's per-type
//! transforms have no coverage anywhere else. No ECS parity here, only the
//! things true regardless of expected output:
//!
//! 1. The transform does not panic. A panic takes the pod and stalls a
//!    partition, and these are the sources with the most grok in them.
//! 2. The error count does not rise. `cisco_ios` errors on every event today;
//!    that is pinned rather than hidden.
//! 3. Something is actually extracted. A transform that returns `Continue`
//!    having done nothing is indistinguishable from a working one without
//!    this.
//!
//! A `public` case reads a sample under `tests/fixtures/unencumbered/`, and a
//! `synthetic` case one under `tests/fixtures/synthetic/`.

use std::path::{Path, PathBuf};

use dfe_runtime::{Event, Transform, TransformResult};

/// What one source's fixture produced.
struct Outcome {
    events: usize,
    errored: usize,
    /// Events that gained at least one field beyond what they arrived with.
    enriched: usize,
    /// The first error, so a failure names its cause instead of a count.
    first_error: Option<String>,
}

/// The licence-clean samples committed in this repository.
fn public_fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/unencumbered")
}

/// Events from a `.log`, in whichever of the two shapes it holds.
///
/// Most fixtures are already a JSON envelope carrying the vendor payload as a
/// string in `message`, which is how filebeat delivers it. A few are bare
/// syslog lines. Wrapping an envelope again puts JSON text where the transform
/// expects a log line, so the shape is detected rather than assumed.
fn beats_events(path: &Path) -> Vec<Event> {
    let raw = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("fixture {} reads: {e}", path.display()));

    raw.lines()
        .filter(|line| !line.trim().is_empty())
        .map(
            |line| match serde_json::from_str::<serde_json::Value>(line) {
                Ok(value) if value.is_object() => Event::new(value),
                _ => Event::new(serde_json::json!({ "message": line })),
            },
        )
        .collect()
}

fn run(transform: &dyn Transform, path: &Path) -> Outcome {
    let events = beats_events(path);
    assert!(!events.is_empty(), "{}: no events parsed", path.display());

    let mut outcome = Outcome {
        events: events.len(),
        errored: 0,
        enriched: 0,
        first_error: None,
    };

    for mut event in events {
        let before = field_count(&event);
        match transform.transform(&mut event) {
            // An `on_failure` handler returns Ok with `error.message` set, and
            // the field it adds would otherwise count as enrichment.
            Ok(_) if event.as_value().pointer("/error/message").is_some() => {
                outcome.errored += 1;
                if outcome.first_error.is_none() {
                    outcome.first_error = Some(event.as_value()["error"]["message"].to_string());
                }
            }
            Ok(TransformResult::Continue | TransformResult::Drop) => {
                if field_count(&event) > before {
                    outcome.enriched += 1;
                }
            }
            Err(e) => {
                outcome.errored += 1;
                if outcome.first_error.is_none() {
                    outcome.first_error = Some(e.to_string());
                }
            }
        }
    }
    outcome
}

/// Every scalar in the document, however deep.
///
/// The same measure `common/mod.rs::run_floor` uses, and for its reason: a
/// transform that consumes `message` and grows one nested vendor object is a
/// wash at the top level however much it extracted.
fn field_count(event: &Event) -> usize {
    fn leaves(value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Object(map) => map.values().map(leaves).sum(),
            serde_json::Value::Array(items) => items.iter().map(leaves).sum(),
            _ => 1,
        }
    }
    leaves(event.as_value())
}

/// One source's floor: errors at or under `max_errors`, and at least
/// `min_enriched` events gaining a field.
///
/// `max_errors` is today's measured count, not an aspiration. Lower it as
/// these are fixed; a rise is a regression.
fn check(
    name: &str,
    transform: &dyn Transform,
    path: &Path,
    max_errors: usize,
    min_enriched: usize,
) {
    let outcome = run(transform, path);
    println!(
        "{name}: {} events, {} errored, {} enriched -- {}",
        outcome.events,
        outcome.errored,
        outcome.enriched,
        outcome.first_error.as_deref().unwrap_or("no errors")
    );

    assert!(
        outcome.errored <= max_errors,
        "{name}: errors rose from {max_errors} to {} of {} events",
        outcome.errored,
        outcome.events
    );
    // Skipped where the floor is zero, which is the case for a source that
    // errors on every event.
    if min_enriched > 0 {
        assert!(
            outcome.enriched >= min_enriched,
            "{name}: only {} of {} events gained a field, expected at least {min_enriched}",
            outcome.enriched,
            outcome.events,
        );
    }
}

/// `(where, source, transform, fixture, max_errors, min_enriched)`, where
/// `where` is `public` for `tests/fixtures/unencumbered/` or `synthetic` for
/// `tests/fixtures/synthetic/`.
macro_rules! source_case {
    (synthetic, $name:ident, $transform:expr, $fixture:literal, $max_errors:expr, $min_enriched:expr) => {
        #[test]
        fn $name() {
            let path = dfe_runtime::testutil::fixtures()
                .join("synthetic")
                .join($fixture);
            check(
                stringify!($name),
                &$transform,
                &path,
                $max_errors,
                $min_enriched,
            );
        }
    };
    (public, $name:ident, $transform:expr, $fixture:literal, $max_errors:expr, $min_enriched:expr) => {
        #[test]
        fn $name() {
            let path = public_fixtures().join($fixture);
            check(
                stringify!($name),
                &$transform,
                &path,
                $max_errors,
                $min_enriched,
            );
        }
    };
}

// Each floor is every event that does not fail: a lower one would pass a
// transform collapsed to a single working event.

// Raw Office 365 Management Activity records, as the API returns them and the
// filebeat o365 input nests them. Splunk Boss of the SOC v3, CC0-1.0.
source_case!(
    public,
    raw_o365_default,
    dfe_transforms::filebeat::o365::default::Default,
    "o365/botsv3-o365audit.log",
    0,
    1071
);

source_case!(
    public,
    raw_fortinet_sample,
    dfe_transforms::filebeat::fortinet::default::Default,
    "fortinet/fortigate-event.log",
    0,
    5000
);

// The Meraki events sample carries a BSD syslog header before Meraki's own
// `1 <epoch>`. dfe-receiver strips it, so that floor runs through the receiver
// envelope, in `tests/envelope.rs`.

// Six of the seven lines put a year-first timestamp straight after the
// priority, which the pipeline's last syslog pattern takes through a patch.
source_case!(
    public,
    raw_cisco_nexus_sample,
    dfe_transforms::filebeat::cisco_nexus::default::Default,
    "cisco_nexus/cisco-nxos-syslog.log",
    0,
    7
);

// A whole PAN-OS line goes through `default`, which strips the seven-field
// prefix the per-type transforms expect to be gone. One TRAFFIC record in the
// sample is cut off mid-row.
source_case!(
    public,
    raw_panw_traffic_sample,
    dfe_transforms::filebeat::panw::default::Default,
    "panw/opensoc-panos-csv.log",
    1,
    99
);

source_case!(
    public,
    raw_cisco_ios_sample,
    dfe_transforms::filebeat::cisco_ios::default::Default,
    "cisco_ios/cisco-ios-syslog.log",
    0,
    11
);

// The same records, written for this repository: one sample per record type.
source_case!(
    synthetic,
    raw_o365_exchange_admin,
    dfe_transforms::filebeat::o365::default::Default,
    "o365/exchange-admin.log",
    0,
    2
);

source_case!(
    synthetic,
    raw_o365_sharepoint,
    dfe_transforms::filebeat::o365::default::Default,
    "o365/sharepoint.log",
    0,
    2
);

source_case!(
    synthetic,
    raw_o365_azuread,
    dfe_transforms::filebeat::o365::default::Default,
    "o365/azuread.log",
    0,
    2
);
