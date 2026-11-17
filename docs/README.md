# docs

Everything deeper than the [README](../README.md). Start at [architecture.md](architecture.md) for the parts and how data flows, and at [parity.md](parity.md) to see what it promises against Elastic's output.

| Page | What it is for |
|---|---|
| [architecture.md](architecture.md) | The parts, how data flows, and why it is built this way |
| [envelopes.md](envelopes.md) | Unwrap envelope: the three producer envelopes and the syslog framing |
| [transform.md](transform.md) | Transform: the generated pipeline, the `Event` model and what a failed processor does |
| [delivery.md](delivery.md) | One batch through the service, the sink's answers and the dead letters |
| [configuration.md](configuration.md) | Where the values come from, which env spelling reaches a `--config` file, the `bus`/`direct` transport, agentless mode, and the sections the file cannot hold |
| [parity.md](parity.md) | What the service promises against Elastic's output, what it leaves alone, and how parity is checked |
| [parsers.md](parsers.md) | The grok dispatch, the parsers in `dfe-parse`, every ingest processor the transforms use, and how far the Painless matchers reach |
| [enrichment.md](enrichment.md) | GeoIP, user agent and community ID, the three runtime enrichers |
| [performance.md](performance.md) | SIMD, borrowed values, the release profile, the benchmarks and the key dependencies |
| [testing.md](testing.md) | The floors, the scanners and the shared Kafka broker |
| [compat.md](compat.md) | The compare policy this repository holds, and the known limits |
| [compat-policy.md](compat-policy.md) | What counts as a difference: the `compare-policy.yaml` sections and the divergences kept on purpose |
| [schema-generation.md](schema-generation.md) | The design for publishing the Elastic-side field knowledge to dfe-schemas |
| [shared-crates.md](shared-crates.md) | What dfe-transform-splack took a copy of, and what extracting the common half would take |

`dfe-transform-elastic generate-artefacts --output-dir docs` writes everything else in this directory: `metrics-manifest.json`, `deployment-contract.json`, `container-manifest.json`, `Dockerfile.runtime`, `config-schema.{json,yaml}` and `capability-catalog.{json,yaml}`. Re-run the command rather than editing them.
