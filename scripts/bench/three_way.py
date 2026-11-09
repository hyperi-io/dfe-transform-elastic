#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Three-way throughput benchmark: dfe-transform-elastic, Vector VRL, Elasticsearch ingest.

One corpus per source, fed to three engines on the same host, each pinned to the same
cores. Method, scope and results are in docs/benchmark.md.

Runs where Docker is local, which is desktop-derek. Every timed `run` goes inside
`flock /home/derek/.dfe-bench.lock` so no other timed job shares the host with it.

Examples:
    three_way.py corpus --compat testdata/compat --out work/corpus
    three_way.py fetch-pipelines --out work/integrations
    three_way.py up --es
    flock /home/derek/.dfe-bench.lock three_way.py run --engine transform-elastic \\
        --source cisco_ios --mode clean --rows 1000000
    three_way.py report --results results
    three_way.py down
"""

import argparse
import functools
import json
import logging
import os
import queue
import random
import statistics
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request
from collections.abc import Callable, Sequence
from dataclasses import dataclass
from pathlib import Path
from typing import Any

log = logging.getLogger("three_way")

SCRIPT_DIR = Path(__file__).resolve().parent

# --------------------------------------------------------------------------
# Pins
# --------------------------------------------------------------------------

# dfe-transform-elastic release image, anonymous pull. The tag is an argument.
TE_IMAGE = "ghcr.io/hyperi-io/dfe-transform-elastic"

# The Vector the 2.1 data layer ran: dfe-core gitOps/addons/helm/vector/common.yaml
# `image.tag`. dfe-vector-templates' own test Dockerfile pins 0.47.0-debian. Both are
# historical pins, so the latest-release rule does not apply to them.
VECTOR_IMAGE = "timberio/vector:0.48.0-debian"

# Elasticsearch 9.5.6, published 2026-11-02, the newest 9.x on
# api.github.com/repos/elastic/elasticsearch/releases when this was written. The compat
# corpus was captured on 9.2.2.
ES_IMAGE = "docker.elastic.co/elasticsearch/elasticsearch:9.5.6"

# elastic/integrations commit the compat corpus was generated from (meta.json
# `integrations_sha`), so the pipelines are the ones that produced the corpus.
INTEGRATIONS_SHA = "2c934eb5223bdfcf0ea0db9e3230154933352bda"

# The broker pin scripts/pgo-workload.sh already uses, by digest.
REDPANDA_IMAGE = (
    "docker.redpanda.com/redpandadata/redpanda:v26.2.3"
    "@sha256:9e83cfa99278f30d0133271c26bf670cd69c94ffa6ba0b42830dd0c3bd9dcfd9"
)

NET = "bench162-net"
BROKER = "bench162-redpanda"
ES_NAME = "bench162-es"
BROKER_EXTERNAL = "localhost:29092"
BROKER_INTERNAL = f"{BROKER}:9092"
ES_URL = "http://127.0.0.1:29200"

PARTITIONS = 4
SAMPLE_INTERVAL = 0.2
WINDOW_LO = 0.1
WINDOW_HI = 0.9

ENGINES = ("transform-elastic", "vector", "es-bulk", "es-simulate")

ROUTE2 = "125_transform_route_filebeat_events_2"
ROUTE1 = "125_transform_route_filebeat_events"
FLATTEN = "101_transform_flatten_message"


@dataclass(frozen=True, slots=True)
class VrlSource:
    """How the 2.1 Vector templates are chained for one source.

    Attributes:
        templates: Template files in chain order, under `src/core_templates`.
        env: Input variables the templates read, name to input list.
        sink_input: Component id whose output the sink takes.
        wrap: Wrap the raw line as {message, tags, timestamp}, else send the corpus doc.
    """

    templates: tuple[str, ...]
    env: dict[str, str]
    sink_input: str
    wrap: bool


@dataclass(frozen=True, slots=True)
class Source:
    """One benchmarked source across the three engines.

    Attributes:
        compat: Corpus directory under the compat root, `<package>/<data_stream>`.
        es: Elasticsearch package and data stream.
        te: dfe-transform-elastic source name.
        vrl: The Vector chain.
    """

    compat: str
    es: tuple[str, str]
    te: str
    vrl: VrlSource


def _vrl(
    route: str, number: str, name: str, input_var: str, sink: str, *, wrap: bool
) -> VrlSource:
    """Build the chain flatten -> route -> source transform that the template tests use."""
    route_file = {
        ROUTE2: "125-transform-route-filebeat-events-2.yml",
        ROUTE1: "125-transform-route-filebeat-events.yml",
    }[route]
    return VrlSource(
        (
            "101-transform-flatten-message.yml",
            route_file,
            f"{number}-transform-filebeat-{name}.yml",
        ),
        {
            "_101_TRANSFORM_FLATTEN_MESSAGE_INPUT": "[bench_in]",
            f"_{route.upper()}_INPUT": f"[{FLATTEN}]",
            input_var: f"[{route}.{sink}]",
        },
        sink,
        wrap,
    )


SOURCES: dict[str, Source] = {
    "cisco_ios": Source(
        "cisco_ios/log",
        ("cisco_ios", "log"),
        "filebeat.cisco_ios.default",
        _vrl(
            ROUTE2,
            "141",
            "cisco-ios-logs",
            "_CISCO_IOS_LOGS_INPUT",
            "cisco_ios_logs",
            wrap=True,
        ),
    ),
    "crowdstrike_falcon": Source(
        "crowdstrike/falcon",
        ("crowdstrike", "falcon"),
        "filebeat.crowdstrike.default",
        _vrl(
            ROUTE2,
            "153",
            "crowdstrike-falcon-logs",
            "_CROWDSTRIKE_FALCON_INPUT",
            "crowdstrike_falcon",
            wrap=True,
        ),
    ),
    "okta_system": Source(
        "okta/system",
        ("okta", "system"),
        "filebeat.okta.default",
        _vrl(
            ROUTE1,
            "155",
            "okta-system-logs",
            "_OKTA_SYSTEM_INPUT",
            "okta_system",
            wrap=True,
        ),
    ),
    "fortinet_fortigate": Source(
        "fortinet_fortigate/log",
        ("fortinet_fortigate", "log"),
        "filebeat.fortinet.default",
        _vrl(
            ROUTE1,
            "154",
            "fortinet-fortigate-logs",
            "_FORTINET_FORTIGATE_INPUT",
            "fortinet_fortigate",
            wrap=False,
        ),
    ),
}

COLLECTOR_TAGS = {
    "collector": {
        "host": "10.42.5.101",
        "hostname": "test-collector",
        "source": "logstash",
        "timestamp": "2024-05-23T01:13:13Z",
        "timezone": "Australia/Sydney",
    },
    "event": {
        "category": "logs_beats_filebeat",
        "org_id": "hypersecinfra",
        "site_id": "cbr",
        "type": "event.beats",
    },
}

CORRUPT_SEED = 162
CORRUPT_TAIL = "\x1f}{"


def sh(cmd: Sequence[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    """Run a command with a UTF-8 decode that never raises.

    Args:
        cmd: argv, never a shell string.
        check: Raise on a non-zero exit.

    Returns:
        The completed process.
    """
    return subprocess.run(
        list(cmd),
        check=check,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )


def docker(*args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    """Run the local docker CLI."""
    return sh(["docker", *args], check=check)


def container_id(name: str) -> str:
    """Return the full id of a running container."""
    return docker("inspect", "-f", "{{.Id}}", name).stdout.strip()


def cgroup_dir(cid: str) -> Path:
    """Return a container's cgroup v2 directory."""
    path = Path(f"/sys/fs/cgroup/system.slice/docker-{cid}.scope")
    if not path.is_dir():
        raise RuntimeError(
            f"no cgroup at {path}: cgroup v2 with the systemd driver is required"
        )
    return path


