// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The service loop: receive a block, transform it, produce the survivors.
//!
//! The transform is resolved once at startup, not per event.
//!
//! ## Delivery
//!
//! At-least-once, held by scalo's `BatchEngine` pipeline. Each block's source
//! acknowledgement -- a Kafka offset commit on the bus, the answer to a push on
//! direct -- is released only once the sink has taken every event built from
//! it. A sink that refuses for now (a broker outage, a leader election, a full
//! producer queue) is retried by the engine for as long as it refuses, so an
//! outage is waited out rather than crashed on. Only a refusal no retry can fix
//! stops the loop, with the block unreleased so a restart reads it again.
//!
//! Duplicates are the accepted cost. A block goes out as several Kafka records
//! or RPCs, and a retry re-sends the ones that already landed. The consumer
//! downstream must be idempotent, which is the contract every DFE stage carries.
//!
//! An event that cannot be delivered is dropped and counted, never counted
//! delivered: one that will not parse, transform or serialise, one over
//! `sink.max_message_bytes`, and one the sink transport would refuse, which the
//! pipeline screens out before the send and counts in
//! `pipeline_dead_letters_dropped_total`.

use std::sync::Arc;
use std::time::{Duration, Instant};

use scalo::cli::ServiceRuntime;
#[cfg(feature = "grpc")]
use scalo::transport::grpc::{GrpcConfig, GrpcTransport};
#[cfg(feature = "kafka")]
use scalo::transport::kafka::{KafkaConfig, total_consumer_lag};
use scalo::transport::{
    AcknowledgementsConfig, AnyReceiver, AnySender, AnyToken, DeliveryStatus, PayloadFormat,
    PieceFinalizer, Record, RecordMeta, SendResult, TransportConfig, TransportError,
    TransportSender, TransportType, WorkBatch,
};
use scalo::worker::engine::BlockPieces;
use scalo::worker::{BatchEngine, BatchProcessingConfig, EngineError};
use tokio_util::sync::CancellationToken;

use crate::config::{Config, Transport};
use crate::metrics::TransformMetrics;
use crate::pipeline::{
    EnvelopeOutcome, message_budget, parse_batch, serialise_events, transform_batch_resolved,
};

/// The longest the loop goes without pushing scaling signals.
///
/// A block pushes them as it is transformed too, so a busy partition signals at
/// the block cadence and this is the floor for an idle one -- KEDA still sees
/// the lag on a topic that has stopped producing.
const SCALING_SIGNAL_INTERVAL: Duration = Duration::from_secs(5);

/// Longest a push is held for delivery: inside the stage in front's 20 s send
/// deadline, so it is answered before that sender gives up.
pub const PUSH_MAX_HOLD: Duration = Duration::from_secs(18);

/// Send deadline to the next hop -- a push listener, or a Kafka delivery report
/// while a push is held: inside [`PUSH_MAX_HOLD`], so a slow hop is retried
/// before the hold runs out.
pub const NEXT_HOP_SEND_TIMEOUT_MS: u64 = 15_000;

/// The librdkafka producer property [`NEXT_HOP_SEND_TIMEOUT_MS`] caps.
#[cfg(feature = "kafka")]
const MESSAGE_TIMEOUT: &str = "message.timeout.ms";

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
/// [`crate::Error::Transport`] if either side cannot be created or fails in a
/// way no retry can fix.
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

    let engine = batch_engine(&config, &runtime);
    let consumer =
        build_receiver(&config, runtime.governor.as_ref(), &runtime.memory_guard).await?;
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
        &engine,
        &consumer,
        &producer,
        &runtime.shutdown,
        &metrics,
        scaling.as_ref(),
    )
    .await
}

/// The engine the loop runs on, wired to the runtime the way scalo wires its
/// own: the memory guard leases each block's bytes, and the governor's byte
/// budget makes every receive a byte-bounded `recv_limited`.
///
/// Built here rather than taken from the runtime because the runtime's engine
/// reads its record cap from the cascade, which a `--config` file is not a
/// layer of, and this one takes it from `source.batch_size`.
fn batch_engine(config: &Config, runtime: &ServiceRuntime) -> BatchEngine {
    let settings = engine_config(config);
    let mut engine = match runtime.worker_pool.as_ref() {
        Some(pool) => BatchEngine::with_pool(Arc::clone(pool), settings),
        None => BatchEngine::new(settings),
    };
    engine.auto_wire(&runtime.metrics, Some(&runtime.memory_guard));
    if let Some(governor) = runtime.governor.as_ref() {
        engine.set_byte_budget(governor.budget());
    }
    engine
}

/// The engine settings this service decides: at most `source.batch_size`
/// records per receive.
#[must_use]
pub fn engine_config(config: &Config) -> BatchProcessingConfig {
    BatchProcessingConfig {
        max_chunk_size: config.source.batch_size,
        ..BatchProcessingConfig::default()
    }
}

