# dfe-transform-elastic

Beats and Elastic Agent JSON in, DFE-normalised events out.

## Overview

dfe-transform-elastic is a batch transform service. It consumes JSON event
batches produced by filebeat, winlogbeat, metricbeat, auditbeat, heartbeat,
packetbeat or Elastic Agent, from Kafka or over a direct gRPC push, applies the
Elastic ingest-pipeline logic for that source, and produces normalised events
downstream the same way.

The transform logic is **compiled in**. There is no interpreter, no scripting
VM and no plugin system: each supported source is a Rust module, and the
service resolves one of them by name at startup. That is the whole design
decision, and everything else follows from it -- the container is just the
binary, the hot path has no dynamic dispatch per event, and adding a source
means a release rather than a config change.

It is built on the [scalo](https://github.com/hyperi-io/scalo-rs) data-plane
runtime: config cascade, logging, metrics, the Kafka and gRPC transports, health
probes, memory guard and scaling pressure.

**Sibling services.** `dfe-transform-vrl` runs user-supplied VRL programs and
is the right choice when the transform must be changed without a release. This
service is the right choice when the transform is a known Elastic pipeline and
throughput matters.

## Quick start

```bash
cargo build --release

./target/release/dfe-transform-elastic --config config.yaml
```

List the sources a build can transform:

```bash
./target/release/dfe-transform-elastic sources
```

## Configuration

Loaded from an explicit `--config` path, or from scalo's config cascade under
the `DFE_TRANSFORM_ELASTIC` environment prefix.

```yaml
source:
  name: filebeat.okta.default    # one of `sources` above
  brokers: ["kafka:9092"]
  topics: ["raw_events"]
  group_id: dfe-transform-elastic-my-pipeline
  batch_size: 20000

sink:
  topic: normalised_events
  brokers: ["kafka:9092"]        # defaults to the source brokers
  max_message_bytes: 900000      # ceiling on one produced record
```

That is the shape, not the whole surface. `config.example.yaml` is the COMPLETE
set of defaults with every key commented, generated from the deployment contract
and pinned against drift by a test -- read it rather than this snippet when you
need a key that is not here. `dfe-transform-elastic emit-config` reprints it.

Every transformed event goes out as its OWN record, because dfe-loader parses
one JSON document per message. `max_message_bytes` bounds each one, so keep it
below your broker's `message.max.bytes`.

A config naming a source this build does not carry is rejected at startup, not
discovered at the first batch.

Events arrive as NDJSON, one JSON object per line, wrapped by one of three
producers: Beats and Elastic Agent (`beats`), dfe-receiver (`receiver`) or
dfe-fetcher (`fetcher`). `source.envelope` defaults to `auto`, which reads the
family off each event, and naming one pins it. The producer's own field names
are stripped once they have been lifted onto ECS, except the one dfe-loader
routes on -- `_source` from the receiver and `_source_fetcher` from the fetcher
come through unchanged. What each family carries, and
what a payload with no marker does, are in
[docs/architecture.md](docs/architecture.md#envelopes-the-same-pipeline-a-different-wrapper).

Each side also carries a `transport`: `bus`, the default, is Kafka, and
`direct` is a scalo Push listener inbound and a gRPC push outbound. Where the
values come from, which env spelling reaches a `--config` file, and which
sections that file cannot carry are in
[docs/configuration.md](docs/configuration.md).

## Behaviour under bad input

The service reads whatever is on the topic, so its failure modes are stated
rather than assumed:

- **Invalid UTF-8** is decoded with U+FFFD replacements, matching what Beats
  itself substitutes for a file it cannot decode. The payload survives; the
  substitution is counted on `lossy_payloads_total`.
- **A line that is not valid JSON** is skipped and counted on
  `parse_errors_total`. The rest of the payload is unaffected.
- **An event whose transform errors** is counted on `events_errored_total` and
  left out of the output. The batch continues.
- **An event too large for one Kafka record** is dropped and counted on
  `events_oversize_total`. No broker would accept it, and retrying it forever
  would block the partition behind it.
- **A backpressured sink** is retried with a bounded backoff, counted on
  `send_backpressure_total`. Backpressure is not delivery.
- **A send that cannot be completed** STOPS the service with the batch
  uncommitted, and the restarted consumer replays it. Carrying on would let the
  next batch's commit acknowledge the failed one, because Kafka commits are
  cumulative -- that is loss, not replay. Delivery is at-least-once, so a
  downstream consumer must be idempotent.

Non-English text is a tested case, not an edge case. `tests/unicode.rs`
runs every registered transform against seventeen scripts and a set of
degenerate inputs.

## Deployment

The container image, Helm chart, compose fragment and KEDA scaler are all
generated from one deployment contract in `src/deployment.rs`:

```bash
dfe-transform-elastic emit-dockerfile > Dockerfile
dfe-transform-elastic emit-chart chart/dfe-transform-elastic
dfe-transform-elastic emit-compose
dfe-transform-elastic generate-artefacts --output-dir docs
```

`generate-artefacts` writes into `docs/`: `metrics-manifest.json`,
`deployment-contract.json`, `container-manifest.json`, `Dockerfile.runtime`,
`argocd-application.yaml`, and the reflectable config pair `config-schema.*` and
`capability-catalog.*`.

**Not every artefact is pinned against a fresh regen.** `committed_config_artefacts_do_not_drift` in `src/deployment.rs` compares `config-schema.{json,yaml}` and `capability-catalog.{json,yaml}`, and its sibling tests do the same for the committed `deployment-contract.json`, `Dockerfile`, `config.example.yaml` and the chart's `config:` block. Nothing compares `container-manifest.json`, `Dockerfile.runtime` or `argocd-application.yaml`, so those can and do fall behind -- re-run the command when `src/deployment.rs` changes rather than assuming a test caught it.

`metrics-manifest.json` is compared by metric NAME SET in `src/metrics.rs`, not byte for byte, because it also records the version and commit of the build that wrote it.

The image expects the release binary in the build context:

```bash
cargo build --release
cp target/release/dfe-transform-elastic .
docker build -t dfe-transform-elastic .
```

## Observability

- `/livez`, `/readyz`, `/metrics` and `/metrics/manifest`, all served from the
  one listener on port 9090. There is no second health port.
- `/scaling/pressure` serves a single weighted figure to KEDA: consumer lag at
  0.70, batch saturation at 0.30, with memory as a hard gate that forces
  pressure to 100 before an OOM.
- `dfe-transform-elastic metrics-manifest` prints the full metric catalogue
  without starting the service. The committed copy is `docs/metrics-manifest.json`.

## Development

```bash
cargo test --workspace --all-features
hyperi-ci check
```

`librdkafka` 2.12.1 or later is needed for the `kafka` feature, which is on by
default. Building without it still works:

```bash
cargo build --no-default-features
```

[docs/](docs/README.md) carries everything deeper than this page --
[architecture.md](docs/architecture.md) for the code map,
[parity.md](docs/parity.md) for what the service promises against Elastic's own
output, and [compat.md](docs/compat.md) for working a source towards it.

## Licence

BUSL-1.1. See [LICENSE](LICENSE), and [COMMERCIAL.md](COMMERCIAL.md) for
commercial terms.
