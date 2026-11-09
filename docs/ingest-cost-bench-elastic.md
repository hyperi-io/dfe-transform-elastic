# What Elasticsearch ingest costs

The parse is the cheap bit. The tier you run it on is where the cost adds up.

Up to 16x less. That's the path from Elastic's own ingest to DFE 2.2, in two steps:

- **Up to 4x** moving parsing off Elasticsearch and OpenSearch ingest onto an autoscaled DFE 2.1 pipeline. Proven in production.
- **Up to 4x again** moving that pipeline to purpose-built Rust in DFE 2.2. Proven in production, and up to 3.7x in this bench.

Against Elastic's own ingest, for the parse alone:

- **Up to 17x less memory** for the same throughput, and never less than 5x. That holds wherever the parsed data goes next: ClickHouse, or back into Elasticsearch or OpenSearch.
- **Up to 5x less CPU** over a working year, once the ingest tier is sized for the peak.
- **Zero wrong fields** against Elastic's own output, on every source tested.

An external pipeline, at least 5x lighter on memory than the stack's own ingest. The detail is below.

This page keeps three kinds of claim apart: what we MEASURED, what we DERIVED from those numbers, and what we only ESTIMATE.

What we tested comes down to one question. What does it cost to turn raw vendor lines into ECS ('the field names Elastic's dashboards and rules expect')?

Three engines, the same rows, the same four pinned cores, on 2026-11-09:

- dfe-transform-elastic v1.1.27, Kafka in and Kafka out.
- The DFE 2.1 Vector VRL templates, Kafka in and Kafka out.
- Elasticsearch 9.5.6 running Elastic's own integration pipelines. Once through `_simulate` ('run the pipeline, index nothing') and once through `_bulk` ('run the pipeline, then index').

Four sources, one per parser family: `cisco_ios`, `crowdstrike_falcon`, `okta_system` and `fortinet_fortigate`. Each ran clean and with 20% of rows corrupted. The full method, the per-run numbers and how to re-run it are in [benchmark.md](benchmark.md).

Why both Elasticsearch modes? `_simulate` is the parse alone, the share an ingest node pays. `_bulk` adds indexing, which in this bench ran on the same node.

## MEASURED

Records per CPU-second, clean rows, mean of three runs:

| engine | cisco_ios | crowdstrike_falcon | okta_system | fortinet_fortigate | peak memory | cores used of 4 |
|---|---|---|---|---|---|---|
| dfe-transform-elastic | 12,502 | 8,510 | 6,154 | 8,102 | 222-622 MiB | 0.97-1.05 |
| Vector VRL | 3,644 | 2,385 | 2,336 | 2,870 | 512-1,023 MiB | 3.87-3.96 |
| Elasticsearch `_simulate` | 6,656 | 6,995 | 3,613 | 3,615 | 5.1-5.7 GiB | 3.03-3.51 |
| Elasticsearch `_bulk` | 3,427 | 2,387 | 1,289 | 2,247 | 5.1-5.7 GiB | 3.61-3.88 |

The limits, stated plainly:

- The host was shared. Load before a run was 4.6-20.3, so read a ratio as good to about 20%. The worst cell, dfe-transform-elastic on `cisco_ios`, spread 8,207-15,080.
- Elasticsearch ran as one node with every role. Its memory figure is a fixed 4 GiB heap plus the JVM, so it tracks the heap, not the load.
- dfe-transform-elastic ran under an 8 GiB container limit, not its 512Mi pod limit.
- Output was checked for dfe-transform-elastic only: zero wrong fields against Elasticsearch on all four sources. Vector's output wasn't compared.

## DERIVED

Worked out from the table above. Nothing new was run.

- Against the 2.1 Vector pipeline, dfe-transform-elastic uses 2.6-3.6x less CPU per record.
- Against an Elasticsearch ingest node (`_simulate`), it uses 1.2-2.2x less. Against parse plus index (`_bulk`), 3.6-4.8x less.
- Those are the clean rows in the table above. With 20% of rows corrupted, the top ends reach 3.7x and 4.9x (per source in [benchmark.md](benchmark.md)).
- Memory for the same throughput (peak MiB per record a second): dfe-transform-elastic needs 5.0-17.1x less than an Elasticsearch ingest node, and 1.1-2.4x less than Vector. Vector needs 2.6-9.9x less than the Elasticsearch node, 4.4-4.6x on most sources. Elasticsearch's figure is its fixed heap, so read it as the memory you provision, not what the parse uses.
- Vector is slower than Elasticsearch's own pipeline at the parse alone: 0.34-0.79x of `_simulate`. So the first step's gain is memory and sizing for the peak, not CPU per record. The production section shows it held anyway.
- dfe-transform-elastic uses about one core per instance and scales out with pods. Per pinned core, one instance trails Vector on `okta_system` (1,537 vs 2,314 rec/s).

## PROVEN in production

The bench says Elasticsearch's own pipeline beats Vector at the parse alone. Production said the opposite, and by a lot.

On DFE 2.1, parsing Filebeat and Winlogbeat data in Vector cost less than Elastic's Painless pipelines for existing Elasticsearch and OpenSearch clusters. Two live deployments proved it, a large telco and a large-scale government agency. Each ran OpenSearch at 10-100 TB a day as the analyst discovery service, a sidecar to the ClickHouse instance doing the heavy lifting. (The DFE console replaces that sidecar in 2.2.) Running both side by side gave us a bake-off of the two approaches as a bonus.

The PB-scale tests proved it again. We ran two tests on DFE 2.1 to prove its schema models and ingest scale, the design that became DFE 2.2. That design moved the hot path out of Vector into purpose-built Rust, which is dfe-transform-elastic.

The bench is one node at a steady rate. A real cluster pays for the tier the parse runs on, sized for the peak.

## Scaling with the load

dfe-transform-elastic scales with the data. KEDA ('the autoscaler that watches the backlog') adds pods when input queues up and removes them when it drains.

Dedicated ingest nodes can't do that. On Elastic Cloud on Kubernetes, Elastic Cloud Hosted and Elastic Cloud Enterprise, autoscaling covers data tiers and machine learning nodes only. A node with just the ingest role isn't autoscaled. On Hosted, data tiers only scale up, and not on CPU or memory. So ingest nodes get sized for the peak and sit there at the lows. Elastic Cloud Serverless is the exception: it scales to usage.

Sources: https://www.elastic.co/docs/deploy-manage/autoscaling and https://github.com/elastic/cloud-on-k8s/issues/6236

## ESTIMATE: what scaling saves over a working year

**An estimate from a stated load profile, not a measurement.**

Security and ops data follows the working day. Take a typical Australian organisation, as an hourly weekday share of peak: 25% from midnight to 6am, then 40, 70, 100, 100 (morning logins), 85, 80, 70 (lunch), 100, 100 (back from lunch), 85, 70, 55, 40 and 30, then 25% from 8pm. Weekends and 11 weekday public holidays (8 national, about 3 state) sit at 25% all day.

Over a year that averages 44% of peak, and only 11% of hours run at 100%.

Fixed ingest nodes are sized for the peak, so they carry 1 / 0.44 = 2.26x the capacity the load needs on average. The transform scales to the load, so it doesn't. Give both the same headroom and it cancels out.

That multiplies the per-record saving:

| | per record (MEASURED) | over the year (ESTIMATE) |
|---|---|---|
| dfe-transform-elastic vs an Elasticsearch ingest node | 1.2-2.2x less CPU | about 2.8-5.1x less provisioned CPU |
| dfe-transform-elastic vs the 2.1 pipeline | 2.6-3.6x less CPU | the same 2.6-3.6x (2.1 autoscaled too) |

Your own profile changes the 2.26x. Divide peak by mean load and multiply.

## ESTIMATE: what a real cluster adds

**This is an estimate, not a measurement.** None of it is in the numbers above. It's counted from the documented write path, and a multi-node run is the only way to size it.

```mermaid
flowchart LR
    S[Shipper: Logstash, Beats, Elastic Agent] --> I[Ingest node: parse]
    I --> P[Data node: primary shard, index]
    P --> R[Data node: each replica, index again]
```

- Ingest nodes are cluster members. Each holds cluster state, takes part in fault detection and runs its own JVM heap and GC, busy or not. For heavy ingest, Elastic recommends dedicated ingest nodes.
- Every parsed document crosses the cluster again, to its primary shard's node.
- Every replica gets the document over the network and indexes it again. With `n` replicas that's `1 + n` hops and `1 + n` indexes per record.
- A Logstash shipper is one more JVM fleet in front.

OpenSearch does the same by default, so treat the two as one. Two newer models change the replica cost. OpenSearch segment replication (2.7 on, set when the index is created) indexes once and copies finished segments. Elastic Cloud Serverless writes segments to object storage, and search nodes read them from there. Self-managed Elasticsearch keeps the model above.

DFE has hops too: receiver, Kafka or gRPC, transform, loader, ClickHouse and its replicas. But the transform's Kafka read and write are inside its measured CPU, and no DFE stage holds a cluster seat just to parse.

The replication models above come from:

- https://www.elastic.co/search-labs/blog/elasticsearch-serverless-stateless-architecture
- https://www.elastic.co/search-labs/blog/thin-indexing-shards-elasticsearch-serverless
- https://docs.opensearch.org/latest/tuning-your-cluster/availability-and-recovery/segment-replication/index/
- https://opensearch.org/blog/remote-backed-storage/

No ingest tier. That's the saving.
