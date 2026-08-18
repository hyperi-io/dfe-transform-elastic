// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The metric catalogue this service emits.
//!
//! Two layers, both from scalo:
//!
//! - [`ServiceMetrics`] -- the platform-wide `dfe_*` data-plane metrics every
//!   DFE component reports, so a dashboard can sum across components.
//! - [`AppMetrics`] plus the handful below -- namespaced
//!   `dfe_transform_elastic_*`, for the questions only this service can answer.
//!
//! Names registered here are BARE. The `MetricsManager` namespace layer
//! prepends `dfe_transform_elastic_` once at emit time, so a name written with
//! the prefix already on it would double up.
//!
//! Every metric here is emitted by [`crate::service`].
//! `ServiceApp::register_metrics` builds the same set without starting the
//! service, so `metrics-manifest` describes what actually runs.

use metrics::{Counter, Histogram};
use scalo::metrics::groups::AppMetrics;
use scalo::metrics::{MetricsManager, ServiceMetrics};

/// Every metric the transform service reports.
pub struct TransformMetrics {
    /// Platform `dfe_*` data-plane metrics shared with the rest of DFE.
    pub dfe: ServiceMetrics,

    /// Mandatory app-level group: build info, record counts, memory gauges.
    pub app: AppMetrics,

    /// Events the transform emitted.
    pub events_transformed: Counter,

    /// Events the transform deliberately dropped, per its source pipeline.
    pub events_dropped: Counter,

    /// Events whose transform returned an error. Counted, not fatal.
    pub events_errored: Counter,

    /// NDJSON lines that would not parse as JSON. The line is skipped.
    pub parse_errors: Counter,

    /// Payloads that were not valid UTF-8 and were decoded with U+FFFD
    /// replacements. A non-zero rate means an upstream encoding problem.
    pub lossy_payloads: Counter,

    /// Painless scripts the runtime recognised and ran.
    pub painless_handled: Counter,

    /// Painless scripts skipped because nothing recognised them. The event is
    /// still emitted, so a high rate is silent data loss rather than an error.
    pub painless_unhandled: Counter,

    /// Sends that failed, leaving the batch uncommitted for replay.
    pub send_failures: Counter,

    /// Offset commits that failed after a successful send.
    pub commit_failures: Counter,

    /// Events per received batch. Shows whether `batch_size` is being reached.
    pub batch_events: Histogram,

    /// Wall-clock seconds to transform one batch.
    pub batch_duration: Histogram,
}

impl TransformMetrics {
    /// Register the whole catalogue against `manager`.
    ///
    /// `version` and `commit` become labels on the `info` gauge.
    #[must_use]
    pub fn register(manager: &MetricsManager, version: &str, commit: &str) -> Self {
        Self {
            dfe: ServiceMetrics::register(manager),
            app: AppMetrics::new(manager, version, commit),
            events_transformed: manager.counter(
                "events_transformed_total",
                "Events emitted by the transform",
            ),
            events_dropped: manager.counter(
                "events_dropped_total",
                "Events the transform dropped by design",
            ),
            events_errored: manager.counter(
                "events_errored_total",
                "Events whose transform returned an error",
            ),
            parse_errors: manager.counter(
                "parse_errors_total",
                "NDJSON lines that would not parse as JSON",
            ),
            lossy_payloads: manager.counter(
                "lossy_payloads_total",
                "Payloads decoded with U+FFFD replacements for invalid UTF-8",
            ),
            painless_handled: manager.counter(
                "painless_handled_total",
                "Painless scripts the runtime recognised and ran",
            ),
            painless_unhandled: manager.counter(
                "painless_unhandled_total",
                "Painless scripts skipped because nothing recognised them",
            ),
            send_failures: manager.counter(
                "send_failures_total",
                "Sends that failed, leaving the batch uncommitted",
            ),
            commit_failures: manager.counter(
                "commit_failures_total",
                "Offset commits that failed after a successful send",
            ),
            batch_events: manager.histogram("batch_events", "Events per received batch"),
            batch_duration: manager
                .histogram("batch_duration_seconds", "Seconds to transform one batch"),
        }
    }

    /// The commit hash this binary was built from, or `unknown`.
    ///
    /// CI sets `GIT_COMMIT`; a local `cargo build` does not.
    #[must_use]
    pub fn commit() -> &'static str {
        option_env!("GIT_COMMIT").unwrap_or("unknown")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The registry stores each descriptor namespaced, so the assertions match
    /// on the bare suffix rather than the whole name.
    fn manifest_names(manager: &MetricsManager) -> Vec<String> {
        manager
            .registry()
            .manifest()
            .metrics
            .into_iter()
            .map(|m| m.name)
            .collect()
    }

    /// An empty catalogue is the exact failure `register_metrics` exists to
    /// prevent, and both layers have to be in it.
    #[test]
    fn registration_populates_the_manifest() {
        let manager = MetricsManager::new("dfe-transform-elastic");
        let _m = TransformMetrics::register(&manager, "0.1.0", "abc1234");
        let names = manifest_names(&manager);

        for expected in [
            "_events_transformed_total",
            "_events_dropped_total",
            "_events_errored_total",
            "_parse_errors_total",
            "_lossy_payloads_total",
            "_painless_handled_total",
            "_painless_unhandled_total",
            "_send_failures_total",
            "_commit_failures_total",
            "_batch_events",
            "_batch_duration_seconds",
            // AppMetrics layer.
            "_info",
            "_records_received_total",
        ] {
            assert!(
                names.iter().any(|n| n.ends_with(expected)),
                "{expected} missing from the manifest"
            );
        }
    }

    /// A metric registered twice appears twice in the manifest and doubles in
    /// any dashboard that sums the catalogue.
    ///
    /// `records_received_total` is the one known overlap: scalo's own
    /// `ServiceMetrics` and `AppMetrics` both declare it, and both layers are
    /// mandatory. Any other duplicate is ours and is a defect.
    #[test]
    fn only_the_known_scalo_overlap_is_registered_twice() {
        let manager = MetricsManager::new("dfe-transform-elastic");
        let _m = TransformMetrics::register(&manager, "0.1.0", "abc1234");

        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for name in manifest_names(&manager) {
            *counts.entry(name).or_default() += 1;
        }

        let mut duplicates: Vec<&str> = counts
            .iter()
            .filter(|(_, n)| **n > 1)
            .map(|(name, _)| name.as_str())
            .collect();
        duplicates.sort_unstable();

        assert_eq!(duplicates, ["dfe-transform-elastic_records_received_total"]);
    }

    #[test]
    fn commit_is_never_empty() {
        assert!(!TransformMetrics::commit().is_empty());
    }
}
