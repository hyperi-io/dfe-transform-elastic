// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The scalo Push listener this service's direct transport is built on.
//!
//! A record pushed at a listener comes back out of its own `recv`, with no
//! broker anywhere. That is the transport half of issue #19: it proves the
//! `grpc` feature wires scalo's listener and client through to this crate, and
//! it needs no container, so it runs by default rather than behind `#[ignore]`.
//!
//! What it does NOT cover is the service: nothing here drives the batch loop,
//! because that loop takes `KafkaTransport` today. The end-to-end assertion --
//! pushed in one side, transformed out the other -- arrives with the loop's
//! transport-agnostic rewrite.

#![cfg(feature = "grpc")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Duration;

use bytes::Bytes;
use scalo::transport::grpc::{GrpcConfig, GrpcTransport};
use scalo::transport::{TransportReceiver, TransportSender};

/// Allocate a free loopback port.
fn random_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);
    port
}

/// Poll until the port accepts a connection, or fail after 15s.
async fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..300 {
        if tokio::net::TcpStream::connect(&addr).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("nothing listening on 127.0.0.1:{port} within 15s");
}

/// Start a Push listener and return its endpoint plus the transport.
///
/// The port is free when picked and can be taken before the bind lands under a
/// parallel test run, so a failed bind retries on a fresh one.
async fn start_listener() -> (String, GrpcTransport) {
    let mut last_err = String::new();
    for _ in 0..20 {
        let port = random_port();
        match GrpcTransport::new(&GrpcConfig::server(&format!("127.0.0.1:{port}"))).await {
            Ok(transport) => {
                wait_for_port(port).await;
                return (format!("http://127.0.0.1:{port}"), transport);
            }
            Err(e) => {
                last_err = e.to_string();
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }
    }
    panic!("listener failed to start after 20 attempts: {last_err}");
}

#[tokio::test]
async fn a_record_pushed_at_the_listener_comes_back_out_of_it() {
    let (endpoint, listener) = start_listener().await;
    let pusher = GrpcTransport::new(&GrpcConfig::client(&endpoint))
        .await
        .expect("push client connects to the listener");

    let sent: Vec<String> = (0..3)
        .map(|id| format!(r#"{{"message":"line {id}"}}"#))
        .collect();
    for payload in &sent {
        let result = pusher
            .send("elastic_in", Bytes::from(payload.clone()))
            .await;
        assert!(result.is_ok(), "push failed: {result:?}");
    }

    let mut received: Vec<Vec<u8>> = Vec::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while received.len() < sent.len() && tokio::time::Instant::now() < deadline {
        if let Ok(batch) = listener.recv(10).await {
            received.extend(batch.records.into_iter().map(|r| r.payload.to_vec()));
        }
    }

    assert_eq!(
        received.len(),
        sent.len(),
        "every pushed record must reach the listener"
    );
    // Byte-for-byte: the transport carries the payload, it does not parse it.
    for (payload, expected) in received.iter().zip(&sent) {
        assert_eq!(
            std::str::from_utf8(payload).expect("payload is utf-8"),
            expected
        );
    }
}

#[tokio::test]
async fn a_client_pointed_at_nothing_fails_rather_than_hanging() {
    // A dead endpoint is what a misconfigured `sink.endpoint` produces, and the
    // service must learn about it at startup rather than at the first batch.
    let port = random_port();
    let client = GrpcTransport::new(&GrpcConfig::client(&format!("http://127.0.0.1:{port}"))).await;

    // Refusing at construction and refusing at the send are both correct, so
    // the branch is named rather than skipped.
    let refused_at = match client {
        Ok(transport) => {
            let result = transport
                .send("elastic_in", Bytes::from_static(b"{}"))
                .await;
            assert!(
                !matches!(result, scalo::transport::SendResult::Ok),
                "a send to a dead endpoint reported success"
            );
            "send"
        }
        Err(_) => "construction",
    };
    println!("a dead endpoint was refused at {refused_at}");
}
