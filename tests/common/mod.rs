// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Broker test infrastructure.
//!
//! A live broker is used when one is reachable; otherwise an ephemeral Kafka
//! container is started for the test and removed when it drops. A suite that
//! silently skips both is a suite that reports green while testing nothing, so
//! in CI the absence of both is a failure rather than a skip.

use std::time::Duration;

use scalo::transport::kafka::{KafkaConfig, KafkaProfile, KafkaTransport};
use testcontainers::ContainerAsync;
use testcontainers_modules::kafka::apache::Kafka;

/// Label on every container this suite starts, for bulk cleanup:
/// `docker rm -f $(docker ps -aq --filter label=io.hyperi.test.suite=dfe-transform-elastic-broker)`
pub const TEST_SUITE_LABEL: (&str, &str) = ("io.hyperi.test.suite", "dfe-transform-elastic-broker");

/// A broker for one test, live or containerised.
///
/// Dropping this stops any container it started.
pub struct Broker {
    brokers: String,
    container: Option<ContainerAsync<Kafka>>,
}

impl Broker {
    /// Resolve a broker, preferring a live one.
    ///
    /// `test` names the calling test and goes into the container name, so
    /// concurrent tests do not collide. Returns `None` when there is neither a
    /// live broker nor a working container runtime.
    pub async fn ensure(test: &str) -> Option<Self> {
        if let Some(live) = live_brokers() {
            eprintln!("using live Kafka at {live}");
            return Some(Self {
                brokers: live,
                container: None,
            });
        }

        match Self::spawn(test).await {
            Ok(broker) => {
                eprintln!("spawned Kafka container at {}", broker.brokers);
                Some(broker)
            }
            Err(e) => {
                eprintln!("could not spawn a Kafka container: {e}");
                None
            }
        }
    }

    async fn spawn(test: &str) -> Result<Self, String> {
        use testcontainers::ImageExt;
        use testcontainers::runners::AsyncRunner;
        use testcontainers_modules::kafka::apache::KAFKA_PORT;

        // Pinned here rather than left to the module default: a tag baked into
        // a dependency's source is invisible to dependency review. The org
        // Renovate preset caps this image at the version Strimzi runs, so the
        // fixture cannot pass on a broker production cannot deploy.
        // renovate: datasource=docker depName=apache/kafka-native
        const KAFKA_TAG: &str = "4.2.0";

        let name = container_name(test);
        reap_stale(&name);

        let container = Kafka::default()
            .with_tag(KAFKA_TAG)
            .with_container_name(&name)
            .with_labels(labels())
            .start()
            .await
            .map_err(|e| format!("start: {e}"))?;

        let host = container
            .get_host()
            .await
            .map_err(|e| format!("host: {e}"))?;
        let port = container
            .get_host_port_ipv4(KAFKA_PORT)
            .await
            .map_err(|e| format!("port: {e}"))?;

        Ok(Self {
            brokers: format!("{host}:{port}"),
            container: Some(container),
        })
    }

    /// Broker list, as the config wants it.
    pub fn list(&self) -> Vec<String> {
        self.brokers.split(',').map(String::from).collect()
    }

    /// Whether this test started a container rather than using a live broker.
    pub const fn is_container(&self) -> bool {
        self.container.is_some()
    }

    /// A producer for `topic` on this broker.
    pub async fn producer(&self, client: &str) -> KafkaTransport {
        KafkaTransport::new(&KafkaConfig {
            profile: KafkaProfile::DevTest,
            brokers: self.list(),
            group: format!("{client}-producer"),
            client_id: client.to_string(),
            ..KafkaConfig::devtest()
        })
        .await
        .expect("producer connects")
    }

    /// A consumer subscribed to `topic`, reading from the beginning.
    pub async fn consumer(&self, client: &str, topic: &str, group: &str) -> KafkaTransport {
        KafkaTransport::new(&KafkaConfig {
            profile: KafkaProfile::DevTest,
            brokers: self.list(),
            group: group.to_string(),
            client_id: client.to_string(),
            topics: vec![topic.to_string()],
            auto_offset_reset: "earliest".to_string(),
            ..KafkaConfig::devtest()
        })
        .await
        .expect("consumer connects")
    }
}

/// A live broker from `KAFKA_BROKERS`, if it answers a TCP connect.
fn live_brokers() -> Option<String> {
    use std::net::ToSocketAddrs;

    let brokers = std::env::var("KAFKA_BROKERS").ok()?;
    let first = brokers.split(',').next().unwrap_or(&brokers);
    let addr = first.to_socket_addrs().ok()?.next()?;
    std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(3))
        .ok()
        .map(|_| brokers)
}

/// Container name: which repo, which suite, which test.
///
/// testcontainers defaults to a random hex name, which is untraceable the
/// moment one survives a killed run.
fn container_name(test: &str) -> String {
    let slug: String = test
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    format!("dfe-transform-elastic-test-broker-{slug}")
}

fn labels() -> Vec<(String, String)> {
    vec![
        (
            TEST_SUITE_LABEL.0.to_string(),
            TEST_SUITE_LABEL.1.to_string(),
        ),
        (
            "io.hyperi.test.repo".to_string(),
            "dfe-transform-elastic".to_string(),
        ),
        ("io.hyperi.test.service".to_string(), "kafka".to_string()),
        (
            "io.hyperi.test.owner-pid".to_string(),
            std::process::id().to_string(),
        ),
    ]
}

/// Remove a DEAD container holding `name`, so a leak from a killed run cannot
/// block this one.
///
/// A RUNNING container is left alone: two concurrent runs share these names,
/// and force-removing a live one would sabotage the other run. The start then
/// fails with "name is already in use", which says what actually happened.
fn reap_stale(name: &str) {
    let running = std::process::Command::new("docker")
        .args(["ps", "--quiet", "--filter", &format!("name=^{name}$")])
        .output();
    if let Ok(out) = &running
        && !out.stdout.is_empty()
    {
        return;
    }
    let _ = std::process::Command::new("docker")
        .args(["rm", "--force", "--volumes", name])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

/// A topic name unique to this run.
pub fn topic(suffix: &str) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("dfe-transform-elastic-test-{suffix}-{ts}")
}

/// Fail rather than skip when CI has neither a live broker nor a runtime.
///
/// CI is not promised an external Kafka, but it does provide a container
/// runtime, so one of the two should always be there. Finding neither means
/// the test would pass vacuously.
pub fn require_broker_in_ci() {
    assert!(
        std::env::var_os("CI").is_none(),
        "no live broker and no container runtime in CI -- this test must RUN \
         here, not skip. Skipping would report green while testing nothing."
    );
}

/// Resolve a broker or return from the test.
#[macro_export]
macro_rules! broker_or_skip {
    ($test:expr) => {{
        match $crate::common::Broker::ensure($test).await {
            Some(broker) => broker,
            None => {
                $crate::common::require_broker_in_ci();
                eprintln!("SKIP: no live Kafka and no container runtime.");
                return;
            }
        }
    }};
}
