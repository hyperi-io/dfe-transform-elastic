# Enrichment

GeoIP, user agent and community ID are the three runtime enrichers in `crates/dfe-runtime/src/enrichment/`, loaded once and shared across every transform. A generated transform calls one where its pipeline has a `geoip`, `user_agent` or `community_id` processor ([parsers.md](parsers.md#processor-taxonomy)). The step they run in is [Transform](transform.md).

Each enricher reads one kind of field and writes one subtree back onto the same event.

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
flowchart LR
    accTitle: L4 Detail -- Enrichers
    accDescr: The GeoIP enricher reads an IP field and writes geo fields. The user agent enricher reads a user agent string and writes user_agent fields. The community ID enricher reads the flow fields and writes network.community_id.

    ipf["<b>IP field</b>"]
    uaf["<b>User agent<br/>string</b>"]
    flow["<b>Flow fields</b>"]

    subgraph enrichers["dfe-runtime enrichment"]
        geoip["<b>GeoIP</b>"]
        ua["<b>User agent</b>"]
        cid["<b>Community ID</b>"]
    end

    geo["<b>geo.*</b>"]
    uaout["<b>user_agent.*</b>"]
    cidout["<b>network.<br/>community_id</b>"]

    ipf --> geoip
    uaf --> ua
    flow --> cid
    geoip --> geo
    ua --> uaout
    cid --> cidout
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
  class ipf,uaf,flow,geoip,ua,cid,geo,uaout,cidout part
  style enrichers fill:none,stroke:#0072B2,stroke-width:1.5px,stroke-dasharray:6 4
```

*Figure 1. L4 Detail -- Enrichers.* Up: [L4 Detail -- Transform](transform.md).

GeoIP provisioning, storage, cache, output and failure:

- Provisioning: the config's `geoip` section is scalo's `GeoIpConfig` as-is, because provisioning is shared across the fleet, and the lookup and its cache stay in `dfe-runtime`. It defaults to enabled, with auto-download from DB-IP Lite, and is read once at startup.
- Storage: the MMDB is memory-mapped through the `maxminddb` crate.
- Cache: a bounded cache in front of the reader (`geoip_cache.rs`), 100,000 entries by default. When full it evicts the oldest-stored quarter rather than tracking use, so a hit takes the read lock and updates nothing. A transform with several `geoip` processors hits the cache several times an event, counted on `geoip_cache_hits_total` and `geoip_cache_misses_total`.
- Output fields: `continent_name`, `country_name`, `country_iso_code`, `region_name`, `region_iso_code`, `city_name`, `location` (`lat`, `lon`) and `timezone`, plus `asn`, `organization_name` and `network` from an ASN database.
- Failure: none reaches the event. A generated `geoip` call skips an absent field, and a value that is not an IP, a private address or a lookup miss writes no fields (`geoip_global.rs`).

The user agent parser matches pre-compiled regexes against the string and writes `original`, `name`, `version`, `os.name`, `os.version`, `os.full`, `device.name` and, where it can tell, `device.type`. Its output differs from Elastic's ua-parser in small ways.

Community ID is v1, a SHA-1 hash of the flow, read from `source.ip`, `destination.ip`, `source.port`, `destination.port` and `network.transport` or `network.iana_number`, with `icmp.type` and `icmp.code` for ICMP. It writes `network.community_id`, such as `1:LQU9qZlK+B5F3KDmev6m5PMibrg=`.
