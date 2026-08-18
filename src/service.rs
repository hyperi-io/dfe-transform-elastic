// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The service loop: consume a batch, transform it, produce the survivors.
//!
//! The transform is resolved once at startup, not per event. The loop commits
//! only after a successful send, so a crash between transform and send replays
//! rather than loses.

use std::time::{Duration, Instant};

use scalo::cli::ServiceRuntime;
use scalo::metrics::TransportKind;
use scalo::transport::kafka::{KafkaConfig, KafkaProfile, KafkaTransport, total_consumer_lag};
use scalo::transport::{TransportReceiver, TransportSender};

use crate::config::Config;
use crate::metrics::TransformMetrics;
use crate::pipeline::{parse_batch, serialise_batch, transform_batch};

/// How often the loop pushes scaling signals. One librdkafka stats read per
/// interval, independent of batch cadence.
const SCALING_SIGNAL_INTERVAL: Duration = Duration::from_secs(5);

/// Run until the shutdown token is cancelled.
///
/// # Errors
///
/// Returns [`crate::Error::UnknownSource`] if the configured source has no
/// transform, or [`crate::Error::Transport`] if either Kafka side cannot be
/// created.
// Batch and byte counts are bounded far below 2^53, so the f64 casts are exact.
#[allow(clippy::cast_precision_loss)]
pub async fn run(config: Config, runtime: ServiceRuntime) -> crate::Result<()> {
    let transform = crate::registry::lookup(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;

    let consumer = KafkaTransport::new(&consumer_config(&config))
        .await
        .map_err(|e| crate::Error::Transport(format!("consumer: {e}")))?;

    let producer = KafkaTransport::new(&producer_config(&config))
        .await
        .map_err(|e| crate::Error::Transport(format!("producer: {e}")))?;

    let metrics = TransformMetrics::register(
        &runtime.metrics,
        env!("CARGO_PKG_VERSION"),
        TransformMetrics::commit(),
    );
    metrics.dfe.pipeline_ready(true);

    tracing::info!(
        source = %config.source.name,
        transform = transform.name(),
        topics = ?config.source.topics,
        sink_topic = %config.sink.topic,
        batch_size = config.source.batch_size,
        "transform service started"
    );

    push_scaling_signals(&runtime, &consumer, 0.0);
    let mut last_signal = Instant::now();

    while !runtime.shutdown.is_cancelled() {
        if last_signal.elapsed() >= SCALING_SIGNAL_INTERVAL {
            push_scaling_signals(&runtime, &consumer, 0.0);
            last_signal = Instant::now();
        }

        let batch = tokio::select! {
            () = runtime.shutdown.cancelled() => break,
            result = consumer.recv(config.source.batch_size) => match result {
                Ok(batch) => batch,
                Err(e) => {
                    tracing::error!(error = %e, "receive failed");
                    continue;
                }
            },
        };

        if batch.records.is_empty() {
            continue;
        }

        let received_bytes: usize = batch.records.iter().map(|r| r.payload.len()).sum();
        metrics
            .dfe
            .transport_received_bytes(TransportKind::Kafka, received_bytes as u64);
        metrics.app.bytes_received.increment(received_bytes as u64);
        metrics.batch_events.record(batch.records.len() as f64);

        let started = Instant::now();
        let mut events = Vec::with_capacity(batch.records.len());
        for record in &batch.records {
            let (parsed, parse_outcome) = parse_batch(&record.payload);
            if parse_outcome.bad_lines > 0 {
                metrics
                    .parse_errors
                    .increment(parse_outcome.bad_lines as u64);
                metrics
                    .app
                    .records_error
                    .increment(parse_outcome.bad_lines as u64);
            }
            if parse_outcome.lossy {
                metrics.lossy_payloads.increment(1);
            }
            events.extend(parsed);
        }

        let received = events.len() as u64;
        metrics.dfe.records_received(received);
        metrics
            .dfe
            .transport_received_events(TransportKind::Kafka, received);
        metrics.app.records_received.increment(received);

        let (transformed, outcome) = transform_batch(transform, events);
        metrics
            .batch_duration
            .record(started.elapsed().as_secs_f64());

        metrics.events_transformed.increment(outcome.emitted as u64);
        metrics.events_dropped.increment(outcome.dropped as u64);
        metrics.events_errored.increment(outcome.errored as u64);
        metrics.dfe.records_filtered(outcome.dropped as u64);
        metrics
            .app
            .records_processed
            .increment(outcome.emitted as u64);
        metrics.app.records_error.increment(outcome.errored as u64);

        tracing::debug!(
            emitted = outcome.emitted,
            dropped = outcome.dropped,
            errored = outcome.errored,
            "batch transformed"
        );

        // Batch saturation: how full the pull came back. A consistently full
        // batch means the transform is the constraint, not the topic.
        let saturation = batch.records.len() as f64 / config.source.batch_size as f64;
        push_scaling_signals(&runtime, &consumer, saturation.min(1.0));
        last_signal = Instant::now();

        if !transformed.is_empty() {
            let payload = serialise_batch(&transformed)?;
            let sent_bytes = payload.len() as u64;
            if let scalo::transport::SendResult::Fatal(e) = producer
                .send(&config.sink.topic, bytes::Bytes::from(payload))
                .await
            {
                metrics.send_failures.increment(1);
                metrics.dfe.transport_send_errors(TransportKind::Kafka, 1);
                tracing::error!(error = %e, "send failed, batch not committed");
                continue;
            }
            metrics.dfe.transport_sent(TransportKind::Kafka, 1);
            metrics
                .dfe
                .transport_sent_bytes(TransportKind::Kafka, sent_bytes);
            metrics.dfe.records_delivered(outcome.emitted as u64);
            metrics.app.bytes_written.increment(sent_bytes);
        }

        if let Err(e) = consumer.commit(&batch.commit_tokens).await {
            metrics.commit_failures.increment(1);
            tracing::warn!(error = %e, "offset commit failed");
        }
    }

    metrics.dfe.pipeline_ready(false);
    tracing::info!("transform service stopped");
    Ok(())
}

/// Push the per-pod signals `/scaling/pressure` serves to KEDA.
///
/// The component names must match those declared in
/// [`crate::cli::App::scaling_components`]; `set_component` silently ignores an
/// unregistered name.
#[allow(clippy::cast_precision_loss)]
fn push_scaling_signals(runtime: &ServiceRuntime, consumer: &KafkaTransport, saturation: f64) {
    let Some(scaling) = runtime.scaling.as_ref() else {
        return;
    };
    let lag = total_consumer_lag(&consumer.stats()).max(0);
    scaling.set_component("kafka_lag", lag as f64);
    scaling.set_component("batch_saturation", saturation);
    scaling.set_memory(
        runtime.memory_guard.current_bytes(),
        runtime.memory_guard.limit_bytes(),
    );
}

fn consumer_config(config: &Config) -> KafkaConfig {
    KafkaConfig {
        profile: KafkaProfile::Production,
        brokers: config.source.brokers.clone(),
        group: config.source.group_id.clone(),
        client_id: "dfe-transform-elastic-consumer".to_string(),
        topics: config.source.topics.clone(),
        ..KafkaConfig::production()
    }
}

fn producer_config(config: &Config) -> KafkaConfig {
    KafkaConfig {
        profile: KafkaProfile::Production,
        brokers: config.sink_brokers().to_vec(),
        client_id: "dfe-transform-elastic-producer".to_string(),
        ..KafkaConfig::production()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{SinkConfig, SourceConfig};

    fn config() -> Config {
        Config {
            pipeline_name: "test".into(),
            source: SourceConfig {
                name: "filebeat.okta.default".into(),
                topics: vec!["in".into()],
                batch_size: 100,
                group_id: "g".into(),
                brokers: vec!["localhost:9092".into()],
            },
            sink: SinkConfig {
                topic: "out".into(),
                brokers: Some(vec!["other:9092".into()]),
            },
        }
    }

    #[test]
    fn consumer_subscribes_to_the_configured_topics() {
        let kc = consumer_config(&config());
        assert_eq!(kc.topics, vec!["in".to_string()]);
        assert_eq!(kc.group, "g");
        assert_eq!(kc.brokers, ["localhost:9092".to_string()]);
    }

    #[test]
    fn producer_uses_the_sink_brokers_when_set() {
        assert_eq!(
            producer_config(&config()).brokers,
            ["other:9092".to_string()]
        );
    }

    #[test]
    fn producer_falls_back_to_source_brokers() {
        let mut c = config();
        c.sink.brokers = None;
        assert_eq!(producer_config(&c).brokers, ["localhost:9092".to_string()]);
    }

    #[test]
    fn consumer_and_producer_have_distinct_client_ids() {
        let c = config();
        assert_ne!(consumer_config(&c).client_id, producer_config(&c).client_id);
    }
}
