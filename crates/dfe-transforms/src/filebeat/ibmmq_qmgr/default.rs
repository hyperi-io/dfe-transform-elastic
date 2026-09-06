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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("ibmmq");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("ibmmq");
                if !painless_is_empty_value(&v) {
                    event.set("service.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("web")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("ibmmq");
                if !painless_is_empty_value(&v) {
                    event.set("prometheus.labels.job", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels.qmgr") {
                    event.rename("prometheus.labels.qmgr", "ibmmq.labels.qmgr")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels.job") {
                    event.rename("prometheus.labels.job", "ibmmq.labels.job")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_commit_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_commit_total",
                        "ibmmq.qmgr.messages.commit.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqcb_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqcb_total",
                        "ibmmq.qmgr.calls.failed.callback.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqclose_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqclose_total",
                        "ibmmq.qmgr.calls.failed.close.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqconn_mqconnx_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqconn_mqconnx_total",
                        "ibmmq.qmgr.calls.failed.connections.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqget_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqget_total",
                        "ibmmq.qmgr.calls.failed.get.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqinq_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqinq_total",
                        "ibmmq.qmgr.calls.failed.inquire.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqopen_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqopen_total",
                        "ibmmq.qmgr.calls.failed.open.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqset_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqset_total",
                        "ibmmq.qmgr.calls.failed.set.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqsubrq_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqsubrq_total",
                        "ibmmq.qmgr.calls.failed.subscription_request.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqcb_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqcb_total",
                        "ibmmq.qmgr.calls.succeeded.callback.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqclose_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqclose_total",
                        "ibmmq.qmgr.calls.succeeded.close.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqconn_mqconnx_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqconn_mqconnx_total",
                        "ibmmq.qmgr.calls.succeeded.connections.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqctl_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqctl_total",
                        "ibmmq.qmgr.calls.succeeded.control.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqdisc_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqdisc_total",
                        "ibmmq.qmgr.calls.succeeded.disconnect.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqinq_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqinq_total",
                        "ibmmq.qmgr.calls.succeeded.inquire.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqopen_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqopen_total",
                        "ibmmq.qmgr.calls.succeeded.open.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqset_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqset_total",
                        "ibmmq.qmgr.calls.succeeded.set.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqstat_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqstat_total",
                        "ibmmq.qmgr.calls.succeeded.status.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqsubrq_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqsubrq_total",
                        "ibmmq.qmgr.calls.succeeded.subscription_request.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_destructive_get_bytes_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_destructive_get_bytes_total",
                        "ibmmq.qmgr.destructive.get.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_destructive_get_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_destructive_get_total",
                        "ibmmq.qmgr.destructive.get.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_logical_written_bytes_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_logical_written_bytes_total",
                        "ibmmq.qmgr.log.written.bytes.logical",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_physical_written_bytes_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_physical_written_bytes_total",
                        "ibmmq.qmgr.log.written.bytes.physical",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_expired_message_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_expired_message_total",
                        "ibmmq.qmgr.messages.expired.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_browse_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_browse_total",
                        "ibmmq.qmgr.messages.failed.browse.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqput_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqput_total",
                        "ibmmq.qmgr.messages.failed.mq.put.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_mqput1_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_mqput1_total",
                        "ibmmq.qmgr.messages.failed.mq.put1.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_mqput_mqput1_bytes_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_mqput_mqput1_bytes_total",
                        "ibmmq.qmgr.messages.mq.put.bytes",
                    )?;
                }
                Ok(())
            })();

            if event.has_value("prometheus.metrics.ibmmq_qmgr_mqput_mqput1_total") {
                event.rename(
                    "prometheus.metrics.ibmmq_qmgr_mqput_mqput1_total",
                    "ibmmq.qmgr.messages.mq.put.count",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_persistent_message_browse_bytes_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_message_browse_bytes_total",
                        "ibmmq.qmgr.messages.non_persistent.browse.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_non_persistent_message_browse_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_message_browse_total",
                        "ibmmq.qmgr.messages.non_persistent.browse.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_persistent_message_destructive_get_total",
                ) {
                    event.rename("prometheus.metrics.ibmmq_qmgr_non_persistent_message_destructive_get_total", "ibmmq.qmgr.messages.non_persistent.destructive.get.count")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_persistent_message_get_bytes_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_message_get_bytes_total",
                        "ibmmq.qmgr.messages.non_persistent.get.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_non_persistent_message_mqput_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_message_mqput_total",
                        "ibmmq.qmgr.messages.non_persistent.mq.put.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_non_persistent_message_mqput1_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_message_mqput1_total",
                        "ibmmq.qmgr.messages.non_persistent.mq.put1.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_persistent_message_put_bytes_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_message_put_bytes_total",
                        "ibmmq.qmgr.messages.non_persistent.put.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_persistent_message_browse_bytes_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_browse_bytes_total",
                        "ibmmq.qmgr.messages.persistent.browse.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_persistent_message_browse_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_browse_total",
                        "ibmmq.qmgr.messages.persistent.browse.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_persistent_message_destructive_get_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_destructive_get_total",
                        "ibmmq.qmgr.messages.persistent.destructive.get.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_persistent_message_get_bytes_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_get_bytes_total",
                        "ibmmq.qmgr.messages.persistent.get.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_persistent_message_mqput_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_mqput_total",
                        "ibmmq.qmgr.messages.persistent.mq.put.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_persistent_message_mqput1_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_mqput1_total",
                        "ibmmq.qmgr.messages.persistent.mq.put1.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_persistent_message_put_bytes_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_message_put_bytes_total",
                        "ibmmq.qmgr.messages.persistent.put.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_published_to_subscribers_bytes_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_published_to_subscribers_bytes_total",
                        "ibmmq.qmgr.messages.published.subscribers.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_published_to_subscribers_message_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_published_to_subscribers_message_total",
                        "ibmmq.qmgr.messages.published.subscribers.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_purged_queue_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_purged_queue_total",
                        "ibmmq.qmgr.messages.purged.queue.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_rollback_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_rollback_total",
                        "ibmmq.qmgr.rollback.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_durable_subscription_alter_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_durable_subscription_alter_total",
                        "ibmmq.qmgr.subscription.durable.alter.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_durable_subscription_create_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_durable_subscription_create_total",
                        "ibmmq.qmgr.subscription.durable.create.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_durable_subscription_delete_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_durable_subscription_delete_total",
                        "ibmmq.qmgr.subscription.durable.delete.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_durable_subscription_resume_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_durable_subscription_resume_total",
                        "ibmmq.qmgr.subscription.durable.resume.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_failed_subscription_create_alter_resume_total",
                ) {
                    event.rename("prometheus.metrics.ibmmq_qmgr_failed_subscription_create_alter_resume_total", "ibmmq.qmgr.subscription.failed.create_alter_resume.count")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_subscription_delete_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_subscription_delete_total",
                        "ibmmq.qmgr.subscription.failed.delete.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_durable_subscription_create_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_durable_subscription_create_total",
                        "ibmmq.qmgr.subscription.non_durable.create.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_durable_subscription_delete_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_durable_subscription_delete_total",
                        "ibmmq.qmgr.subscription.non_durable.delete.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_failed_topic_mqput_mqput1_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_failed_topic_mqput_mqput1_total",
                        "ibmmq.qmgr.topic.mq.put.failed.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_topic_mqput_mqput1_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_topic_mqput_mqput1_total",
                        "ibmmq.qmgr.topic.mq.put.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_non_persistent_topic_mqput_mqput1_total",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_non_persistent_topic_mqput_mqput1_total",
                        "ibmmq.qmgr.topic.mq.put.non_persistent.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_persistent_topic_mqput_mqput1_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_persistent_topic_mqput_mqput1_total",
                        "ibmmq.qmgr.topic.mq.put.persistent.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_topic_put_bytes_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_topic_put_bytes_total",
                        "ibmmq.qmgr.topic.put.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_cpu_load_fifteen_minute_average_percentage",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_cpu_load_fifteen_minute_average_percentage",
                        "ibmmq.qmgr.cpu.load.fifteen_minute.average.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_cpu_load_five_minute_average_percentage",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_cpu_load_five_minute_average_percentage",
                        "ibmmq.qmgr.cpu.load.five_minute.average.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_cpu_load_one_minute_average_percentage",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_cpu_load_one_minute_average_percentage",
                        "ibmmq.qmgr.cpu.load.one_minute.average.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_errors_file_system_free_space_percentage",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_errors_file_system_free_space_percentage",
                        "ibmmq.qmgr.errors.file_system.free_space.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_errors_file_system_in_use_bytes")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_errors_file_system_in_use_bytes",
                        "ibmmq.qmgr.errors.file_system.in_use.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_fdc_files") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_fdc_files",
                        "ibmmq.qmgr.fdc.files",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_log_file_system_free_space_percentage",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_file_system_free_space_percentage",
                        "ibmmq.qmgr.log.file_system.free_space.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_file_system_in_use_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_file_system_in_use_bytes",
                        "ibmmq.qmgr.log.file_system.in_use.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_file_system_max_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_file_system_max_bytes",
                        "ibmmq.qmgr.log.file_system.max.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_in_use_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_in_use_bytes",
                        "ibmmq.qmgr.log.in_use.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_max_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_max_bytes",
                        "ibmmq.qmgr.log.max.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_occupied_by_extents_waiting_to_be_archived_bytes") {
                    event.rename("prometheus.metrics.ibmmq_qmgr_log_occupied_by_extents_waiting_to_be_archived_bytes", "ibmmq.qmgr.log.occupied.extents.waiting_to_be_archived.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_log_occupied_by_reusable_extents_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_occupied_by_reusable_extents_bytes",
                        "ibmmq.qmgr.log.occupied.reusable_extents.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_qmgr_log_primary_space_in_use_percentage")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_primary_space_in_use_percentage",
                        "ibmmq.qmgr.log.primary_space.in_use.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_log_required_for_media_recovery_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_required_for_media_recovery_bytes",
                        "ibmmq.qmgr.log.required_for_media_recovery.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_sequence_number_disk_total") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_sequence_number_disk_total",
                        "ibmmq.qmgr.log.sequence_number.disk",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_sequence_number_quorum_total")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_sequence_number_quorum_total",
                        "ibmmq.qmgr.log.sequence_number.quorum",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_log_slowest_write_since_restart_seconds",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_slowest_write_since_restart_seconds",
                        "ibmmq.qmgr.log.slowest_write.since_restart.seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_workload_primary_space_utilization_percentage") {
                    event.rename("prometheus.metrics.ibmmq_qmgr_log_workload_primary_space_utilization_percentage", "ibmmq.qmgr.log.workload.primary_space.utilization.percentage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_write_latency_seconds") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_write_latency_seconds",
                        "ibmmq.qmgr.log.write.latency.seconds",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_log_write_size_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_log_write_size_bytes",
                        "ibmmq.qmgr.log.write.size.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_queue_manager_file_system_free_space_percentage",
                ) {
                    event.rename("prometheus.metrics.ibmmq_qmgr_queue_manager_file_system_free_space_percentage", "ibmmq.qmgr.queue_manager.file_system.free_space.percentage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_queue_manager_file_system_in_use_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_queue_manager_file_system_in_use_bytes",
                        "ibmmq.qmgr.queue_manager.file_system.in_use.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_ram_free_percentage") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_ram_free_percentage",
                        "ibmmq.qmgr.ram.free.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_ram_usage_estimate_for_queue_manager_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_ram_usage_estimate_for_queue_manager_bytes",
                        "ibmmq.qmgr.ram.usage.estimate.queue_manager.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_system_cpu_time_estimate_for_queue_manager_percentage") {
                    event.rename("prometheus.metrics.ibmmq_qmgr_system_cpu_time_estimate_for_queue_manager_percentage", "ibmmq.qmgr.system.cpu.time.estimate.queue_manager.percentage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_system_cpu_time_percentage") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_system_cpu_time_percentage",
                        "ibmmq.qmgr.system.cpu.time.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_qmgr_trace_file_system_free_space_percentage",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_trace_file_system_free_space_percentage",
                        "ibmmq.qmgr.trace.file_system.free_space.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_trace_file_system_in_use_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_trace_file_system_in_use_bytes",
                        "ibmmq.qmgr.trace.file_system.in_use.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_user_cpu_time_estimate_for_queue_manager_percentage") {
                    event.rename("prometheus.metrics.ibmmq_qmgr_user_cpu_time_estimate_for_queue_manager_percentage", "ibmmq.qmgr.user.cpu.time.estimate.queue_manager.percentage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_qmgr_user_cpu_time_percentage") {
                    event.rename(
                        "prometheus.metrics.ibmmq_qmgr_user_cpu_time_percentage",
                        "ibmmq.qmgr.user.cpu.time.percentage",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_recovery_average_network_round_trip_time_seconds",
                ) {
                    event.rename("prometheus.metrics.ibmmq_nha_recovery_average_network_round_trip_time_seconds", "ibmmq.qmgr.nha.recovery.average.network_round_trip.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_backlog_average_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_backlog_average_bytes",
                        "ibmmq.qmgr.nha.recovery.backlog.average.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_backlog_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_backlog_bytes",
                        "ibmmq.qmgr.nha.recovery.backlog.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_nha_recovery_compressed_log_sent_bytes")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_compressed_log_sent_bytes",
                        "ibmmq.qmgr.nha.recovery.log.sent.compressed.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_log_data_average_compression_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_recovery_log_data_average_compression_time_seconds", "ibmmq.qmgr.nha.recovery.log.data.average.compression.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_log_data_average_decompression_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_recovery_log_data_average_decompression_time_seconds", "ibmmq.qmgr.nha.recovery.log.data.average.decompression.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_log_decompressed_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_log_decompressed_bytes",
                        "ibmmq.qmgr.nha.recovery.log.decompressed.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_log_sent_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_log_sent_bytes",
                        "ibmmq.qmgr.nha.recovery.log.sent.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_recovery_rebase_count") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_rebase_count",
                        "ibmmq.qmgr.nha.recovery.rebase.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_nha_recovery_recovery_log_sequence_number")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_recovery_recovery_log_sequence_number",
                        "ibmmq.qmgr.nha.recovery.log.sequence_number.recovery",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_acknowledged_log_sequence_number_total") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_acknowledged_log_sequence_number_total", "ibmmq.qmgr.nha.replication.log.sequence_number.acknowledged")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_average_network_round_trip_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_average_network_round_trip_time_seconds", "ibmmq.qmgr.nha.replication.average.network_round_trip.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_backlog_average_bytes")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_backlog_average_bytes",
                        "ibmmq.qmgr.nha.replication.backlog.average.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_backlog_bytes") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_backlog_bytes",
                        "ibmmq.qmgr.nha.replication.backlog.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_catch_up_compressed_log_sent_bytes",
                ) {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_catch_up_compressed_log_sent_bytes", "ibmmq.qmgr.nha.replication.catch_up.log.sent.compressed.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_catch_up_log_data_average_compression_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_catch_up_log_data_average_compression_time_seconds", "ibmmq.qmgr.nha.replication.catch_up.log.data.average.compression.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_catch_up_log_data_average_decompression_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_catch_up_log_data_average_decompression_time_seconds", "ibmmq.qmgr.nha.replication.catch_up.log.data.average.decompression.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_catch_up_log_decompressed_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_catch_up_log_decompressed_bytes",
                        "ibmmq.qmgr.nha.replication.catch_up.log.decompressed.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_catch_up_uncompressed_log_sent_bytes",
                ) {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_catch_up_uncompressed_log_sent_bytes", "ibmmq.qmgr.nha.replication.catch_up.log.sent.uncompressed.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("prometheus.metrics.ibmmq_nha_replication_catchup_log_sent_bytes")
                {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_catchup_log_sent_bytes",
                        "ibmmq.qmgr.nha.replication.catch_up.log.sent.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_log_file_system_free_space_percent",
                ) {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_log_file_system_free_space_percent", "ibmmq.qmgr.nha.replication.log.file_system.free_space.percentage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_log_file_system_in_use_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_log_file_system_in_use_bytes",
                        "ibmmq.qmgr.nha.replication.log.file_system.in_use.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_log_write_average_acknowledgement_latency_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_log_write_average_acknowledgement_latency_seconds", "ibmmq.qmgr.nha.replication.log.write.average.acknowledgement.latency.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_log_write_average_acknowledgement_size_bytes") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_log_write_average_acknowledgement_size_bytes", "ibmmq.qmgr.nha.replication.log.write.average.acknowledgement.size.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_mq_fdc_file_count") {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_mq_fdc_file_count",
                        "ibmmq.qmgr.nha.replication.mq.fdc.file.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_queue_manager_file_system_free_space_percent") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_queue_manager_file_system_free_space_percent", "ibmmq.qmgr.nha.replication.queue_manager.file_system.free_space.percentage")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_queue_manager_file_system_in_use_bytes") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_queue_manager_file_system_in_use_bytes", "ibmmq.qmgr.nha.replication.queue_manager.file_system.in_use.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_synchronous_compressed_log_sent_bytes") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_synchronous_compressed_log_sent_bytes", "ibmmq.qmgr.nha.replication.synchronous.log.sent.compressed.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_synchronous_log_data_average_compression_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_synchronous_log_data_average_compression_time_seconds", "ibmmq.qmgr.nha.replication.synchronous.log.data.average.compression.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_synchronous_log_data_average_decompression_time_seconds") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_synchronous_log_data_average_decompression_time_seconds", "ibmmq.qmgr.nha.replication.synchronous.log.data.average.decompression.time.seconds")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_synchronous_log_decompressed_bytes",
                ) {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_synchronous_log_decompressed_bytes", "ibmmq.qmgr.nha.replication.synchronous.log.decompressed.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "prometheus.metrics.ibmmq_nha_replication_synchronous_log_sent_bytes",
                ) {
                    event.rename(
                        "prometheus.metrics.ibmmq_nha_replication_synchronous_log_sent_bytes",
                        "ibmmq.qmgr.nha.replication.synchronous.log.sent.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.metrics.ibmmq_nha_replication_synchronous_uncompressed_log_sent_bytes") {
                    event.rename("prometheus.metrics.ibmmq_nha_replication_synchronous_uncompressed_log_sent_bytes", "ibmmq.qmgr.nha.replication.synchronous.log.sent.uncompressed.bytes")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("prometheus");
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
