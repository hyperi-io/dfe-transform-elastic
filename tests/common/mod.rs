// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Broker test infrastructure.
//!
//! A live broker is used when one is reachable. Otherwise the run's one shared
//! broker, from scalo's `shared_kafka`, which the first test process starts
//! and the run removes when it ends. A test that stops its broker, or needs
//! broker settings of its own, holds one alone. A suite that silently skips
//! all of these reports green while testing nothing, so in CI the absence of
//! a broker is a failure rather than a skip.

use std::time::Duration;

use scalo::deployment::test_support::shared_service::{
    OwnedContainer, Suite, container_name, start_alone,
};
use scalo::transport::kafka::test_support::{KAFKA_PORT, docker_answers, kafka_spec, shared_kafka};
use scalo::transport::kafka::{KafkaConfig, KafkaProfile, KafkaTransport};

/// Names and labels every container this suite starts, and keeps its run log.
pub const SUITE: Suite = Suite {
    repo: "dfe-transform-elastic",
    manifest_dir: env!("CARGO_MANIFEST_DIR"),
    tmp_dir: option_env!("CARGO_TARGET_TMPDIR"),
};

/// The host ports a broker a test holds alone may publish on: fixed, below
/// 10240, because a test that restarts its broker must keep its address, and
/// a port Docker picks comes from the ephemeral range other stacks use.
const HOST_PORTS: std::ops::Range<u16> = 9400..10240;

/// Where a test's broker came from.
enum Held {
    /// `KAFKA_BROKERS`, owned by nobody here.
    Live,
    /// The run's shared broker, by container name.
    Shared(String),
    /// A broker this test holds alone, removed when it drops.
    Owned(OwnedContainer),
}

/// A broker for one test.
pub struct Broker {
    brokers: String,
    held: Held,
}

impl Broker {
    /// A live broker, else the run's shared one. `None` when there is neither
    /// a live broker nor a Docker daemon.
    pub async fn ensure(_test: &str) -> Option<Self> {
        if let Some(live) = live_brokers() {
            eprintln!("using live Kafka at {live}");
            return Some(Self {
                brokers: live,
                held: Held::Live,
            });
        }
        if !docker_answers() {
            return None;
        }
        let shared = shared_kafka(SUITE).await;
        Some(Self {
            brokers: shared.addr(KAFKA_PORT),
            held: Held::Shared(shared.name),
        })
    }

    /// A broker this test holds alone, never a live one: for a test that stops
    /// the broker.
    pub async fn container(test: &str) -> Option<Self> {
        Self::alone(test, &[]).await
    }

    /// A broker this test holds alone that takes records up to `ceiling`
    /// bytes: a stock broker refuses anything over about 1 MB.
    pub async fn container_with_ceiling(test: &str, ceiling: usize) -> Option<Self> {
        Self::alone(test, &[("KAFKA_MESSAGE_MAX_BYTES", ceiling.to_string())]).await
    }

    async fn alone(test: &str, env: &[(&str, String)]) -> Option<Self> {
        if !docker_answers() {
            return None;
        }
        let port = free_host_port()?;
        let mut spec = kafka_spec(Some(port));
        spec.env
            .extend(env.iter().map(|(k, v)| ((*k).to_string(), v.clone())));
        let name = container_name(&SUITE, Some(test), "kafka");
        let owned = tokio::task::spawn_blocking(move || start_alone(&SUITE, &spec, &name))
            .await
            .expect("the broker a test holds alone starts");
        eprintln!("started Kafka container {}", owned.container.name);
        Some(Self {
            brokers: owned.container.addr(KAFKA_PORT),
            held: Held::Owned(owned),
        })
    }

    /// Stop the broker this test holds, leaving its data and its port.
    pub async fn stop(&self) {
        self.docker("stop").await;
    }

    /// Start the broker again after [`stop`](Self::stop), on the same port.
    pub async fn start(&self) {
        self.docker("start").await;
    }

    async fn docker(&self, verb: &'static str) {
        let Held::Owned(owned) = &self.held else {
            panic!("{verb} needs a broker this test holds alone");
        };
        let name = owned.container.name.clone();
        let status = tokio::process::Command::new("docker")
            .args([verb, &name])
            .stdout(std::process::Stdio::null())
            .status()
            .await
            .expect("docker runs");
        assert!(status.success(), "docker {verb} {name} failed");
    }

    /// The name of the container behind this broker, `None` for a live one.
    pub fn container_name(&self) -> Option<String> {
        match &self.held {
            Held::Live => None,
            Held::Shared(name) => Some(name.clone()),
            Held::Owned(owned) => Some(owned.container.name.clone()),
        }
    }

    /// A producer whose librdkafka settings are overridden, for a test that
    /// needs a delivery to fail fast or a record ceiling to be low.
    pub async fn producer_with(&self, client: &str, overrides: &[(&str, &str)]) -> KafkaTransport {
        let config = KafkaConfig {
            profile: KafkaProfile::DevTest,
            brokers: self.list(),
            group: format!("{client}-producer"),
            client_id: client.to_string(),
            ..KafkaConfig::devtest()
        }
        .with_overrides(overrides);
        KafkaTransport::new(&config)
            .await
            .expect("producer connects")
    }

    /// Broker list, as the config wants it.
    pub fn list(&self) -> Vec<String> {
        self.brokers.split(',').map(String::from).collect()
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

/// A host port in [`HOST_PORTS`] that nothing is listening on, starting at an
/// offset taken from the process id so concurrent runs start apart.
fn free_host_port() -> Option<u16> {
    let span = HOST_PORTS.end - HOST_PORTS.start;
    let offset = u16::try_from(std::process::id() % u32::from(span)).unwrap_or(0);
    (0..span)
        .map(|i| HOST_PORTS.start + (offset + i) % span)
        .find(|port| std::net::TcpListener::bind(("0.0.0.0", *port)).is_ok())
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
