# Configuration

Where the values come from, which env spelling reaches a `--config` file, how the transport is selected, and which sections the file cannot carry. The field-by-field reference is generated rather than written here: `config-schema.yaml` beside this page carries every key with its description, and `config.example.yaml` at the repo root is the complete default set, pinned against drift by a test. `dfe-transform-elastic emit-config` reprints it.

## The configuration is read once, at startup

`Config::load` reads the configuration once and hands it to the batch loop by reference, so every value needs a restart to change. There is no hot reload: the broker connections, the resolved transform, the GeoIP databases and the metrics labels are all fixed at startup.

The two ways in are not equivalent. With no `--config` the scalo cascade applies: `defaults`, `settings` and `settings.<env>` files, `.yaml` or `.yml`, searched in `./`, `./config/`, `/config/` and `~/.config/dfe-transform-elastic/`, then the `DFE_TRANSFORM_ELASTIC_*` environment. With `--config`, which is what the container passes, the named file IS the configuration: `Config::load` reads it directly and installs the scalo cascade without it, so the file is not one of the cascade's layers. Kafka credentials are unaffected either way: they are read from `KAFKA_*` separately.

## Only the flat env spelling reaches a `--config` file

The FLAT, single-underscore form is applied to the loaded configuration on both branches, so it works against a `--config` deployment:

    DFE_TRANSFORM_ELASTIC_SOURCE_TOPICS=a,b
    DFE_TRANSFORM_ELASTIC_SOURCE_BATCH_SIZE=5000
    DFE_TRANSFORM_ELASTIC_SINK_TOPIC=out

It covers every `source.*` and `sink.*` field. `geoip` is not among them: it is scalo's own type, so it stays settable from the file and the cascade only. scalo's DOUBLE-underscore form is resolved from the cascade, which a named file is not a layer of, so it never reaches a `--config` deployment's `source.*` or `sink.*`.

## `transport` selects the bus or a direct push, per side

Each side carries a `transport` of `bus` (the default) or `direct`. A config file and the flat env form also take `kafka` for `bus` and `grpc` for `direct`, the names dfe-engine renders, and a config always writes `bus` and `direct` back. A config file refuses any other name, and an unrecognised env value keeps the configured transport rather than moving the deployment. The chart publishes the push port 6000 when `source.transport` is `direct` or `grpc`. Both transports are compiled in by default and both are constructed: on `direct` the service binds a scalo Push listener on `source.listen` (default `0.0.0.0:6000`) and pushes to `sink.endpoint` (default `http://dfe-loader:6000`). The listener answers a push only once its events are delivered, so a refused push is the upstream sender's to retry.

    DFE_TRANSFORM_ELASTIC_SOURCE_TRANSPORT=direct
    DFE_TRANSFORM_ELASTIC_SOURCE_LISTEN=0.0.0.0:6000
    DFE_TRANSFORM_ELASTIC_SINK_ENDPOINT=http://dfe-loader:6000

Every `source` and `sink` key may be left out of the file. `source.name`, `source.topics`, `source.group_id`, `source.brokers` and `sink.topic` then default to empty. An instance with no source name idles, as does a bus instance with no topics. Once the configuration names work, startup refuses a bus source with no brokers or group.

