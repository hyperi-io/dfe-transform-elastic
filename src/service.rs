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
//!
//! All of that describes the BUS arm. The direct transport has no offsets to
//! commit -- a Push RPC acknowledges itself and scalo's gRPC `commit` is a
//! documented no-op -- so there the replay guarantee belongs to whatever
//! pushed the batch, not to this service (issue #19).

use std::time::{Duration, Instant};

use scalo::cli::ServiceRuntime;
use scalo::metrics::TransportKind;
#[cfg(feature = "grpc")]
use scalo::transport::grpc::GrpcConfig;
#[cfg(feature = "kafka")]
use scalo::transport::kafka::{KafkaConfig, total_consumer_lag};
use scalo::transport::{
    AnyReceiver, AnySender, SendResult, TransportConfig, TransportReceiver, TransportSender,
    TransportType,
};
use tokio_util::sync::CancellationToken;

use crate::config::{Config, Transport};
use crate::metrics::TransformMetrics;
use crate::pipeline::{EnvelopeOutcome, parse_batch, serialise_chunks, transform_batch_resolved};

/// The longest the loop goes without pushing scaling signals.
///
/// A batch pushes them as it finishes too, so a busy partition signals at the
/// batch cadence and this is the floor for an idle one -- KEDA still sees the
/// lag on a topic that has stopped producing.
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

/// First wait after a failed receive, doubling to [`RECV_BACKOFF_MAX`].
///
/// An unreachable broker fails every `recv` immediately, so without a wait the
/// loop spins as fast as the call returns and logs an error on every turn.
const RECV_BACKOFF_BASE: Duration = Duration::from_millis(100);

