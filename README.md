# dfe-transform-elastic

[![Build Status](https://github.com/hyperi-io/dfe-transform-elastic/actions/workflows/ci.yml/badge.svg)](https://github.com/hyperi-io/dfe-transform-elastic/actions)
[![License](https://img.shields.io/badge/license-BUSL--1.1-blue)](LICENSE)

Turns Beats and Elastic Agent events into DFE-normalised events, applying the Elastic ingest-pipeline logic for each source. In a DFE deployment it feeds dfe-loader, so data from an existing Elastic shipper lands in DFE without the shipper changing.

## What it gives you

- Filebeat, Winlogbeat, Metricbeat, Auditbeat, Heartbeat, Packetbeat and Elastic Agent data. `dfe-transform-elastic sources` lists every source a build carries.
- Each source's pipeline compiled in as a Rust module, with no interpreter or plugin system on the hot path.
- Events wrapped by Beats, by dfe-receiver or by dfe-fetcher, the wrapper read off each event.
- Kafka in and out, or a direct gRPC push with no broker.
- A config that names a source the build does not carry is refused at startup, not at the first batch.
- Stated failure modes for bad input. Invalid UTF-8 is decoded with replacements and counted. Non-JSON, an event that will not transform and an oversize event are dropped and counted, and the batch carries on.
- At-least-once delivery for what it can deliver: by default the offset commit, or the answer to a push, is held until the sink takes the block. `source.acknowledgements.enabled: false` acknowledges at receipt instead.

Use dfe-transform-vrl instead when the transform has to change without a release.

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
  max_message_bytes: 8000000     # ceiling on one outbound record (the DFE record cap)
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

JSON is the only payload format. MessagePack, supported in DFE/XDR 2.0 and 2.1, is deprecated in DFE 2.2 and no longer accepted: the JSON path (SIMD parsing with sonic-rs, zstd on the wire) is fast enough that MessagePack gave no CPU saving.

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
- **Input that is not JSON** is refused and dead-lettered with the reason `payload is not JSON`, and counted on `parse_errors_total`. A record no line of which parses -- MessagePack, or any other binary -- is refused whole, with the bytes the producer sent. In a record that is otherwise JSON, only the bad line is refused and the events beside it still go. This service configures no DLQ, so the dead letter is dropped and counted on `pipeline_dead_letters_dropped_total{reason="dead_letter"}`, and its record's source is released.
- **An event whose transform errors** is counted on `events_errored_total` and
  left out of the output. The batch continues.
- **An event too large for one Kafka record** is dropped and counted on
  `events_oversize_total`. No broker would accept it, and retrying it forever
  would block the partition behind it.
- **A sink that refuses for now** -- a full producer queue, a broker outage -- is retried for as long as it refuses, counted on `send_backpressure_total`. The Kafka offset commit, or the answer to a push, is held until the sink takes the block, so an outage is waited out and nothing after the block is committed past it.
- **A send no retry can fix** STOPS the service with the block unreleased, and the restarted consumer reads it again. Delivery is at-least-once, so a downstream consumer must be idempotent.
- **A record the sink transport would refuse** is dropped before the send and counted on `pipeline_dead_letters_dropped_total`, never counted delivered.

Non-English text is a tested case, not an edge case. `tests/unicode.rs`
runs every registered transform against seventeen scripts and a set of
degenerate inputs.

## Deployment

The container image, compose fragment and released chart are all generated
from one deployment contract in `src/deployment.rs`:

```bash
dfe-transform-elastic emit-dockerfile > Dockerfile
dfe-transform-elastic emit-compose
dfe-transform-elastic generate-artefacts --output-dir docs
```

`generate-artefacts` writes into `docs/`: `metrics-manifest.json`,
`deployment-contract.json`, `container-manifest.json`, `Dockerfile.runtime`,
and the reflectable config pair `config-schema.*` and `capability-catalog.*`.

The deployed chart is not committed here. At release, hyperi-ci runs `generate-artefacts` for the deployment contract and assembles a thin chart from it on the scalo-service library chart, at the version `release.helm.library` names in `.hyperi-ci.yaml`. Keep that version at the scalo version in `Cargo.toml`: a library renders only the contract version its scalo release writes.

**Not every artefact is pinned against a fresh regen.** `committed_config_artefacts_do_not_drift` in `src/deployment.rs` compares `config-schema.{json,yaml}` and `capability-catalog.{json,yaml}`, and its sibling tests do the same for the committed `deployment-contract.json`, `Dockerfile` and `config.example.yaml`. Nothing compares `container-manifest.json` or `Dockerfile.runtime`, so those can and do fall behind -- re-run the command when `src/deployment.rs` changes rather than assuming a test caught it.

`metrics-manifest.json` is compared by metric NAME SET in `src/metrics.rs`, not byte for byte, because it also records the version and commit of the build that wrote it.

The image expects the release binary in the build context:

```bash
cargo build --release
cp target/release/dfe-transform-elastic .
docker build -t dfe-transform-elastic .
```

The source catalogue, `sources.yaml`, is compiled into the binary. `dfe-transform-elastic emit-catalogue` prints it, so a deployment takes the catalogue dfe-engine offers from the image it already pulls, with no credential for this repo.

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

## Issues

Open bugs: [this repo](https://github.com/hyperi-io/dfe-transform-elastic/issues?q=is%3Aopen+is%3Aissue+type%3ABug+-label%3Abacklog+-label%3Anext-minor) | [whole DFE suite](https://github.com/search?type=issues&q=is%3Aopen+is%3Aissue+type%3ABug+-label%3Abacklog+-label%3Anext-minor+repo%3Ahyperi-io%2Fdfe-infra+repo%3Ahyperi-io%2Fdfe-docker+repo%3Ahyperi-io%2Fdfe-deploy+repo%3Ahyperi-io%2Fdfe-engine+repo%3Ahyperi-io%2Fdfe-loader+repo%3Ahyperi-io%2Fdfe-receiver+repo%3Ahyperi-io%2Fdfe-fetcher+repo%3Ahyperi-io%2Fdfe-archiver+repo%3Ahyperi-io%2Fdfe-transform-vrl+repo%3Ahyperi-io%2Fdfe-transform-vector+repo%3Ahyperi-io%2Fdfe-transform-elastic+repo%3Ahyperi-io%2Fdfe-ui+repo%3Ahyperi-io%2Fdfe-hyperdx+repo%3Ahyperi-io%2Fscalo-rs+repo%3Ahyperi-io%2Fscalo-py+repo%3Ahyperi-io%2Flogreducer+repo%3Ahyperi-io%2Fdfe-schemas+repo%3Ahyperi-io%2Fvector-vrl+repo%3Ahyperi-io%2Fculvert+repo%3Ahyperi-io%2Fdfe-clickhouse-rs+repo%3Ahyperi-io%2Ffactbook-rs)

## Licence

BUSL-1.1. See [LICENSE](LICENSE), and [COMMERCIAL.md](COMMERCIAL.md) for
commercial terms.
