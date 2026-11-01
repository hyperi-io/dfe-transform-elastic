// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The service binary consumes and produces over SASL_SSL with a private CA.
//!
//! A chart delivers that CA as a file named by `SSL_CERT_FILE`, with no CA in
//! the service's own config. The binary runs as its own process, so the
//! variable reaches OpenSSL as it does in a pod, and verification stays on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::{Child, Command, Stdio};
use std::time::Duration;

use scalo::deployment::test_support::shared_service::Suite;
use scalo::transport::kafka::test_support::{
    SCRAM_USER, TlsBroker, require_docker, scram_password,
};
use scalo::transport::kafka::{KafkaConfig, KafkaTransport};
use scalo::transport::{TransportReceiver, TransportSender};

const SUITE: Suite = Suite {
    repo: "dfe-transform-elastic",
    manifest_dir: env!("CARGO_MANIFEST_DIR"),
    tmp_dir: option_env!("CARGO_TARGET_TMPDIR"),
};

/// The service binary, killed when dropped.
struct Service(Child);

impl Drop for Service {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// A client of `broker`'s SASL_SSL listener that names the CA itself.
async fn client(broker: &TlsBroker, group: &str, topics: Vec<String>) -> KafkaTransport {
    KafkaTransport::new(&KafkaConfig {
        brokers: vec![broker.sasl_ssl.clone()],
        group: group.to_string(),
        client_id: group.to_string(),
        topics,
        auto_offset_reset: "earliest".to_string(),
        security_protocol: "sasl_ssl".to_string(),
        sasl_mechanism: Some("SCRAM-SHA-512".to_string()),
        sasl_username: Some(SCRAM_USER.to_string()),
        sasl_password: Some(scalo::SensitiveString::new(scram_password())),
        ssl_ca_location: Some(broker.pki.file("ca.crt")),
        ssl_skip_verify: false,
        ..KafkaConfig::devtest()
    })
    .await
    .expect("a SASL_SSL client connects")
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .expect("a free port")
}

#[tokio::test(flavor = "multi_thread")]
async fn events_flow_over_sasl_ssl_with_the_ca_from_ssl_cert_file() {
    if !require_docker() {
        return;
    }
    let broker = TlsBroker::start(&SUITE, "kafka-tls-ssl-cert-file")
        .await
        .expect("TLS broker");
    let id = std::process::id();
    let (source, sink) = (format!("tls-src-{id}"), format!("tls-sink-{id}"));

    let dir = tempfile::tempdir().expect("config dir");
    let config_path = dir.path().join("config.yaml");
    std::fs::write(
        &config_path,
        format!(
            "source:\n  name: filebeat.okta.default\n  envelope: beats\n  topics: [{source}]\n  \
             group_id: tls-{id}\n  brokers: [\"{brokers}\"]\nsink:\n  topic: {sink}\n\
             geoip:\n  enabled: false\ndlq:\n  file:\n    path: {dlq}\n",
            brokers = broker.sasl_ssl,
            dlq = dir.path().join("dlq").display(),
        ),
    )
    .expect("write config");
    std::fs::create_dir(dir.path().join("dlq")).expect("dlq dir");

    let log_path = dir.path().join("service.log");
    let log = std::fs::File::create(&log_path).expect("log file");
    let _service = Service(
        Command::new(env!("CARGO_BIN_EXE_dfe-transform-elastic"))
            .arg("--config")
            .arg(&config_path)
            .arg("--metrics-addr")
            .arg(format!("127.0.0.1:{}", free_port()))
            .arg("run")
            .env("KAFKA_SECURITY_PROTOCOL", "sasl_ssl")
            .env("KAFKA_SASL_MECHANISM", "SCRAM-SHA-512")
            .env("KAFKA_SASL_USERNAME", SCRAM_USER)
            .env("KAFKA_SASL_PASSWORD", scram_password())
            .env("KAFKA_SSL_SKIP_VERIFY", "false")
            .env("SSL_CERT_FILE", broker.pki.file("ca.crt"))
            .stdout(Stdio::null())
            .stderr(log)
            .spawn()
            .expect("the service binary starts"),
    );

    let producer = client(&broker, &format!("tls-producer-{id}"), Vec::new()).await;
    let consumer = client(&broker, &format!("tls-verify-{id}"), vec![sink]).await;
    let event = serde_json::json!({ "message": r#"{"uuid":"over-sasl-ssl"}"# }).to_string();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(90);
    let mut landed = None;
    let mut next_send = tokio::time::Instant::now();
    while landed.is_none() && tokio::time::Instant::now() < deadline {
        // Sent again until it lands: the service may not have joined its group yet.
        if tokio::time::Instant::now() >= next_send {
            let _ = producer
                .send(&source, bytes::Bytes::from(event.clone()))
                .await;
            next_send = tokio::time::Instant::now() + Duration::from_secs(5);
        }
        if let Ok(batch) = consumer.recv(16).await {
            landed = batch
                .records
                .iter()
                .find(|record| String::from_utf8_lossy(&record.payload).contains("over-sasl-ssl"))
                .map(|record| record.payload.clone());
        }
        if landed.is_none() {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    let service_log = std::fs::read_to_string(&log_path).unwrap_or_default();
    let landed =
        landed.unwrap_or_else(|| panic!("no event reached {source}'s sink\n{service_log}"));
    let document: serde_json::Value = serde_json::from_slice(&landed).expect("a JSON document");
    assert!(
        document.to_string().contains("over-sasl-ssl"),
        "the event lost its uuid: {document}"
    );
}
