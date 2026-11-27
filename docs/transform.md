# Transform

Transform is the step in [dfe-transform-elastic](architecture.md) that runs a source's Elastic ingest pipeline on each event, as one generated Rust function.

| | |
|---|---|
| Repo | [hyperi-io/dfe-transform-elastic](https://github.com/hyperi-io/dfe-transform-elastic), `crates/dfe-transforms*`, `crates/dfe-runtime`, `crates/dfe-painless`, `crates/dfe-parse` and `src/registry.rs` |
| Image and chart | `ghcr.io/hyperi-io/dfe-transform-elastic`, and the thin chart each release assembles from `src/deployment.rs` |
| Config reference | `source.name`, `source.mode` and `geoip` in [config.example.yaml](../config.example.yaml) |
| Endpoints | None of its own |
| Metrics | `events_transformed_total`, `events_errored_total`, `events_dropped_total`, `painless_handled_total`, `painless_unhandled_total` on `:9090/metrics` |

The registry resolves the configured source name to one transform at startup, so nothing is looked up per event. `dfe-transform-elastic sources` lists every source a build has.

```mermaid
---
config:
  theme: base
  look: neo
  fontFamily: Inter, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif
  flowchart:
    curve: linear
    nodeSpacing: 40
    rankSpacing: 64
    padding: 18
    wrappingWidth: 280
    htmlLabels: true
  elk:
    preset: modelOrder
  themeVariables:
    fontSize: 15px
    lineColor: "#64748B"
    primaryTextColor: "#0F172A"
    nodeBorder: "#334155"
    clusterBorder: "#0072B2"
    edgeLabelBackground: "#FFFFFF"
  themeCSS: >-
    p{margin:0}
    .node[data-look] rect,.node[data-look] path,.node[data-look] polygon,.node[data-look] .outer-path{filter:drop-shadow(0 1px 2px rgba(15,23,42,.10)) drop-shadow(0 6px 14px rgba(15,23,42,.10))}
    .node rect{rx:10px;ry:10px}
    .cluster rect{rx:16px;ry:16px}
    .node .label div{min-width:150px}
    .node.decision .label div{min-width:0}
    .cluster[data-look] rect{filter:none}
    .nodeLabel{line-height:1.35}
    .nodeLabel b{font-weight:600;font-size:15px;letter-spacing:-0.01em}
    .nodeLabel span{font-size:12.5px;font-weight:400;color:#334155}
    .cluster-label span.nodeLabel{display:inline-block;background:#FFFFFF;color:#0B2A3F;border:1.5px solid #0072B2;border-radius:999px;padding:2px 12px;font-size:11.5px;font-weight:700;letter-spacing:.09em;text-transform:uppercase}
    .flowchart-link{stroke-width:1.75px}
    .labelBkg{background:transparent!important}
    span.edgeLabel{display:inline-block;background:#FFFFFF!important;color:#1E293B;border:1px solid #94A3B8;border-radius:999px;padding:2px 10px;font-size:12px;font-weight:500}
    span.edgeLabel p{background:transparent!important}
    .label-icon{width:20px;height:20px;vertical-align:-4px;margin-right:6px}
    .icon-shape path[stroke-width="0"][fill="none"]{fill:#FFFFFF;stroke:#FFFFFF;stroke-width:12px;stroke-linejoin:round}
    .icon-shape p{background:transparent!important;color:#0F172A}
    .ingest .label-icon{color:#0E8A7D}
    .xform .label-icon{color:#7B4FB0}
    .load .label-icon{color:#0072B2}
    .bus .label-icon{color:#A8517F}
    .store .label-icon{color:#00785A}
    .ui .label-icon{color:#D9480F}
    .ext .label-icon{color:#64748B}
---
flowchart TB
    accTitle: L4 Detail -- Transform
    accDescr: Unwrap envelope hands a bare event to the transform. Inside, the registry picks the source's generated transform, which runs its processors in order and calls on grok and the parsers, the Painless matchers and the enrichers. The transformed event goes to Serialise.

    subgraph IN["From"]
        direction LR
        unwrap("<b>Unwrap envelope</b><br/><span>the bare event</span>")
    end

    subgraph TR["Transform"]
        direction TB
        subgraph RUN["Runs"]
            direction LR
            registry("<b>Registry</b><br/><span>source name to transform</span>")
            generated("<b>Generated transform</b><br/><span>processors in order</span>")
            registry --> generated
        end
        subgraph HELP["Calls on"]
            direction LR
            grok("<b>Grok and parsers</b><br/><span>regex or dfe-parse</span>")
            painless("<b>Painless matchers</b><br/><span>dfe-painless</span>")
            enrich("<b>Enrichers</b><br/><span>three runtime enrichers</span>")
            grok ~~~ painless ~~~ enrich
        end
        RUN --> HELP
    end

    subgraph OUT["To"]
        direction LR
        serialise("<b>Serialise</b><br/><span>one record per event</span>")
    end

    IN --> TR
    TR --> OUT
  class unwrap,registry,generated,grok,painless,enrich,serialise part
  classDef ext fill:#F4F6F8,stroke:#94A3B8,stroke-width:1.5px,stroke-dasharray:5 4,color:#0F172A
  classDef ingest fill:#FFFFFF,stroke:#0E8A7D,stroke-width:3px,color:#0F172A
  classDef xform fill:#FFFFFF,stroke:#7B4FB0,stroke-width:3px,color:#0F172A
  classDef load fill:#FFFFFF,stroke:#0072B2,stroke-width:3px,color:#0F172A
  classDef bus fill:#FFFFFF,stroke:#A8517F,stroke-width:3px,color:#0F172A
  classDef store fill:#FFFFFF,stroke:#00785A,stroke-width:3px,color:#0F172A
  classDef ui fill:#FFFFFF,stroke:#D9480F,stroke-width:3px,color:#0F172A
  classDef decision fill:#FFFFFF,stroke:#9A6700,stroke-width:3px,color:#0F172A
  classDef error fill:#FFFFFF,stroke:#C2410C,stroke-width:3px,color:#0F172A
  classDef control fill:#FFFFFF,stroke:#334155,stroke-width:3px,color:#0F172A
  classDef part fill:#FFFFFF,stroke:#94A3B8,stroke-width:1.5px,color:#0F172A
  style IN fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style TR fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style RUN fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style HELP fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style OUT fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
```

*Figure 1. L4 Detail -- Transform.* Up: [L3 Components -- dfe-transform-elastic](architecture.md). Down: [L4 Detail -- Grok dispatch](parsers.md), [L4 Detail -- Enrichers](enrichment.md).

## How it works

A generated transform is one function that runs its pipeline's processors in order, each behind its `if` condition. The `Transform` trait is object-safe, and the registry hands out `&'static dyn Transform`.

`Event` in `dfe-core` wraps a `serde_json::Value` with dotted-path access for ECS fields. `set()` creates intermediate objects. A read of a segment that is not a field falls back to a key with the dot inside it, and a numeric segment indexes an array, the way Elastic's templates read `{{_temp.user_parts.0}}`.

```rust
// Reads {"source": {"geo": {"city_name": ...}}}
event.get_str("source.geo.city_name")

// Creates {"source": {"geo": {}}} if missing, then sets city_name
event.set("source.geo.city_name", "Sydney")?;
```

In agentless mode a detector names each event's source from a marker table, runs that source's transform and stamps `_source` ([configuration.md](configuration.md#agentless-mode-serves-one-syslog-stream-of-many-device-types)).

## Interfaces

A transform mutates an `Event` in place and returns `Continue`, `Drop` or an error.

```mermaid
---
config:
  theme: base
  fontFamily: Inter, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif
  themeVariables:
    fontSize: 15px
    lineColor: "#64748B"
    primaryTextColor: "#0F172A"
    actorBkg: "#FFFFFF"
    actorBorder: "#94A3B8"
    actorTextColor: "#0F172A"
    actorLineColor: "#64748B"
    signalColor: "#64748B"
    signalTextColor: "#0F172A"
    noteBkgColor: "#FFFFFF"
    noteBorderColor: "#94A3B8"
    noteTextColor: "#0F172A"
  themeCSS: >-
    .messageText{paint-order:stroke;stroke:#FFFFFF;stroke-width:3px;stroke-linejoin:round}
---
classDiagram
    accTitle: L4 Detail -- Event and Transform types
    accDescr: A Transform mutates an Event and returns a TransformResult, or a TransformError on failure.
    class Event {
        -inner: serde_json::Value
        +from_json(json: &str) Result~Event~
        +get_str(path: &str) Option~&str~
        +set(path: &str, value) Result
        +remove(path: &str) Option~Value~
        +rename(from: &str, to: &str) Result
    }
    class Transform {
        <<trait>>
        +name(&self) &str
        +transform(&self, event: &mut Event) Result~TransformResult~
    }
    class TransformResult {
        <<enumeration>>
        Continue
        Drop
    }
    class TransformError {
        <<enumeration>>
        FieldNotFound
        TypeMismatch
        ParseError
        ProcessorError
    }
    Transform --> Event : mutates
    Transform --> TransformResult : returns
    Transform --> TransformError : may return
```

*Figure 2. L4 Detail -- Event and Transform types. The enumerations list the main variants, not all of them.* Up: [L4 Detail -- Transform](transform.md).

- The map under `serde_json::Value` is an `IndexMap`, because `preserve_order` is on. Elasticsearch's ingest documents keep insertion order, and parity rests on it.
- `Event::from_bytes` parses with `serde_json`, not simd-json. Getting a `serde_json::Value` out of simd-json builds two trees for one document, and `crates/dfe-runtime/benches/json_parse.rs` shows it slower.

## Scaling and failure

A failed processor is skipped, handled, or ends the pipeline. The event is lost when the pipeline has no handler.

```mermaid
---
config:
  theme: base
  look: neo
  fontFamily: Inter, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif
  flowchart:
    curve: linear
    nodeSpacing: 40
    rankSpacing: 64
    padding: 18
    wrappingWidth: 280
    htmlLabels: true
  elk:
    preset: modelOrder
  themeVariables:
    fontSize: 15px
    lineColor: "#64748B"
    primaryTextColor: "#0F172A"
    nodeBorder: "#334155"
    clusterBorder: "#0072B2"
    edgeLabelBackground: "#FFFFFF"
  themeCSS: >-
    p{margin:0}
    .node[data-look] rect,.node[data-look] path,.node[data-look] polygon,.node[data-look] .outer-path{filter:drop-shadow(0 1px 2px rgba(15,23,42,.10)) drop-shadow(0 6px 14px rgba(15,23,42,.10))}
    .node rect{rx:10px;ry:10px}
    .cluster rect{rx:16px;ry:16px}
    .node .label div{min-width:150px}
    .node.decision .label div{min-width:0}
    .cluster[data-look] rect{filter:none}
    .nodeLabel{line-height:1.35}
    .nodeLabel b{font-weight:600;font-size:15px;letter-spacing:-0.01em}
    .nodeLabel span{font-size:12.5px;font-weight:400;color:#334155}
    .cluster-label span.nodeLabel{display:inline-block;background:#FFFFFF;color:#0B2A3F;border:1.5px solid #0072B2;border-radius:999px;padding:2px 12px;font-size:11.5px;font-weight:700;letter-spacing:.09em;text-transform:uppercase}
    .flowchart-link{stroke-width:1.75px}
    .labelBkg{background:transparent!important}
    span.edgeLabel{display:inline-block;background:#FFFFFF!important;color:#1E293B;border:1px solid #94A3B8;border-radius:999px;padding:2px 10px;font-size:12px;font-weight:500}
    span.edgeLabel p{background:transparent!important}
    .label-icon{width:20px;height:20px;vertical-align:-4px;margin-right:6px}
    .icon-shape path[stroke-width="0"][fill="none"]{fill:#FFFFFF;stroke:#FFFFFF;stroke-width:12px;stroke-linejoin:round}
    .icon-shape p{background:transparent!important;color:#0F172A}
    .ingest .label-icon{color:#0E8A7D}
    .xform .label-icon{color:#7B4FB0}
    .load .label-icon{color:#0072B2}
    .bus .label-icon{color:#A8517F}
    .store .label-icon{color:#00785A}
    .ui .label-icon{color:#D9480F}
    .ext .label-icon{color:#64748B}
---
flowchart TB
    accTitle: L4 Detail -- A failed processor
    accDescr: A failed processor with ignore_failure set is skipped. Otherwise the processor's on_failure handlers run if it has any. Failing that, the pipeline's on_failure handlers run and keep the event, and with none the event is dropped and counted.

    subgraph Q1["Processor"]
        direction LR
        ignore{"<b>ignore_failure<br/>set?</b>"}
        skip(["<b>Next processor</b>"])
        ignore -->|yes| skip
    end

    subgraph Q2["Processor handler"]
        direction LR
        proc{"<b>Processor's<br/>on_failure?</b>"}
        handlers(["<b>Its handlers run</b>"])
        proc -->|yes| handlers
    end

    subgraph Q3["Pipeline handler"]
        direction LR
        pipe{"<b>Pipeline<br/>on_failure?</b>"}
        kept(["<b>Pipeline handlers<br/>run, event kept</b>"])
        dropped(["<b>Dropped and<br/>counted</b>"])
        pipe -->|yes| kept
        pipe -->|no| dropped
    end

    Q1 -->|no| Q2
    Q2 -->|no| Q3
  class ignore,proc,pipe decision
  class skip,handlers,kept part
  class dropped error
  classDef ext fill:#F4F6F8,stroke:#94A3B8,stroke-width:1.5px,stroke-dasharray:5 4,color:#0F172A
  classDef ingest fill:#FFFFFF,stroke:#0E8A7D,stroke-width:3px,color:#0F172A
  classDef xform fill:#FFFFFF,stroke:#7B4FB0,stroke-width:3px,color:#0F172A
  classDef load fill:#FFFFFF,stroke:#0072B2,stroke-width:3px,color:#0F172A
  classDef bus fill:#FFFFFF,stroke:#A8517F,stroke-width:3px,color:#0F172A
  classDef store fill:#FFFFFF,stroke:#00785A,stroke-width:3px,color:#0F172A
  classDef ui fill:#FFFFFF,stroke:#D9480F,stroke-width:3px,color:#0F172A
  classDef decision fill:#FFFFFF,stroke:#9A6700,stroke-width:3px,color:#0F172A
  classDef error fill:#FFFFFF,stroke:#C2410C,stroke-width:3px,color:#0F172A
  classDef control fill:#FFFFFF,stroke:#334155,stroke-width:3px,color:#0F172A
  classDef part fill:#FFFFFF,stroke:#94A3B8,stroke-width:1.5px,color:#0F172A
  style Q1 fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style Q2 fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style Q3 fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
```

*Figure 3. L4 Detail -- A failed processor. Dropped and counted is the event lost to the output.* Up: [L4 Detail -- Transform](transform.md).

| Elastic behaviour | Generated Rust |
|---|---|
| `if` | The processor runs inside `if _cond { ... }` |
| `ignore_failure: true` | The processor runs in a closure whose error is discarded |
| A processor's `on_failure` | Its handlers run with `_ingest.on_failure_message`, `_ingest.on_failure_processor_type` and `_ingest.on_failure_processor_tag` set, then cleared |
| The pipeline's `on_failure` | Its handlers run, and the event is emitted |
| No handler at all | The error leaves the transform, and `pipeline.rs` drops the event and counts it on `events_errored_total` |
| `drop` | The transform returns `TransformResult::Drop` |

Most Elastic pipelines declare a pipeline-level `on_failure` that sets `event.kind: pipeline_error`, appends to `error.message` and tags `preserve_original_event`, so the generated transform does the same.

## Key metrics

| Metric | Healthy when |
|---|---|
| `events_errored_total` | Flat. A rise is events no handler kept |
| `events_dropped_total` | Matches what the pipeline's `drop` processors mean to drop |
| `painless_unhandled_total` | Flat. Each is a script that was skipped |

## Related

- [parsers.md](parsers.md), the grok dispatch, every processor the transforms use and the Painless coverage
- [enrichment.md](enrichment.md), GeoIP, user agent and community ID
- [parity.md](parity.md), what the transform does in place of Elastic's ingest pipeline
- [performance.md](performance.md), the benchmarks and the libraries
