# Envelope fixtures

One file per inbound shape the service must recognise. Each is the wrapper a
producer actually writes, taken from that producer's own tests or from the
compat corpus -- never invented, because a detector asserted against a guessed
shape proves nothing.

Every file is the same object:

| key | holds |
|---|---|
| `provenance` | the file the shape was read from, and what fixes it there |
| `shape` | what the event actually is |
| `detect` | what `envelope::detect` must return for it |
| `event` | the event itself, as the producer emits it |

`shape` and `detect` differ only where a producer writes no marker. Splitting
them is the point: a fixture recording only the truth would make an undetectable
shape look like a bug, and one recording only the detector's answer would hide
which shapes we are blind to.

`tests/envelope_detection.rs` walks this directory and asserts `detect`, so
adding a shape is adding a file. These are committed, unlike `testdata/`, which
holds the fetched Elastic corpus.

The `beats` shapes are read off Elastic's own test documents, so they are Elastic License 2.0 and live in `dfe-transform-elastic-dev` under `fixtures/elastic/tests/envelopes/beats/`. They join the set where `DFE_ELASTIC_FIXTURES` names that directory, and `the_beats_shapes_load_from_the_elastic_data` fails a run that asked for them and found none.

## The families

| family | who writes it | marker |
|---|---|---|
| `beats` | Beats, Elastic Agent, and anything passed through unaltered | `fileset`, or `data_stream` + `elastic_agent` |
| `receiver` | dfe-receiver, on any transport | `_source`, `_signal`, `_vector_type`, `sourcetype` |
| `fetcher` | dfe-fetcher | `_source_fetcher` |

`beats` is the family name the code uses for everything Elastic-shaped, agent
output included -- see `Envelope` in `src/envelope.rs`.

## Three findings the shapes forced

**The Elastic wrapper does not vary by version -- it varies by input
mechanism.** `panw/panos/traffic` is filebeat 8.13.2 and still carries
`fileset.name` with no `data_stream`, so a beats-7-versus-8 detector would be
answering a question the data never asks. The real split is module output
(`@metadata.beat` + `fileset`) against Fleet integration output (`data_stream` +
`elastic_agent`), with a bare `{message, tags}` under both.

**The receiver's marker is not one key.** Six transports tag `_source`, OTLP
tags `_signal` in its generic mode only, the Vector gRPC source tags
`_vector_type` on metrics and traces, and Splunk HEC, lumberjack, gRPC logs and
OTLP's two ClickHouse modes tag nothing at all.

**Lumberjack is the Beats wire protocol, so its payload is a Beats document.**
It lands in the `beats` family despite arriving through the receiver, which is
why family is decided by what the payload looks like rather than by who
delivered it.
