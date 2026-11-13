# Where these fixtures come from

Every directory here except `unencumbered/` is one data stream's fixtures:
sample lines, their test configs, and beside each sample, as
`<sample>-expected.json`, the documents the real Elastic engine produced for it.
They are this repository's own test data.

They are made by `dfe-transform-elastic-dev`, the project that maintains the
transforms:

- `scripts/rewrite_fixtures.py` writes each sample. Every address, MAC, UUID,
  digest, host and domain name, email address, username, personal name, session
  or request id is a seeded stand-in of the same shape. IPv4 sits in
  192.0.2.0/24, 198.51.100.0/24 and 203.0.113.0/24 or a private block, IPv6 in
  2001:db8::/32, and every dated value is 28 years on. Every other token keeps
  its place: delimiters, quoting, field order, event codes, action words and
  enum values.
- `scripts/fixture_shapes.py` holds the rewrite to that promise: logreducer's
  pattern mode must find the same template set in every data stream before and
  after.
- `scripts/compat.py expect` runs each sample through its pipeline in a real
  Elasticsearch and writes the `-expected.json` beside it.
- `scripts/publish_fixtures.py` copies the result here, with the Beats envelope
  shapes under `tests/envelopes/beats/`.

Change them there and publish again, never by hand here: a hand edit is lost on
the next publish, and its expected documents no longer match what the engine
returns.
