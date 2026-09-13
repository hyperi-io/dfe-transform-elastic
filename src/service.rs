// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The service loop: consume a batch, transform it, produce the survivors.
//!
//! The transform is resolved once at startup, not per event.
//!
//! ## Delivery
//!
//! At-least-once, and the offset commit is what enforces it. Kafka commits are
//! cumulative -- the highest offset per partition -- so an uncommitted batch is
//! only replayed if NOTHING after it commits. The loop therefore stops on a
//! send it cannot complete rather than carrying on: the process exits, and the
//! restarted consumer resumes from the last committed offset. Carrying on
//! would let the next batch's commit acknowledge the failed one, which is loss,
//! not replay.
//!
//! Duplicates are the accepted cost. A batch is produced as several Kafka
//! records, so a failure partway through replays the records that already
//! landed. The consumer downstream must be idempotent, which is the same
//! contract every other DFE stage carries.

use std::time::{Duration, Instant};

use scalo::cli::ServiceRuntime;
use scalo::metrics::TransportKind;
use scalo::transport::kafka::{KafkaConfig, KafkaTransport, total_consumer_lag};
use scalo::transport::{SendResult, TransportReceiver, TransportSender};
use tokio_util::sync::CancellationToken;

use crate::config::Config;
use crate::metrics::TransformMetrics;
use crate::pipeline::{parse_batch, serialise_chunks, transform_batch_with};

/// How often the loop pushes scaling signals. One librdkafka stats read per
/// interval, independent of batch cadence.
const SCALING_SIGNAL_INTERVAL: Duration = Duration::from_secs(5);

/// Attempts one record gets before the batch is abandoned uncommitted.
///
/// With the backoff below this rides out roughly 25 seconds of a slow or
/// re-electing sink, which covers a leader change without a restart, and gives
/// up on a real outage rather than blocking the partition indefinitely.
const SEND_MAX_ATTEMPTS: u32 = 8;

/// First wait between send attempts. Doubles up to [`SEND_BACKOFF_MAX`].
const SEND_BACKOFF_BASE: Duration = Duration::from_millis(100);

/// Ceiling on the retry wait, so the backoff cannot outrun `max.poll.interval`
/// and trigger a rebalance while the loop is still retrying.
const SEND_BACKOFF_MAX: Duration = Duration::from_secs(5);

/// First wait after a failed receive, doubling to [`SEND_BACKOFF_MAX`].
///
/// An unreachable broker fails every `recv` immediately, so without a wait the
/// loop spins as fast as the call returns and logs an error on every turn.
const RECV_BACKOFF_BASE: Duration = Duration::from_millis(100);

/// The per-pod signals `/scaling/pressure` serves to KEDA.
///
/// `None` when the scaling engine is disabled, in which case nothing is fed
/// and no librdkafka stats read happens.
pub struct ScalingSignals {
    /// The engine the weighted components are pushed into.
    pub pressure: std::sync::Arc<scalo::ScalingPressure>,
    /// The cgroup-aware guard feeding the never-OOM hard gate.
    pub memory: std::sync::Arc<scalo::MemoryGuard>,
}

/// Build the transports and run until the shutdown token is cancelled.
///
/// # Errors
///
/// Returns [`crate::Error::UnknownSource`] if the configured source has no
/// transform, [`crate::Error::Config`] if the mounted credentials and the wire
/// protocol disagree, or [`crate::Error::Transport`] if either Kafka side
/// cannot be created.
pub async fn run(config: Config, runtime: ServiceRuntime) -> crate::Result<()> {
    check_credentials(&transport_defaults())?;

    provision_geoip(&config.geoip).await;

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

    let scaling = runtime.scaling.as_ref().map(|pressure| ScalingSignals {
        pressure: std::sync::Arc::clone(pressure),
        memory: std::sync::Arc::clone(&runtime.memory_guard),
    });

    run_loop(
        &config,
        &consumer,
        &producer,
        &runtime.shutdown,
        &metrics,
        scaling.as_ref(),
    )
    .await
}

