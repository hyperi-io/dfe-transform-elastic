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

/// The host ports a test broker may publish on: fixed, below 10240, because a
/// port Docker picks comes from the ephemeral range that other stacks on a
/// shared host publish fixed ports in.
const HOST_PORTS: std::ops::Range<u16> = 9400..10240;

/// How many free ports a container start tries before giving up, since a port
/// free when probed can be taken by a concurrent run before Docker binds it.
const PORT_ATTEMPTS: usize = 8;

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

        match Self::spawn(test, &[]).await {
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

    /// Start a container for one test, never a live broker: for a test that
    /// stops the broker, which it must own.
    pub async fn container(test: &str) -> Option<Self> {
        Self::container_with(test, &[]).await
    }

    /// A container whose broker takes records up to `ceiling` bytes, never a
    /// live broker: a stock broker refuses anything over about 1 MB.
    pub async fn container_with_ceiling(test: &str, ceiling: usize) -> Option<Self> {
        let ceiling = ceiling.to_string();
        Self::container_with(test, &[("KAFKA_MESSAGE_MAX_BYTES", ceiling.as_str())]).await
    }

    async fn container_with(test: &str, env: &[(&str, &str)]) -> Option<Self> {
        match Self::spawn(test, env).await {
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

    async fn spawn(test: &str, env: &[(&str, &str)]) -> Result<Self, String> {
        let name = container_name(test);
        let mut last_err = String::from("no free host port below 10240");
        for port in free_host_ports().take(PORT_ATTEMPTS) {
            reap_stale(&name);
            match start_on(&name, port, env).await {
                Ok(container) => {
                    let host = container
                        .get_host()
                        .await
                        .map_err(|e| format!("host: {e}"))?;
                    return Ok(Self {
                        brokers: format!("{host}:{port}"),
                        container: Some(container),
                    });
                }
                Err(e) => last_err = e,
            }
        }
        Err(last_err)
    }

    /// Stop the broker this test started, leaving its data and its port.
    pub async fn stop(&self) {
        let container = self.container.as_ref().expect("stop needs a container");
        container
            .stop_with_timeout(Some(10))
            .await
            .expect("the broker stops");
    }

    /// Start the broker again after [`stop`](Self::stop), on the same port.
    pub async fn start(&self) {
        let container = self.container.as_ref().expect("start needs a container");
        container.start().await.expect("the broker starts again");
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

/// Start the broker container `name`, published on host `port`, with `env` set
/// on it as broker properties.
async fn start_on(
    name: &str,
    port: u16,
    env: &[(&str, &str)],
) -> Result<ContainerAsync<Kafka>, String> {
    use testcontainers::ImageExt;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::kafka::apache::KAFKA_PORT;

    // Pinned here rather than left to the module default: a tag baked into
    // a dependency's source is invisible to dependency review. The JVM image,
    // because `apache/kafka-native` before 4.4.0 segfaults in `getpwuid` on
    // about 2% of starts.
    // renovate: datasource=docker depName=apache/kafka
    const KAFKA_TAG: &str = "4.3.1";
    // Digest of `KAFKA_TAG`, apart from it because the Renovate regex stops at a colon.
    const KAFKA_DIGEST: &str =
        "sha256:77e3df9054047a88b520d0cc46e16696d3b22022e1d580aeccd2632df6532837";
    // A JVM broker takes 5-12 s to become ready, longer on a busy runner.
    const KAFKA_STARTUP_TIMEOUT: Duration = Duration::from_secs(180);

    let mut image = Kafka::default()
        .with_jvm_image()
        .with_tag(format!("{KAFKA_TAG}@{KAFKA_DIGEST}"))
        .with_container_name(name)
        .with_labels(labels())
        .with_startup_timeout(KAFKA_STARTUP_TIMEOUT)
        .with_mapped_port(port, KAFKA_PORT);
    for (key, value) in env {
        image = image.with_env_var(*key, *value);
    }
    image
        .start()
        .await
        .map_err(|e| format!("start on port {port}: {e}"))
}

/// Host ports in [`HOST_PORTS`] that nothing is listening on, starting at an
/// offset taken from the process id so concurrent runs start apart.
fn free_host_ports() -> impl Iterator<Item = u16> {
    let span = HOST_PORTS.end - HOST_PORTS.start;
    let offset = u16::try_from(std::process::id() % u32::from(span)).unwrap_or(0);
    (0..span)
        .map(move |i| HOST_PORTS.start + (offset + i) % span)
        .filter(|port| std::net::TcpListener::bind(("0.0.0.0", *port)).is_ok())
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