/// Build the inbound transport with the governor's brake attached and
/// `source.acknowledgements` applied.
///
/// The factory attaches the brake, and the two arms brake DIFFERENTLY. On the
/// bus the consumer's assigned partitions are paused, so the member stays in
/// the group, no rebalance follows, and consumer lag rises. On direct the
/// listener refuses the push with `unavailable` and the sender upstream wears
/// it.
async fn build_receiver(
    config: &Config,
    governor: Option<&scalo::SelfRegulationGovernor>,
    #[cfg_attr(not(feature = "grpc"), allow(unused_variables))] memory_guard: &Arc<
        scalo::MemoryGuard,
    >,
) -> crate::Result<AnyReceiver> {
    let transport = source_transport(config)?;
    let acknowledgements = config.source.acknowledgements;
    let consumer_error = |e: TransportError| crate::Error::Transport(format!("consumer: {e}"));

    #[cfg(feature = "grpc")]
    if let Some(grpc) = transport.grpc.as_ref() {
        return armed_listener(grpc, acknowledgements, governor, memory_guard)
            .await
            .map(AnyReceiver::Grpc)
            .map_err(consumer_error);
    }

    // `None` is self-regulation turned off, which leaves intake unchanged.
    let receiver = match governor {
        Some(governor) => {
            AnyReceiver::from_transport_config_with_governor(&transport, governor).await
        }
        None => AnyReceiver::from_transport_config(&transport).await,
    };
    receiver
        .map(|receiver| with_acknowledgements(receiver, acknowledgements))
        .map_err(consumer_error)
}

/// The direct listener, armed before it binds.
///
/// Unarmed, a push that lands before the loop starts is answered at enqueue,
/// with nothing yet able to deliver it, so the listener is armed at
/// construction rather than by the loop. A push is held at most
/// [`PUSH_MAX_HOLD`], and the bytes it holds are leased on the memory guard.
#[cfg(feature = "grpc")]
async fn armed_listener(
    grpc: &GrpcConfig,
    acknowledgements: AcknowledgementsConfig,
    governor: Option<&scalo::SelfRegulationGovernor>,
    memory_guard: &Arc<scalo::MemoryGuard>,
) -> scalo::transport::TransportResult<GrpcTransport> {
    let builder = GrpcTransport::builder(grpc)
        .acknowledgements(acknowledgements)
        .armed(true)
        .max_hold(PUSH_MAX_HOLD)
        .memory_guard(Arc::clone(memory_guard));
    match governor {
        Some(governor) => builder.pressure(governor.pressure()),
        None => builder,
    }
    .start()
    .await
}

/// Apply `source.acknowledgements` to a pull source, which the pipeline arms
/// before its first receive.
fn with_acknowledgements(
    receiver: AnyReceiver,
    #[cfg_attr(not(feature = "kafka"), allow(unused_variables))]
    acknowledgements: AcknowledgementsConfig,
) -> AnyReceiver {
    match receiver {
        #[cfg(feature = "kafka")]
        AnyReceiver::Kafka(kafka) => {
            AnyReceiver::Kafka(kafka.with_acknowledgements(acknowledgements))
        }
        // scalo's enum, whose other variants hold no acknowledgement to apply.
        #[allow(unreachable_patterns)]
        other => other,
    }
}

/// Build the outbound transport, which is never governed.
///
/// Braking the outbound drain would deadlock the loop, because a block cannot
/// be released until it is sent. `sink.max_message_bytes` bounds each record
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

