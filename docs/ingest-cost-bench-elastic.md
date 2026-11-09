# What Elasticsearch ingest costs

The parse is the cheap bit. The tier you run it on is where the cost adds up.

Elasticsearch parses on ingest nodes. dfe-transform-elastic and Vector parse in one process, with no extra tier. This page keeps three kinds of claim apart: what we MEASURED, what we DERIVED from those numbers, and what we only ESTIMATE.

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
- Vector is slower than Elasticsearch's own pipeline at the parse alone: 0.34-0.79x of `_simulate`.
- dfe-transform-elastic sits on about one core. So one instance is roughly level with Vector per pinned core on `cisco_ios` and behind on `okta_system` (1,537 vs 2,314 rec/s). It's cheap per record but doesn't parallelise. You scale it with pods, and that's ours to fix.

## In production, and why it differs

The bench says Elasticsearch's own pipeline beats Vector at the parse alone. Production said the opposite. On DFE 2.1, parsing Filebeat and Winlogbeat data in Vector was cheaper than Elastic's Painless pipelines for existing Elasticsearch and OpenSearch clusters. That held in production at customers running OpenSearch at 10-100 TB a day, where OpenSearch was the analyst discovery service beside DFE. (The DFE console replaces it in 2.2.) This is field experience, not a number from this bench.

Both are true. The bench is one node at a steady rate. A real cluster pays for the tier the parse runs on, sized for the peak, and the next two sections are that cost.

## Scaling with the load

dfe-transform-elastic scales with the data. KEDA ('the autoscaler that watches the backlog') adds pods when input queues up and removes them when it drains. So it runs few pods at the lows and more at a spike.

Dedicated ingest nodes can't do that. On Elastic Cloud on Kubernetes, Elastic Cloud Hosted and Elastic Cloud Enterprise, autoscaling covers data tiers and machine learning nodes only. A node with just the ingest role isn't autoscaled. On Hosted, data tiers only scale up, and not on CPU or memory. So ingest nodes get sized for the peak and sit there at the lows. Elastic Cloud Serverless is the exception: it scales to usage.

Sources: https://www.elastic.co/docs/deploy-manage/autoscaling and https://github.com/elastic/cloud-on-k8s/issues/6236

## ESTIMATE: what a real cluster adds

**This is an estimate, not a measurement.** None of it is in the numbers above. It's counted from the documented write path, and a multi-node run is the only way to size it.

```mermaid
flowchart LR
    S[Shipper: Logstash, Beats, Elastic Agent] --> I[Ingest node: parse]
    I --> P[Data node: primary shard, index]
    P --> R[Data node: each replica, index again]
```

- Ingest nodes are cluster members. Each holds cluster state, takes part in fault detection and runs its own JVM heap and GC, busy or not. For heavy ingest, Elastic recommends dedicated ingest nodes.
- Every parsed document crosses the cluster again, from the ingest node to its primary shard's node.
- Every replica gets the document over the network and indexes it again. With `n` replicas that's `1 + n` hops and `1 + n` indexes per record.
- A Logstash shipper is one more JVM fleet in front.

OpenSearch does the same by default, so treat the two as one. Two newer models change the replica cost. OpenSearch segment replication (2.7 on, set when the index is created) indexes once and copies finished segments. Elastic Cloud Serverless writes segments to object storage, and search nodes read them from there. Self-managed Elasticsearch keeps the model above.

DFE has hops too: receiver, Kafka or gRPC, transform, loader, ClickHouse and its replicas. But the transform's Kafka read and write are inside its measured CPU, and no DFE stage holds a cluster seat just to parse.

Sizing it needs one dedicated ingest node and two data nodes with one replica, with CPU and network bytes counted per node.

The replication models above come from:

- https://www.elastic.co/search-labs/blog/elasticsearch-serverless-stateless-architecture
- https://www.elastic.co/search-labs/blog/thin-indexing-shards-elasticsearch-serverless
- https://docs.opensearch.org/latest/tuning-your-cluster/availability-and-recovery/segment-replication/index/
- https://opensearch.org/blog/remote-backed-storage/

No ingest tier. That's the saving.
