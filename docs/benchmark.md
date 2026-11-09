# Benchmark

How dfe-transform-elastic compares with the DFE 2.1 Vector VRL pipeline and with Elasticsearch ingest, on one corpus per source, and the numbers from the last run. The rig is `scripts/bench/three_way.py`. Results are in `scripts/bench/results/`, one JSON file per engine, source and mode, with every run kept.

## What is compared

Three engines parse the same raw lines into ECS documents.

| Engine | What runs | Pin |
|---|---|---|
| dfe-transform-elastic | The release image `ghcr.io/hyperi-io/dfe-transform-elastic:v1.1.27`, Kafka in, Kafka out, GeoIP off | image label `io.hyperi.optimized=true`, `io.hyperi.profile=production` |
| Vector VRL | The 2.1 templates `101` flatten, `125` route, and the source's own `141`, `153`, `154` or `155` transform, from dfe-vector-templates at `96eb306`, Kafka in, Kafka out | `timberio/vector:0.48.0-debian`, the tag dfe-core's Helm values ran (the templates' own test image is 0.47.0) |
| Elasticsearch ingest | The elastic/integrations pipelines at the commit the compat corpus came from, by `_bulk` indexing and by `_simulate` | integrations `2c934eb5`, Elasticsearch 9.5.6 |

The Vector chain stops where the Elastic pipeline stops. The 2.1 timestamp, enrichment and ClickHouse remap templates that follow it are not run. Only the source under test is wired to its route, as the templates' own tests do, so the route fan-out a full 2.1 deployment pays across its ten sources is not measured. Both choices favour Vector.

Vector's Kafka source and sink keep the librdkafka options of `003-source-kafka-sasl-scram.yml` and `202-sink-kafka-sasl-scram.yml`. SASL and the 19 GB disk buffer are dropped for the bench broker.

## Corpus

Four sources, one per parser family: `cisco_ios` (grok over syslog), `fortinet_fortigate` (key=value), `okta_system` and `crowdstrike_falcon` (JSON in `message`). The inputs are the `input.ndjson` files of the compat corpus (`testdata/compat/`, generated from Elastic's pipeline-test fixtures, Elastic License 2.0, gitignored), cycled to the row count the run needs. The corpus is not in this repository. Point `--compat` at a generated one.

The malformed set corrupts 20% of the rows, chosen with a fixed seed (`CORRUPT_SEED`). Each chosen row's `message` is cut at 20-80% of its length and a `\x1f}{` run is appended. The envelope stays valid JSON, so the damage reaches each engine's own parser. All three engines read the same rows.

## Method

- Every engine runs in a container with `--cpuset-cpus 24-27`, four cores. Redpanda has its own four cores. The Elasticsearch client and `rpk` run on the rest.
- Kafka engines read a topic of 4 partitions loaded beforehand, and write to a topic on the same broker. Elasticsearch gets the same rows over HTTP from 4 client threads in chunks of 2,000.
- A cumulative processed-record counter is read every 0.2 s: `events_transformed_total` plus errored and dropped for dfe-transform-elastic, Vector's sink `sent_events_total` plus discarded, and completed documents for Elasticsearch. The rate is taken between the 10% and 90% marks, which leaves out the consumer join, JIT warm-up and the drain tail.
- CPU time is the container cgroup's `cpu.stat usage_usec` over the same window. Peak memory is the cgroup's `memory.peak`, and for the binaries the process's `VmHWM`.
- One warm-up run, then three measured runs per cell. A run is dropped and repeated when the 1-minute load average before or after it is above the cores not pinned (28 of 32), or when it did not reach its record count.
- Timed runs hold `flock ~/.dfe-bench.lock` (`--lock` to change it). Each result row carries the load averages and cpu count from before and after.

dfe-transform-elastic used 0.96 to 1.05 of the four pinned cores in every clean run, and its pod contract requests `100m` of CPU with a `500m` limit and 512Mi of memory. Vector and Elasticsearch use all four. So the headline unit is records per CPU-second, which is per core actually used. Records per second per pinned core is in the raw files and the tables.

## Results

Measured 2026-11-09 on a shared build host (AMD EPYC 7302 VM, 32 vCPU, 1 thread per core, kernel 7.0.0-42, Docker 29.9.3). Each figure is the mean of three measured runs, with the minimum and maximum beside it. The host was shared, so the load average before a run ranged from 4.6 to 20.3. No run tripped the noise gate, and none stalled.

Records per CPU-second, clean (malformed in brackets):

| engine | cisco_ios | crowdstrike_falcon | okta_system | fortinet_fortigate |
|---|---|---|---|---|
| dfe-transform-elastic | 12,502 (15,859) | 8,510 (9,658) | 6,154 (7,357) | 8,102 (8,555) |
| Vector VRL | 3,644 (4,335) | 2,385 (2,707) | 2,336 (2,791) | 2,870 (3,100) |
| Elasticsearch `_bulk` | 3,427 (4,007) | 2,387 (2,708) | 1,289 (1,499) | 2,247 (2,275) |
| Elasticsearch `_simulate` | 6,656 (7,025) | 6,995 (7,360) | 3,613 (4,239) | 3,615 (3,825) |

Ratios of those means, clean and malformed within 0.1x of each other except where shown:

| | cisco_ios | crowdstrike_falcon | okta_system | fortinet_fortigate |
|---|---|---|---|---|
| dfe-transform-elastic over Vector | 3.4x (3.7x) | 3.6x | 2.6x | 2.8x |
| Vector over Elasticsearch `_bulk` | 1.1x | 1.0x | 1.8x (1.9x) | 1.3x (1.4x) |
| dfe-transform-elastic over Elasticsearch `_bulk` | 3.6x (4.0x) | 3.6x | 4.8x (4.9x) | 3.6x (3.8x) |
| Vector over Elasticsearch `_simulate` | 0.55x (0.62x) | 0.34x (0.37x) | 0.65x (0.66x) | 0.79x (0.81x) |

Peak memory in MiB, the largest container figure over every run of a source, and cores used of the four pinned, clean:

| engine | cisco_ios | crowdstrike_falcon | okta_system | fortinet_fortigate | cores used |
|---|---|---|---|---|---|
| dfe-transform-elastic | 305 | 378 | 622 | 222 | 0.97-1.05 |
| Vector VRL | 733 | 946 | 1,023 | 512 | 3.87-3.96 |
| Elasticsearch (both modes) | 5,111 | 5,628 | 5,674 | 5,674 | 3.61-3.88 `_bulk`, 3.03-3.51 `_simulate` |

The bench container had an 8 GiB limit, not the pod's 512Mi, so `okta_system`'s 622 MiB does not show what a 512Mi pod would do. Elasticsearch runs a fixed 4 GiB heap, and its figure is the container's peak since it started, so it tracks the heap more than the workload.

What Elasticsearch costs beyond the parse -- the ingest node tier, the node-to-node copies, and an estimate of a real cluster -- is in [ingest-cost-bench-elastic.md](ingest-cost-bench-elastic.md).

The malformed set is faster for every engine. A line cut at 20-80% of its length is shorter. Why each engine speeds up on it was not tested. No engine dropped a record. dfe-transform-elastic emitted all of them, Vector emitted all of them and counted component errors on 10-20% of the rows, and Elasticsearch indexed all but 61 of 100,000 fortinet rows.

Per-source detail, in the units `report` prints:

| source | engine | rec/CPU-s clean | rec/CPU-s malformed | cores used clean | rec/s per pinned core clean | peak memory MiB | runs clean+malformed |
|---|---|---|---|---|---|---|---|
| cisco_ios | transform-elastic | 12,502 (8,207-15,080) | 15,859 (15,831-15,894) | 0.97 | 3,052 (1,950-3,755) | 305 | 6+3 |
| cisco_ios | vector | 3,644 (3,501-3,905) | 4,335 (4,315-4,351) | 3.87 | 3,530 (3,324-3,855) | 733 | 3+3 |
| cisco_ios | es-bulk | 3,427 (3,203-3,652) | 4,007 (3,866-4,088) | 3.64 | 3,127 (2,759-3,436) | 5,111 | 3+3 |
| cisco_ios | es-simulate | 6,656 (6,208-6,993) | 7,025 (6,964-7,107) | 3.37 | 5,605 (5,231-5,915) | 5,111 | 3+3 |
| crowdstrike_falcon | transform-elastic | 8,510 (8,399-8,592) | 9,658 (9,615-9,679) | 1.05 | 2,239 (2,207-2,264) | 378 | 3+3 |
| crowdstrike_falcon | vector | 2,385 (2,344-2,464) | 2,707 (2,659-2,734) | 3.91 | 2,329 (2,286-2,413) | 946 | 3+3 |
| crowdstrike_falcon | es-bulk | 2,387 (2,323-2,454) | 2,708 (2,639-2,805) | 3.75 | 2,238 (2,180-2,293) | 5,628 | 3+3 |
| crowdstrike_falcon | es-simulate | 6,995 (6,973-7,020) | 7,360 (7,296-7,400) | 3.03 | 5,294 (5,218-5,393) | 5,628 | 3+3 |
| okta_system | transform-elastic | 6,154 (6,014-6,263) | 7,357 (7,260-7,450) | 1.00 | 1,537 (1,511-1,561) | 622 | 3+3 |
| okta_system | vector | 2,336 (2,331-2,342) | 2,791 (2,741-2,859) | 3.96 | 2,314 (2,310-2,319) | 1,023 | 3+3 |
| okta_system | es-bulk | 1,289 (1,283-1,296) | 1,499 (1,471-1,519) | 3.61 | 1,163 (1,158-1,168) | 5,674 | 3+3 |
| okta_system | es-simulate | 3,613 (3,523-3,675) | 4,239 (4,150-4,308) | 3.11 | 2,814 (2,676-2,895) | 5,674 | 3+3 |
| fortinet_fortigate | transform-elastic | 8,102 (7,940-8,313) | 8,555 (8,506-8,649) | 1.05 | 2,119 (2,081-2,169) | 222 | 3+3 |
| fortinet_fortigate | vector | 2,870 (2,849-2,891) | 3,100 (2,886-3,227) | 3.94 | 2,828 (2,810-2,841) | 512 | 3+3 |
| fortinet_fortigate | es-bulk | 2,247 (2,197-2,301) | 2,275 (2,163-2,355) | 3.88 | 2,181 (2,134-2,230) | 5,674 | 3+3 |
| fortinet_fortigate | es-simulate | 3,615 (3,563-3,675) | 3,825 (3,762-3,875) | 3.51 | 3,170 (3,130-3,218) | 5,674 | 3+3 |

## What the numbers do and do not say

dfe-transform-elastic is 2.6x to 3.6x Vector per CPU-second on these four sources, clean, and up to 3.7x with corrupted rows. Vector is 1.0x to 1.9x Elasticsearch `_bulk` indexing. Against Elasticsearch's `_simulate`, which runs the pipeline and nothing else, Vector is slower. `_bulk` pays for indexing a document that the other two engines never do, so it is the right comparison for ingest and the wrong one for the parse step alone.

The numbers move with the host. The first three clean runs of dfe-transform-elastic on `cisco_ios` ran at 8,207 to 12,847 records per CPU-second under a load of 15-20. Three more, after the rest of the matrix had finished and the host was quiet, ran at 12,242 to 15,080. All six are in the mean. Treat a ratio as good to about 20%.

What is not measured: the 2.1 pipeline's stages after the source transform, the route fan-out of a full deployment, a disk buffer, TLS, a multi-pod run, and Elasticsearch with replicas or several shards. Vector's output was not compared field by field with Elastic's. dfe-transform-elastic's output was, through the parity ratchet in `tests/compat-baseline.json`, which holds all four sources at zero wrong fields against Elasticsearch 9.2.2 on this corpus.

## Reproducing

```bash
scripts/bench/three_way.py fetch-pipelines --out work/integrations
scripts/bench/three_way.py corpus --compat <testdata/compat> --out work/corpus --rows 100000
scripts/bench/three_way.py up --es
scripts/bench/three_way.py matrix --plan <plan.json> --results results --vector-templates <dfe-vector-templates>
scripts/bench/three_way.py report --results results
scripts/bench/three_way.py down
```

The host needs Docker on cgroup v2, `rpk`, `gh` for the pipeline fetch, and PyYAML. `matrix` takes each cell under the bench lock and re-runs a cell when a run was noisy. `run --rows 400` against one engine is a functional check of the rig, and its rates mean nothing.
