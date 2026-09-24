# Architecture

**Project:** dfe-transform-elastic
**Purpose:** Rust-optimised transform service for Elastic Stack data (Beats + Elastic Agent) ingested over Kafka or a direct gRPC push

Converts Elastic ingest pipeline logic (Painless scripts + processors) into compiled Rust
transform functions, one module per source. There is no interpreter, no scripting VM and no
plugin system: the service resolves a source name to a compiled transform at startup, and the
hot path carries no dynamic dispatch per event.

This page is the code map: where the service sits, what the crates are, what the binary does,
and what an event is on the way through. The neighbouring pages carry the rest --
[parity.md](parity.md), [parsers.md](parsers.md), [enrichment.md](enrichment.md),
[testing.md](testing.md) and [performance.md](performance.md).

---

## System context

Where dfe-transform-elastic fits in the Data Fusion Engine pipeline:

```mermaid
flowchart LR
    subgraph Sources
        FB[Filebeat]
        WB[Winlogbeat]
        AB[Auditbeat]
        MB[Metricbeat]
        HB[Heartbeat]
        PB[Packetbeat]
        EA[Elastic Agent]
        RX[dfe-receiver<br/>pushed transports]
        FX[dfe-fetcher<br/>pulled APIs]
    end

    K[Kafka<br/>JSON events]

    subgraph dfe-transform-elastic
        D[serde_json<br/>Deserialise]
        T[Rust Transform<br/>Pipeline]
        E[Enrichment<br/>GeoIP / UA / CID]
    end

    O[Output<br/>Kafka, or a gRPC push<br/>to dfe-loader]

    FB & WB & AB & MB & HB & PB & EA & RX & FX --> K
    RX & FX -.->|direct gRPC push| D
    K --> D --> T --> O
    T <--> E
```

JSON events arrive from Kafka or over a direct gRPC push, in any of the three producer
envelopes, and leave the same way as normalised events. Between the two, Elastic's ingest
pipeline logic runs as native Rust.

---

## Crate dependency graph

The workspace is five library crates plus the root service package.

```mermaid
flowchart TD
    service[dfe-transform-elastic<br/><i>Service binary: cli, config,<br/>envelope, pipeline, registry</i>]
    transforms[dfe-transforms<br/><i>Transform modules,<br/>one per Elastic pipeline</i>]
    runtime[dfe-runtime<br/><i>Grok cache, enrichment,<br/>codegen_api, Transform trait</i>]
    painless[dfe-painless<br/><i>Painless matchers, bespoke<br/>runners, plan, params</i>]
    core[dfe-core<br/><i>Event, errors, date formats,<br/>syslog priority, regex cache</i>]
    parse[dfe-parse<br/><i>High-performance parsers<br/>replacing grok/regex</i>]

    service --> transforms
    service --> runtime
    transforms --> runtime
    transforms --> parse
    runtime --> painless
    runtime --> core
    runtime --> parse
    painless --> core
```

| Crate | Type | Purpose |
|-------|------|---------|
| `dfe-transform-elastic` | Binary + Library | The service, module by module in the table below |
| `dfe-transforms` | Library | Transform modules per data source (`filebeat::<source>::<pipeline>`) |
| `dfe-runtime` | Library | Grok caching, enrichment, `codegen_api`, the Transform trait, and the prelude |
| `dfe-painless` | Library | Painless pattern matching: the `known_patterns` ladder, `plan`, `params`, `patterns.lock`, and `bespoke/` -- one module per source holding the scripts transcribed by hand, resolved by script hash ahead of every matcher |
| `dfe-core` | Library | `Event`, error types, date formats, syslog priority, the regex cache |
| `dfe-parse` | Library | Zero-copy parsers replacing grok/regex patterns |

`dfe-transforms` has no direct edge to `dfe-painless`. It reaches the matchers through
`dfe-runtime`, which re-exports every moved module at its old `painless_*` path -- which
is why the thousand generated modules did not change when the split landed.

`dfe-parse` is reached, but only for two whole-pattern forms: `grok_cache` dispatches
`^%{IPV4:f}$` and `^%{IPV4:a}:%{PORT:p}$` to it and falls back to the compiled regex for
everything else. Each call site holds that regex in its own `OnceLock` rather than a shared
map, for the reason [performance.md](performance.md) measures.