On `direct` the brokers and group may stay empty. Empty topics do not idle there, because the listener is the work. The fleet routes over the bus until dfe-infra flips the selector (issue #19).

`sink.topic` is required on both transports once the configuration names work. On `direct` it is the routing key every push carries, and dfe-loader routes a record with no `_source` on that key, so a push without one lands under the loader's `default_topic`.

The listener speaks plaintext gRPC with no authentication of its own, and the push to the endpoint travels in the clear. Both belong behind the mesh route dfe-infra provisions for them, never on an interface reachable from outside the cluster.

## Agentless mode serves one syslog stream of many device types

`source.mode: agentless` reads the source off each event instead of from `source.name`, so one pod can consume a dfe-receiver syslog topic that carries firewalls, switches and proxies together. Each event is matched against a table of markers (literals, key sets and CEF vendor and product pairs), the winner's transform runs, and the output is stamped `_source` with that source's dfe-schemas catalogue label, which is what dfe-loader routes on. `source.name` must stay empty. `source.envelope` must be `auto` or `receiver`.

- `source.mode`: `agent` (default) or `agentless`.
- `source.detect.unmatched`: what an event no marker claims gets. `passthrough` (default) unwraps it to the Beats layout, runs no transform and leaves `_source` off, so the loader lands it under `sink.topic`'s default table. `fallback` runs `source.detect.fallback`'s transform and stamps its label. `stop` ends the service with the block unreleased, so a restart reads it again.
- `source.detect.fallback`: a registered source, required when `unmatched` is `fallback` and refused otherwise.
- `source.detect.candidates`: the sources the detector may name. Empty means all of them. A name it does not know is refused at startup. Narrowing it hands shared lines to the source left in: ASA-format lines go to ASA, and to the IOS catch-all once ASA is out. Citrix CEF goes to the ADC source unless it is excluded, and then to WAF.

    DFE_TRANSFORM_ELASTIC_SOURCE_MODE=agentless
    DFE_TRANSFORM_ELASTIC_SOURCE_DETECT_UNMATCHED=fallback
    DFE_TRANSFORM_ELASTIC_SOURCE_DETECT_FALLBACK=filebeat.cisco_ios.default
    DFE_TRANSFORM_ELASTIC_SOURCE_DETECT_CANDIDATES=filebeat.cisco_asa.default,filebeat.cisco_ftd.default

Fetcher events carry the provider's own payload, so they count as unmatched. Three counters report the split: `events_detected_total{source}`, `events_unmatched_total` and `events_fallback_total`.

Detection costs about 2,080 ns an event (481K events a second on one core), so a 20,000-event batch spends about 42 ms on it. That is 20,000 compat-corpus events of 718 bytes on average, measured on desktop-derek (AMD EPYC 7302) with the bench profile. `cargo bench --bench detect` re-derives it.

Run it on the receiver's `_raw` capture where you can. The receiver splits each line with `syslog_loose`, and a marker that straddled the split (`Pleasant Password Server:`, an `amp:` tag) is gone from the parsed fields. On the compat corpus the detector names 8,243 of 8,885 events of detectable sources from the raw line and 8,004 from the parsed fields.

`scripts/replay_syslog.py` feeds an agentless stack from the committed fixtures under `tests/fixtures/unencumbered/`, standard library only. `--sources` takes catalogue labels from the same table the detector stamps, and `--list` shows the ones with fixtures. A syslog header is added only to a fixture line that lacks one, and the `%` the ASA fixtures dropped is put back. UDP sends one datagram per line and TCP is newline framed. There is no `http://` form, because dfe-receiver's `/ingest` takes JSON and not syslog.

    scripts/replay_syslog.py --list
    scripts/replay_syslog.py --target udp://dfe-receiver:514 --sources cisco-asa,fortinet,panw --rate 50 --loop --duration 120

## `source.acknowledgements` holds the source until delivery

`source.acknowledgements.enabled` (default `true`) holds the source's acknowledgement -- the Kafka offset commit on the bus, the answer to a push on direct -- until every event built from a record is delivered, or dropped and counted. A sink outage is then waited out, and a crash redelivers rather than loses. `false` acknowledges at receipt, before the transform runs, and a crash loses what was in flight. The flat env form is `DFE_TRANSFORM_ELASTIC_SOURCE_ACKNOWLEDGEMENTS_ENABLED`.

On a direct source a push is held at most 18 s, so every send it waits on gives up at 15 s: a direct sink's gRPC deadline, and a bus sink's `message.timeout.ms` (librdkafka's default is 300 s).

## `sink.max_message_bytes` caps one outbound record

The default is the DFE record cap, 8,000,000 bytes: dfe-receiver refuses a request body over it, and every broker DFE deploys takes a record that size (Confluent Basic's 8,388,608 limit leaves room for the framing). The producer this service builds allows 16 MiB, so a deployment with larger brokers raises the budget up to that. An event that serialises larger is dropped, logged, counted on `events_oversize_total`, and its source is still acknowledged.

Against a broker or topic whose `message.max.bytes` is lower -- a stock Kafka broker takes about 1 MB -- set `sink.max_message_bytes` below that limit. Left at the default, a broker below 8,000,000 bytes refuses the events between the two instead: each is dropped as a dead letter, counted on `send_filtered_dlq_total`, and the partition moves on. Validation refuses a value under 4096 or over 16,777,216.

    DFE_TRANSFORM_ELASTIC_SINK_MAX_MESSAGE_BYTES=900000

## A record that will not parse or transform is dropped and counted

There is no dead-letter backend. A record that is not JSON is dropped and counted on `parse_errors_total` and `pipeline_dead_letters_dropped_total`. An event whose transform returns an error, or that will not serialise, is dropped and counted on `events_errored_total`. Either way the source is acknowledged -- the offset commits on the bus, the push is answered on direct -- so the record is not redelivered. Holding the acknowledgement protects what the sink refuses for now, never what can never be delivered.

## A section scalo resolves for itself cannot be set from the file

`scaling` is the one that bites. The container starts with `--config`, and scalo reads `scaling` from its own cascade, so a `scaling:` block in the named file reaches nothing and the defaults stand. Set `DFE_TRANSFORM_ELASTIC_SCALING__ENABLED` or `DFE_TRANSFORM_ELASTIC_SCALING__MEMORY_GATE_THRESHOLD` instead. The effective values are logged once at startup, with whether each came from the cascade or the default.

`CASCADE_ONLY_SECTIONS` in `src/config/loader.rs` is the full list. The service WARNS for each such section rather than refusing the file, naming the env form that does reach it, because the file is rendered from what dfe-engine publishes and a surplus section is not this service's to stop a pod over. `geoip` is deliberately absent from the list: the service declares that section itself and hands it to scalo, which is what makes it settable from a file. KEDA replica scaling is separate and unaffected: it is driven by the chart's `keda.*` values and the `scaling_pressure` gauge.

## The metrics listener is scalo's, not a config field

`/livez`, `/readyz`, `/metrics` and `/metrics/manifest` are served from scalo's `--metrics-addr` (env `METRICS_ADDR`, default `0.0.0.0:9090`), so charts and deployments override that rather than a YAML field. On `direct` the push listener is a second port, 6000 by default, declared in the deployment contract so the generated Dockerfile exposes both.
