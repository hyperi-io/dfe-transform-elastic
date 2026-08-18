# dfe-transform-elastic

Beats and Elastic Agent JSON in, DFE-normalised events out.

## Overview

dfe-transform-elastic is a Kafka-to-Kafka transform service. It consumes JSON
event batches produced by filebeat, winlogbeat, metricbeat, auditbeat,
heartbeat, packetbeat or Elastic Agent, applies the Elastic ingest-pipeline
logic for that source, and produces normalised events downstream.

The transform logic is **compiled in**. There is no interpreter, no scripting
VM and no plugin system: each supported source is a Rust module, and the
service resolves one of them by name at startup. That is the whole design
decision, and everything else follows from it -- the container is just the
binary, the hot path has no dynamic dispatch per event, and adding a source
means a release rather than a config change.

It is built on the [scalo](https://github.com/hyperi-io/scalo-rs) data-plane
runtime: config cascade, logging, metrics, Kafka transport, health probes,
memory guard and scaling pressure.

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
pipeline_name: my-pipeline

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

A batch is split into as many records as `max_message_bytes` allows -- 20,000
events do not fit in one. Keep it below your broker's `message.max.bytes`.

A config naming a source this build does not carry is rejected at startup, not
discovered at the first batch.

### Input shape

Events arrive as NDJSON, one JSON object per line, and each object must be
**Beats-shaped**: the raw vendor payload is a STRING in `message`, which is how
filebeat and Elastic Agent deliver it.

```json
{"message": "{\"actor\":{\"displayName\":\"...\"},\"eventType\":\"user.session.end\"}"}
```

Handing the service a bare vendor object instead is the one failure mode that
is not loud. The transform still runs and still emits, but almost nothing is
renamed, because every processor after the first reads fields that only exist
once `message` has been unpacked. If the output looks like the input with an
`ecs.version` bolted on, this is why.

### Envelopes: the same pipeline, a different wrapper

A device that emits over syslog can be fed from
[dfe-receiver](https://github.com/hyperi-io/dfe-receiver) instead of from
Beats. It is the same transform, the same output; only the wrapper differs.

```yaml
source:
  name: filebeat.fortinet.default
  envelope: syslog          # beats (default) | syslog
  topics: ["logs_syslog_land"]
```

`envelope: syslog` reads the receiver's syslog JSON: the MSG body in
`message`, the header parsed into siblings. Unwrapping it puts back whatever
that pipeline groks, and lifts the parsed header onto `log.syslog.*` so it
survives regardless.

The two families want different things, and the registry records which is
which:

- **Body** — `panw.*` and `cisco_meraki` read `message` as CSV or key-value.
  The receiver's body goes through untouched; a prefixed header would corrupt
  the first field.
- **Line** — `fortinet`, `cisco_ios` and `cisco_nexus` grok the header out of
  `message`, so a line is put back. If the receiver supplied `_raw` that is
  used verbatim; otherwise an RFC 3164 line is rebuilt from the parsed fields.

Reconstruction is enough for `fortinet`, which only needs `<PRI>`. It is NOT
enough for `cisco_ios` (wants a source IP) or `cisco_nexus` (wants a sequence
number) — the receiver keeps neither, so those two need `_raw`.

The envelope applies only to sources a device can actually emit. Asking for
`envelope: syslog` on an API source such as okta is rejected at startup, with
the list of sources that would work.

The metrics and probe listener is not configured here. scalo's `--metrics-addr`
(env `METRICS_ADDR`, default `0.0.0.0:9090`) is the single source of truth, so
charts and deployments override that rather than a YAML field.

### What reloads and what does not

`source.batch_size` and the `scaling` section take effect on the next batch.

Everything else needs a restart: broker and topic settings, because the Kafka
connections are established at startup; `source.name`, because the transform is
resolved once rather than per event; and `pipeline_name`, because the metrics
labels are set at startup.

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

Non-English text is a first-class case, not an edge case. `tests/unicode.rs`
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

The committed `Dockerfile` and the artefacts under `docs/` are pinned against a
fresh regen by tests, so they cannot drift from the contract.

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

## Licence

BUSL-1.1. See [LICENSE](LICENSE), and [COMMERCIAL.md](COMMERCIAL.md) for
commercial terms.