/// The outbound half of the same, with each direct send bounded by
/// [`NEXT_HOP_SEND_TIMEOUT_MS`] rather than scalo's 30 s default.
fn sink_transport(config: &Config) -> crate::Result<TransportConfig> {
    match config.sink.transport {
        #[cfg(feature = "grpc")]
        Transport::Direct => {
            let mut grpc = GrpcConfig::client(&config.sink.endpoint);
            grpc.send_timeout_ms = NEXT_HOP_SEND_TIMEOUT_MS;
            Ok(TransportConfig {
                transport_type: TransportType::Grpc,
                grpc: Some(grpc),
                ..TransportConfig::default()
            })
        }
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

/// The batch loop, over an engine and transports the caller already built.
///
/// Separate from [`run`] so a broker round-trip can drive it without a
/// `ServiceRuntime`, which scalo only constructs inside its own lifecycle.
///
/// # Errors
///
/// Returns [`crate::Error::UnknownSource`] if the configured source has no
/// transform, or [`crate::Error::Transport`] if the source or the sink failed
/// in a way no retry can fix. The block in hand is left unreleased then, so a
/// restart reads it again.
pub async fn run_loop<S: TransportSender>(
    config: &Config,
    engine: &BatchEngine,
    consumer: &AnyReceiver,
    producer: &S,
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

    let stage = Stage {
        config,
        transform,
        resolver: crate::envelope::Resolver::new(config.source.envelope, intake, dataset),
        topic: Arc::from(config.sink.topic.as_str()),
        metrics,
        scaling,
        consumer,
        state: parking_lot::Mutex::new(LoopState::new()),
    };
    let budget = message_budget(config.sink.max_message_bytes);

    metrics.dfe.pipeline_ready(true);

    tracing::info!(
        source = %config.source.name,
        transform = transform.name(),
        source_transport = ?config.source.transport,
        sink_transport = ?config.sink.transport,
        topics = ?config.source.topics,
        sink_topic = %config.sink.topic,
        batch_size = config.source.batch_size,
        acknowledgements = config.source.acknowledgements.enabled,
        "transform service started"
    );

    push_scaling_signals(scaling, consumer, 0.0);

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

    let result = engine
        .pipeline(consumer)
        .shutdown(shutdown.clone())
        .sender(producer)
        .ticker(SCALING_SIGNAL_INTERVAL, || {
            stage.signal_if_idle();
            std::future::ready(Ok(()))
        })
        .run_with_pieces(
            |block| Ok(block.map_records(|records| stage.transform(&records))),
            |out: &WorkBatch<AnyToken>, pieces: &BlockPieces<'_>| {
                send_blocks(
                    producer,
                    metrics,
                    out.records.clone(),
                    budget,
                    pieces.piece(),
                )
            },
        )
        .await;

    metrics.dfe.pipeline_ready(false);
    match result {
        Ok(()) => {
            tracing::info!("transform service stopped");
            Ok(())
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                "stopping: the block in hand is left unreleased, so a restart reads it again"
            );
            Err(crate::Error::Transport(e.to_string()))
        }
    }
}

/// What the loop carries from one block to the next.
struct LoopState {
    /// `painless_stats` counts cumulatively for the process; the metrics want
    /// per-block deltas.
    painless_seen: (u64, u64),
    /// The `GeoIP` cache's cumulative hits and misses, for the same reason.
    geoip_seen: (u64, u64),
    /// The shape last reported, so a steady stream logs once rather than per
    /// block and a producer change is still visible the block it happens.
    last_envelope: Option<crate::envelope::Detected>,
    /// When the scaling signals were last pushed.
    last_signal: Instant,
}

impl LoopState {
    fn new() -> Self {
        Self {
            painless_seen: (0, 0),
            geoip_seen: (0, 0),
            last_envelope: None,
            last_signal: Instant::now(),
        }
    }
}

/// Everything the transform half of the loop needs, shared with the ticker.
struct Stage<'a> {
    config: &'a Config,
    transform: &'static (dyn dfe_runtime::Transform + Sync),
    resolver: crate::envelope::Resolver,
    /// The sink topic, carried by every outbound record.
    topic: Arc<str>,
    metrics: &'a TransformMetrics,
    scaling: Option<&'a ScalingSignals>,
    consumer: &'a AnyReceiver,
    /// Locked once per block and once per tick, never across an await.
    state: parking_lot::Mutex<LoopState>,
}

impl Stage<'_> {
    /// Parse, transform and serialise one block, returning a record per event
    /// that survived.
    // Block and byte counts are bounded far below 2^53, so the f64 casts are exact.
    #[allow(clippy::cast_precision_loss)]
    fn transform(&self, records: &[Record]) -> Vec<Record> {
        if records.is_empty() {
            return Vec::new();
        }
        let metrics = self.metrics;

        let received_bytes: usize = records.iter().map(|r| r.payload.len()).sum();
        metrics.app.bytes_received.increment(received_bytes as u64);
        metrics.batch_events.record(records.len() as f64);

        let started = Instant::now();
        let mut events = Vec::with_capacity(records.len());
        for record in records {
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

        // One series, counted once. The source transport counts its own wire
        // records in `transport_received_*`; these are the events parsed out.
        metrics.dfe.records_received(events.len() as u64);

        let (transformed, outcome, envelopes) =
            transform_batch_resolved(self.transform, &self.resolver, events);
        {
            let mut state = self.state.lock();
            let state = &mut *state;
            report_envelope(&envelopes, &mut state.last_envelope, metrics);
            record_batch(
                metrics,
                outcome,
                &mut state.painless_seen,
                &mut state.geoip_seen,
            );
            state.last_signal = Instant::now();
        }
        metrics
            .batch_duration
            .record(started.elapsed().as_secs_f64());

        tracing::debug!(
            emitted = outcome.emitted,
            dropped = outcome.dropped,
            errored = outcome.errored,
            "batch transformed"
        );

        // Batch saturation: how full the pull came back. A consistently full
        // batch means the transform is the constraint, not the topic.
        let saturation = records.len() as f64 / self.config.source.batch_size as f64;
        push_scaling_signals(self.scaling, self.consumer, saturation.min(1.0));

        let (payloads, serialised) =
            serialise_events(&transformed, self.config.sink.max_message_bytes);
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

        payloads
            .into_iter()
            .map(|payload| outbound_record(payload, &self.topic))
            .collect()
    }

    /// Push a zero saturation when no block has pushed the signals for
    /// [`SCALING_SIGNAL_INTERVAL`], so an idle pod still reports its lag.
    fn signal_if_idle(&self) {
        let due = {
            let mut state = self.state.lock();
            let due = state.last_signal.elapsed() >= SCALING_SIGNAL_INTERVAL;
            if due {
                state.last_signal = Instant::now();
            }
            due
        };
        if due {
            push_scaling_signals(self.scaling, self.consumer, 0.0);
        }
    }
}

