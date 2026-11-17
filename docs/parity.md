# What dfe-transform-elastic does

dfe-transform-elastic turns Beats and Elastic Agent events, and syslog from firewalls, switches and other devices, into the ECS output Elastic's ingest pipeline gives for the same integration. Field names, types and enrichment match, apart from the differences named below, so dashboards, rules and queries built for that output keep working. Where Elasticsearch's output is wrong, you get the correct value instead ([compat-policy.md](compat-policy.md#correct-beats-bug-compatible)).

It is deliberately not a reverse-engineered copy of the Elasticsearch pipeline engine. Elastic runs each pipeline on an ingest node as a chain of generic processors, with Painless scripts and grok and regex doing most of the parsing, and every event pays that CPU cost. dfe-transform-elastic compiles each integration's pipeline into native Rust ahead of time. The output is the same or better, and the hot path pays only for the work each event needs.

## Pick what each source pays for

Each level costs more CPU than the one above it, and each source sits at whichever level it needs:

- No transform: the source names none, and its events land as sent. Nothing is parsed, so this costs least.
- One integration: the source names it, such as `filebeat.cisco_asa.default`, and every event runs that one compiled pipeline.
- Agentless: one syslog stream carries many device types. A detector reads a marker off each line, runs the matching integration and stamps the source. A line no marker claims passes through unparsed ([configuration.md](configuration.md#agentless-mode-serves-one-syslog-stream-of-many-device-types)).

## How each part of an ingest pipeline runs

| In Elastic | In dfe-transform-elastic |
|---|---|
| Ingest processors | Compiled Rust per processor, in the pipeline's order. Every processor a shipped pipeline uses is covered ([parsers.md](parsers.md#processor-taxonomy)) |
| Grok | Each pattern is compiled once and reused. Whole-line patterns skip the regex for a native parser: 29 ns against 214 ns for an IPv4 address, 11 ns against 1,098 ns for a whole line ([parsers.md](parsers.md)) |
| Painless scripts | A script with a Rust equivalent runs as Rust. One without is skipped, so the fields it would set are missing. `painless_handled_total` and `painless_unhandled_total` count both ([parsers.md](parsers.md#painless-coverage)) |
| Pipeline calls | The called pipeline is built into the caller, so there is no call per event |
| `foreach` | A loop |
| Conditions | A native `if`. A condition with no Rust equivalent never runs the processor it guards |
| `on_failure` | Runs as Elastic runs it, beside `ignore_failure` |
| GeoIP | One MMDB lookup with a bounded cache, on DB-IP Lite in place of MaxMind GeoLite2, so geo fields can differ from Elastic's |
| User agent | Compiled regexes, with small differences from Elastic's parser |
| Community ID | The same flow hash |

Elastic keeps the raw line in `event.original` beside the parsed fields. Here it is kept once:

- An event tagged `preserve_original_event`, Elastic's own opt-in, keeps `event.original`.
- An event that parsed into fields drops it, because the fields carry the content.
- An event where nothing parsed gets the line back in `message`, counted on `events_unparsed_original_restored_total`.

## What runs elsewhere

Some of Elastic's work happens before or after the ingest pipeline, and in DFE it sits with another component:

| Elastic | In DFE |
|---|---|
| Collection by a Beat or Agent | The Beat or Agent still collects. DFE reads what it sends |
| Syslog and CEF transport parsing | dfe-receiver, or the Beat or Agent |
| Agent-side processors such as `add_host_metadata` | Run on the source machine, so the fields arrive set |
| `enrich` processor | dfe-loader's enrichment columns |
| `reroute` and `final_pipeline` | Kafka topic routing |
| `@custom` pipelines and the `inference` processor | Not run |

o365 and panw need Filebeat's own processors upstream. Part of their parsing runs inside Filebeat, before the ingest pipeline ([compat.md](compat.md#known-limits)).
