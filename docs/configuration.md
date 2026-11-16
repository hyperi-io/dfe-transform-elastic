# Configuration

Where the values come from, which env spelling reaches a `--config` file, how the transport is selected, and which sections the file cannot hold. The field-by-field reference is generated: `config-schema.yaml` beside this page lists every key with its description, and `config.example.yaml` at the repo root is the deployment contract's configuration, with illustrative source and sink values, pinned against drift by a test. `dfe-transform-elastic emit-config` reprints it.

## The configuration is read once, at startup

`Config::load` reads the configuration once and hands it to the batch loop by reference, so every value needs a restart to change. The broker connections, the resolved transform, the GeoIP databases and the metrics labels are fixed at startup.

The two ways in are not equivalent. With no `--config` the scalo cascade applies: `defaults`, `settings` and `settings.<env>` files, `.yaml` or `.yml`, searched in `./`, `./config/` and `/config/`, then the `DFE_TRANSFORM_ELASTIC_*` environment. `~/.config/dfe-transform-elastic/` is searched when `APP_NAME=dfe-transform-elastic` is set. With `--config`, which is what the container passes, `Config::load` reads the named file into `Config` and hands it to scalo's cascade as its settings layer, so the scalo sections written in it apply. Kafka credentials come from `KAFKA_*` either way.

The flat, single-underscore env form is applied to the loaded configuration on both paths, so it is the form that works against a `--config` deployment:

    DFE_TRANSFORM_ELASTIC_SOURCE_TOPICS=a,b
    DFE_TRANSFORM_ELASTIC_SOURCE_BATCH_SIZE=5000
    DFE_TRANSFORM_ELASTIC_SINK_TOPIC=out

It covers every `source.*` and `sink.*` field. `geoip` is scalo's type, so it takes no flat form and is set from the file or the cascade. scalo's double-underscore form is a cascade key, and a `--config` deployment reads `source.*` and `sink.*` from the file, so that form never reaches them.

## `transport` selects the bus or a direct push, per side

Each side has a `transport` of `bus` (the default) or `direct`. The file and the flat env form take `kafka` for `bus` and `grpc` for `direct` as well, and a config always writes `bus` and `direct` back. The file rejects any other name, and an unrecognised env value keeps the configured transport. The chart publishes push port 6000 when `source.transport` is `direct` or `grpc`.

On `direct` the service binds a scalo Push listener on `source.listen` (default `0.0.0.0:6000`) and pushes to `sink.endpoint` (default `http://dfe-loader:6000`). The listener answers a push once its events are delivered, so a rejected push is the sender's to retry.

    DFE_TRANSFORM_ELASTIC_SOURCE_TRANSPORT=direct
    DFE_TRANSFORM_ELASTIC_SOURCE_LISTEN=0.0.0.0:6000
    DFE_TRANSFORM_ELASTIC_SINK_ENDPOINT=http://dfe-loader:6000

Every `source` and `sink` key may be left out. `source.name`, `source.topics`, `source.group_id`, `source.brokers` and `sink.topic` then default to empty, and an instance with no source name idles, as does a bus instance with no topics. Once the configuration names work, startup rejects a bus source with no brokers or group. On `direct` the brokers and group may stay empty, and empty topics do not idle, because the listener is the work. In a DFE deployment the slim and mesh profiles run both sides on `direct`, and single and scale run on `bus`.

`sink.topic` is required on both transports once the configuration names work. On `direct` it is the routing key every push has, and dfe-loader routes a record with no `_source` on that key, so a push without one lands under the loader's `default_topic`. The listener speaks plaintext gRPC with no authentication, and the push to the endpoint is unencrypted. Both belong behind the mesh route dfe-infra provisions, never on an interface reachable from outside the cluster.

## Agentless mode serves one syslog stream of many device types

`source.mode: agentless` reads the source off each event instead of from `source.name`, so one pod can consume a dfe-receiver syslog topic with firewalls, switches and proxies on it together. Each event is matched against a table of markers (literals, key sets and CEF `Device Vendor` and `Device Product` pairs), the winner's transform runs, and the output is stamped `_source` with that source's dfe-schemas catalogue label, which dfe-loader routes on. The markers are each source's `catalogue.detect` block in dfe-schemas, compiled into `src/detect/markers.rs` from the release `scripts/dfe-schemas.toml` pins. `source.name` must stay empty, and `source.envelope` must be `auto` or `receiver`.

- `source.mode`: `agent` (default) or `agentless`.
- `source.detect.unmatched`: what an event no marker claims gets. `passthrough` (default) unwraps it to the Beats layout, runs no transform and leaves `_source` off, so the loader lands it under `sink.topic`'s default table. `fallback` runs `source.detect.fallback`'s transform and stamps its label. `stop` ends the service with the block unreleased, so a restart reads it again.
- `source.detect.fallback`: a registered source, required when `unmatched` is `fallback` and rejected otherwise.
- `source.detect.candidates`: the sources the detector may pick. Empty means all of them, and an unknown name fails startup. Narrowing it hands shared lines to the source left in: ASA-format lines go to ASA, and to the IOS catch-all once ASA is out. Citrix CEF goes to the ADC source unless it is excluded, and then to WAF.

    DFE_TRANSFORM_ELASTIC_SOURCE_MODE=agentless
    DFE_TRANSFORM_ELASTIC_SOURCE_DETECT_UNMATCHED=fallback
    DFE_TRANSFORM_ELASTIC_SOURCE_DETECT_FALLBACK=filebeat.cisco_ios.default
    DFE_TRANSFORM_ELASTIC_SOURCE_DETECT_CANDIDATES=filebeat.cisco_asa.default,filebeat.cisco_ftd.default

