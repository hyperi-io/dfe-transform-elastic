# Testing

The two test tracks, the floors and the scanners, run on every clone, and neither measures parity. The code map is [architecture.md](architecture.md), and what parity means is [parity.md](parity.md).

The floors read the committed fixtures, and the scanners read the generated tree against the ratchet file.

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
flowchart LR
    accTitle: L4 Detail -- Floors and scanners
    accDescr: The floor tests read the committed fixtures. The scanner tests read the generated transforms and compare their counts with the ratchet file. Both run in the CI test job.

    fixtures[("<b>tests/fixtures</b>")]
    tree[("<b>Generated<br/>transforms</b>")]
    ratchets[("<b>ratchets.json</b>")]
    floors["<b>Floor tests</b>"]
    scanners["<b>Scanner tests</b>"]
    ci(["<b>CI test job</b>"])

    fixtures --> floors
    tree --> scanners
    ratchets --> scanners
    floors --> ci
    scanners --> ci
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
  class fixtures,tree,ratchets store
  class floors,scanners,ci part
```

*Figure 1. L4 Detail -- Floors and scanners. Green is committed data.* Up: [L3 Components -- dfe-transform-elastic](architecture.md).

The floors are `integration.rs`, `untested_sources.rs` and `unencumbered.rs`. The scanners are `painless_binding.rs`, `bespoke_registry.rs`, `grok_pattern_coverage.rs` and `skipped_processors.rs`, and `tests/ratchets.json` holds the counts they ratchet, such as `dead_branch_sites` and `skipped_processors`. `tests/compare-policy.yaml` is read through `dfe_runtime::testutil::policy` by the parity comparison.

## The fixtures

`tests/fixtures/` holds two sets, and `tests/fixtures/PROVENANCE.md` says where each comes from:

- `unencumbered/` -- log samples from CC0, MIT and Apache-2.0 projects, under their licences.
- `synthetic/` -- samples written for this repository, one file per log variant, each following the layout the product's maker publishes, with invented values.

Neither has expected output, so the floors assert what holds regardless: no panic, errors at or under each fixture's ceiling, and every event without an error gains a field. `untested_sources.rs` sets its floors per case. An event the pipeline's failure handler marks with `error.message` counts as an error. The Beats envelopes under `tests/envelopes/beats/` are written the same way as the synthetic set.

The fixture tests read the committed data with no variable to set, so they run whenever CI does:

```bash
cargo test -p dfe-transforms --test integration --test untested_sources \
  --test grok_capture_paths --test unencumbered
cargo test -p dfe-runtime --all-features --lib --test date_repro
cargo test --no-default-features --test envelope --test envelope_detection
```

`crates/dfe-transforms/tests/unencumbered.rs` fails when `tests/fixtures/` holds anything but `unencumbered/`, `synthetic/` and the provenance, or when any file there contains one of its `MARKERS` strings.

The test diff helper's `MatchMode` is defined in `crates/dfe-runtime/src/testutil/diff.rs`:

| Mode | Use | Behaviour |
|---|---|---|
| `Exact` | Strict comparison | Field-for-field JSON match |
| `Semantic` | Against Elastic's output | Like `Exact`, skipping every path the compare policy excludes |
| `Subset` | Fields the Beats runtime sets | Expected is a subset of actual, extra fields allowed |

`tests/unicode.rs` runs every registered transform against seventeen samples: scripts, right-to-left text, combining marks, emoji and degenerate inputs such as a byte-order mark mid-string.

## Core and the shared broker

Core is every test, and it runs on every push that triggers CI: the unit tests, the floors and scanners above, `tests/broker.rs` and `tests/kafka_tls.rs` against a broker, and the memory tests (`tests/ndjson_fanout.rs`, `tests/asa_teardown.rs`) held to the deployment contract. No test here is `#[ignore]`d.

Nextest is required (`test.rust.nextest` in `.hyperi-ci.yaml`): the memory tests read the process's peak resident set and `tests/config_cascade.rs` reads scalo's process-wide cascade, so each needs a process to itself.

The Kafka tests share one broker per run, on the image scalo pins as `test_support::KAFKA_IMAGE`. The first test process starts it through scalo's `shared_kafka`, the rest join it by name, and it is removed when the run's nextest process exits, locally and in CI alike. The two tests that stop the broker or raise its record ceiling start one alone (`start_alone`), and `tests/kafka_tls.rs` starts scalo's `TlsBroker`. Each is removed when its test ends. A broker listed in `KAFKA_BROKERS` is used instead where one answers. Without Docker the tests skip locally and fail in CI.

The standard these follow is hyperi-ai's `standards/universal/testing.md`.
