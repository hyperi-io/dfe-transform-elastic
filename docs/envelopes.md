# Unwrap envelope

Unwrap envelope is the step in [dfe-transform-elastic](architecture.md) that reads which producer wrapped each event and hands the transform the bare event it was written for.

| | |
|---|---|
| Repo | [hyperi-io/dfe-transform-elastic](https://github.com/hyperi-io/dfe-transform-elastic), `src/envelope.rs` and `src/registry.rs` |
| Image and chart | `ghcr.io/hyperi-io/dfe-transform-elastic`, and the thin chart each release assembles from `src/deployment.rs` |
| Config reference | `source.envelope` in [config.example.yaml](../config.example.yaml) |
| Endpoints | None of its own |
| Metrics | `envelope_unaccepted_total`, `envelope_contradicted_total` on `:9090/metrics` |

`source.envelope` is `auto` (the default), `beats`, `receiver` or `fetcher`.

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
    accTitle: L4 Detail -- Unwrap envelope
    accDescr: Events arrive in one of three producer envelopes: Beats, dfe-receiver or dfe-fetcher. Unwrap envelope resolves the family, per event under auto or pinned by config, and unwraps it into the bare event. The bare event goes to the transform.

    subgraph PATH["Event path"]
        direction TB
        subgraph FAM["Producer envelopes"]
            direction LR
            beats("<b>Beats</b><br/><span>payload in message</span>")
            receiver("<b>dfe-receiver</b><br/><span>any transport</span>")
            fetcher("<b>dfe-fetcher</b><br/><span>top-level payload</span>")
            beats ~~~ receiver ~~~ fetcher
        end

        subgraph UW["Unwrap envelope"]
            direction LR
            resolve("<b>Resolve family</b><br/><span>per event, or pinned</span>")
            unwrap("<b>Unwrap</b><br/><span>to the bare event</span>")
            resolve --> unwrap
        end

        subgraph NEXT["Next step"]
            direction LR
            transform("<b>Transform</b><br/><span>the source's pipeline</span>")
        end

        FAM --> UW
        UW --> NEXT
    end
  class beats ext
  class receiver,fetcher ingest
  class resolve,unwrap,transform part
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
  style PATH fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style FAM fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style UW fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
  style NEXT fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
```

*Figure 1. L4 Detail -- Unwrap envelope. Beats is outside DFE.* Up: [L3 Components -- dfe-transform-elastic](architecture.md).

## How it works

`auto` resolves the family per event. A scalo `WorkBatch` spans partitions, so one batch can hold two producers' wrappers. The cost is a few top-level key checks, against an unwrap that cost 2,740 ns an event when benchmarked.

A bare payload with no producer marker is the one quiet failure. Detection falls back to `beats`, there is no `message` to unpack, and the transform runs and emits with almost nothing renamed. Output that looks like the input with an `ecs.version` added is this.

## Interfaces

- `beats` -- the raw payload as a string in `message`. The transforms are written for this, and Elastic Agent and anything passing a Beats document through unaltered produce it.
- `receiver` -- [dfe-receiver](https://github.com/hyperi-io/dfe-receiver)'s JSON on any of its transports. The syslog arm rebuilds a line into `message`, and the rest pass their payload through with the receiver's field names moved onto the ECS paths they would shadow.
- `fetcher` -- [dfe-fetcher](https://github.com/hyperi-io/dfe-fetcher)'s JSON, the provider's payload at the top level, for sources where Elastic's agent input is a pure transport.

`registry.rs` records a syslog framing for every source the receiver can deliver:

- `Body` sources, such as `panw` and `cisco_meraki`, read `message` as CSV or key-value, so the body goes through with no header. A tagless PAN-OS line arrives with its first CSV field in `appname`, and when that is not a valid tag the body is rebuilt from `appname`, `procid` and `message`.
- `Line` sources, such as `fortinet`, `cisco_ios` and `cisco_nexus`, grok the header out of `message`. The line is the receiver's opt-in `_raw` where present, else an RFC 3164 line rebuilt from the parsed fields. That is enough for `fortinet`, which wants `<PRI>`, and not for `cisco_ios` or `cisco_nexus`, which want a source IP and a sequence number the receiver does not keep.

## Scaling and failure

A pinned envelope a source cannot arrive in is rejected at startup against the source's `Intake`. okta is pulled from an API, so it accepts `beats` and `fetcher` and rejects `receiver`. `auto` has no event to check at startup, so the same mismatch is counted per batch on `envelope_unaccepted_total`.

## Key metrics

| Metric | Healthy when |
|---|---|
| `envelope_unaccepted_total` | Zero. A count is an envelope the source can't arrive in |
| `envelope_contradicted_total` | Zero. A count is a batch that did not look like the pinned envelope |

## Related

- [Transform](transform.md), which runs on the bare event
- [configuration.md](configuration.md), agentless mode and the transports
