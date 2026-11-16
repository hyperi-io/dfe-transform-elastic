# compat -- measuring against the Elastic engine

compat runs a source's sample lines through that source's ingest pipeline in a throwaway Elasticsearch container and keeps the documents the engine returns. The corpus test scores each transform against them field by field. The corpus and its test are not part of this repository.

What a difference means, and which differences are kept on purpose, is [compat-policy.md](compat-policy.md).

Elasticsearch's output is the reference, and the score is per source.

```mermaid
flowchart LR
    accTitle: L4 Detail -- How a transform is scored
    accDescr: Sample lines go through the source's ingest pipeline in Elasticsearch, which returns confirmed documents. The same lines go through the transform, and the two outputs are compared field by field under the compare policy.

    samples[("Sample lines")]

    subgraph engines["Both engines"]
        es["Elasticsearch<br/>ingest pipeline"]
        transform["The transform"]
    end

    compare["Compare<br/>per field"]
    score(["Per-source<br/>score"])

    samples --> engines
    engines --> compare
    compare --> score

    classDef default fill:#F8FAFC,stroke:#64748B,stroke-width:1px,color:#0F172A
    classDef core fill:#DCEBF7,stroke:#0072B2,stroke-width:2px,color:#0B2A3F
    classDef store fill:#DDF3EC,stroke:#00785A,stroke-width:1px,color:#0B3327
    classDef ext fill:#EEF0F2,stroke:#6B7280,stroke-width:1px,stroke-dasharray:4 3,color:#1F2937
    classDef group fill:none,stroke:#64748B,stroke-width:1px,stroke-dasharray:2 4
    class samples store
    class es ext
    class transform core
    class engines group
```

*Figure 1. L4 Detail -- How a transform is scored. The dashed box is external.* Up: [L3 Components -- dfe-transform-elastic](architecture.md#four-steps-between-the-producers-and-the-sink).

The comparison reads `tests/compare-policy.yaml` through `dfe_runtime::testutil::policy`, so the rules live here beside the runtime.

## Known limits

- o365 and panw are incomplete. Part of their transformation runs inside Filebeat before Elasticsearch sees the event: o365 has a JavaScript processor chain, and panw applies `decode_csv_fields` to `message`. The simulate API reaches neither, so for those two the confirmed output covers the Elasticsearch half.
- GeoIP is not compared. The corpus test turns its GeoIP enricher off and excludes GeoIP-derived fields. An event can still have geo fields its source supplied.
- The engine never ships. Elasticsearch is a development dependency of the measurement, never part of the service.
