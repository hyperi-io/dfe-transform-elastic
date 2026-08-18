// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The service loop: consume a batch, transform it, produce the survivors.
//!
//! The transform is resolved once at startup, not per event. The loop commits
//! only after a successful send, so a crash between transform and send replays
//! rather than loses.

use scalo::cli::ServiceRuntime;
use scalo::transport::kafka::{KafkaConfig, KafkaProfile, KafkaTransport};
use scalo::transport::{TransportReceiver, TransportSender};

use crate::config::Config;
use crate::pipeline::{parse_batch, serialise_batch, transform_batch};

/// Run until the shutdown token is cancelled.
///
/// # Errors
///
/// Returns [`crate::Error::UnknownSource`] if the configured source has no
/// transform, or [`crate::Error::Transport`] if either Kafka side cannot be
/// created.
pub async fn run(config: Config, runtime: ServiceRuntime) -> crate::Result<()> {
    let transform = crate::registry::lookup(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;

    let consumer = KafkaTransport::new(&consumer_config(&config))
        .await
        .map_err(|e| crate::Error::Transport(format!("consumer: {e}")))?;

    let producer = KafkaTransport::new(&producer_config(&config))
        .await
        .map_err(|e| crate::Error::Transport(format!("producer: {e}")))?;

    tracing::info!(
        source = %config.source.name,
        transform = transform.name(),
        topics = ?config.source.topics,
        sink_topic = %config.sink.topic,
        batch_size = config.source.batch_size,
        "transform service started"
    );

    while !runtime.shutdown.is_cancelled() {
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

        let mut events = Vec::with_capacity(batch.records.len());
        for record in &batch.records {
            match parse_batch(&record.payload) {
                Ok(parsed) => events.extend(parsed),
                Err(e) => tracing::warn!(error = %e, "payload parse failed, record skipped"),
            }
        }

        let (transformed, outcome) = transform_batch(transform, events);

        tracing::debug!(
            emitted = outcome.emitted,
            dropped = outcome.dropped,
            errored = outcome.errored,
            "batch transformed"
        );

        if !transformed.is_empty() {
            let payload = serialise_batch(&transformed)?;
            if let scalo::transport::SendResult::Fatal(e) = producer
                .send(&config.sink.topic, bytes::Bytes::from(payload))
                .await
            {
                tracing::error!(error = %e, "send failed, batch not committed");
                continue;
            }
        }

        if let Err(e) = consumer.commit(&batch.commit_tokens).await {
            tracing::warn!(error = %e, "offset commit failed");
        }
    }

    tracing::info!("transform service stopped");
    Ok(())
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
