// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("etcd.self.leaderinfo.starttime") {
                event.rename(
                    "etcd.self.leaderinfo.starttime",
                    "etcd.self.leaderinfo.start_time",
                )?;
            }

            if event.has_value("etcd.self.recv.appendrequest") {
                event.rename(
                    "etcd.self.recv.appendrequest",
                    "etcd.self.recv.append_request",
                )?;
            }

            if event.has_value("etcd.self.recv.bandwidthrate") {
                event.rename(
                    "etcd.self.recv.bandwidthrate",
                    "etcd.self.recv.bandwidth_rate",
                )?;
            }

            if event.has_value("etcd.self.recv.pkgrate") {
                event.rename("etcd.self.recv.pkgrate", "etcd.self.recv.pkg_rate")?;
            }

            if event.has_value("etcd.self.send.appendrequest") {
                event.rename(
                    "etcd.self.send.appendrequest",
                    "etcd.self.send.append_request",
                )?;
            }

            if event.has_value("etcd.self.send.bandwidthrate") {
                event.rename(
                    "etcd.self.send.bandwidthrate",
                    "etcd.self.send.bandwidth_rate",
                )?;
            }

            if event.has_value("etcd.self.send.pkgrate") {
                event.rename("etcd.self.send.pkgrate", "etcd.self.send.pkg_rate")?;
            }

            if event.has_value("etcd.self.starttime") {
                event.rename("etcd.self.starttime", "etcd.self.start_time")?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