Fetcher events hold the provider's payload, so they count as unmatched. Three counters report the split: `events_detected_total{source}`, `events_unmatched_total` and `events_fallback_total`.

Run it on the receiver's `_raw` capture where you can. The receiver splits each line with `syslog_loose`, and a marker that straddled the split (`Pleasant Password Server:`, an `amp:` tag) is gone from the parsed fields.

`scripts/replay_syslog.py` feeds an agentless stack from the committed fixtures under `tests/fixtures/unencumbered/`, standard library only. `--sources` takes catalogue labels from the table the detector stamps, and `--list` shows the ones with fixtures. A syslog header is added to a fixture line that lacks one, and the `%` the ASA fixtures dropped is put back. UDP sends one datagram per line and TCP is newline framed. There is no `http://` form, because dfe-receiver's `/ingest` takes JSON and not syslog.

    scripts/replay_syslog.py --list
    scripts/replay_syslog.py --target udp://dfe-receiver:514 --sources cisco-asa,fortinet,panw --rate 50 --loop --duration 120

## `source.acknowledgements` holds the source until delivery

`source.acknowledgements.enabled` (default `true`) holds the source's acknowledgement, the Kafka offset commit on the bus or the answer to a push on direct, until every event built from a record is delivered, or dropped and counted. A sink outage is then waited out, and a crash redelivers rather than loses. `false` acknowledges at receipt, before the transform runs, and a crash loses what was in flight. The flat env form is `DFE_TRANSFORM_ELASTIC_SOURCE_ACKNOWLEDGEMENTS_ENABLED`.

On a direct source a push is held at most 18 s, so every send it waits on gives up at 15 s: a direct sink's gRPC deadline, and a bus sink's `message.timeout.ms` (librdkafka's default is 300 s).

## `sink.max_message_bytes` caps one outbound record

The default is the DFE record cap, 8,000,000 bytes: dfe-receiver rejects a request body over it, and every broker DFE deploys takes a record that size (Confluent Basic's 8,388,608 limit leaves room for the framing). The producer this service builds allows 16 MiB, so a deployment with larger brokers can raise the budget up to that. An event that serialises larger is logged, counted on `events_oversize_total` and sent to the DLQ with its size and the budget as the reason. The DLQ cuts the body to `dlq.max_payload_bytes`, keeping `original_bytes` and a truncated head.

Against a broker or topic with a lower `message.max.bytes` (a stock Kafka broker takes about 1 MB), set `sink.max_message_bytes` below that limit. Left at the default, the broker rejects the events between the two: each is dead-lettered by the sink transport, counted on `send_filtered_dlq_total`, and the partition moves on. Validation rejects a value under 4096 or over 16,777,216.

    DFE_TRANSFORM_ELASTIC_SINK_MAX_MESSAGE_BYTES=900000

## A record that will not parse goes to the DLQ, one that will not transform is counted

A record that is not JSON, and an event the sink transport would reject, go to the DLQ `dlq` configures: by default NDJSON files under `/var/spool/scalo/dlq`, on the `dlq` volume the chart mounts at 1Gi. The source is acknowledged once the DLQ holds them, so a full or broken DLQ holds the block rather than losing it. `dlq` is scalo's `DlqConfig`, so `mode`, `file.rotation` and `file.max_age_days` are set there. With `dlq.enabled: false` they are dropped and counted on `pipeline_dead_letters_dropped_total`.

Invalid UTF-8 is decoded with U+FFFD replacements, as Beats does for a file it cannot decode, and counted on `lossy_payloads_total`. Input that is not JSON is counted on `parse_errors_total`. An event that fails to transform, or will not serialise, is dropped and counted on `events_errored_total`, and its source is acknowledged, so it is not redelivered. Each `Backpressured` answer from the sink counts on `send_backpressure_total`.

## scalo's sections are set in the same file

The `--config` file is a layer of scalo's cascade, so `scaling`, `self_regulation`, `logger` and the other sections scalo reads for itself apply when written in it. The double-underscore env form overrides them, for example `DFE_TRANSFORM_ELASTIC_SCALING__MEMORY_GATE_THRESHOLD`. The effective scaling values are logged once at startup, with whether they came from the cascade or the default.

`SCALO_SECTIONS` in `src/config/loader.rs` is the full list, and the unread-key warning skips it. `batch_processing` does nothing here: it sizes scalo's batch engine, which the service loop does not run. The loop takes its record cap from `source.batch_size` and its byte cap from `source.max_batch_bytes`, though scalo still fails to start on a `batch_processing` section that does not parse. Every other key that no field and no scalo section reads gets one warning at startup. `geoip` is not on the list, because the service declares that section and hands it to scalo.

KEDA replica scaling is set by the chart's `keda.*` values and the `scaling_pressure` gauge, not this file. The contract's ScaledObject scales on CPU alone (80%, 1 to 10 replicas). In a DFE deployment, dfe-infra's values set the CPU target to 70% in every profile, and the single, scale and mesh profiles add the scaling-pressure trigger (target 70, read through dfe-keda-shim). The engine's [KEDA scaling page](https://github.com/hyperi-io/dfe-engine/blob/main/docs/deployment/keda-scaling.md) covers the triggers for every app.

## The metrics listener is scalo's, not a config field

`/livez`, `/readyz`, `/metrics`, `/metrics/manifest` and `/scaling/pressure` are served from scalo's `--metrics-addr` (env `METRICS_ADDR`, default `0.0.0.0:9090`), so charts and deployments override that rather than a YAML field. `/scaling/pressure` is the figure KEDA reads. `dfe-transform-elastic metrics-manifest` prints every metric without starting the service. On `direct` the push listener is a second port, 6000 by default, declared in the deployment contract so the generated Dockerfile exposes both.