The third-party libraries underneath all six crates are
[Key dependencies](performance.md#key-dependencies).

---

## Service binary

`src/` is the service that resolves a configured source name to one compiled transform and
runs it over every batch on the scalo runtime, over the bus (Kafka) or the direct (gRPC push)
transport.

| Module | Responsibility |
|---|---|
| `main.rs` | Entry point, hands off to `cli.rs` |
| `cli.rs` | Subcommands: `sources` (list registered transforms), `emit-dockerfile`, `emit-chart`, `emit-compose`, `emit-config`, plus scalo's `run`, `version`, `config-check`, `config-schema`, `generate-artefacts` and `metrics-manifest` |
| `config.rs` | The config shape: `source.*`, `sink.*`, `geoip`, read once at startup, and `work_state`, which decides whether an instance has work or idles. Also `Transport`, the `bus`/`direct` selector on each side |
| `config/loader.rs` | Reading it, from the scalo cascade or an explicit `--config` file, and `CASCADE_ONLY_SECTIONS` -- the scalo sections that file is warned for carrying |
| `config/validate.rs` | Refusing a configuration that cannot work. The range checks run ahead of the idle gate, so a value someone set out of range still refuses while an unconfigured instance idles |
| `registry.rs` | Source name to `Transform` lookup, plus each source's `Intake` (which envelopes it accepts), `Framing` and dataset |
| `envelope.rs` | Detects which of the three producer families wrapped an event and unwraps it into the shape every transform expects |
| `pipeline.rs` | Batch processing: NDJSON parse, envelope unwrap, transform, serialise, with per-batch outcome counts |
| `service.rs` | Wires the scalo consumer and producer to `pipeline.rs` -- Kafka on the bus, a Push listener and a gRPC client on direct -- and owns the send/commit semantics below |
| `deployment.rs` | The single deployment contract: Dockerfile, Helm chart, compose fragment and KEDA scaler are all generated from here. Its tests pin the Dockerfile, `config.example.yaml`, the chart's `config:` block and the `docs/` config artefacts against a fresh regen -- not every artefact, see the README |
| `metrics.rs` | Metric definitions registered with scalo's `MetricsManager` |
| `error.rs` | The service's top-level error type |

Nothing is hot-reloaded, the two ways in (`--config` against the scalo cascade)
are not equivalent, and only one env spelling reaches a `--config` file:
[configuration.md](configuration.md) carries all three, and `src/config.rs` states them at the
top of the file.

## Envelopes: the same pipeline, a different wrapper

`source.envelope` selects `auto` (the default), `beats`, `receiver` or `fetcher`.

- **beats** -- the raw vendor payload as a string in `message`, which is what the transforms are
  written for, and what Elastic Agent and anything passing a Beats document through unaltered
  also produce.
- **receiver** -- [dfe-receiver](https://github.com/hyperi-io/dfe-receiver)'s JSON on any of its
  transports. The syslog arm rebuilds a line into `message`, and the rest pass their payload
  through with their own field names moved onto the ECS paths those names would otherwise shadow.
- **fetcher** -- [dfe-fetcher](https://github.com/hyperi-io/dfe-fetcher)'s JSON, the provider's
  own payload at the top level, for the sources where Elastic's agent input is a pure transport.

`auto` resolves the family PER EVENT rather than per batch: a scalo `WorkBatch` spans
partitions, so one batch can carry two producers' wrappers and reading only its first event
unwrapped every one of them the same way. The cost is a handful of top-level key checks against
an unwrap measured at 2,740 ns an event.

A bare vendor object carrying no producer marker is the one failure that is not loud. Detection
falls back to `beats`, there is no `message` to unpack, and the transform still runs and emits
with almost nothing renamed. Output that looks like the input with an `ecs.version` bolted on is
this.

Two syslog framings, recorded per source in `registry.rs` as the `Framing` enum:

- **Body** -- `panw.*` and `cisco_meraki` read `message` as CSV or key-value. The receiver's
  body goes through untouched, because a prefixed header would corrupt the first field.
- **Line** -- `fortinet`, `cisco_ios` and `cisco_nexus` grok the header out of `message`, so a
  line is put back: the receiver's `_raw` verbatim if present, otherwise an RFC 3164 line
  rebuilt from the parsed fields. Reconstruction is enough for `fortinet` (`<PRI>` only); it is
  not enough for `cisco_ios` (wants a source IP) or `cisco_nexus` (wants a sequence number),
  since the receiver keeps neither.

A PINNED envelope a source cannot arrive in is rejected at startup, checked against that
source's `Intake` -- okta is pulled from an API, so it accepts `beats` and `fetcher` and refuses
`receiver`. `auto` cannot be checked that way, since there is no event yet, so the same
mismatch is counted per batch instead.

## Delivery: at-least-once, enforced by stopping

Kafka commits are **cumulative** -- the highest offset per partition -- so an uncommitted batch is
only replayed if nothing after it commits. The loop therefore stops on a send it cannot complete
rather than continuing to the next batch, whose commit would acknowledge the failed one. The
process exits, and the restarted consumer resumes from the last committed offset. scalo's
transport exposes no consumer seek, so stopping is the only way to keep the guarantee.

All four `SendResult` variants are handled, and three of them are not delivery:

| Variant | Meaning | Response |
|---|---|---|
| `Ok` | The broker accepted it | Count as delivered |
| `Backpressured` | The local producer queue is full | Retry, bounded backoff to ~25s |
| `Fatal` | The send failed | Retry, then stop uncommitted |
| `FilteredDlq` | An outbound filter wants DLQ routing | Handled, not retried -- the same filter matches every time |

Backpressure is the NORMAL response from a slow sink, so treating it as success would
acknowledge batches that were never written.

**One record per event.** `pipeline.rs::serialise_events` gives each event its own payload and
`service.rs::publish` sends them via `TransportSender::send_batch`, because dfe-loader parses
exactly one JSON document per message
([#67](https://github.com/hyperi-io/dfe-transform-elastic/issues/67)).
`sink.max_message_bytes` (default 900 KB) bounds each payload against librdkafka's
1,000,000-byte producer default, and the `send_batch` block against gRPC's 16 MiB ceiling. An
event over the budget is dropped and counted on `events_oversize_total`: no broker would take
it, and retrying blocks the partition.

Duplicates are the accepted cost: a batch that fails partway through replays the records that
already landed, so every downstream consumer must be idempotent.

---

## Event lifecycle

From Kafka message bytes to transformed output:

```mermaid
sequenceDiagram
    participant K as Kafka Consumer
    participant E as Event
    participant C as TransformChain
    participant T1 as Transform 1<br/>(set)
    participant T2 as Transform 2<br/>(grok to regex)
    participant T3 as Transform 3<br/>(geoip)
    participant O as Output

    K->>E: Event::from_bytes(payload)
    E->>C: chain.execute(&mut event)
    C->>T1: transform(&mut event)
    T1->>E: event.set("event.kind", "event")
    T1-->>C: Ok(Continue)
    C->>T2: transform(&mut event)
    T2->>E: event.get_str("message")
    T2->>E: event.set("source.ip", parsed.ip)
    T2-->>C: Ok(Continue)
    C->>T3: transform(&mut event)
    T3->>E: event.get_str("source.ip")
    T3->>E: event.set("source.geo.*", geoip_result)
    T3-->>C: Ok(Continue)
    C-->>O: TransformResult::Continue
    O->>O: serde_json::to_vec(&event)
```

---

## Event model

The `Event` type wraps `serde_json::Value` with dotted-path navigation for ECS field access.

```mermaid
classDiagram
    class Event {
        -inner: serde_json::Value
        +new(value: Value) Event
        +from_json(json: &str) Result~Event~
        +from_bytes(buf: &mut [u8]) Result~Event~
        +as_value(&self) &Value
        +into_value(self) Value
        +get(path: &str) Option~&Value~
        +get_str(path: &str) Option~&str~
        +get_i64(path: &str) Option~i64~
        +get_f64(path: &str) Option~f64~
        +get_bool(path: &str) Option~bool~
        +get_array(path: &str) Option~&Vec~
        +get_object(path: &str) Option~&Map~
        +has(path: &str) bool
        +set(path: &str, value: Value) Result
        +remove(path: &str) Option~Value~
        +rename(from: &str, to: &str) Result
        +append(path: &str, value: Value) Result
        +merge(other: &Event, deep: bool) Result
    }

    class TransformResult {
        <<enumeration>>
        Continue
        Drop
    }

    class Transform {
        <<trait>>
        +name(&self) &str
        +transform(&self, event: &mut Event) Result~TransformResult~
    }

    class TransformChain {
        -transforms: Vec~Box~dyn Transform~~
        +new(transforms: Vec) TransformChain
        +execute(&self, event: &mut Event) Result~TransformResult~
    }

    class TransformError {
        <<enumeration>>
        FieldNotFound
        TypeMismatch
        ParseError
        EnrichmentError
        ProcessorError
    }

    TransformChain --> Transform : contains
    Transform --> Event : mutates
    Transform --> TransformResult : returns
    Transform --> TransformError : may return
```

### Dotted-path access

All field access uses ECS-style dotted paths (e.g., `source.geo.city_name`). The `set()` method
auto-creates intermediate objects:

```rust
// Navigates into {"source": {"geo": {"city_name": ...}}}
event.get_str("source.geo.city_name")

// Creates {"source": {"geo": {}}} if missing, then sets city_name
event.set("source.geo.city_name", "Sydney")?;
```

### Design decisions

- **`serde_json::Value` over custom types:** Simpler to implement, and the map underneath is an `IndexMap` because `preserve_order` is not optional here -- Elasticsearch's own ingest documents are insertion-ordered and parity rests on it. Typed structs can be layered on later for hot-path fields.
- **`from_bytes` uses `serde_json`, not simd-json:** measured on this workload and it is the faster of the two, because getting a `serde_json::Value` out of simd-json goes tape to serde deserializer to `Value`, which is strictly more work than parsing straight into the same tree. simd-json is a dev-dependency of `dfe-runtime` now, kept only for `crates/dfe-runtime/benches/json_parse.rs`, which is the measurement. Re-run it before reopening this.
- **Dotted-path splitting:** Split on `.` with no escaping. ECS field names never contain dots within a single field segment.

---

## Transform pipeline

How transforms compose and handle errors:

```mermaid
flowchart TD
    start([Event enters chain]) --> t1

    subgraph TransformChain
        t1[Transform 1] -->|Continue| t2[Transform 2]
        t2 -->|Continue| t3[Transform 3]
        t3 -->|Continue| tn[Transform N]
    end

    t1 -->|Drop| dropped([Event dropped])
    t2 -->|Drop| dropped
    t3 -->|Drop| dropped

    t1 -->|Error| err{on_failure<br/>handler?}
    t2 -->|Error| err
    t3 -->|Error| err

    err -->|yes| onfail[Run on_failure<br/>transforms]
    err -->|no / ignore_failure| next[Continue to<br/>next transform]
    onfail --> next

    tn -->|Continue| output([Event emitted])
```

### Error handling strategy

Mirrors Elastic ingest pipeline semantics:

| Elastic Behaviour | Rust Implementation |
|---|---|
| `on_failure` block | Nested `TransformChain` executed on error |
| `ignore_failure: true` | Wrap in `.ok()` / `let _ =`, continue |
| No error handling | Propagate error, stop chain |
| `tag` on failure | `event.append("tags", "error_tag")` |

At the service boundary, `pipeline.rs::transform_batch_with` treats a transform error the same
way as a rejected envelope: the event is counted and left out of the batch output, and
processing continues with the next event. One malformed record cannot stall a partition.

### Transform trait contract

```rust
pub trait Transform: Send + Sync {
    /// Human-readable processor name (for error context)
    fn name(&self) -> &str;

    /// Apply transform to event, returning Continue or Drop
    fn transform(&self, event: &mut Event) -> Result<TransformResult>;
}
```

- **Object-safe:** `dyn Transform` for heterogeneous chains
- **`Send + Sync`:** Safe for concurrent use across threads
- **Infallible transforms** (set, remove, rename) still return `Result` for uniform error propagation
