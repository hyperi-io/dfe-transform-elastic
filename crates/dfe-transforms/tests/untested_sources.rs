// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! First coverage for the six sources with no parity test.
//!
//! fortinet, panw, o365, cisco_ios, cisco_meraki and cisco_nexus have
//! committed `.log` inputs but no `-expected.json` the harness can read, so
//! nothing has ever run them. This does not check ECS parity -- it checks the
//! things that are true regardless of what the expected output is:
//!
//! 1. The transform does not panic. A panic takes the pod and stalls a
//!    partition, and these are the sources with the most grok in them.
//! 2. The error count does not rise. `cisco_ios` errors on every event today;
//!    that is pinned rather than hidden.
//! 3. Something is actually extracted. A transform that returns `Continue`
//!    having done nothing is indistinguishable from a working one without
//!    this.
//!
//! Parity comes when the fixture licence question is settled and real
//! expectations can land.

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

/// Events from a committed `.log`, in whichever of the two shapes it holds.
///
/// Most fixtures are already a JSON envelope carrying the vendor payload as a
/// string in `message`, which is how filebeat delivers it. A few are bare
/// syslog lines. Wrapping an envelope again puts JSON text where the transform
/// expects a log line, so the shape is detected rather than assumed.
fn beats_events(relative: &str) -> Vec<Event> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(relative);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} is committed: {e}", path.display()));

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

fn run(transform: &dyn Transform, relative: &str) -> Outcome {
    let events = beats_events(relative);
    assert!(!events.is_empty(), "{relative}: no events parsed");

    let mut outcome = Outcome {
        events: events.len(),
        errored: 0,
        enriched: 0,
        first_error: None,
    };

    for mut event in events {
        let before = field_count(&event);
        match transform.transform(&mut event) {
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

/// Top-level keys, which is enough to tell "something happened" from "nothing
/// happened" without asserting on any particular field.
fn field_count(event: &Event) -> usize {
    event.as_value().as_object().map_or(0, serde_json::Map::len)
}

/// `(source, fixture, max_errors, min_enriched)`.
///
/// `max_errors` is today's measured count, not an aspiration. Lower it as
/// these are fixed; a rise is a regression.
macro_rules! source_case {
    ($name:ident, $transform:expr, $fixture:literal, $max_errors:expr, $min_enriched:expr) => {
        #[test]
        fn $name() {
            let outcome = run(&$transform, $fixture);
            println!(
                "{}: {} events, {} errored, {} enriched -- {}",
                stringify!($name),
                outcome.events,
                outcome.errored,
                outcome.enriched,
                outcome.first_error.as_deref().unwrap_or("no errors")
            );

            assert!(
                outcome.errored <= $max_errors,
                "{}: errors rose from {} to {} of {} events",
                stringify!($name),
                $max_errors,
                outcome.errored,
                outcome.events
            );
            // Skipped where the floor is zero, which is the case for a source
            // that errors on every event. Bound to a variable so the
            // comparison is not const-folded into a type-limit warning.
            let min_enriched: usize = $min_enriched;
            if min_enriched > 0 {
                assert!(
                    outcome.enriched >= min_enriched,
                    "{}: only {} of {} events gained a field, expected at least {min_enriched}",
                    stringify!($name),
                    outcome.enriched,
                    outcome.events,
                );
            }
        }
    };
}

source_case!(
    fortinet_default,
    dfe_transforms::filebeat::fortinet::default::Default,
    "fortinet/fortigate/test-fortinet.log",
    0,
    1
);

source_case!(
    cisco_meraki_default,
    dfe_transforms::filebeat::cisco_meraki::default::Default,
    "cisco/meraki/logs/test-events.log",
    0,
    33
);

source_case!(
    cisco_nexus_default,
    dfe_transforms::filebeat::cisco_nexus::default::Default,
    "cisco/nexus/test-nexus.log",
    0,
    1
);

// panw has no `default`; its pipeline splits by log type.
source_case!(
    panw_traffic,
    dfe_transforms::filebeat::panw::traffic::Traffic,
    "panw/panos/traffic.log",
    0,
    1
);

// Errors on every event, reading `o365audit.Parameters` unconditionally.
source_case!(
    o365_default,
    dfe_transforms::filebeat::o365::default::Default,
    "o365/audit/08-azuread.log",
    100,
    0
);

/// `cisco_ios` errors on EVERY event: a processor reads
/// `_temp_.generic_message`, which only a grok pattern this build does not
/// carry would set. Pinned at the full count so the number is stated rather
/// than rediscovered, and so a fix shows up as a test failure.
#[test]
fn cisco_ios_errors_on_every_event() {
    let outcome = run(
        &dfe_transforms::filebeat::cisco_ios::default::Default,
        "cisco/ios/test-cisco-ios.log",
    );
    println!(
        "cisco_ios: {} events, {} errored, {} enriched -- {}",
        outcome.events,
        outcome.errored,
        outcome.enriched,
        outcome.first_error.as_deref().unwrap_or("no errors")
    );

    assert_eq!(
        outcome.errored, outcome.events,
        "cisco_ios used to error on every event -- if that changed, the fix \
         landed and this test should be replaced with a real baseline"
    );
}
