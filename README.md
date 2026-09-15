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

A batch is split into as many records as `max_message_bytes` allows -- 20,000
events do not fit in one. Keep it below your broker's `message.max.bytes`.

A config naming a source this build does not carry is rejected at startup, not
discovered at the first batch.

### Input shape

Events arrive as NDJSON, one JSON object per line, wrapped by one of the three
producers below. Whichever it is, the transform sees the same thing once the
wrapper is off: the raw vendor payload as a STRING in `message`, which is how
filebeat and Elastic Agent deliver it and what every transform is written for.

```json
{"message": "{\"actor\":{\"displayName\":\"...\"},\"eventType\":\"user.session.end\"}"}
```

Handing the service a bare vendor object carrying no producer marker is the one
failure mode that is not loud. Detection finds nothing to recognise, falls back
to `beats`, and there is no `message` to unpack -- so the transform still runs
and still emits, but almost nothing is renamed, because every processor after
the first reads fields that only exist once `message` has been unpacked. If the
output looks like the input with an `ecs.version` bolted on, this is why.

### Envelopes: the same pipeline, a different wrapper

The same transform and the same output whatever wrapped the payload. Three
families:

- **`beats`** -- the raw vendor payload as a string in `message`, which is how
  Beats and Elastic Agent deliver it, and what the transforms are written for.
- **`receiver`** -- [dfe-receiver](https://github.com/hyperi-io/dfe-receiver)'s
  JSON, on any of its transports. The syslog arm rebuilds a line into
  `message` because that is what the vendor groks match, and lifts the parsed
  header onto `log.syslog.*` so it survives a grok that does not. The rest pass
  their payload through with their own field names moved onto the ECS paths
  those names would otherwise shadow.
- **`fetcher`** -- [dfe-fetcher](https://github.com/hyperi-io/dfe-fetcher)'s
  JSON, the provider's own payload at the top level. It applies wherever
  Elastic's agent input is a pure transport, because there the ingest pipeline
  does all the parsing.

```yaml
source:
  name: filebeat.fortinet.default
  envelope: auto            # auto (default) | beats | receiver | fetcher
  topics: ["logs_syslog_land"]
```

**`auto` reads the family off each event**, not off the batch, because one
Kafka batch spans partitions and can carry two producers' wrappers at once. It
costs a handful of top-level key checks against an unwrap measured at 2,740 ns,
so a deployment that changes producer needs no config change. Name a family to
pin it instead, which is what a shape carrying no marker needs.

Two syslog framings, recorded per source in the registry: `panw.*` and
`cisco_meraki` read `message` as CSV or key-value, so the receiver's body goes
through untouched -- a prefixed header would corrupt the first field. Everything
else groks the header out of `message`, so a line is put back: the receiver's
`_raw` verbatim if it kept one, otherwise an RFC 3164 line rebuilt from the
parsed fields. Reconstruction is enough for `fortinet`, which only needs
`<PRI>`. It is NOT enough for `cisco_ios` (wants a source IP) or `cisco_nexus`
(wants a sequence number) -- the receiver keeps neither, so those two need
`_raw`.

A pinned envelope a source cannot arrive in is rejected at startup, with the
list of sources that would work: okta is pulled from an API, so
`envelope: receiver` on it is a config error rather than a silent no-op at the
first batch.

The metrics and probe listener is not configured here. scalo's `--metrics-addr`
(env `METRICS_ADDR`, default `0.0.0.0:9090`) is the single source of truth, so
charts and deployments override that rather than a YAML field.

### Nothing reloads, and only one env spelling reaches a `--config` file

The configuration is read ONCE at startup and handed to the batch loop by
reference, so **every value needs a restart to change** -- the Kafka connections
are established at startup, the transform is resolved once rather than per
event, the GeoIP databases are loaded at startup, and the metrics labels are set
at startup.

The two ways in are not equivalent. With no `--config` the whole scalo cascade
applies. With `--config` -- which is what the container passes -- the named file
IS the configuration, because scalo cannot merge an arbitrarily-named file into
the cascade as a layer. Kafka credentials are unaffected either way: they are
read from `KAFKA_*` separately.

**Which spelling you use decides whether it reaches that file.** The FLAT,
single-underscore form is applied to the loaded configuration on both branches,
so it works against a `--config` deployment:

    DFE_TRANSFORM_ELASTIC_SOURCE_TOPICS=a,b
    DFE_TRANSFORM_ELASTIC_SOURCE_BATCH_SIZE=5000
    DFE_TRANSFORM_ELASTIC_SINK_TOPIC=out

The fields it covers are every `source.*` and every `sink.*`, including the
transport selector:

    DFE_TRANSFORM_ELASTIC_SOURCE_TRANSPORT=direct
    DFE_TRANSFORM_ELASTIC_SOURCE_LISTEN=0.0.0.0:6000
    DFE_TRANSFORM_ELASTIC_SINK_ENDPOINT=http://dfe-loader:6000

`bus` and `kafka` name the same transport, as do `direct` and `grpc`, and an
unrecognised value keeps the configured one rather than silently moving the
deployment. **`direct` is declared and validated but nothing constructs it yet**
(issue #19), so today every deployment runs on the bus.

`geoip` is not among them -- it is scalo's own type, so the orphan rule puts it
out of reach, and it stays settable from the file and the cascade. scalo's
DOUBLE-underscore form is resolved from the cascade instead, which a named file
is not a layer of, so it reaches such a deployment never.

One consequence to know before tuning it: a section scalo resolves for itself
cannot be set from a `--config` file at all. `scaling` is the one that bites.
The service WARNS for each such section rather than refusing the file, naming it
and the `DFE_TRANSFORM_ELASTIC_SCALING__<KEY>` form that does reach it --
refusing would let a surplus key stop the pod, and that file is rendered from
what dfe-engine publishes rather than written here. `CASCADE_ONLY_SECTIONS` in
`src/config.rs` is the full list and nothing this repo ships carries one.
`geoip` is deliberately absent: the service declares that section itself and
hands it to scalo, which is what makes it work from a file.

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

`generate-artefacts` writes into `docs/`: `metrics-manifest.json`,
`deployment-contract.json`, `container-manifest.json`, `Dockerfile.runtime`,
`argocd-application.yaml`, and the reflectable config pair `config-schema.*` and
`capability-catalog.*`.

**Only the config pair is pinned against a fresh regen.**
`committed_config_artefacts_do_not_drift` in `src/deployment.rs` compares
`config-schema.{json,yaml}` and `capability-catalog.{json,yaml}`, and its
sibling tests do the same for the committed `Dockerfile`, `config.example.yaml`
and the chart's `config:` block. Nothing compares `deployment-contract.json`,
`container-manifest.json`, `Dockerfile.runtime` or `argocd-application.yaml`, so
those can and do fall behind -- re-run the command when `src/deployment.rs`
changes rather than assuming a test caught it.

`metrics-manifest.json` is the one that could not be drift-tested as it stands:
it carries a `registered_at` timestamp written at generation time, so a
byte-comparison would fail on every run that did not regenerate it. Expect that
line to change whenever the command is run, and ignore it in review.

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

## Documentation

[docs/](docs/README.md) carries everything deeper than this page --
[architecture.md](docs/architecture.md) for the code map,
[parity.md](docs/parity.md) for what the service promises against Elastic's own
output, and [compat.md](docs/compat.md) for working a source towards it.

## Licence

BUSL-1.1. See [LICENSE](LICENSE), and [COMMERCIAL.md](COMMERCIAL.md) for
commercial terms.