def cpu_usec(cg: Path) -> int:
    """Return cumulative CPU time of a cgroup in microseconds."""
    for line in (cg / "cpu.stat").read_text(encoding="utf-8").splitlines():
        if line.startswith("usage_usec "):
            return int(line.split()[1])
    raise RuntimeError(f"no usage_usec in {cg}/cpu.stat")


def mem_peak(cg: Path) -> int:
    """Return a cgroup's high-water memory in bytes."""
    return int((cg / "memory.peak").read_text(encoding="utf-8").strip())


def vm_hwm(name: str) -> int | None:
    """Return the main process's peak resident set in bytes, or None once it has gone."""
    pid = docker("inspect", "-f", "{{.State.Pid}}", name, check=False).stdout.strip()
    if not pid or pid == "0":
        return None
    try:
        text = Path(f"/proc/{pid}/status").read_text(encoding="utf-8")
    except OSError:
        return None
    for line in text.splitlines():
        if line.startswith("VmHWM:"):
            return int(line.split()[1]) * 1024
    return None


def container_ip(name: str) -> str:
    """Return a container's address on the bench network."""
    template = f'{{{{(index .NetworkSettings.Networks "{NET}").IPAddress}}}}'
    return docker("inspect", "-f", template, name).stdout.strip()


def host_load() -> list[float]:
    """Return the 1, 5 and 15 minute load averages."""
    return [
        float(x) for x in Path("/proc/loadavg").read_text(encoding="utf-8").split()[:3]
    ]


def host_state() -> dict[str, Any]:
    """Return the load averages and cpu count, for the noise check on a run."""
    return {"loadavg": host_load(), "nproc": os.cpu_count()}


def is_noisy(before: dict[str, Any], after: dict[str, Any], pinned: int) -> bool:
    """A run is noisy when either 1-minute load exceeds the cores it did not pin."""
    spare = int(before["nproc"]) - pinned
    return max(before["loadavg"][0], after["loadavg"][0]) > spare


def wait_quiet(pinned: int, limit_s: float = 900.0) -> None:
    """Hold a run back until the 1-minute load drops to the cores the run did not pin."""
    deadline = time.monotonic() + limit_s
    while (
        host_load()[0] > (os.cpu_count() or 0) - pinned and time.monotonic() < deadline
    ):
        time.sleep(15)


def parse_cpuset(spec: str) -> list[int]:
    """Expand `24-27,30` to a list of cpu numbers."""
    cpus: list[int] = []
    for part in spec.split(","):
        if "-" in part:
            lo, hi = part.split("-")
            cpus.extend(range(int(lo), int(hi) + 1))
        else:
            cpus.append(int(part))
    return cpus


def cpu_busy(cpus: str, seconds: float = 1.0) -> float:
    """Return the mean busy percentage of a cpuset over a short interval."""
    wanted = parse_cpuset(cpus)

    def snap() -> dict[int, tuple[int, int]]:
        out: dict[int, tuple[int, int]] = {}
        for line in Path("/proc/stat").read_text(encoding="utf-8").splitlines():
            if line.startswith("cpu") and line[3:4].isdigit():
                fields = line.split()
                vals = [int(v) for v in fields[1:]]
                out[int(fields[0][3:])] = (sum(vals), vals[3] + vals[4])
        return out

    first = snap()
    time.sleep(seconds)
    second = snap()
    busy = []
    for idx in wanted:
        total = second[idx][0] - first[idx][0]
        idle = second[idx][1] - first[idx][1]
        busy.append(100.0 * (total - idle) / total if total else 0.0)
    return statistics.fmean(busy)


# --------------------------------------------------------------------------
# Corpus
# --------------------------------------------------------------------------


def read_corpus(compat: Path, source: Source) -> list[dict[str, Any]]:
    """Read a source's input documents from the compat corpus, in a stable order."""
    docs: list[dict[str, Any]] = []
    for path in sorted((compat / source.compat).glob("*/input.ndjson")):
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.strip():
                docs.append(json.loads(line))
    if not docs:
        raise SystemExit(f"no input.ndjson under {compat / source.compat}")
    return docs


def corrupt(message: str, rng: random.Random) -> str:
    """Cut a raw line at 20-80% of its length and append a stray control run.

    The envelope around the line stays valid JSON, so the damage reaches the
    source's own parser, where a real truncated or interleaved line lands.
    """
    cut = rng.randint(int(len(message) * 0.2), max(1, int(len(message) * 0.8)))
    return message[:cut] + CORRUPT_TAIL


