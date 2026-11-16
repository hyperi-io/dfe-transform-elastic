# compat -- comparing output against Elastic's

Parity is not scored in this repository. What this repository holds is `tests/compare-policy.yaml`, which `dfe_runtime::testutil::policy` reads to decide which differences between this service's output and Elasticsearch's are defects. [compat-policy.md](compat-policy.md) covers its sections and the divergences kept on purpose.

## Known limits

- o365 and panw depend on processing inside Filebeat, before the ingest pipeline runs: o365 has a JavaScript processor chain, and panw applies `decode_csv_fields` to `message`. The transforms implement the ingest pipeline half.
- GeoIP resolves against DB-IP Lite, not MaxMind GeoLite2, so geo fields can differ from Elastic's, and `compare-policy.yaml` excludes them.
