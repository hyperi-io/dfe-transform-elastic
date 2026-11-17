# dfe-transform-elastic architecture

dfe-transform-elastic turns Beats and Elastic Agent events into DFE-normalised events, applying each source's Elastic ingest pipeline as compiled Rust. It sits between a source's land topic and dfe-loader, so an existing Elastic shipper lands in DFE unchanged.

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
    .ingest .label-icon{color:#0E8A7D}
    .xform .label-icon{color:#7B4FB0}
    .load .label-icon{color:#0072B2}
    .bus .label-icon{color:#A8517F}
    .store .label-icon{color:#00785A}
    .ui .label-icon{color:#D9480F}
    .ext .label-icon{color:#64748B}
---
flowchart TB
    accTitle: L3 Components -- dfe-transform-elastic
    accDescr: Producers send events over Kafka or a direct push. Inside the service each batch is parsed, unwrapped, transformed and serialised in that order. Events go to Kafka or dfe-loader, and dead letters go to the DLQ.

    subgraph IN["Producers"]
        direction LR
        producers("<b>Beats, Agent, receiver, fetcher</b><br/><span>Kafka or a direct push</span>")
    end

    subgraph SVC["dfe-transform-elastic"]
        direction TB
        subgraph S1["Read"]
            direction LR
            parse("<b>Parse NDJSON</b><br/><span>one event per line</span>")
            unwrap("<b>Unwrap envelope</b><br/><span>beats, receiver, fetcher</span>")
            parse --> unwrap
        end
        subgraph S2["Process"]
            direction LR
            transform("<b>Transform</b><br/><span>the source's pipeline</span>")
            serialise("<b>Serialise</b><br/><span>one record per event</span>")
            transform --> serialise
        end
        S1 --> S2
    end

    subgraph OUT["Next stage"]
        direction LR
        sink("<b>Kafka or dfe-loader</b><br/><span>*_load topic or Push</span>")
    end

    subgraph DL["Dead letters"]
        direction LR
        dlq[("<b>DLQ</b><br/><span>file by default</span>")]
    end

    IN -->|events| SVC
    SVC -->|events| OUT
    SVC -.->|dead letters| DL
  class producers ext
  class parse,unwrap,transform,serialise part
  class sink,dlq bus
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
  style SVC fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style S1 fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style S2 fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style OUT fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style DL fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
```

*Figure 1. L3 Components -- dfe-transform-elastic. Dashed boxes are outside DFE, and the dotted arrow is the dead-letter path.* Up: [L2 Services -- DFE deployment](https://github.com/hyperi-io/dfe-infra/blob/main/docs/architecture.md). Down: [L4 Detail -- Unwrap envelope](envelopes.md), [L4 Detail -- Transform](transform.md), [L4 Detail -- One batch through the service](delivery.md#how-it-works).

## How data flows

1. A block arrives from Kafka or a Push listener on port 6000. Parse NDJSON splits it into events. A line that isn't JSON is a dead letter.
2. Unwrap envelope reads which producer wrapped each event and unwraps it into the bare event the transforms expect.
3. Transform runs the source's generated pipeline, its processors in order. An event that fails with no handler is dropped and counted.
4. Serialise gives each event its own record. One over `sink.max_message_bytes` goes to the DLQ.
5. The sink takes the block, and the source is acknowledged.

The source name in the config picks one transform at startup. In agentless mode a detector picks it per event instead, for one syslog stream of many device types ([configuration.md](configuration.md#agentless-mode-serves-one-syslog-stream-of-many-device-types)).

## Components

| Part | Role | Page |
|---|---|---|
| Parse NDJSON | Splits a record into events, and dead-letters what isn't JSON | [delivery.md](delivery.md#how-it-works) |
| Unwrap envelope | Detects the producer family per event and unwraps it | [envelopes.md](envelopes.md) |
| Transform | Runs the source's Elastic pipeline as compiled Rust | [transform.md](transform.md) |
| Serialise | One record per event, under the size budget | [delivery.md](delivery.md#interfaces) |

## External dependencies

| Dependency | Needed for | Swappable |
|---|---|---|
| Kafka | The bus transport, in and out | Any broker the scalo transport reaches. Direct needs none |
| A DFE service over gRPC | The direct sink | dfe-loader or another stage's Push listener |
| Dead-letter queue | Non-JSON input and oversize records | A file DLQ by default, or Kafka, through `dlq` |
| GeoIP database (MMDB) | The `geoip` processor | Downloaded from DB-IP Lite by default, or any MMDB scalo's `geoip` config names ([enrichment.md](enrichment.md)) |

## Deployment variants

| Variant | What runs |
|---|---|
| Kubernetes `single`, `scale`, Docker `single` | On by default. dfe-engine runs one instance per source that binds an Elastic transform, and removes it once no source does |
| Other profiles | Not deployed by default |

An instance with no source name or no topics starts and idles. The chart scales on CPU, and dfe-infra's charts add the scaling-pressure composite, never raw Kafka lag. Nothing reloads: every config value needs a restart. [configuration.md](configuration.md) covers where values come from.

## Design considerations

**Security.** There is no interpreter, scripting VM or plugin system. A Painless script runs where a hand-written runner or a known pattern covers it. Anything else is skipped and counted on `painless_unhandled_total`.

**Reliability.** Delivery is at least once. The source acknowledgement is held until the sink takes every event built from a block, so a retried block re-sends what already landed, and consumers downstream must be idempotent. A failure no retry can fix stops the loop with the block unreleased.

**Operations.** Invalid UTF-8 is decoded with replacements and counted. An event that won't transform is dropped and counted, and the batch carries on. `dfe-transform-elastic sources` lists every source a build has, and a config naming one it lacks fails at startup.

**Performance.** The transform is resolved once, so there's no lookup per event. Grok runs on compiled regex, with a native path for three whole-pattern forms ([parsers.md](parsers.md)). [performance.md](performance.md) has the benchmarks.

## Why it's built this way

**Compiled, not interpreted.** Each Elastic ingest pipeline is generated into a Rust module. A pipeline change is a release, which is the trade for no per-event interpretation. For a transform that has to change without a release, use dfe-transform-vrl.

**Same output as Elastic.** The goal is the ECS output Elastic's pipeline gives for each source, scored per source ([parity.md](parity.md)). Correct beats bug-compatible ([compat-policy.md](compat-policy.md)).

**Envelope per event.** A scalo batch spans partitions, so one batch can hold two producers' wrappers. A few top-level key checks per event cost far less than the unwrap itself.

**One record per event.** dfe-loader parses one JSON document per message ([#67](https://github.com/hyperi-io/dfe-transform-elastic/issues/67)).

## Diagram map

- [Figure 1. L3 Components -- dfe-transform-elastic](architecture.md)
  - [L4 Detail -- Unwrap envelope](envelopes.md)
  - [L4 Detail -- Transform](transform.md)
    - [L4 Detail -- A failed processor](transform.md#scaling-and-failure)
    - [L4 Detail -- Event and Transform types](transform.md#interfaces)
    - [L4 Detail -- Grok dispatch](parsers.md)
    - [L4 Detail -- Enrichers](enrichment.md)
  - [L4 Detail -- One batch through the service](delivery.md)
  - [L4 Detail -- Floors and scanners](testing.md)
  - [L4 Detail -- Crate dependencies](../CONTRIBUTING.md#crates-and-modules)

## What's next

- [config.example.yaml](../config.example.yaml) and [configuration.md](configuration.md), the config and where its values come from
- [parity.md](parity.md), what the service promises against Elastic's output
- [docs/README.md](README.md), every page in this directory
- [CONTRIBUTING.md](../CONTRIBUTING.md#working-in-this-repo), the crates, the generated artefacts and the commands that test a change