def vrl_value(doc: dict[str, Any], source: Source) -> dict[str, Any]:
    """Shape a corpus document as the Kafka value the 2.1 receiver produced."""
    if not source.vrl.wrap:
        return doc
    return {
        "message": doc["message"],
        "tags": COLLECTOR_TAGS,
        "timestamp": "2024-05-22T15:13:13Z",
    }


def build_rows(
    docs: list[dict[str, Any]], rows: int, share: float
) -> list[dict[str, Any]]:
    """Cycle the corpus to `rows` documents and corrupt a fixed share of them.

    The same rows, corrupted at the same indexes, feed all three engines.
    """
    out = [dict(docs[i % len(docs)]) for i in range(rows)]
    if share > 0:
        rng = random.Random(CORRUPT_SEED)
        for idx in rng.sample(range(rows), round(rows * share)):
            out[idx]["message"] = corrupt(out[idx]["message"], rng)
    return out


def write_partitioned(path_stem: Path, values: list[dict[str, Any]]) -> None:
    """Write NDJSON, dealt round-robin into one file per partition."""
    buckets: list[list[str]] = [[] for _ in range(PARTITIONS)]
    for idx, value in enumerate(values):
        line = json.dumps(value, separators=(",", ":"), ensure_ascii=False)
        buckets[idx % PARTITIONS].append(line + "\n")
    for part, lines in enumerate(buckets):
        Path(f"{path_stem}.p{part}").write_text(
            "".join(lines), encoding="utf-8", newline="\n"
        )


def cmd_corpus(args: argparse.Namespace) -> int:
    """Build every source's input files, clean and malformed."""
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    for name in args.source or list(SOURCES):
        source = SOURCES[name]
        docs = read_corpus(Path(args.compat), source)
        for mode, share in (("clean", 0.0), ("malformed", args.malformed)):
            rows = build_rows(docs, args.rows, share)
            write_partitioned(out / f"{name}.{mode}.beats", rows)
            write_partitioned(
                out / f"{name}.{mode}.vrl", [vrl_value(r, source) for r in rows]
            )
            log.info(
                "%s %s: %d rows from %d corpus documents",
                name,
                mode,
                args.rows,
                len(docs),
            )
    return 0


def head_lines(path: Path, count: int) -> bytes:
    """Return `count` lines of a file, replaying it from the top when it runs short.

    A replay repeats the file whole, so the corrupted share of the rows is kept.
    """
    data = path.read_bytes()
    lines = data.splitlines(keepends=True)
    if not lines:
        raise SystemExit(f"{path} is empty")
    whole, rest = divmod(count, len(lines))
    return data * whole + b"".join(lines[:rest])


# --------------------------------------------------------------------------
# Containers and the broker
# --------------------------------------------------------------------------


def cmd_up(args: argparse.Namespace) -> int:
    """Start the network, Redpanda and, with --es, Elasticsearch."""
    docker("network", "create", NET, check=False)
    if not docker("ps", "-q", "-f", f"name=^{BROKER}$").stdout.strip():
        docker(
            "run",
            "-d",
            "--name",
            BROKER,
            "--network",
            NET,
            "--cpuset-cpus",
            args.broker_cpus,
            "--memory",
            "6g",
            "-p",
            "127.0.0.1:29092:29092",
            REDPANDA_IMAGE,
            "redpanda",
            "start",
            "--mode",
            "dev-container",
            "--smp",
            str(len(parse_cpuset(args.broker_cpus))),
            "--memory",
            "4G",
            "--reserve-memory",
            "0M",
            "--kafka-addr",
            "internal://0.0.0.0:9092,external://0.0.0.0:29092",
            "--advertise-kafka-addr",
            f"internal://{BROKER}:9092,external://{BROKER_EXTERNAL}",
        )
        for _ in range(60):
            if (
                "true"
                in docker(
                    "exec", BROKER, "rpk", "cluster", "health", check=False
                ).stdout.lower()
            ):
                break
            time.sleep(2)
        else:
            raise SystemExit("redpanda did not become healthy in 120s")
        log.info("redpanda up on %s", BROKER_EXTERNAL)
    if args.es and not docker("ps", "-q", "-f", f"name=^{ES_NAME}$").stdout.strip():
        docker(
            "run",
            "-d",
            "--name",
            ES_NAME,
            "--network",
            NET,
            "--cpuset-cpus",
            args.engine_cpus,
            "--memory",
            "10g",
            "-p",
            "127.0.0.1:29200:9200",
            "-e",
            "discovery.type=single-node",
            "-e",
            "xpack.security.enabled=false",
            "-e",
            "xpack.license.self_generated.type=basic",
            "-e",
            "ingest.geoip.downloader.enabled=false",
            "-e",
            "ES_JAVA_OPTS=-Xms4g -Xmx4g",
            ES_IMAGE,
        )
        for _ in range(90):
            try:
                es_request("GET", "/_cluster/health")
                break
            except (urllib.error.URLError, OSError, RuntimeError):
                time.sleep(2)
        else:
            raise SystemExit("elasticsearch did not answer in 180s")
        log.info("elasticsearch up on %s", ES_URL)
    return 0


def cmd_down(_args: argparse.Namespace) -> int:
    """Remove every container this rig started, and the network."""
    names = docker(
        "ps", "-a", "--format", "{{.Names}}", "-f", "name=^bench162-"
    ).stdout.split()
    if names:
        docker("rm", "-f", *names, check=False)
    docker("network", "rm", NET, check=False)
    print("removed:", " ".join(names) or "(none)")
    return 0


def rpk(*args: str) -> subprocess.CompletedProcess[bytes]:
    """Run rpk on the host against the external listener."""
    return subprocess.run(
        ["rpk", *args, "-X", f"brokers={BROKER_EXTERNAL}"],
        capture_output=True,
        check=True,
    )


