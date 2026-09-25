# Configuration

Where the values come from, which env spelling reaches a `--config` file, how the transport is selected, and which sections the file cannot carry. The field-by-field reference is generated rather than written here: `config-schema.yaml` beside this page carries every key with its description, and `config.example.yaml` at the repo root is the complete default set, pinned against drift by a test. `dfe-transform-elastic emit-config` reprints it.

## The configuration is read once, at startup

`Config::load` reads the configuration once and hands it to the batch loop by reference, so every value needs a restart to change. There is no hot reload: the broker connections, the resolved transform, the GeoIP databases and the metrics labels are all fixed at startup.

The two ways in are not equivalent. With no `--config` the scalo cascade applies: `defaults`, `settings` and `settings.<env>` files, `.yaml` or `.yml`, searched in `./`, `./config/`, `/config/` and `~/.config/dfe-transform-elastic/`, then the `DFE_TRANSFORM_ELASTIC_*` environment. With `--config`, which is what the container passes, the named file IS the configuration, because scalo cannot merge an arbitrarily-named file into the cascade as a layer. Kafka credentials are unaffected either way: they are read from `KAFKA_*` separately.

## Only the flat env spelling reaches a `--config` file

The FLAT, single-underscore form is applied to the loaded configuration on both branches, so it works against a `--config` deployment:

    DFE_TRANSFORM_ELASTIC_SOURCE_TOPICS=a,b
    DFE_TRANSFORM_ELASTIC_SOURCE_BATCH_SIZE=5000
    DFE_TRANSFORM_ELASTIC_SINK_TOPIC=out

It covers every `source.*` and `sink.*` field. `geoip` is not among them: it is scalo's own type, so it stays settable from the file and the cascade only. scalo's DOUBLE-underscore form is resolved from the cascade, which a named file is not a layer of, so it never reaches a `--config` deployment's `source.*` or `sink.*`.

## `transport` selects the bus or a direct push, per side

Each side carries a `transport` of `bus` (the default) or `direct`, and in a config file those two spellings are the only ones accepted. The flat env form also takes `kafka` for `bus` and `grpc` for `direct`, and an unrecognised env value keeps the configured transport rather than moving the deployment. Both transports are compiled in by default and both are constructed: on `direct` the service binds a scalo Push listener on `source.listen` (default `0.0.0.0:6000`) and pushes to `sink.endpoint` (default `http://dfe-loader:6000`). The listener answers a push only once its events are delivered, so a refused push is the upstream sender's to retry.

    DFE_TRANSFORM_ELASTIC_SOURCE_TRANSPORT=direct
    DFE_TRANSFORM_ELASTIC_SOURCE_LISTEN=0.0.0.0:6000
    DFE_TRANSFORM_ELASTIC_SINK_ENDPOINT=http://dfe-loader:6000

On `direct`, `source.brokers`, `source.group_id` and `sink.topic` are not required, and an instance with empty topics does not idle, because the listener is the work. The fleet routes over the bus until dfe-infra flips the selector (issue #19).

The listener speaks plaintext gRPC with no authentication of its own, and the push to the endpoint travels in the clear. Both belong behind the mesh route dfe-infra provisions for them, never on an interface reachable from outside the cluster.

## `source.acknowledgements` holds the source until delivery

`source.acknowledgements.enabled` (default `true`) holds the source's acknowledgement -- the Kafka offset commit on the bus, the answer to a push on direct -- until every event built from a record is delivered, or dropped and counted. A sink outage is then waited out, and a crash redelivers rather than loses. `false` acknowledges at receipt, before the transform runs, and a crash loses what was in flight. The flat env form is `DFE_TRANSFORM_ELASTIC_SOURCE_ACKNOWLEDGEMENTS_ENABLED`.

## A section scalo resolves for itself cannot be set from the file

`scaling` is the one that bites. The container starts with `--config`, and scalo reads `scaling` from its own cascade, so a `scaling:` block in the named file reaches nothing and the defaults stand. Set `DFE_TRANSFORM_ELASTIC_SCALING__ENABLED` or `DFE_TRANSFORM_ELASTIC_SCALING__MEMORY_GATE_THRESHOLD` instead. The effective values are logged once at startup, with whether each came from the cascade or the default.

`CASCADE_ONLY_SECTIONS` in `src/config/loader.rs` is the full list. The service WARNS for each such section rather than refusing the file, naming the env form that does reach it, because the file is rendered from what dfe-engine publishes and a surplus section is not this service's to stop a pod over. `geoip` is deliberately absent from the list: the service declares that section itself and hands it to scalo, which is what makes it settable from a file. KEDA replica scaling is separate and unaffected: it is driven by the chart's `keda.*` values and the `scaling_pressure` gauge.

## The metrics listener is scalo's, not a config field

`/livez`, `/readyz`, `/metrics` and `/metrics/manifest` are served from scalo's `--metrics-addr` (env `METRICS_ADDR`, default `0.0.0.0:9090`), so charts and deployments override that rather than a YAML field. On `direct` the push listener is a second port, 6000 by default, declared in the deployment contract so the generated Dockerfile exposes both.