/// One event's payload, addressed at the sink topic.
///
/// ONE record per event: dfe-loader parses exactly one JSON document per
/// message and has no NDJSON path, so a batch concatenated into a single
/// payload was dead-lettered whole rather than loaded
/// (hyperi-io/dfe-transform-elastic#67).
///
/// `key` is the DESTINATION rather than a partition key: Kafka's `send_batch`
/// produces each record to the topic its `key` names.
fn outbound_record(payload: Vec<u8>, topic: &Arc<str>) -> Record {
    Record {
        // Refcounted, so each retry re-sends the same buffer.
        payload: bytes::Bytes::from(payload),
        key: Some(Arc::clone(topic)),
        headers: Vec::new(),
        metadata: RecordMeta {
            timestamp_ms: None,
            format: PayloadFormat::Json,
        },
    }
}

/// Split `records` into blocks of at most `max_bytes` of payload each.
///
/// `max_bytes` is the per-MESSAGE budget on both arms, which is why one number
/// bounds both halves: a Kafka record is one message, and a gRPC `send_batch`
/// block is one RPC message against scalo's own 16 MiB ceiling. A block always
/// takes at least one record, so a payload at the budget goes out alone rather
/// than not at all.
fn blocks(records: &[Record], max_bytes: usize) -> Vec<&[Record]> {
    let mut blocks = Vec::new();
    let mut start = 0;
    let mut block_bytes = 0_usize;

    for (index, record) in records.iter().enumerate() {
        let bytes = record.payload.len();
        if index > start && block_bytes.saturating_add(bytes) > max_bytes {
            blocks.push(&records[start..index]);
            start = index;
            block_bytes = 0;
        }
        block_bytes = block_bytes.saturating_add(bytes);
    }

    if start < records.len() {
        blocks.push(&records[start..]);
    }
    blocks
}

/// Produce one transformed block, in sends of at most `budget` bytes.
///
/// A refusal the sink may lift -- backpressure, a broker outage, a timeout -- is
/// returned as a transient error, which the pipeline retries for as long as it
/// lasts; the retry re-sends every send in the block, including any that
/// already landed. A refusal no retry can fix is returned as a permanent error,
/// which stops the loop with the block unreleased.
///
/// [`TransportSender::send_batch`] is NOT atomic on Kafka: a failed send may
/// already have left any subset of its records on the broker. That is the
/// duplicate half of at-least-once; the other half is never counting a record
/// delivered until the send it went in returned `Ok`.
///
/// `piece` reports `Dropped` when the sink dead-lettered a send, which releases
/// the source without calling those records delivered. Otherwise it reports
/// `Delivered`, the floor of the merge, and leaves the block's status to the
/// loop's own piece.
async fn send_blocks<S: TransportSender>(
    producer: &S,
    metrics: &TransformMetrics,
    records: Vec<Record>,
    budget: usize,
    piece: PieceFinalizer,
) -> Result<(), EngineError> {
    let mut filtered = false;
    let result = send_each(producer, metrics, &records, budget, &mut filtered).await;
    piece.report(if filtered {
        DeliveryStatus::Dropped
    } else {
        DeliveryStatus::Delivered
    });
    result
}