def create_topic(topic: str) -> None:
    """Create a topic whose batch ceiling matches the 10 MiB brokers the DFE templates assume."""
    rpk(
        "topic", "create", topic, "-p", str(PARTITIONS), "-r", "1",
        "-c", "max.message.bytes=10485760",
    )  # fmt: skip


def preload(topic: str, stem: Path, per_partition: int) -> None:
    """Create a topic and fill each partition from its slice file, untimed."""
    create_topic(topic)
    procs: list[subprocess.Popen[bytes]] = []
    for part in range(PARTITIONS):
        data = head_lines(Path(f"{stem}.p{part}"), per_partition)
        proc = subprocess.Popen(
            [
                "rpk",
                "topic",
                "produce",
                topic,
                "-p",
                str(part),
                "-f",
                "%v\\n",
                "-X",
                f"brokers={BROKER_EXTERNAL}",
            ],
            stdin=subprocess.PIPE,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        threading.Thread(target=_feed, args=(proc, data), daemon=True).start()
        procs.append(proc)
    for proc in procs:
        err = proc.stderr.read() if proc.stderr else b""
        if proc.wait() != 0:
            raise RuntimeError(
                f"rpk produce failed: {err.decode('utf-8', 'replace')[:300]}"
            )


def _feed(proc: subprocess.Popen[bytes], data: bytes) -> None:
    """Write a payload to a process's stdin and close it."""
    assert proc.stdin is not None
    proc.stdin.write(data)
    proc.stdin.close()


def topic_end(topic: str) -> int:
    """Return the sum of a topic's partition high watermarks."""
    out = rpk("topic", "describe", topic, "-p").stdout.decode("utf-8", "replace")
    total = 0
    for line in out.splitlines():
        fields = line.split()
        if len(fields) >= 6 and fields[0].isdigit():
            total += int(fields[-1])
    return total


# --------------------------------------------------------------------------
# Elasticsearch
# --------------------------------------------------------------------------


def es_request(
    method: str, path: str, body: dict[str, Any] | None = None, timeout: float = 120.0
) -> dict[str, Any]:
    """Call the loopback Elasticsearch REST API."""
    data = None if body is None else json.dumps(body).encode("utf-8")
    request = urllib.request.Request(
        f"{ES_URL}{path}",
        data=data,
        method=method,
        headers={"Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:  # nosec B310 # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected.dynamic-urllib-use-detected
            return json.loads(response.read().decode("utf-8"))
    except urllib.error.HTTPError as exc:
        raise RuntimeError(
            f"{method} {path} -> {exc.code}: {exc.read().decode('utf-8', 'replace')[:300]}"
        ) from exc


def cmd_fetch_pipelines(args: argparse.Namespace) -> int:
    """Download the pinned ingest pipelines for every source with `gh`."""
    out = Path(args.out) / "integrations"
    for name, source in SOURCES.items():
        package, stream = source.es
        directory = (
            f"packages/{package}/data_stream/{stream}/elasticsearch/ingest_pipeline"
        )
        listing = json.loads(
            sh(
                [
                    "gh",
                    "api",
                    f"repos/elastic/integrations/contents/{directory}?ref={INTEGRATIONS_SHA}",
                ],
            ).stdout
        )
        for item in listing:
            if item["name"].endswith(".link"):
                raise SystemExit(
                    f"{name}: {item['name']} is a .link pipeline, which this fetch does not follow"
                )
            if not item["name"].endswith(".yml"):
                continue
            target = out / directory / item["name"]
            target.parent.mkdir(parents=True, exist_ok=True)
            blob = sh(
                ["gh", "api", "-H", "Accept: application/vnd.github.raw", item["url"]]
            ).stdout
            target.write_text(blob, encoding="utf-8", newline="\n")
            log.info("%s: %s", name, item["name"])
    (Path(args.out) / "INTEGRATIONS_SHA").write_text(
        INTEGRATIONS_SHA + "\n", encoding="utf-8", newline="\n"
    )
    return 0


@functools.cache
def load_compat(sources_dir: str) -> Any:
    """Import scripts/compat.py, pointed at the fetched pipelines and the bench Elasticsearch."""
    os.environ["DFE_ELASTIC_SOURCES"] = sources_dir
    sys.path.insert(0, str(SCRIPT_DIR.parent))
    import compat

    compat.ES_URL = ES_URL
    return compat


# --------------------------------------------------------------------------
# Sampling and analysis
# --------------------------------------------------------------------------


@dataclass(slots=True)
class Sample:
    """One reading: seconds since launch, cumulative records processed, cumulative CPU time."""

    t: float
    count: int
    cpu_us: int


class Sampler(threading.Thread):
    """Reads a cumulative counter and the cgroup's CPU time on a fixed beat."""

    def __init__(
        self,
        count_fn: Callable[[], int],
        cg: Path,
        target: int,
        stall_s: float,
        limit_s: float,
    ):
        super().__init__(daemon=True)
        self.count_fn = count_fn
        self.cg = cg
        self.target = target
        self.stall_s = stall_s
        self.limit_s = limit_s
        self.samples: list[Sample] = []
        self.t0 = time.monotonic()
        self.reason = "running"

    def run(self) -> None:
        """Sample until the target is reached, progress stalls or the limit passes."""
        last_change = time.monotonic()
        last_count = -1
        while True:
            now = time.monotonic()
            try:
                count = self.count_fn()
            except (urllib.error.URLError, OSError, ValueError):
                count = max(last_count, 0)
            self.samples.append(Sample(now - self.t0, count, cpu_usec(self.cg)))
            if count != last_count:
                last_count = count
                last_change = now
            if count >= self.target:
                self.reason = "target"
                return
            if now - last_change > self.stall_s and count > 0:
                self.reason = "stalled"
                return
            if now - self.t0 > self.limit_s:
                self.reason = "timeout"
                return
            time.sleep(SAMPLE_INTERVAL)


def crossing(samples: list[Sample], level: float) -> tuple[float, float] | None:
    """Return (time, cpu_us) where the cumulative count first reaches `level`, interpolated."""
    prev = Sample(0.0, 0, samples[0].cpu_us if samples else 0)
    for cur in samples:
        if cur.count >= level:
            span = cur.count - prev.count
            frac = 0.0 if span <= 0 else (level - prev.count) / span
            return prev.t + frac * (cur.t - prev.t), prev.cpu_us + frac * (
                cur.cpu_us - prev.cpu_us
            )
        prev = cur
    return None


def analyse(samples: list[Sample], total: int, cores: int) -> dict[str, float | None]:
    """Reduce samples to the 10%-90% window rate and the whole-run rate.

    The window excludes start-up (consumer join, JIT) and the drain tail.
    """
    lo = crossing(samples, WINDOW_LO * total)
    hi = crossing(samples, WINDOW_HI * total)
    out: dict[str, float | None] = {
        "rate": None,
        "rate_per_core": None,
        "cpu_seconds": None,
        "rec_per_cpu_second": None,
        "whole_rate": None,
        "window_seconds": None,
    }
    if samples and samples[-1].count > 0:
        out["whole_rate"] = samples[-1].count / samples[-1].t
    if lo and hi and hi[0] > lo[0]:
        records = (WINDOW_HI - WINDOW_LO) * total
        seconds = hi[0] - lo[0]
        cpu_s = (hi[1] - lo[1]) / 1e6
        out["rate"] = records / seconds
        out["rate_per_core"] = records / seconds / cores
        out["cpu_seconds"] = cpu_s
        out["rec_per_cpu_second"] = records / cpu_s if cpu_s > 0 else None
        out["window_seconds"] = seconds
    return out


def prom_sum(text: str, name: str, labels: dict[str, str] | None = None) -> float:
    """Sum every series of a Prometheus metric, optionally filtered by label values."""
    total = 0.0
    for line in text.splitlines():
        if not line.startswith(name):
            continue
        if "{" in line:
            series, _, rest = line.rpartition("}")
            series += "}"
            value = rest.split()[0]
        else:
            series, value = line.split()[:2]
        if series.split("{", 1)[0] != name:
            continue
        if labels and not all(f'{k}="{v}"' in series for k, v in labels.items()):
            continue
        total += float(value)
    return total


def http_text(url: str, timeout: float = 3.0) -> str:
    """GET a URL and return the body."""
    with urllib.request.urlopen(url, timeout=timeout) as response:  # nosec B310 # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected.dynamic-urllib-use-detected
        return response.read().decode("utf-8", "replace")


# --------------------------------------------------------------------------
# Engines
# --------------------------------------------------------------------------


def te_config(source: Source, topic_in: str, topic_out: str, group: str) -> str:
    """Render the service config the way scripts/pgo-workload.sh does, GeoIP off."""
    return f"""source:
  name: {source.te}
  envelope: auto
  topics:
  - {topic_in}
  batch_size: 20000
  max_batch_bytes: 16777216
  group_id: {group}
  brokers:
  - {BROKER_INTERNAL}
sink:
  topic: {topic_out}
  brokers:
  - {BROKER_INTERNAL}
geoip:
  enabled: false
  auto_download:
    enabled: false
"""


def vector_config(topic_in: str, topic_out: str, group: str, sink_input: str) -> str:
    """Render the bench source and sink around the 2.1 templates.

    The source and sink keep the librdkafka options of 003-source-kafka-sasl-scram.yml and
    202-sink-kafka-sasl-scram.yml. SASL is dropped for the plaintext bench broker and the
    19 GB disk buffer is left at the default memory buffer.
    """
    return f"""sources:
  bench_in:
    type: kafka
    bootstrap_servers: "{BROKER_INTERNAL}"
    group_id: "{group}"
    topics: ["{topic_in}"]
    auto_offset_reset: earliest
    librdkafka_options:
      max.partition.fetch.bytes: "10485760"
      fetch.max.bytes: "10485760"
      fetch.min.bytes: "100000"
      fetch.wait.max.ms: "20"
      queued.min.messages: "100000"
      enable.auto.commit: "false"
      enable.partition.eof: "false"
      session.timeout.ms: "30000"
      socket.receive.buffer.bytes: "1048576"
  bench_metrics:
    type: internal_metrics
sinks:
  bench_out:
    type: kafka
    inputs: ["{sink_input}"]
    bootstrap_servers: "{BROKER_INTERNAL}"
    topic: "{topic_out}"
    acknowledgements:
      enabled: true
    encoding:
      codec: json
    message_timeout_ms: 300000
    socket_timeout_ms: 60000
    librdkafka_options:
      batch.size: "8388608"
      linger.ms: "20"
      batch.num.messages: "10000"
      acks: "all"
      queue.buffering.max.kbytes: "1048576"
      compression.type: "zstd"
      socket.nagle.disable: "true"
      message.max.bytes: "8388608"
  bench_prom:
    type: prometheus_exporter
    inputs: ["bench_metrics"]
    address: "0.0.0.0:9598"
"""


def run_kafka_engine(
    engine: str,
    name: str,
    source: Source,
    mode: str,
    rows: int,
    args: argparse.Namespace,
    run_id: str,
) -> dict[str, Any]:
    """One timed run of a Kafka-to-Kafka engine. Returns the run's raw numbers."""
    topic_in, topic_out = f"b162-in-{run_id}", f"b162-out-{run_id}"
    group = f"b162-{run_id}"
    cores = len(parse_cpuset(args.engine_cpus))
    fmt = "beats" if engine == "transform-elastic" else "vrl"
    create_topic(topic_out)
    preload(topic_in, Path(args.corpus) / f"{name}.{mode}.{fmt}", rows // PARTITIONS)

    cname = f"bench162-{run_id}"
    docker("rm", "-f", cname, check=False)
    cfg_dir = Path(args.work).resolve() / "cfg"
    cfg_dir.mkdir(parents=True, exist_ok=True)
    base = [
        "run",
        "-d",
        "--name",
        cname,
        "--network",
        NET,
        "--cpuset-cpus",
        args.engine_cpus,
        "--memory",
        "8g",
    ]
    cfg = cfg_dir / f"{engine}-{run_id}.yaml"
    if engine == "transform-elastic":
        cfg.write_text(
            te_config(source, topic_in, topic_out, group),
            encoding="utf-8",
            newline="\n",
        )
        cfg.chmod(0o644)
        cmd = [
            *base,
            "-e",
            "METRICS_ADDR=0.0.0.0:9090",
            "-v",
            f"{cfg}:/etc/dfe-transform-elastic/config.yaml:ro",
            f"{TE_IMAGE}:{args.te_tag}",
            "--config",
            "/etc/dfe-transform-elastic/config.yaml",
            "run",
        ]
    else:
        cfg.write_text(
            vector_config(topic_in, topic_out, group, source.vrl.sink_input),
            encoding="utf-8",
            newline="\n",
        )
        cfg.chmod(0o644)
        env = {
            "VECTOR_GEOIP_PATH": "/templates/src/geoip_mappings",
            "VECTOR_ENRICHMENT_PATH": "/templates/src/standard_mappings",
            "GEOIP_CITY": "GeoLite2-City.mmdb",
            "GEOIP_ASN": "GeoLite2-ASN.mmdb",
            "TIMEZONES_DB": "hypersec-enrichment-timezones.csv",
            **source.vrl.env,
        }
        env_args = [x for k, v in env.items() for x in ("-e", f"{k}={v}")]
        files = [
            x
            for t in source.vrl.templates
            for x in ("-c", f"/templates/src/core_templates/{t}")
        ]
        cmd = [
            *base,
            *env_args,
            "-v",
            f"{cfg}:/bench/bench.yaml:ro",
            "-v",
            f"{Path(args.vector_templates).resolve()}:/templates:ro",
            VECTOR_IMAGE,
            "-c",
            "/bench/bench.yaml",
            *files,
        ]

    state_before = host_state()
    busy_before = cpu_busy(args.engine_cpus, 1.0)
    launched = time.monotonic()
    docker(*cmd)
    cg = cgroup_dir(container_id(cname))
    time.sleep(0.3)
    ip = container_ip(cname)
    if engine == "transform-elastic":
        url = f"http://{ip}:9090/metrics"

        def count() -> int:
            text = http_text(url)
            return int(
                prom_sum(text, "events_transformed_total")
                + prom_sum(text, "events_errored_total")
                + prom_sum(text, "events_dropped_total")
            )
    else:
        url = f"http://{ip}:9598/metrics"

        def count() -> int:
            text = http_text(url)
            sent = prom_sum(
                text,
                "vector_component_sent_events_total",
                {"component_id": "bench_out"},
            )
            return int(sent + prom_sum(text, "vector_component_discarded_events_total"))

    sampler = Sampler(count, cg, rows, stall_s=args.stall, limit_s=args.limit)
    sampler.t0 = launched
    sampler.start()
    sampler.join()
    state_after = host_state()
    peak_cg = mem_peak(cg)
    hwm = vm_hwm(cname)
    try:
        final_text = http_text(url)
    except (urllib.error.URLError, OSError):
        final_text = ""
    logs = docker("logs", "--tail", "40", cname, check=False)
    docker("rm", "-f", cname, check=False)
    out_end = topic_end(topic_out)
    sample = ""
    if out_end:
        sample = rpk(
            "topic", "consume", topic_out, "-n", "1", "-o", "start", "-f", "%v"
        ).stdout.decode("utf-8", "replace")[:700]
    for topic in (topic_in, topic_out):
        rpk("topic", "delete", topic)
    cfg.unlink(missing_ok=True)

    detail: dict[str, Any] = {
        "output_records": out_end,
        "sample_output": sample,
        "counter_lines": "\n".join(
            ln
            for ln in final_text.splitlines()
            if not ln.startswith("#") and ("events_total" in ln or "errors_total" in ln)
        )[:6000],
    }
    if engine == "transform-elastic":
        for key, metric in (
            ("transformed", "events_transformed_total"),
            ("errored", "events_errored_total"),
            ("dropped", "events_dropped_total"),
            ("dlq_records", "records_dlq_total"),
        ):
            detail[key] = int(prom_sum(final_text, metric))
        detail["engine_metrics"] = "\n".join(
            ln
            for ln in final_text.splitlines()
            if ln.startswith(
                ("worker_pool_", "batch_engine_", "self_regulation_", "pipeline_")
            )
        )[:12000]
    else:
        detail["sent"] = int(
            prom_sum(
                final_text,
                "vector_component_sent_events_total",
                {"component_id": "bench_out"},
            )
        )
        detail["discarded"] = int(
            prom_sum(final_text, "vector_component_discarded_events_total")
        )
        detail["errors"] = int(prom_sum(final_text, "vector_component_errors_total"))
    result = {
        "engine": engine,
        "source": name,
        "mode": mode,
        "rows": rows,
        "cores": cores,
        "stop_reason": sampler.reason,
        "host_before": state_before,
        "host_after": state_after,
        "noisy": is_noisy(state_before, state_after, cores),
        "engine_cpus_busy_before_pct": busy_before,
        "peak_cgroup_memory_bytes": peak_cg,
        "peak_rss_bytes": hwm,
        "samples": len(sampler.samples),
        **analyse(sampler.samples, rows, cores),
        **detail,
    }
    result["log_tail"] = (logs.stdout + logs.stderr)[-1500:]
    return result


class Counter:
    """A thread-safe running total."""

    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._value = 0

    def add(self, n: int) -> None:
        """Add to the total."""
        with self._lock:
            self._value += n

    @property
    def value(self) -> int:
        """The current total."""
        with self._lock:
            return self._value


def run_es_engine(
    engine: str,
    name: str,
    source: Source,
    mode: str,
    rows: int,
    args: argparse.Namespace,
    run_id: str,
) -> dict[str, Any]:
    """One timed run against Elasticsearch, by `_bulk` indexing or by `_simulate`."""
    compat = load_compat(str(Path(args.integrations).resolve()))
    package, stream = source.es
    pipelines = compat.load_pipelines(package, stream)
    compat.install_pipelines(pipelines)
    entry = pipelines.entry

    stem = Path(args.corpus) / f"{name}.{mode}.beats"
    lines: list[bytes] = []
    for part in range(PARTITIONS):
        lines.extend(
            head_lines(Path(f"{stem}.p{part}"), rows // PARTITIONS).splitlines()
        )
    chunk = args.es_chunk
    index = f"b162-{run_id}".lower()
    work: queue.Queue[tuple[bytes, int]] = queue.Queue()
    for start in range(0, len(lines), chunk):
        part_lines = lines[start : start + chunk]
        if engine == "es-bulk":
            body = b"".join(b'{"index":{}}\n' + line + b"\n" for line in part_lines)
        else:
            body = (
                b'{"docs":['
                + b",".join(b'{"_source":' + line + b"}" for line in part_lines)
                + b"]}"
            )
        work.put((body, len(part_lines)))

    if engine == "es-bulk":
        es_request(
            "PUT",
            f"/{index}",
            {
                "settings": {
                    "number_of_shards": 1,
                    "number_of_replicas": 0,
                    "index.mapping.total_fields.limit": 20000,
                }
            },
        )
        path = f"/{index}/_bulk?pipeline={entry}&filter_path=errors,took,items.*.error,items.*.status"
        content_type = "application/x-ndjson"
    else:
        path = f"/_ingest/pipeline/{entry}/_simulate"
        content_type = "application/json"

    cg = cgroup_dir(container_id(ES_NAME))
    cores = len(parse_cpuset(args.engine_cpus))
    done, failed = Counter(), Counter()

    def worker() -> None:
        while True:
            try:
                body, n = work.get_nowait()
            except queue.Empty:
                return
            req = urllib.request.Request(
                f"{ES_URL}{path}",
                data=body,
                method="POST",
                headers={"Content-Type": content_type},
            )
            with urllib.request.urlopen(req, timeout=600) as response:  # nosec B310 # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected.dynamic-urllib-use-detected
                payload = json.loads(response.read())
            if engine == "es-bulk":
                if payload.get("errors"):
                    failed.add(
                        sum(
                            1
                            for it in payload["items"]
                            if "error" in it.get("index", {})
                        )
                    )
            else:
                failed.add(sum(1 for d in payload["docs"] if "error" in d))
            done.add(n)

    state_before = host_state()
    busy_before = cpu_busy(args.engine_cpus, 1.0)
    launched = time.monotonic()
    sampler = Sampler(
        lambda: done.value, cg, len(lines), stall_s=args.stall, limit_s=args.limit
    )
    sampler.t0 = launched
    sampler.start()
    threads = [
        threading.Thread(target=worker, daemon=True) for _ in range(args.es_clients)
    ]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    sampler.join()
    state_after = host_state()
    peak_cg = mem_peak(cg)
    if engine == "es-bulk":
        es_request("DELETE", f"/{index}")
    return {
        "engine": engine,
        "source": name,
        "mode": mode,
        "rows": len(lines),
        "cores": cores,
        "stop_reason": sampler.reason,
        "host_before": state_before,
        "host_after": state_after,
        "noisy": is_noisy(state_before, state_after, cores),
        "engine_cpus_busy_before_pct": busy_before,
        "peak_cgroup_memory_bytes": peak_cg,
        "peak_rss_bytes": None,
        "samples": len(sampler.samples),
        "item_errors": failed.value,
        "clients": args.es_clients,
        "chunk": chunk,
        **analyse(sampler.samples, len(lines), cores),
    }


def cmd_run(args: argparse.Namespace) -> int:
    """Run one engine, source and mode: warm-up runs, then measured runs.

    Runs are appended to the cell's result file, so a re-run after a noisy
    attempt keeps every attempt.
    """
    source = SOURCES[args.source]
    results_dir = Path(args.results)
    results_dir.mkdir(parents=True, exist_ok=True)
    path = results_dir / f"{args.engine}.{args.source}.{args.mode}.json"
    runs: list[dict[str, Any]] = []
    if path.exists():
        runs = json.loads(path.read_text(encoding="utf-8"))["runs"]
    for idx in range(args.warmup + args.runs):
        wait_quiet(len(parse_cpuset(args.engine_cpus)))
        stamp = int(time.time())
        run_id = f"{args.engine}-{args.source}-{args.mode}-{idx}-{stamp}".replace(
            "_", "-"
        )
        if args.engine in ("transform-elastic", "vector"):
            result = run_kafka_engine(
                args.engine, args.source, source, args.mode, args.rows, args, run_id
            )
        else:
            result = run_es_engine(
                args.engine, args.source, source, args.mode, args.rows, args, run_id
            )
        result["measured"] = idx >= args.warmup
        result["run"] = idx
        result["started_unix"] = stamp
        runs.append(result)
        log.info(
            "%s %s %s run %d%s: stop=%s noisy=%s rate/core=%s rec/cpu-s=%s peak=%s",
            args.engine,
            args.source,
            args.mode,
            idx,
            "" if result["measured"] else " (warm-up)",
            result["stop_reason"],
            result["noisy"],
            result["rate_per_core"],
            result["rec_per_cpu_second"],
            result["peak_cgroup_memory_bytes"],
        )
        path.write_text(
            json.dumps({"rows": args.rows, "runs": runs}, indent=2) + "\n",
            encoding="utf-8",
            newline="\n",
        )
    return 0


def valid(run: dict[str, Any]) -> bool:
    """A measured run that reached its target, was not noisy and produced a rate."""
    return bool(
        run.get("measured")
        and run.get("stop_reason") == "target"
        and not run.get("noisy")
        and run.get("rate_per_core") is not None
    )


def cmd_matrix(args: argparse.Namespace) -> int:
    """Run every cell of a plan, each under the bench lock, re-running noisy cells.

    The plan is JSON: `rows` maps engine to source to records per run, and
    `engines`, `sources`, `modes`, `runs`, `warmup` and `extra_attempts` shape the loop.
    """
    plan = json.loads(Path(args.plan).read_text(encoding="utf-8"))
    for source in plan["sources"]:
        for mode in plan["modes"]:
            for engine in plan["engines"]:
                cell = Path(args.results) / f"{engine}.{source}.{mode}.json"
                for attempt in range(1 + plan.get("extra_attempts", 2)):
                    have = 0
                    if cell.exists():
                        runs = json.loads(cell.read_text(encoding="utf-8"))["runs"]
                        have = sum(valid(r) for r in runs)
                    if have >= plan["runs"]:
                        break
                    warmup = plan["warmup"] if not cell.exists() else 0
                    cmd = [
                        "flock", args.lock, sys.executable, str(Path(__file__).resolve()),
                        "run", "--engine", engine, "--source", source, "--mode", mode,
                        "--rows", str(plan["rows"][engine][source]),
                        "--runs", str(plan["runs"] - have), "--warmup", str(warmup),
                        "--corpus", args.corpus, "--work", args.work,
                        "--results", args.results, "--engine-cpus", args.engine_cpus,
                        "--vector-templates", args.vector_templates,
                        "--integrations", args.integrations,
                    ]  # fmt: skip
                    log.info("cell %s %s %s attempt %d", engine, source, mode, attempt)
                    subprocess.run(cmd, check=False)
    return 0


def cmd_report(args: argparse.Namespace) -> int:
    """Print a table per source from every result file."""
    cells: dict[tuple[str, str, str], list[dict[str, Any]]] = {}
    for path in sorted(Path(args.results).glob("*.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        for run in data["runs"]:
            if valid(run):
                cells.setdefault(
                    (run["source"], run["engine"], run["mode"]), []
                ).append(run)

    def spread(vals: list[float]) -> str:
        return f"{statistics.fmean(vals):,.0f} ({min(vals):,.0f}-{max(vals):,.0f})"

    print(
        "| source | engine | rec/CPU-s clean | rec/CPU-s malformed | cores used clean "
        "| rec/s per pinned core clean | peak memory MiB | runs clean+malformed |"
    )
    print("|---|---|---|---|---|---|---|---|")
    for source in SOURCES:
        for engine in ENGINES:
            clean = cells.get((source, engine, "clean"), [])
            bad = cells.get((source, engine, "malformed"), [])
            if not clean and not bad:
                continue
            mem = [r["peak_cgroup_memory_bytes"] / 2**20 for r in clean + bad]

            def column(runs: list[dict[str, Any]], key: str) -> str:
                vals = [r[key] for r in runs if r.get(key)]
                return spread(vals) if vals else "-"

            used = [r["cpu_seconds"] / r["window_seconds"] for r in clean]
            print(
                f"| {source} | {engine} | {column(clean, 'rec_per_cpu_second')} "
                f"| {column(bad, 'rec_per_cpu_second')} "
                f"| {statistics.fmean(used) if used else 0:.2f} "
                f"| {column(clean, 'rate_per_core')} "
                f"| {max(mem):,.0f} | {len(clean)}+{len(bad)} |"
            )
    return 0


def main(argv: list[str] | None = None) -> int:
    """CLI entry."""
    for stream in (sys.stdout, sys.stderr):
        reconfigure = getattr(stream, "reconfigure", None)
        if callable(reconfigure):
            reconfigure(encoding="utf-8", errors="replace")
    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(message)s")
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("corpus", help="build input files from the compat corpus")
    p.add_argument("--compat", required=True, help="testdata/compat directory")
    p.add_argument("--out", required=True)
    p.add_argument(
        "--rows", type=int, default=2_000_000, help="rows per source, a multiple of 4"
    )
    p.add_argument(
        "--malformed",
        type=float,
        default=0.2,
        help="share of rows corrupted in the malformed set",
    )
    p.add_argument("--source", action="append", choices=list(SOURCES))
    p.set_defaults(func=cmd_corpus)

    p = sub.add_parser(
        "fetch-pipelines", help="download the pinned ingest pipelines with gh"
    )
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_fetch_pipelines)

    p = sub.add_parser("up", help="start the broker, and Elasticsearch with --es")
    p.add_argument("--es", action="store_true")
    p.add_argument("--engine-cpus", default="24-27")
    p.add_argument("--broker-cpus", default="20-23")
    p.set_defaults(func=cmd_up)

    p = sub.add_parser("down", help="remove every bench162-* container and the network")
    p.set_defaults(func=cmd_down)

    p = sub.add_parser(
        "run", help="one engine, source and mode: warm-up then measured runs"
    )
    p.add_argument("--engine", required=True, choices=ENGINES)
    p.add_argument("--source", required=True, choices=list(SOURCES))
    p.add_argument("--mode", required=True, choices=("clean", "malformed"))
    p.add_argument(
        "--rows", type=int, required=True, help="records per run, a multiple of 4"
    )
    p.add_argument("--runs", type=int, default=3)
    p.add_argument("--warmup", type=int, default=1)
    p.add_argument("--corpus", default="work/corpus")
    p.add_argument("--work", default="work")
    p.add_argument("--results", default="results")
    p.add_argument("--engine-cpus", default="24-27")
    p.add_argument("--te-tag", default="v1.1.27")
    p.add_argument("--vector-templates", help="local dfe-vector-templates checkout")
    p.add_argument(
        "--integrations", default="work/integrations", help="output of fetch-pipelines"
    )
    p.add_argument("--es-clients", type=int, default=4)
    p.add_argument("--es-chunk", type=int, default=2000)
    p.add_argument(
        "--stall",
        type=float,
        default=20.0,
        help="seconds without progress before a run is abandoned",
    )
    p.add_argument(
        "--limit", type=float, default=900.0, help="seconds before a run is abandoned"
    )
    p.set_defaults(func=cmd_run)

    p = sub.add_parser("matrix", help="run every cell of a plan under the bench lock")
    p.add_argument("--plan", required=True)
    p.add_argument("--lock", default="/home/derek/.dfe-bench.lock")
    p.add_argument("--corpus", default="work/corpus")
    p.add_argument("--work", default="work")
    p.add_argument("--results", default="results")
    p.add_argument("--engine-cpus", default="24-27")
    p.add_argument("--vector-templates", required=True)
    p.add_argument("--integrations", default="work/integrations")
    p.set_defaults(func=cmd_matrix)

    p = sub.add_parser("report", help="print a markdown table per source")
    p.add_argument("--results", default="results")
    p.set_defaults(func=cmd_report)

    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
