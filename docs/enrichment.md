# Enrichment

GeoIP, user agent and community ID: the three runtime enrichers loaded once and shared across
every transform. The code map around them is [architecture.md](architecture.md), and the
processors that call them are listed in [parsers.md](parsers.md).

Runtime enrichment modules loaded once, shared across transforms:

```mermaid
flowchart TD
    subgraph "Enrichment Runtime (dfe-runtime)"
        geoip[GeoIP<br/>MMDB via maxminddb<br/>mmap + bounded cache]
        ua[User Agent<br/>Regex-based<br/>UA parsing]
        cid[Community ID<br/>v1 hash<br/>algorithm]
    end

    subgraph "Data Sources"
        mmdb[(DB-IP Lite city mmdb<br/>provisioned by scalo)]
        uareg[(UA regex<br/>patterns)]
    end

    event[Event] -->|ip field| geoip
    event -->|ua field| ua
    event -->|network fields| cid

    mmdb --> geoip
    uareg --> ua

    geoip -->|geo.*| event
    ua -->|user_agent.*| event
    cid -->|network.community_id| event
```

---

## Enrichment trait

```rust
pub trait Enrichment: Send + Sync {
    fn enrich(&self, event: &mut Event) -> Result<()>;
}
```

---

## GeoIP design

- **Provisioning:** the config's `geoip` section is scalo's `GeoIpConfig` verbatim, because provisioning is shared across the fleet while the lookup engine and its cache stay in `dfe-runtime`. It defaults to ENABLED with auto-download from DB-IP Lite, so an operator gets databases without configuring anything, and it is read once at startup.
- **Storage:** mmap'd MMDB via `maxminddb` crate (zero-copy read)
- **Cache:** a bounded cache in front of the MMDB reader (`crates/dfe-runtime/src/enrichment/geoip_cache.rs`), 100,000 entries by default. When full it evicts the oldest-stored quarter, not the least recently used: a hit takes only the read lock and updates no recency. A generated data-stream module can carry several geoip calls, so one event can hit the cache several times.
- **Output fields:** `country_name`, `country_iso_code`, `city_name`, `location.lat`, `location.lon`, `continent_name`, `timezone`
- **Failure:** `ignore_missing` support, errors tagged not fatal

---

## User agent design

- **Parser:** Regex-based UA string parsing
- **Output fields:** `name`, `version`, `os.name`, `os.version`, `device.name`, `device.type`

---

## Community ID design

- **Algorithm:** Community ID v1 (SHA-1 based network flow hash)
- **Input fields:** `source.ip`, `destination.ip`, `source.port`, `destination.port`, `network.transport`
- **Output:** `network.community_id` (e.g., `1:LQU9qZlK+B5F3KDmev6m5PMibrg=`)
