# dfe-transform-elastic

Turns Beats and Elastic Agent events into DFE-normalised events.

[![Build Status](https://github.com/hyperi-io/dfe-transform-elastic/actions/workflows/ci.yml/badge.svg)](https://github.com/hyperi-io/dfe-transform-elastic/actions)
[![License](https://img.shields.io/badge/license-BUSL--1.1-blue)](LICENSE)

## What it does

- Applies each source's Elastic ingest-pipeline logic, compiled in as a Rust module, with no interpreter or plugin on the hot path. In a DFE deployment it feeds dfe-loader, so an existing Elastic shipper lands in DFE without changing.
- Filebeat, Winlogbeat, Metricbeat, Auditbeat, Heartbeat, Packetbeat and Elastic Agent data. `dfe-transform-elastic sources` lists every source a build has, and a config naming one it lacks fails at startup.
- Events wrapped by Beats, by dfe-receiver or by dfe-fetcher, the wrapper read off each event.
- Kafka in and out, or a direct gRPC push with no broker.
- At-least-once for what it can deliver: the offset commit, or the answer to a push, is held until the sink takes the block. `source.acknowledgements.enabled: false` acknowledges at receipt instead.
- Invalid UTF-8 is decoded with replacements and counted. Non-JSON goes to the DLQ with its reason, or is dropped and counted with the DLQ off. An event that will not transform, or is too large for one Kafka record, is dropped and counted, and the batch carries on. A retryable sink refusal is retried, and with acknowledgements on nothing is committed past the block.

Use dfe-transform-vrl instead when the transform has to change without a release.

## Quick start

```bash
cargo build --release
./target/release/dfe-transform-elastic sources
./target/release/dfe-transform-elastic --config config.example.yaml
```

The smallest config names the source and the two Kafka ends:

```yaml
source:
  name: filebeat.okta.default    # one of `sources` above
  brokers: ["kafka:9092"]
  topics: ["raw_events"]
  group_id: dfe-transform-elastic-my-pipeline

sink:
  topic: normalised_events
  max_message_bytes: 8000000     # per-record ceiling, 4096 to 16 MiB
```

Health and metrics answer on the metrics listener, port 9090 by default: `/livez`, `/readyz`, `/metrics`, `/metrics/manifest`, and `/scaling/pressure` for KEDA.

## Configuration

[config.example.yaml](config.example.yaml) is rendered from the deployment contract with illustrative values for the source and sink. Set those for your deployment (`source.name`, `source.topics`, `source.brokers`, `source.group_id` and `sink.topic` on the Kafka transport). A test pins the file against drift. `dfe-transform-elastic emit-config` reprints it. Settings load from an explicit `--config` path, or from scalo's config cascade under the `DFE_TRANSFORM_ELASTIC` prefix. [docs/configuration.md](docs/configuration.md) says which env spelling reaches a `--config` file and which sections that file cannot hold.

Each transformed event goes out as a separate record, because dfe-loader parses one JSON document per message. Keep `sink.max_message_bytes` below the broker's `message.max.bytes`.

`source.envelope` defaults to `auto`, which reads the producer's wrapper (`beats`, `receiver` or `fetcher`) off each event. The producer's field names are stripped once lifted onto ECS, except two markers: the receiver's `_source`, which dfe-loader routes on, and the fetcher's `_source_fetcher`, which dfe-engine's routing rule matches.

Each side takes a `transport`: `bus` (Kafka, the default) or `direct` (a scalo Push listener in, a gRPC push out).

The source catalogue, `sources.yaml`, is compiled into the binary and `dfe-transform-elastic emit-catalogue` prints it, so a deployment takes the catalogue from the image it runs.

## Documentation

- [docs/architecture.md](docs/architecture.md): the code map, the envelopes and delivery.
- [docs/parity.md](docs/parity.md): what the service promises against Elastic's output.
- [docs/compat.md](docs/compat.md) and [docs/compat-policy.md](docs/compat-policy.md): what counts as a difference from Elastic's output, and the known limits.
- [docs/configuration.md](docs/configuration.md): the cascade, the env spellings and the sections a file cannot hold.
- [docs/enrichment.md](docs/enrichment.md): GeoIP, user agent and community ID.
- [docs/README.md](docs/README.md): every page, including parsers, performance and testing.

The suite's published docs are at <https://dfe.docs.hyperi.io>, and this repository's pages at <https://dfe.docs.hyperi.io/dfe-transform-elastic/>.

## Help and contributing

How to work in this repo, the generated artefacts, the test tiers and what tends to bite are in [CONTRIBUTING.md](CONTRIBUTING.md). A question that is not a bug goes in an issue.

Open bugs: [this repo](https://github.com/hyperi-io/dfe-transform-elastic/issues?q=is%3Aopen+is%3Aissue+type%3ABug+-label%3Abacklog+-label%3Anext-minor) | [whole DFE suite](https://github.com/search?type=issues&q=is%3Aopen+is%3Aissue+type%3ABug+-label%3Abacklog+-label%3Anext-minor+repo%3Ahyperi-io%2Fdfe-infra+repo%3Ahyperi-io%2Fdfe-docker+repo%3Ahyperi-io%2Fdfe-deploy+repo%3Ahyperi-io%2Fdfe-engine+repo%3Ahyperi-io%2Fdfe-loader+repo%3Ahyperi-io%2Fdfe-receiver+repo%3Ahyperi-io%2Fdfe-fetcher+repo%3Ahyperi-io%2Fdfe-archiver+repo%3Ahyperi-io%2Fdfe-transform-vrl+repo%3Ahyperi-io%2Fdfe-transform-vector+repo%3Ahyperi-io%2Fdfe-transform-elastic+repo%3Ahyperi-io%2Fdfe-ui+repo%3Ahyperi-io%2Fdfe-hyperdx+repo%3Ahyperi-io%2Fscalo-rs+repo%3Ahyperi-io%2Fscalo-py+repo%3Ahyperi-io%2Flogreducer+repo%3Ahyperi-io%2Fdfe-schemas+repo%3Ahyperi-io%2Fvector-vrl+repo%3Ahyperi-io%2Fculvert+repo%3Ahyperi-io%2Fdfe-clickhouse-rs+repo%3Ahyperi-io%2Ffactbook-rs)

## Licence

BUSL-1.1. See [LICENSE](LICENSE), and [COMMERCIAL.md](COMMERCIAL.md) for commercial terms.