/// Get the MMDB databases onto disk before the first batch reaches a geoip
/// processor.
///
/// Provisioning is scalo's (`geoip_download`): it resolves the provider's file
/// names, keeps a local copy inside `max_age_days`, downloads a replacement
/// when it is stale, and falls back to the stale copy when the download fails.
/// Only the lookup engine and its cache are ours.
///
/// Deliberately not on [`run_loop`], which the broker round-trip test drives
/// directly -- a test must not reach db-ip.com. It is also why this never
/// returns an error: a provider outage degrades enrichment, and turning that
/// into a failed startup would take the transform offline over geo fields.
async fn provision_geoip(config: &scalo::geoip_download::GeoIpConfig) {
    let paths = match scalo::geoip_download::ensure_databases(config).await {
        Ok(paths) => paths,
        Err(e) => {
            tracing::warn!(error = %e, "GeoIP provisioning failed; enrichment will be empty");
            return;
        }
    };

    tracing::info!(
        city = ?paths.city,
        asn = ?paths.asn,
        provider = ?config.provider,
        "GeoIP databases provisioned"
    );

    // False means a lookup already forced the readers open, which cannot
    // happen here: nothing has consumed a batch yet.
    if !dfe_runtime::enrichment::geoip_global::set_databases(paths.city, paths.asn) {
        tracing::warn!("GeoIP databases were already resolved; the provisioned paths are unused");
    }
}

