// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One record at the DFE record cap, holding the smallest Beats line the
//! transform still emits, through the whole service loop, with its memory held
//! to the deployment contract.
//!
//! Intake is bounded by bytes, and a record's NDJSON lines by nothing, so this
//! is the most events one accepted record can ask the service to hold. This
//! test is its own process, so the high-water mark is that record's alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use dfe_transform_elastic::config::{Config, default_max_message_bytes};
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::service::{engine_config, run_loop, with_dead_letters};
use scalo::transport::memory::MemoryTransport;
use scalo::transport::{
    AnyReceiver, MemoryConfig, Record, SendResult, TransportBase, TransportError, TransportResult,
    TransportSender,
};
use scalo::worker::BatchEngine;
use tokio_util::sync::CancellationToken;

/// The smallest line okta's pipeline still turns into a document.
const LINE: &[u8] = br#"{"message":"{}"}"#;

/// Sent after the record, so its arrival says every event before it went.
const MARKER: &[u8] = br#"{"message":"{\"uuid\":\"fanout-marker\"}"}"#;

/// A sink that counts what it is given and keeps none of it, so the peak is
/// the service's and not the sink's.
#[derive(Default)]
struct Counting {
    records: AtomicUsize,
    bytes: AtomicUsize,
    marker: AtomicBool,
}

impl TransportBase for Counting {
    fn close(&self) -> impl Future<Output = TransportResult<()>> + Send {
        std::future::ready(Ok(()))
    }
    fn is_healthy(&self) -> bool {
        true
    }
    fn name(&self) -> &'static str {
        "counting"
    }
}

impl TransportSender for Counting {
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
        self.records.fetch_add(records.len(), Ordering::Relaxed);
        let bytes: usize = records.iter().map(|r| r.payload.len()).sum();
        self.bytes.fetch_add(bytes, Ordering::Relaxed);
        let finder = memchr::memmem::Finder::new(b"fanout-marker");
        if records.iter().any(|r| finder.find(&r.payload).is_some()) {
            self.marker.store(true, Ordering::Release);
        }
        std::future::ready(SendResult::Ok)
    }
}

/// `count` copies of [`LINE`], newline-terminated, filling `cap` bytes.
fn record_at_the_cap(cap: usize) -> (Vec<u8>, usize) {
    let count = cap / (LINE.len() + 1);
    let mut payload = Vec::with_capacity(count * (LINE.len() + 1));
    for _ in 0..count {
        payload.extend_from_slice(LINE);
        payload.push(b'\n');
    }
    (payload, count)
}

#[tokio::test]
async fn a_record_of_tiny_lines_at_the_record_cap_stays_inside_the_pod_limit() {
    let config: Config = serde_json::from_value(serde_json::json!({
        "source": { "name": "filebeat.okta.default", "topics": ["in"], "envelope": "beats" },
        "sink": { "topic": "out" },
    }))
    .expect("a valid config");

    let (record, lines) = record_at_the_cap(default_max_message_bytes());
    let source = MemoryTransport::new(&MemoryConfig {
        recv_timeout_ms: 50,
        ..MemoryConfig::default()
    })
    .unwrap();
    source.inject(Some("in"), record).await.unwrap();
    source.inject(Some("in"), MARKER.to_vec()).await.unwrap();
    let consumer = AnyReceiver::Memory(source);

    let engine = with_dead_letters(BatchEngine::new(engine_config(&config)));
    let sink = Counting::default();
    let shutdown = CancellationToken::new();
    let manager = scalo::metrics::MetricsManager::new("transform_elastic_fanout_test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");

    let run = run_loop(
        &config, &engine, &consumer, &sink, &shutdown, &metrics, None,
    );
    tokio::pin!(run);
    let delivered = async {
        while !sink.marker.load(Ordering::Acquire) {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    };
    tokio::select! {
        result = &mut run => panic!("the loop stopped: {result:?}"),
        () = delivered => {}
    }
    shutdown.cancel();
    run.await.expect("the loop stops cleanly on shutdown");

    let sent = sink.records.load(Ordering::Relaxed);
    let bytes = sink.bytes.load(Ordering::Relaxed);
    eprintln!("{lines} lines in one record, {sent} records of {bytes} bytes sent");
    assert_eq!(sent, lines + 1, "every line and the marker became a record");

    // Fails over the contract's memory limit, warns over its request.
    scalo::deployment::test_support::assert_peak_memory(
        "a_record_of_tiny_lines_at_the_record_cap_stays_inside_the_pod_limit",
        &dfe_transform_elastic::deployment::contract().resources,
    );
}