/// The sends behind [`send_blocks`], setting `filtered` when the sink
/// dead-lettered one.
async fn send_each<S: TransportSender>(
    producer: &S,
    metrics: &TransformMetrics,
    records: &[Record],
    budget: usize,
    filtered: &mut bool,
) -> Result<(), EngineError> {
    for block in blocks(records, budget) {
        let count = block.len() as u64;
        match producer.send_batch(block).await {
            SendResult::Ok => {
                // The sink transport counts its own `transport_*` series.
                let bytes: u64 = block.iter().map(|r| r.payload.len() as u64).sum();
                metrics.app.bytes_written.increment(bytes);
                metrics.dfe.records_delivered(count);
            }
            // The NORMAL response from a slow sink, and what a Kafka or gRPC
            // sink answers while its broker or listener is unreachable.
            SendResult::Backpressured => {
                metrics.send_backpressure.increment(1);
                return Err(EngineError::Transport(TransportError::Backpressure));
            }
            SendResult::Fatal(e) if e.is_recoverable() => return Err(EngineError::Transport(e)),
            SendResult::Fatal(e) => {
                metrics.send_failures.increment(1);
                return Err(EngineError::Transport(e));
            }
            // The pipeline screens out what the sink would dead-letter before
            // the send, so this is a refusal the screen could not foresee.
            SendResult::FilteredDlq => {
                *filtered = true;
                metrics.send_filtered_dlq.increment(1);
                metrics.dfe.records_filtered(count);
                tracing::warn!(
                    records = count,
                    "the sink transport dead-lettered a send and this service has no DLQ; \
                     its records are dropped and not counted delivered"
                );
            }
        }
    }
    Ok(())
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
        // The in-memory source the unit tests drive, which only a dev build of
        // scalo carries.
        #[allow(unreachable_patterns, clippy::match_wildcard_for_single_variants)]
        _ => None,
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
/// A receive capped by count alone lets 20,000 records of whatever size the
/// producer chose take an unbounded amount of memory: the outbound side has
/// always been bounded by bytes (`sink.max_message_bytes`) and the inbound side
/// had no equivalent. `fetch.max.bytes` is where librdkafka enforces it, so the
/// ceiling applies before the bytes are in the process rather than after.
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

/// The bus sink's producer config.
#[cfg(feature = "kafka")]
fn producer_config(config: &Config) -> KafkaConfig {
    let producer = KafkaConfig {
        brokers: config.sink_brokers().to_vec(),
        client_id: "dfe-transform-elastic-producer".to_string(),
        ..transport_defaults()
    };
    let holds = config.source.transport.is_direct() && config.source.acknowledgements.enabled;
    cap_delivery_timeout(producer, holds)
}

/// Cap `message.timeout.ms` at [`NEXT_HOP_SEND_TIMEOUT_MS`] while a direct
/// source holds its pushes, unless the producer config already sets it.
///
/// A delivery report that comes after the hold is spent is a duplicate in
/// waiting: the sender was already told to retry. librdkafka's own default is
/// 300 s.
#[cfg(feature = "kafka")]
fn cap_delivery_timeout(mut producer: KafkaConfig, holds: bool) -> KafkaConfig {
    if holds && !producer.librdkafka_overrides.contains_key(MESSAGE_TIMEOUT) {
        producer.librdkafka_overrides.insert(
            MESSAGE_TIMEOUT.to_string(),
            NEXT_HOP_SEND_TIMEOUT_MS.to_string(),
        );
    }
    producer
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
                acknowledgements: AcknowledgementsConfig::default(),
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

        /// The `message.timeout.ms` the bus sink's producer is built with.
        fn delivery_timeout(producer: &KafkaConfig) -> Option<&str> {
            producer
                .librdkafka_overrides
                .get(MESSAGE_TIMEOUT)
                .map(String::as_str)
        }

        /// A delivery report after the hold is spent comes back to a sender
        /// that was already told to retry, so a held push caps the producer's
        /// timeout inside the hold.
        #[test]
        fn a_held_push_caps_the_producer_delivery_timeout_inside_the_hold() {
            let mut c = config();
            assert_eq!(
                delivery_timeout(&producer_config(&c)),
                None,
                "the bus source holds no push, so librdkafka's default stands"
            );

            c.source.transport = crate::config::Transport::Direct;
            let held = producer_config(&c);
            let capped: u64 = delivery_timeout(&held)
                .expect("a held push caps the delivery timeout")
                .parse()
                .unwrap();
            assert_eq!(capped, NEXT_HOP_SEND_TIMEOUT_MS);
            assert!(u128::from(capped) < PUSH_MAX_HOLD.as_millis());

            c.source.acknowledgements = AcknowledgementsConfig::new(false);
            assert_eq!(
                delivery_timeout(&producer_config(&c)),
                None,
                "a source answered at receipt holds nothing"
            );

            let explicit = KafkaConfig::default().with_override(MESSAGE_TIMEOUT, "60000");
            assert_eq!(
                delivery_timeout(&cap_delivery_timeout(explicit, true)),
                Some("60000"),
                "an explicit librdkafka setting is the operator's"
            );
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

    /// The receive cap is the configured batch size, not the engine's default.
    #[test]
    fn the_engine_receives_at_most_batch_size_records() {
        let mut c = config();
        c.source.batch_size = 1_234;
        assert_eq!(engine_config(&c).max_chunk_size, 1_234);
    }

    // -- What the sink answers --------------------------------------------

    /// What the sink answers, scripted, so the branches that need a broker to
    /// reach in production are reachable here.
    #[derive(Clone, Copy)]
    enum Answer {
        Ok,
        Backpressured,
        Fatal,
        Timeout,
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

        fn reply(&self) -> SendResult {
            self.attempts
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            match self.answer {
                Answer::Ok => SendResult::Ok,
                Answer::Backpressured => SendResult::Backpressured,
                Answer::Fatal => {
                    SendResult::Fatal(TransportError::Connection("broker gone".into()))
                }
                Answer::Timeout => SendResult::Fatal(TransportError::Timeout),
                Answer::FilteredDlq => SendResult::FilteredDlq,
            }
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
            std::future::ready(self.reply())
        }

        /// Answer the BLOCK directly: scalo's per-record default absorbs a
        /// filtered record and answers `Ok`.
        fn send_batch(&self, _records: &[Record]) -> impl Future<Output = SendResult> + Send {
            std::future::ready(self.reply())
        }
    }

    /// `count` records addressed at the sink topic, which is what the
    /// transform half hands the sink.
    fn records(count: usize) -> Vec<Record> {
        let topic: Arc<str> = Arc::from("out");
        (0..count)
            .map(|i| outbound_record(format!("{{\"i\":{i}}}").into_bytes(), &topic))
            .collect()
    }

    fn test_metrics() -> TransformMetrics {
        static MANAGER: std::sync::OnceLock<scalo::metrics::MetricsManager> =
            std::sync::OnceLock::new();
        let manager =
            MANAGER.get_or_init(|| scalo::metrics::MetricsManager::new("transform_elastic_test"));
        TransformMetrics::register(manager, "0.0.0-test", "test")
    }

    /// Whether the pipeline waits this failure out rather than stopping.
    fn is_transient(error: &EngineError) -> bool {
        matches!(error, EngineError::Transport(e) if e.is_recoverable())
    }

    /// The piece a sink call reports into, sealed as the loop seals it, and
    /// the status the block is released with.
    fn one_piece() -> (PieceFinalizer, std::sync::mpsc::Receiver<DeliveryStatus>) {
        let (released, merged) = std::sync::mpsc::channel();
        let block = scalo::transport::BatchFinalizer::new(move |status| {
            let _ = released.send(status);
        });
        let piece = block.piece();
        block.seal();
        (piece, merged)
    }

    /// `send_blocks` with a piece nothing reads.
    async fn send(sink: &Scripted, records: Vec<Record>, budget: usize) -> Result<(), EngineError> {
        send_blocks(sink, &test_metrics(), records, budget, one_piece().0).await
    }

    /// Backpressure is the NORMAL answer from a slow or unreachable sink, so it
    /// must reach the pipeline as something it retries, and never as delivery.
    #[tokio::test]
    async fn backpressure_is_handed_back_for_the_pipeline_to_retry() {
        let sink = Scripted::new(Answer::Backpressured);
        let error = send(&sink, records(3), 1_000_000)
            .await
            .expect_err("backpressure is not delivery");

        assert!(is_transient(&error), "the pipeline would stop on {error}");
        assert_eq!(sink.attempts(), 1, "the pipeline retries, not the sink");
    }

    /// A timeout reads the same way: the sink may take it next time.
    #[tokio::test]
    async fn a_timed_out_send_is_retried() {
        let sink = Scripted::new(Answer::Timeout);
        let error = send(&sink, records(1), 1_000_000)
            .await
            .expect_err("a timeout is not delivery");
        assert!(is_transient(&error), "the pipeline would stop on {error}");
    }

    /// A refusal no retry can fix stops the loop, which leaves the block
    /// unreleased so a restart reads it again.
    #[tokio::test]
    async fn a_permanent_refusal_stops_the_loop() {
        let sink = Scripted::new(Answer::Fatal);
        let error = send(&sink, records(1), 1_000_000)
            .await
            .expect_err("a fatal send is not delivery");
        assert!(!is_transient(&error), "a permanent refusal was retried");
    }

    /// A send the sink dead-letters is dropped and counted, and the rest of the
    /// block still goes: a deterministic refusal retried is a stalled partition.
    #[tokio::test]
    async fn a_dead_lettered_send_does_not_stall_the_block() {
        let sink = Scripted::new(Answer::FilteredDlq);
        let payload = records(1)[0].payload.len();

        send(&sink, records(3), payload)
            .await
            .expect("a dead-lettered send is handled");
        assert_eq!(sink.attempts(), 3, "every block must still be offered");
    }

    /// A block with a dead-lettered send still releases its source, so it is
    /// not sent again, but as dropped: those records were not delivered.
    #[tokio::test]
    async fn a_dead_lettered_block_is_released_dropped_not_delivered() {
        let released = |answer| async move {
            let (piece, merged) = one_piece();
            let verdict = send_blocks(
                &Scripted::new(answer),
                &test_metrics(),
                records(3),
                1_000_000,
                piece,
            )
            .await;
            (verdict, merged.try_recv().expect("the piece reported"))
        };

        let (verdict, status) = released(Answer::FilteredDlq).await;
        assert!(
            verdict.is_ok(),
            "a dead-lettered block is not retried: {verdict:?}"
        );
        assert_eq!(status, DeliveryStatus::Dropped);

        // Any other answer leaves the block's status to the loop's own piece.
        for answer in [
            Answer::Ok,
            Answer::Backpressured,
            Answer::Timeout,
            Answer::Fatal,
        ] {
            let (_, status) = released(answer).await;
            assert_eq!(status, DeliveryStatus::Delivered);
        }
    }

    /// The first failed send stops the block, so no later send is counted
    /// delivered while an earlier one is still unconfirmed.
    #[tokio::test]
    async fn the_first_failed_send_ends_the_attempt() {
        let sink = Scripted::new(Answer::Backpressured);
        let payload = records(1)[0].payload.len();

        let _ = send(&sink, records(3), payload).await;
        assert_eq!(sink.attempts(), 1);
    }

    /// A direct send gives up inside the hold, so a slow next hop is retried
    /// before the push in front of it is answered.
    #[cfg(feature = "grpc")]
    #[test]
    fn a_direct_send_gives_up_inside_the_hold() {
        let mut c = config();
        c.sink.transport = crate::config::Transport::Direct;
        c.sink.endpoint = "http://loader:50051".into();

        let grpc = sink_transport(&c).unwrap().grpc.unwrap();
        assert_eq!(grpc.send_timeout_ms, NEXT_HOP_SEND_TIMEOUT_MS);
        assert!(
            u128::from(grpc.send_timeout_ms) < PUSH_MAX_HOLD.as_millis(),
            "a send outlasting the hold lands after its push was answered"
        );
    }

    /// A gRPC `send_batch` block is ONE RPC message against scalo's 16 MiB
    /// ceiling, so the block is bounded by the same budget a Kafka record is.
    #[test]
    fn a_block_is_bounded_by_the_per_message_budget() {
        let topic: Arc<str> = Arc::from("out");
        let records: Vec<Record> = (0..10)
            .map(|_| outbound_record(vec![b'x'; 400], &topic))
            .collect();

        let blocks = blocks(&records, 1000);
        assert_eq!(
            blocks.len(),
            5,
            "two 400-byte records fit a 1000-byte block"
        );
        for block in &blocks {
            let bytes: usize = block.iter().map(|r| r.payload.len()).sum();
            assert!(bytes <= 1000, "block of {bytes} bytes");
            for record in *block {
                assert_eq!(record.key.as_deref(), Some("out"));
            }
        }
    }

    /// A payload at or above the budget still has to go out, so a block always
    /// takes at least one record.
    #[test]
    fn a_payload_at_the_budget_goes_out_in_a_block_of_its_own() {
        let topic: Arc<str> = Arc::from("out");
        let records = vec![
            outbound_record(vec![b'x'; 10], &topic),
            outbound_record(vec![b'y'; 4000], &topic),
            outbound_record(vec![b'z'; 10], &topic),
        ];

        let blocks = blocks(&records, 100);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[1].len(), 1);
        assert_eq!(blocks[1][0].payload.len(), 4000);
    }

    #[test]
    fn no_records_is_no_blocks() {
        assert!(blocks(&[], 100).is_empty());
    }

    // -- The loop over an in-process source ---------------------------------

    /// A sink that refuses its first `refusals` sends, then keeps what it is
    /// given, and names every record over `ceiling` bytes one it would refuse.
    struct Flaky {
        refusals: usize,
        ceiling: usize,
        attempts: std::sync::atomic::AtomicUsize,
        delivered: std::sync::Mutex<Vec<bytes::Bytes>>,
    }

    impl Flaky {
        fn new(refusals: usize, ceiling: usize) -> Self {
            Self {
                refusals,
                ceiling,
                attempts: std::sync::atomic::AtomicUsize::new(0),
                delivered: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn attempts(&self) -> usize {
            self.attempts.load(std::sync::atomic::Ordering::Relaxed)
        }

        fn delivered(&self) -> Vec<bytes::Bytes> {
            self.delivered.lock().unwrap().clone()
        }

        /// Poll until `count` records have been delivered.
        async fn wait_for(&self, count: usize) {
            while self.delivered().len() < count {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    }

    impl scalo::transport::TransportBase for Flaky {
        fn close(&self) -> impl Future<Output = scalo::transport::TransportResult<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn is_healthy(&self) -> bool {
            true
        }
        fn name(&self) -> &'static str {
            "flaky"
        }
    }

    impl TransportSender for Flaky {
        fn send(
            &self,
            _destination: &str,
            _payload: bytes::Bytes,
        ) -> impl Future<Output = SendResult> + Send {
            std::future::ready(SendResult::Fatal(TransportError::Internal(
                "the loop sends blocks".into(),
            )))
        }

        fn send_batch(&self, records: &[Record]) -> impl Future<Output = SendResult> + Send {
            let attempt = self
                .attempts
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let result = if attempt < self.refusals {
                SendResult::Backpressured
            } else {
                self.delivered
                    .lock()
                    .unwrap()
                    .extend(records.iter().map(|r| r.payload.clone()));
                SendResult::Ok
            };
            std::future::ready(result)
        }

        fn dead_letter_reason(
            &self,
            record: &Record,
        ) -> Option<scalo::transport::DeadLetterReason> {
            (record.payload.len() > self.ceiling).then_some(
                scalo::transport::DeadLetterReason::TooLarge {
                    bytes: record.payload.len(),
                    limit: self.ceiling,
                },
            )
        }
    }

    /// One Beats-wrapped okta event with its `uuid` set to `uuid` and `pad`
    /// bytes added to `displayMessage`, which the transform carries through.
    fn okta_event(uuid: &str, pad: usize) -> Vec<u8> {
        let raw = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/okta/system/test-okta-system-events.log"
        ))
        .unwrap();
        let line = raw.lines().find(|l| !l.trim().is_empty()).unwrap();
        let mut vendor: serde_json::Value = serde_json::from_str(line).unwrap();
        vendor["uuid"] = serde_json::Value::String(uuid.into());
        if pad > 0 {
            vendor["displayMessage"] = serde_json::Value::String("x".repeat(pad));
        }
        serde_json::to_vec(&serde_json::json!({ "message": vendor.to_string() })).unwrap()
    }

    /// An in-process source holding `events`, one record each.
    async fn memory_source(events: Vec<Vec<u8>>) -> AnyReceiver {
        let source =
            scalo::transport::memory::MemoryTransport::new(&scalo::transport::MemoryConfig {
                recv_timeout_ms: 50,
                ..scalo::transport::MemoryConfig::default()
            })
            .unwrap();
        for event in events {
            source.inject(Some("in"), event).await.unwrap();
        }
        AnyReceiver::Memory(source)
    }

    /// A sink that refuses for longer than any fixed retry budget is waited out:
    /// the loop retries the same block until the sink takes it, and does not
    /// stop on its own.
    #[tokio::test(start_paused = true)]
    async fn a_sink_refusing_past_a_retry_budget_is_waited_out() {
        // Well past the eight attempts a fixed budget used to allow.
        const REFUSALS: usize = 40;

        let config = config();
        let engine = BatchEngine::new(engine_config(&config));
        let consumer = memory_source(vec![okta_event("held", 0)]).await;
        let sink = Flaky::new(REFUSALS, usize::MAX);
        let shutdown = CancellationToken::new();
        let metrics = test_metrics();

        let run = run_loop(
            &config, &engine, &consumer, &sink, &shutdown, &metrics, None,
        );
        tokio::pin!(run);
        tokio::select! {
            result = &mut run => panic!("the loop stopped while the sink refused: {result:?}"),
            () = sink.wait_for(1) => {}
        }
        assert_eq!(sink.attempts(), REFUSALS + 1, "every refusal was retried");

        shutdown.cancel();
        run.await.expect("the loop stops cleanly on shutdown");
        assert_eq!(
            sink.delivered().len(),
            1,
            "the held event went once it could"
        );
    }

    /// A record the sink would refuse never reaches it and does not hold up
    /// the block: the event beside it is delivered, and so is the next block.
    #[tokio::test(start_paused = true)]
    async fn a_record_the_sink_would_refuse_is_screened_out() {
        const CEILING: usize = 16 * 1024;

        let config = config();
        let engine = BatchEngine::new(engine_config(&config));
        let consumer =
            memory_source(vec![okta_event("kept", 0), okta_event("big", 64 * 1024)]).await;
        let sink = Flaky::new(0, CEILING);
        let shutdown = CancellationToken::new();
        let metrics = test_metrics();

        let run = run_loop(
            &config, &engine, &consumer, &sink, &shutdown, &metrics, None,
        );
        tokio::pin!(run);
        tokio::select! {
            result = &mut run => panic!("the loop stopped: {result:?}"),
            () = sink.wait_for(1) => {}
        }
        if let AnyReceiver::Memory(source) = &consumer {
            source
                .inject(Some("in"), okta_event("next", 0))
                .await
                .unwrap();
        }
        tokio::select! {
            result = &mut run => panic!("the loop stopped: {result:?}"),
            () = sink.wait_for(2) => {}
        }

        shutdown.cancel();
        run.await.expect("the loop stops cleanly on shutdown");

        let delivered = sink.delivered();
        assert_eq!(delivered.len(), 2, "only the events the sink can take");
        for payload in &delivered {
            assert!(
                payload.len() <= CEILING,
                "a refused record reached the sink"
            );
        }
    }
}