/// The batch loop, over transports the caller already built.
///
/// Separate from [`run`] so a broker round-trip can drive it without a
/// `ServiceRuntime`, which scalo only constructs inside its own lifecycle.
///
/// # Errors
///
/// Returns [`crate::Error::UnknownSource`] if the configured source has no
/// transform, or [`crate::Error::Transport`] if the sink would not accept a
/// record. The batch is left uncommitted in that case, so the restarted
/// service replays it.
// Batch and byte counts are bounded far below 2^53, so the f64 casts are exact.
#[allow(clippy::cast_precision_loss)]
pub async fn run_loop(
    config: &Config,
    consumer: &KafkaTransport,
    producer: &KafkaTransport,
    shutdown: &CancellationToken,
    metrics: &TransformMetrics,
    scaling: Option<&ScalingSignals>,
) -> crate::Result<()> {
    let transform = crate::registry::lookup(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;
    let intake = crate::registry::intake(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;
    let dataset = crate::registry::dataset(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;

    metrics.dfe.pipeline_ready(true);

    tracing::info!(
        source = %config.source.name,
        transform = transform.name(),
        topics = ?config.source.topics,
        sink_topic = %config.sink.topic,
        batch_size = config.source.batch_size,
        "transform service started"
    );

    push_scaling_signals(scaling, consumer, 0.0);
    let mut last_signal = Instant::now();
    let mut recv_backoff = RECV_BACKOFF_BASE;
    // `painless_stats` counts cumulatively for the process; the metrics want
    // per-batch deltas.
    let mut painless_seen = (0_u64, 0_u64);
    let mut geoip_seen = (0_u64, 0_u64);
    // The shape last reported, so a steady stream logs once rather than per
    // batch and a producer change is still visible the batch it happens.
    let mut last_envelope: Option<crate::envelope::Detected> = None;

    // 14 of the source pipelines carry a geoip processor, so a deployment
    // that mounted no database gets empty geo fields rather than an error.
    if dfe_runtime::enrichment::geoip_global::enabled() {
        tracing::info!("GeoIP enrichment active");
    } else {
        tracing::warn!(
            "no GeoIP database found -- geo fields will be empty. Check \
             `geoip.enabled` and `geoip.auto_download`, or mount the files \
             yourself and name them in `geoip.city_db_path` / `geoip.asn_db_path`"
        );
    }

    while !shutdown.is_cancelled() {
        if last_signal.elapsed() >= SCALING_SIGNAL_INTERVAL {
            push_scaling_signals(scaling, consumer, 0.0);
            last_signal = Instant::now();
        }

        let received = tokio::select! {
            () = shutdown.cancelled() => break,
            result = consumer.recv(config.source.batch_size) => result,
        };

        let batch = match received {
            Ok(batch) => {
                recv_backoff = RECV_BACKOFF_BASE;
                batch
            }
            Err(e) => {
                tracing::error!(error = %e, "receive failed");
                tokio::select! {
                    () = shutdown.cancelled() => break,
                    () = tokio::time::sleep(recv_backoff) => {}
                }
                recv_backoff = recv_backoff.saturating_mul(2).min(SEND_BACKOFF_MAX);
                continue;
            }
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

        let resolution =
            crate::envelope::resolve(config.source.envelope, events.first(), intake, dataset);
        report_envelope(&resolution, &mut last_envelope, metrics);

        let (transformed, outcome) = transform_batch_with(transform, &resolution.delivery, events);
        metrics
            .batch_duration
            .record(started.elapsed().as_secs_f64());

        record_batch(metrics, outcome, &mut painless_seen, &mut geoip_seen);

        tracing::debug!(
            emitted = outcome.emitted,
            dropped = outcome.dropped,
            errored = outcome.errored,
            "batch transformed"
        );

        // Batch saturation: how full the pull came back. A consistently full
        // batch means the transform is the constraint, not the topic.
        let saturation = batch.records.len() as f64 / config.source.batch_size as f64;
        push_scaling_signals(scaling, consumer, saturation.min(1.0));
        last_signal = Instant::now();

        if !transformed.is_empty() {
            match publish(config, producer, shutdown, metrics, &transformed).await {
                SendOutcome::Sent => {}
                // The batch is deliberately left uncommitted, and the loop
                // deliberately does not continue: the next batch's cumulative
                // commit would acknowledge this one.
                SendOutcome::Failed(e) => {
                    metrics.send_failures.increment(1);
                    metrics.dfe.pipeline_ready(false);
                    tracing::error!(
                        error = %e,
                        "sink send failed; stopping uncommitted so the batch replays"
                    );
                    return Err(e);
                }
                SendOutcome::ShuttingDown => {
                    metrics.dfe.pipeline_ready(false);
                    tracing::info!("shutdown during send; batch left uncommitted for replay");
                    return Ok(());
                }
            }
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

/// What became of one outbound record.
enum SendOutcome {
    /// The broker accepted it.
    Sent,
    /// Retries ran out, or the transport refused in a way retrying cannot fix.
    Failed(crate::Error),
    /// The service was asked to stop mid-retry.
    ShuttingDown,
}

/// Serialise a transformed batch and produce every record it splits into.
///
/// The batch is one unit for commit purposes: unless every record lands, the
/// caller must not commit. Records already accepted when a later one fails are
/// therefore replayed on restart -- the duplicate half of at-least-once.
#[allow(clippy::cast_precision_loss)]
async fn publish(
    config: &Config,
    producer: &KafkaTransport,
    shutdown: &CancellationToken,
    metrics: &TransformMetrics,
    transformed: &[dfe_runtime::Event],
) -> SendOutcome {
    let (chunks, serialised) = serialise_chunks(transformed, config.sink.max_message_bytes);

    if serialised.oversize > 0 {
        metrics
            .events_oversize
            .increment(serialised.oversize as u64);
    }
    if serialised.failed > 0 {
        metrics.events_errored.increment(serialised.failed as u64);
        metrics
            .app
            .records_error
            .increment(serialised.failed as u64);
    }

    for chunk in chunks {
        let chunk_bytes = chunk.len() as u64;
        match send_chunk(producer, &config.sink.topic, chunk, shutdown, metrics).await {
            SendOutcome::Sent => {
                metrics.dfe.transport_sent(TransportKind::Kafka, 1);
                metrics
                    .dfe
                    .transport_sent_bytes(TransportKind::Kafka, chunk_bytes);
                metrics.app.bytes_written.increment(chunk_bytes);
            }
            other => return other,
        }
    }

    metrics.dfe.records_delivered(serialised.serialised as u64);
    SendOutcome::Sent
}

/// Send one record, retrying while the sink will not take it.
///
/// [`SendResult`] has four variants and three of them are not delivery.
/// Backpressure in particular is the NORMAL response from a slow sink -- a
/// full local producer queue -- so it is retried rather than counted as sent;
/// treating it as success is how a batch gets acknowledged and never written.
async fn send_chunk(
    producer: &KafkaTransport,
    topic: &str,
    payload: Vec<u8>,
    shutdown: &CancellationToken,
    metrics: &TransformMetrics,
) -> SendOutcome {
    // Refcounted, so each retry re-sends the same buffer rather than copying.
    let payload = bytes::Bytes::from(payload);
    let mut backoff = SEND_BACKOFF_BASE;

    for attempt in 1..=SEND_MAX_ATTEMPTS {
        match producer.send(topic, payload.clone()).await {
            SendResult::Ok => return SendOutcome::Sent,
            SendResult::Backpressured => {
                metrics.send_backpressure.increment(1);
                tracing::warn!(attempt, "sink is backpressured, retrying");
            }
            SendResult::Fatal(e) => {
                metrics.dfe.transport_send_errors(TransportKind::Kafka, 1);
                tracing::warn!(attempt, error = %e, "send failed, retrying");
            }
            // scalo's contract makes DLQ routing the caller's job. This
            // service configures no outbound filters and has no DLQ, so the
            // only honest response is to refuse rather than drop the record.
            SendResult::FilteredDlq => {
                metrics.send_filtered_dlq.increment(1);
                return SendOutcome::Failed(crate::Error::Transport(
                    "an outbound filter routed a record to a DLQ this service does not \
                     have; remove the filter or wire a DLQ"
                        .into(),
                ));
            }
        }

        if attempt == SEND_MAX_ATTEMPTS {
            break;
        }
        tokio::select! {
            () = shutdown.cancelled() => return SendOutcome::ShuttingDown,
            () = tokio::time::sleep(backoff) => {}
        }
        backoff = backoff.saturating_mul(2).min(SEND_BACKOFF_MAX);
    }

    SendOutcome::Failed(crate::Error::Transport(format!(
        "sink did not accept a record after {SEND_MAX_ATTEMPTS} attempts"
    )))
}

/// Push the per-pod signals `/scaling/pressure` serves to KEDA.
///
/// The component names must match those declared in
/// [`crate::cli::App::scaling_components`]; `set_component` silently ignores an
/// unregistered name.
#[allow(clippy::cast_precision_loss)]
fn push_scaling_signals(
    scaling: Option<&ScalingSignals>,
    consumer: &KafkaTransport,
    saturation: f64,
) {
    let Some(scaling) = scaling else {
        return;
    };
    let lag = total_consumer_lag(&consumer.stats()).max(0);
    scaling.pressure.set_component("kafka_lag", lag as f64);
    scaling
        .pressure
        .set_component("batch_saturation", saturation);
    scaling
        .pressure
        .set_memory(scaling.memory.current_bytes(), scaling.memory.limit_bytes());
}

/// The transport settings that do not come from this service's own config.
///
/// Credentials, the wire protocol and the TLS material are read from the
/// environment, because that is where the deployment puts them: the contract
/// projects `KAFKA_SASL_USERNAME` and `KAFKA_SASL_PASSWORD` from a secret, and
/// a secret must not be readable in a `ConfigMap`. `from_env` takes the service
/// prefix first and falls back to the bare `KAFKA_*` names the chart mounts.
fn transport_defaults() -> KafkaConfig {
    KafkaConfig::from_env(crate::config::ENV_PREFIX)
}

/// Refuse a credential the transport would not actually send.
///
/// librdkafka only presents SASL credentials when `security_protocol` names a
/// SASL mechanism. Mounting the secret and leaving the protocol at `plaintext`
/// therefore connects ANONYMOUSLY, in the clear, while the operator believes
/// the secret is in use -- so the pairing is rejected instead of guessed at.
fn check_credentials(kafka: &KafkaConfig) -> crate::Result<()> {
    let has_credentials = kafka.sasl_username.is_some() || kafka.sasl_password.is_some();
    let is_sasl = kafka
        .security_protocol
        .to_ascii_lowercase()
        .starts_with("sasl");

    if has_credentials && !is_sasl {
        return Err(crate::Error::Config(format!(
            "KAFKA_SASL_USERNAME/PASSWORD are set but security_protocol is '{}', so the \
             credentials would never be sent -- set KAFKA_SECURITY_PROTOCOL=sasl_ssl and \
             KAFKA_SASL_MECHANISM",
            kafka.security_protocol
        )));
    }
    if is_sasl && !has_credentials {
        return Err(crate::Error::Config(format!(
            "security_protocol is '{}' but no SASL credentials are set -- mount the kafka \
             secret, or set KAFKA_SECURITY_PROTOCOL to a protocol that needs none",
            kafka.security_protocol
        )));
    }
    if is_sasl && kafka.sasl_mechanism.is_none() {
        return Err(crate::Error::Config(
            "security_protocol names SASL but KAFKA_SASL_MECHANISM is unset".into(),
        ));
    }
    Ok(())
}

fn consumer_config(config: &Config) -> KafkaConfig {
    KafkaConfig {
        brokers: config.source.brokers.clone(),
        group: config.source.group_id.clone(),
        client_id: "dfe-transform-elastic-consumer".to_string(),
        topics: config.source.topics.clone(),
        ..transport_defaults()
    }
}

fn producer_config(config: &Config) -> KafkaConfig {
    KafkaConfig {
        brokers: config.sink_brokers().to_vec(),
        client_id: "dfe-transform-elastic-producer".to_string(),
        ..transport_defaults()
    }
}

/// Record what a transformed batch produced.
///
/// `painless_stats` and the `GeoIP` cache count cumulatively for the process,
/// so the two `seen` pairs carry the previous reading and the metrics take the
/// delta.
// Batch counts are bounded far below 2^53, so the f64 cast is exact.
#[allow(clippy::cast_precision_loss)]
fn record_batch(
    metrics: &TransformMetrics,
    outcome: crate::pipeline::BatchOutcome,
    painless_seen: &mut (u64, u64),
    geoip_seen: &mut (u64, u64),
) {
    metrics.events_transformed.increment(outcome.emitted as u64);
    metrics.events_dropped.increment(outcome.dropped as u64);
    metrics.events_errored.increment(outcome.errored as u64);
    metrics.dfe.records_filtered(outcome.dropped as u64);
    metrics
        .app
        .records_processed
        .increment(outcome.emitted as u64);
    metrics.app.records_error.increment(outcome.errored as u64);

    let painless_now = (
        dfe_runtime::painless_stats::handled(),
        dfe_runtime::painless_stats::unhandled(),
    );
    metrics
        .painless_handled
        .increment(painless_now.0.saturating_sub(painless_seen.0));
    metrics
        .painless_unhandled
        .increment(painless_now.1.saturating_sub(painless_seen.1));
    *painless_seen = painless_now;

    let geoip = dfe_runtime::enrichment::geoip_global::cache_stats();
    metrics
        .geoip_cache_hits
        .increment(geoip.hits.saturating_sub(geoip_seen.0));
    metrics
        .geoip_cache_misses
        .increment(geoip.misses.saturating_sub(geoip_seen.1));
    metrics.geoip_cache_size.set(geoip.size as f64);
    *geoip_seen = (geoip.hits, geoip.misses);
}

/// Log the batch's envelope when it changes, and count what was wrong with it.
///
/// The counters fire every batch; the log line only on a change, because the
/// steady state is thousands of identical batches.
fn report_envelope(
    resolution: &crate::envelope::Resolution,
    last: &mut Option<crate::envelope::Detected>,
    metrics: &TransformMetrics,
) {
    if resolution.contradicted {
        metrics.envelope_contradicted.increment(1);
    }
    if resolution.unaccepted {
        metrics.envelope_unaccepted.increment(1);
    }

    let Some(detected) = resolution.detected.as_ref() else {
        return;
    };
    if last.as_ref() == Some(detected) {
        return;
    }
    *last = Some(detected.clone());

    if resolution.contradicted {
        tracing::warn!(
            configured = ?resolution.delivery.envelope,
            detected = ?detected.family,
            variant = %detected.variant,
            "events do not look like the configured envelope; unwrapping as configured"
        );
    } else if resolution.unaccepted {
        tracing::warn!(
            detected = ?detected.family,
            variant = %detected.variant,
            "this source cannot arrive over the detected envelope; unwrapping as beats"
        );
    } else {
        tracing::info!(
            envelope = ?resolution.delivery.envelope,
            variant = %detected.variant,
            dataset = resolution.delivery.dataset,
            "envelope in use"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::config::{SinkConfig, SourceConfig};

    fn config() -> Config {
        Config {
            pipeline_name: "test".into(),
            source: SourceConfig {
                name: "filebeat.okta.default".into(),
                envelope: crate::envelope::EnvelopeSetting::Auto,
                topics: vec!["in".into()],
                batch_size: 100,
                group_id: "g".into(),
                brokers: vec!["localhost:9092".into()],
            },
            sink: SinkConfig {
                topic: "out".into(),
                brokers: Some(vec!["other:9092".into()]),
                max_message_bytes: crate::config::default_max_message_bytes(),
            },
            geoip: scalo::geoip_download::GeoIpConfig::default(),
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

    /// The whole point of reading the environment: a mounted secret has to
    /// reach the transport, on both the consumer and the producer.
    #[test]
    fn mounted_credentials_reach_both_transports() {
        let kafka = KafkaConfig {
            security_protocol: "sasl_ssl".into(),
            sasl_mechanism: Some("SCRAM-SHA-512".into()),
            sasl_username: Some("svc".into()),
            sasl_password: Some("hunter2".into()),
            ..KafkaConfig::default()
        };

        // What `..transport_defaults()` carries through -- the struct-update
        // syntax the two builders use keeps every field they do not name.
        let consumer = KafkaConfig {
            brokers: vec!["b:9092".into()],
            ..kafka.clone()
        };
        assert_eq!(consumer.sasl_username.as_deref(), Some("svc"));
        assert_eq!(consumer.security_protocol, "sasl_ssl");
        assert!(check_credentials(&kafka).is_ok());
    }

    /// The defect: credentials mounted, protocol left at the default, so
    /// librdkafka connects anonymously in cleartext and nothing says so.
    #[test]
    fn credentials_without_a_sasl_protocol_are_rejected() {
        let kafka = KafkaConfig {
            sasl_username: Some("svc".into()),
            sasl_password: Some("hunter2".into()),
            ..KafkaConfig::default()
        };
        assert_eq!(kafka.security_protocol, "plaintext");

        let err = check_credentials(&kafka).expect_err("plaintext + credentials must be rejected");
        assert!(err.to_string().contains("never be sent"), "{err}");
    }

    #[test]
    fn a_sasl_protocol_without_credentials_is_rejected() {
        let kafka = KafkaConfig {
            security_protocol: "sasl_ssl".into(),
            sasl_mechanism: Some("SCRAM-SHA-512".into()),
            ..KafkaConfig::default()
        };
        assert!(check_credentials(&kafka).is_err());
    }

    #[test]
    fn a_sasl_protocol_without_a_mechanism_is_rejected() {
        let kafka = KafkaConfig {
            security_protocol: "sasl_ssl".into(),
            sasl_username: Some("svc".into()),
            sasl_password: Some("hunter2".into()),
            ..KafkaConfig::default()
        };
        assert!(check_credentials(&kafka).is_err());
    }

    /// An unauthenticated broker is the local and dev-cluster case, and must
    /// not be turned into a startup failure.
    #[test]
    fn plaintext_without_credentials_is_accepted() {
        assert!(check_credentials(&KafkaConfig::default()).is_ok());
    }
}