/// Ceiling on the receive wait.
///
/// The same five seconds the send side uses, and for the same reason: a wait
/// longer than that outruns `max.poll.interval` and the broker rebalances the
/// partition away while the loop is still backing off.
const RECV_BACKOFF_MAX: Duration = Duration::from_secs(5);

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
/// protocol disagree or a configured transport is not compiled in, or
/// [`crate::Error::Transport`] if either side cannot be created.
pub async fn run(config: Config, runtime: ServiceRuntime) -> crate::Result<()> {
    // The credentials are Kafka's and are read from the environment, so the
    // check runs for whichever side is on the bus and not at all when both
    // sides are direct.
    #[cfg(feature = "kafka")]
    if !config.source.transport.is_direct() || !config.sink.transport.is_direct() {
        let transport = transport_defaults();
        check_credentials(&transport)?;
        // scalo's own floor: SASL PLAIN over a plaintext transport is refused
        // in every environment, and in production so are `ssl_skip_verify` and
        // an unencrypted protocol without `allow_insecure_transport`. The dev
        // escape hatch is `APP_ENV`, which decides the profile everywhere else.
        transport
            .validate(scalo::env::is_production())
            .map_err(crate::Error::Config)?;
    }

    provision_geoip(&config.geoip).await;

    let consumer = build_receiver(&config, runtime.governor.as_ref()).await?;
    let producer = build_sender(&config).await?;

    let metrics = TransformMetrics::register(
        &runtime.metrics,
        env!("CARGO_PKG_VERSION"),
        TransformMetrics::commit(),
    );

    let scaling = runtime.scaling.as_ref().map(|pressure| ScalingSignals {
        pressure: std::sync::Arc::clone(pressure),
        memory: std::sync::Arc::clone(&runtime.memory_guard),
    });
    log_effective_scaling(runtime.scaling.as_deref());

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

/// Build the inbound transport with the governor's brake attached.
///
/// The factory attaches it, and the two arms brake DIFFERENTLY. On the bus the
/// consumer's assigned partitions are paused, so the member stays in the group,
/// no rebalance follows, and consumer lag rises. On direct the listener refuses
/// the push with `unavailable` and the sender upstream wears it.
async fn build_receiver(
    config: &Config,
    governor: Option<&scalo::SelfRegulationGovernor>,
) -> crate::Result<AnyReceiver> {
    let transport = source_transport(config)?;
    // `None` is self-regulation turned off, which leaves intake unchanged.
    let receiver = match governor {
        Some(governor) => {
            AnyReceiver::from_transport_config_with_governor(&transport, governor).await
        }
        None => AnyReceiver::from_transport_config(&transport).await,
    };
    receiver.map_err(|e| crate::Error::Transport(format!("consumer: {e}")))
}

/// Build the outbound transport, which is never governed.
///
/// Braking the outbound drain would deadlock the loop, because a batch cannot
/// be committed until it is sent. `sink.max_message_bytes` bounds each record
/// on both arms and sits well under scalo's 16 MiB gRPC message ceiling.
async fn build_sender(config: &Config) -> crate::Result<AnySender> {
    let transport = sink_transport(config)?;
    AnySender::from_transport_config(&transport)
        .await
        .map_err(|e| crate::Error::Transport(format!("producer: {e}")))
}

/// The inbound half of the transport configuration scalo's factory reads.
///
/// Our own [`consumer_config`] is carried through rather than left to the
/// cascade, so the `fetch.max.bytes` sizing still reaches librdkafka.
fn source_transport(config: &Config) -> crate::Result<TransportConfig> {
    match config.source.transport {
        #[cfg(feature = "grpc")]
        Transport::Direct => Ok(TransportConfig {
            transport_type: TransportType::Grpc,
            grpc: Some(GrpcConfig::server(&config.source.listen)),
            ..TransportConfig::default()
        }),
        #[cfg(feature = "kafka")]
        Transport::Bus => Ok(TransportConfig {
            transport_type: TransportType::Kafka,
            kafka: Some(consumer_config(config)),
            ..TransportConfig::default()
        }),
        // Unreachable in the shipped build, which carries both arms.
        #[allow(unreachable_patterns)]
        other => Err(missing_transport("source", other)),
    }
}

/// The outbound half of the same.
fn sink_transport(config: &Config) -> crate::Result<TransportConfig> {
    match config.sink.transport {
        #[cfg(feature = "grpc")]
        Transport::Direct => Ok(TransportConfig {
            transport_type: TransportType::Grpc,
            grpc: Some(GrpcConfig::client(&config.sink.endpoint)),
            ..TransportConfig::default()
        }),
        #[cfg(feature = "kafka")]
        Transport::Bus => Ok(TransportConfig {
            transport_type: TransportType::Kafka,
            kafka: Some(producer_config(config)),
            ..TransportConfig::default()
        }),
        #[allow(unreachable_patterns)]
        other => Err(missing_transport("sink", other)),
    }
}

/// A configured transport this binary carries no backend for.
///
/// Only a build that dropped a feature reaches it, so the message names the
/// feature rather than the setting.
fn missing_transport(side: &str, transport: Transport) -> crate::Error {
    let (name, feature) = match transport {
        Transport::Bus => ("bus", "kafka"),
        Transport::Direct => ("direct", "grpc"),
    };
    crate::Error::Config(format!(
        "{side}.transport is '{name}', which this binary has no transport for -- rebuild it \
         with `--features {feature}`"
    ))
}

/// The metric label for a configured transport.
///
/// Without it every direct deployment's transport counters carry the kafka
/// label and a dashboard reads them as a bus deployment.
const fn transport_kind(transport: Transport) -> TransportKind {
    match transport {
        Transport::Bus => TransportKind::Kafka,
        Transport::Direct => TransportKind::Grpc,
    }
}

/// State the scaling settings that actually took effect, once, at startup.
///
/// These come from scalo's config cascade and NOT from the `--config` file, so
/// without this line an operator has nothing to read them back from -- which is
/// how a threshold edited in the wrong place goes unnoticed. `source` names the
/// layer that supplied them, so a value left at scalo's default is visibly a
/// default rather than a choice.
fn log_effective_scaling(pressure: Option<&scalo::scaling::ScalingPressure>) {
    let config = scalo::scaling::ScalingPressureConfig::from_cascade();
    let configured = scalo::config::try_get().is_some_and(|cfg| cfg.contains("scaling"));
    let source = if configured { "cascade" } else { "default" };
    tracing::info!(
        enabled = pressure.is_some_and(scalo::scaling::ScalingPressure::is_enabled),
        memory_gate_threshold = config.memory_gate_threshold,
        source,
        "scaling pressure configured; set DFE_TRANSFORM_ELASTIC_SCALING__* to change it"
    );
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
    consumer: &AnyReceiver,
    producer: &AnySender,
    shutdown: &CancellationToken,
    metrics: &TransformMetrics,
    scaling: Option<&ScalingSignals>,
) -> crate::Result<()> {
    // The two sides are configured separately, so a deployment can consume off
    // the bus and push downstream, and the labels have to follow each.
    let source_kind = transport_kind(config.source.transport);
    let sink_kind = transport_kind(config.sink.transport);
    let transform = crate::registry::lookup(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;
    let intake = crate::registry::intake(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;
    let dataset = crate::registry::dataset(&config.source.name)
        .ok_or_else(|| crate::Error::UnknownSource(config.source.name.clone()))?;
    let resolver = crate::envelope::Resolver::new(config.source.envelope, intake, dataset);

    metrics.dfe.pipeline_ready(true);

    tracing::info!(
        source = %config.source.name,
        transform = transform.name(),
        source_transport = ?config.source.transport,
        sink_transport = ?config.sink.transport,
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
                recv_backoff = recv_backoff.saturating_mul(2).min(RECV_BACKOFF_MAX);
                continue;
            }
        };

        if batch.records.is_empty() {
            continue;
        }

        let received_bytes: usize = batch.records.iter().map(|r| r.payload.len()).sum();
        metrics
            .dfe
            .transport_received_bytes(source_kind, received_bytes as u64);
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
        metrics.dfe.transport_received_events(source_kind, received);
        metrics.app.records_received.increment(received);

        let (transformed, outcome, envelopes) =
            transform_batch_resolved(transform, &resolver, events);
        report_envelope(&envelopes, &mut last_envelope, metrics);
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
            match publish(config, producer, shutdown, metrics, sink_kind, &transformed).await {
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
async fn publish<S: TransportSender>(
    config: &Config,
    producer: &S,
    shutdown: &CancellationToken,
    metrics: &TransformMetrics,
    kind: TransportKind,
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
        match send_chunk(producer, &config.sink.topic, chunk, shutdown, metrics, kind).await {
            SendOutcome::Sent => {
                metrics.dfe.transport_sent(kind, 1);
                metrics.dfe.transport_sent_bytes(kind, chunk_bytes);
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
///
/// [`SendResult::Fatal`] is retried on the same schedule, because the transport
/// reports a leader election, an unavailable broker and a rejected record
/// through the one variant and only the first two are worth waiting out. The
/// distinction costs nothing to get wrong in one direction -- eight attempts
/// over ~25 seconds, then the batch is abandoned uncommitted and replayed --
/// and loses the batch in the other.
/// Generic over the sender rather than taking `KafkaTransport`, so the three
/// non-delivery branches are reachable in a test without a broker.
async fn send_chunk<S: TransportSender>(
    producer: &S,
    topic: &str,
    payload: Vec<u8>,
    shutdown: &CancellationToken,
    metrics: &TransformMetrics,
    kind: TransportKind,
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
                metrics.dfe.transport_send_errors(kind, 1);
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

/// How far behind this pod's assigned partitions are, where that has meaning.
///
/// `stats()` is inherent on `KafkaTransport` rather than part of any transport
/// trait, so the lag can only be read off that variant. A Push listener has no
/// consumer group to be behind, which is why the answer is `None` and not zero.
// The `Option` is not redundant across feature sets: the grpc arm below and the
// kafka-less twin both answer `None`, and clippy sees only one build at a time.
#[allow(clippy::unnecessary_wraps)]
#[cfg(feature = "kafka")]
fn consumer_lag(consumer: &AnyReceiver) -> Option<i64> {
    match consumer {
        AnyReceiver::Kafka(consumer) => Some(total_consumer_lag(&consumer.stats()).max(0)),
        // scalo gates the variant itself, so naming it unconditionally would
        // not compile without the feature.
        #[cfg(feature = "grpc")]
        AnyReceiver::Grpc(_) => None,
    }
}

/// The same, in a build with no Kafka transport to read stats from.
#[cfg(not(feature = "kafka"))]
const fn consumer_lag(_consumer: &AnyReceiver) -> Option<i64> {
    None
}

/// Push the per-pod signals `/scaling/pressure` serves to KEDA.
///
/// The component names must match those declared in
/// [`crate::cli::App::scaling_components`]; `set_component` silently ignores an
/// unregistered name.
///
/// `kafka_lag` is skipped entirely on the direct transport rather than pushed
/// as zero, and [`crate::cli::App::scaling_components`] declines to declare it
/// there for the same reason: a declared component holds at zero while still
/// taking its weight out of the composite.
#[allow(clippy::cast_precision_loss)]
fn push_scaling_signals(scaling: Option<&ScalingSignals>, consumer: &AnyReceiver, saturation: f64) {
    let Some(scaling) = scaling else {
        return;
    };
    if let Some(lag) = consumer_lag(consumer) {
        scaling.pressure.set_component("kafka_lag", lag as f64);
    }
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
#[cfg(feature = "kafka")]
fn transport_defaults() -> KafkaConfig {
    KafkaConfig::from_env(crate::config::ENV_PREFIX)
}

/// Refuse a credential the transport would not actually send.
///
/// librdkafka only presents SASL credentials when `security_protocol` names a
/// SASL mechanism. Mounting the secret and leaving the protocol at `plaintext`
/// therefore connects ANONYMOUSLY, in the clear, while the operator believes
/// the secret is in use -- so the pairing is rejected instead of guessed at.
#[cfg(feature = "kafka")]
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

/// The consumer, bounded by BYTES as well as by event count.
///
/// `recv(batch_size)` bounds the batch by count alone, and 20,000 records of
/// whatever size the producer chose is an unbounded amount of memory: the
/// outbound side has always been bounded by bytes (`sink.max_message_bytes`)
/// and the inbound side had no equivalent. `fetch.max.bytes` is where
/// librdkafka enforces it, so the ceiling applies before the bytes are in the
/// process rather than after.
#[cfg(feature = "kafka")]
fn consumer_config(config: &Config) -> KafkaConfig {
    let fetch_bytes = i32::try_from(config.source.max_batch_bytes).unwrap_or(i32::MAX);
    // One partition must not be able to fill the whole fetch on its own.
    let partition_bytes = fetch_bytes / 4;

    let defaults = transport_defaults();
    // The SIZING knobs, not only the plain fields: `KafkaTransport::new` writes
    // the plain fields first and then applies `sizing.resolved_consumer_map()`,
    // which sets `fetch.max.bytes` unconditionally and would put the 50 MiB
    // default back over the top of ours.
    let mut sizing = defaults.sizing.clone();
    sizing.consumer.fetch_max_bytes = Some(fetch_bytes);
    sizing.consumer.max_partition_fetch_bytes = Some(partition_bytes);

    KafkaConfig {
        brokers: config.source.brokers.clone(),
        group: config.source.group_id.clone(),
        client_id: "dfe-transform-elastic-consumer".to_string(),
        topics: config.source.topics.clone(),
        fetch_max_bytes: fetch_bytes,
        max_partition_fetch_bytes: partition_bytes,
        sizing,
        ..defaults
    }
}

#[cfg(feature = "kafka")]
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
/// The counters fire once per batch that saw the condition at all, which is
/// what they have always meant; the log line only on a change, because the
/// steady state is thousands of identical batches. The reported shape is the
/// batch's FIRST event -- detection now runs per event, so a batch spanning two
/// producers raises the counter even where the first event looked ordinary.
fn report_envelope(
    envelopes: &EnvelopeOutcome,
    last: &mut Option<crate::envelope::Detected>,
    metrics: &TransformMetrics,
) {
    if envelopes.contradicted {
        metrics.envelope_contradicted.increment(1);
    }
    if envelopes.unaccepted {
        metrics.envelope_unaccepted.increment(1);
    }

    let Some(resolution) = envelopes.first.as_ref() else {
        return;
    };
    let Some(detected) = resolution.detected.as_ref() else {
        return;
    };
    if last.as_ref() == Some(detected) {
        return;
    }
    *last = Some(detected.clone());

    if envelopes.contradicted {
        tracing::warn!(
            configured = ?resolution.delivery.envelope,
            detected = ?detected.family,
            variant = %detected.variant,
            "events do not look like the configured envelope; unwrapping as configured"
        );
    } else if envelopes.unaccepted {
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
    use std::future::Future;

    use super::*;
    use crate::config::{SinkConfig, SourceConfig};

    fn config() -> Config {
        Config {
            source: SourceConfig {
                name: "filebeat.okta.default".into(),
                transport: crate::config::Transport::Bus,
                listen: String::new(),
                envelope: crate::envelope::EnvelopeSetting::Auto,
                topics: vec!["in".into()],
                batch_size: 100,
                max_batch_bytes: crate::config::default_max_batch_bytes(),
                group_id: "g".into(),
                brokers: vec!["localhost:9092".into()],
            },
            sink: SinkConfig {
                topic: "out".into(),
                transport: crate::config::Transport::Bus,
                endpoint: String::new(),
                brokers: Some(vec!["other:9092".into()]),
                max_message_bytes: crate::config::default_max_message_bytes(),
            },
            geoip: scalo::geoip_download::GeoIpConfig::default(),
        }
    }

    /// The bus arm's own surface: the two Kafka config builders and the
    /// credential pairing they are checked against. A grpc-only build has no
    /// `KafkaConfig` to build, so none of this exists there.
    #[cfg(feature = "kafka")]
    mod bus {
        use super::*;

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

            let err =
                check_credentials(&kafka).expect_err("plaintext + credentials must be rejected");
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

        /// scalo's own floor, wired in beside the credential check: SASL PLAIN over
        /// a plaintext transport is refused whatever the environment.
        #[test]
        fn plain_over_a_plaintext_transport_is_refused_in_every_environment() {
            let kafka = KafkaConfig {
                security_protocol: "sasl_plaintext".into(),
                sasl_mechanism: Some("PLAIN".into()),
                sasl_username: Some("svc".into()),
                sasl_password: Some("hunter2".into()),
                ..KafkaConfig::default()
            };
            // The pairing passes our own check, which is why scalo's is also called.
            assert!(check_credentials(&kafka).is_ok());
            assert!(kafka.validate(false).is_err());
        }

        /// Verification-skipping TLS and an unencrypted protocol are dev-only.
        #[test]
        fn production_refuses_an_insecure_transport() {
            let skip_verify = KafkaConfig {
                security_protocol: "ssl".into(),
                ssl_skip_verify: true,
                ..KafkaConfig::default()
            };
            assert!(skip_verify.validate(false).is_ok());
            assert!(skip_verify.validate(true).is_err());

            let plaintext = KafkaConfig::default();
            assert!(plaintext.validate(false).is_ok());
            assert!(plaintext.validate(true).is_err());
        }

        /// The inbound byte ceiling has to reach librdkafka, or `batch_size` is
        /// still the only bound on how much memory one fetch can take.
        #[test]
        fn the_consumer_is_bounded_by_bytes_as_well_as_by_count() {
            let mut c = config();
            c.source.max_batch_bytes = 32 * 1024 * 1024;
            let kc = consumer_config(&c);

            assert_eq!(kc.fetch_max_bytes, 32 * 1024 * 1024);
            // No single partition may fill the whole fetch.
            assert_eq!(kc.max_partition_fetch_bytes, 8 * 1024 * 1024);
            // And the sizing surface, which the transport applies LAST.
            assert_eq!(kc.sizing.consumer.fetch_max_bytes, Some(32 * 1024 * 1024));
            assert_eq!(
                kc.sizing.consumer.max_partition_fetch_bytes,
                Some(8 * 1024 * 1024)
            );
            assert_eq!(
                kc.sizing.resolved_consumer_map().get("fetch.max.bytes"),
                Some(&(32 * 1024 * 1024).to_string()),
                "the sizing map is what reaches librdkafka"
            );
        }
    }

    // -- The at-least-once failure branch ---------------------------------

    /// What the sink answers, scripted, so the branches that need a broker to
    /// reach in production are reachable here.
    #[derive(Clone, Copy)]
    enum Answer {
        Backpressured,
        Fatal,
        FilteredDlq,
    }

    struct Scripted {
        answer: Answer,
        attempts: std::sync::atomic::AtomicUsize,
    }

    impl Scripted {
        fn new(answer: Answer) -> Self {
            Self {
                answer,
                attempts: std::sync::atomic::AtomicUsize::new(0),
            }
        }

        fn attempts(&self) -> usize {
            self.attempts.load(std::sync::atomic::Ordering::Relaxed)
        }
    }

    impl scalo::transport::TransportBase for Scripted {
        fn close(&self) -> impl Future<Output = scalo::transport::TransportResult<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn is_healthy(&self) -> bool {
            true
        }
        fn name(&self) -> &'static str {
            "scripted"
        }
    }

    impl TransportSender for Scripted {
        fn send(
            &self,
            _destination: &str,
            _payload: bytes::Bytes,
        ) -> impl Future<Output = SendResult> + Send {
            self.attempts
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            std::future::ready(match self.answer {
                Answer::Backpressured => SendResult::Backpressured,
                Answer::Fatal => SendResult::Fatal(scalo::transport::TransportError::Connection(
                    "broker gone".into(),
                )),
                Answer::FilteredDlq => SendResult::FilteredDlq,
            })
        }
    }

    fn test_metrics() -> TransformMetrics {
        static MANAGER: std::sync::OnceLock<scalo::metrics::MetricsManager> =
            std::sync::OnceLock::new();
        let manager =
            MANAGER.get_or_init(|| scalo::metrics::MetricsManager::new("transform_elastic_test"));
        TransformMetrics::register(manager, "0.0.0-test", "test")
    }

    /// A slow sink is the normal case, so backpressure is retried -- but only
    /// to the attempt budget. Past it the batch is abandoned UNCOMMITTED, which
    /// is what makes the replay at-least-once rather than loss.
    #[tokio::test(start_paused = true)]
    async fn backpressure_is_retried_to_the_budget_and_then_fails() {
        let sink = Scripted::new(Answer::Backpressured);
        let outcome = send_chunk(
            &sink,
            "out",
            b"{}\n".to_vec(),
            &CancellationToken::new(),
            &test_metrics(),
            TransportKind::Kafka,
        )
        .await;

        assert!(
            matches!(outcome, SendOutcome::Failed(_)),
            "must not report sent"
        );
        assert_eq!(sink.attempts(), SEND_MAX_ATTEMPTS as usize);
    }

    /// The transport reports a leader election and a rejected record through
    /// the one variant, so a fatal send is retried on the same schedule.
    #[tokio::test(start_paused = true)]
    async fn a_fatal_send_is_retried_and_then_fails() {
        let sink = Scripted::new(Answer::Fatal);
        let outcome = send_chunk(
            &sink,
            "out",
            b"{}\n".to_vec(),
            &CancellationToken::new(),
            &test_metrics(),
            TransportKind::Kafka,
        )
        .await;

        assert!(matches!(outcome, SendOutcome::Failed(_)));
        assert_eq!(sink.attempts(), SEND_MAX_ATTEMPTS as usize);
    }

    /// This service configures no outbound filter and has no DLQ, so a record
    /// routed to one is refused on the first attempt rather than retried into a
    /// livelock or counted as delivered.
    #[tokio::test]
    async fn a_record_filtered_to_a_dlq_fails_without_retrying() {
        let sink = Scripted::new(Answer::FilteredDlq);
        let outcome = send_chunk(
            &sink,
            "out",
            b"{}\n".to_vec(),
            &CancellationToken::new(),
            &test_metrics(),
            TransportKind::Kafka,
        )
        .await;

        let SendOutcome::Failed(e) = outcome else {
            panic!("a filtered record must fail the batch");
        };
        assert!(e.to_string().contains("DLQ"), "{e}");
        assert_eq!(
            sink.attempts(),
            1,
            "a deterministic filter must not be retried"
        );
    }

    /// A shutdown mid-retry is not a failure: the batch is left uncommitted and
    /// the restarted consumer replays it.
    #[tokio::test(start_paused = true)]
    async fn a_shutdown_during_a_retry_stops_rather_than_failing() {
        let sink = Scripted::new(Answer::Backpressured);
        let shutdown = CancellationToken::new();
        shutdown.cancel();

        let outcome = send_chunk(
            &sink,
            "out",
            b"{}\n".to_vec(),
            &shutdown,
            &test_metrics(),
            TransportKind::Kafka,
        )
        .await;

        assert!(matches!(outcome, SendOutcome::ShuttingDown));
        assert_eq!(sink.attempts(), 1, "the retry wait must observe the cancel");
    }

    /// A batch the sink will not take must not report as published, whatever
    /// the chunking did -- `publish` is what the loop reads to decide whether
    /// to commit.
    #[tokio::test(start_paused = true)]
    async fn publish_reports_failure_when_no_chunk_lands() {
        let sink = Scripted::new(Answer::Fatal);
        let events = crate::pipeline::parse_batch(b"{\"a\":1}\n{\"a\":2}\n").0;

        let outcome = publish(
            &config(),
            &sink,
            &CancellationToken::new(),
            &test_metrics(),
            TransportKind::Kafka,
            &events,
        )
        .await;

        assert!(matches!(outcome, SendOutcome::Failed(_)));
    }
}
