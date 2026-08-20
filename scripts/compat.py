#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Generate confirmed ingest output from the real Elasticsearch engine.

Brings up Elasticsearch in Docker, pushes the real package pipelines in, feeds
raw source data through them, and extracts the resulting documents to files on
disk. What it writes is just data: the comparison side needs no Docker and no
knowledge that Elasticsearch was ever involved.

Run occasionally, as needed. This is a development tool and is never part of the
shipped service -- the Elasticsearch image is Elastic Licence 2.0 / SSPL / AGPL
and must not become a runtime or distributed dependency.

Full guide, including the two fixture lineages and the known limits:
docs/COMPAT.md.

Examples:
    scripts/compat.py check
    scripts/compat.py generate --source okta
    scripts/compat.py audit
    scripts/compat.py down
"""

from __future__ import annotations

import argparse
import functools
import hashlib
import json
import logging
import os
import re
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:  # pragma: no cover - environment probe
    sys.exit("PyYAML is required: apt install python3-yaml, or pip install pyyaml")

ES_REPOSITORY = "docker.elastic.co/elasticsearch/elasticsearch"
ES_VERSION_ENV = "DFE_COMPAT_ES_VERSION"
DEFAULT_ES_VERSION = "9.2.2"
ES_URL = "http://127.0.0.1:19200"

REPO_ROOT = Path(__file__).resolve().parent.parent

# Clones of elastic/integrations and elastic/beats, located per machine.
SOURCES_ENV = "DFE_ELASTIC_SOURCES"
DEFAULT_SOURCES = Path("/projects/elastic-stuff")

# Defaults inside the ignored testdata/ tree: the corpus derives from
# Elastic-Licensed pipelines and committing it is not an engineering decision.
CORPUS_ENV = "DFE_COMPAT_CORPUS"
CORPUS_ROOT = REPO_ROOT / "testdata" / "compat"

# The single definition of which differences are defects.
POLICY_PATH = REPO_ROOT / "tests" / "compare-policy.yaml"

# Traces run about 800 KB an event, so they stay out of the committed corpus.
TRACE_ROOT = REPO_ROOT / "testdata" / "compat-traces"

# Mounted under the names the pipelines reference by default, so the compat run
# resolves against the same database as the runtime. Opt-in: Elasticsearch
# matches the mmdb metadata `database_type` against an allowlist and rejects
# DB-IP outright, so these only load once their type string reads GeoLite2-*.
GEOIP_SOURCE = REPO_ROOT / "testdata" / "geoip"
GEOIP_MOUNTS = {
    "dbip-city-lite.mmdb": "GeoLite2-City.mmdb",
    "dbip-asn-lite.mmdb": "GeoLite2-ASN.mmdb",
}
GEOIP_CONTAINER_DIR = "/usr/share/elasticsearch/config/ingest-geoip"
GEOIP_PATCHED = REPO_ROOT / "testdata" / "geoip-patched"

# Elasticsearch reads the database kind from this string and rejects anything
# off its allowlist, so a DB-IP file is refused however it is named.
GEOIP_TYPES = {
    "dbip-city-lite.mmdb": ("DBIP-City-Lite", "GeoLite2-City"),
    "dbip-asn-lite.mmdb": ("DBIP-ASN-Lite (compat=GeoLite2-ASN)", "GeoLite2-ASN"),
}

# Marks the start of an mmdb's metadata section, which runs to end of file.
MMDB_METADATA_MARKER = b"\xab\xcd\xefMaxMind.com"


# The beats build expands this the way the package build expands its own form.
BEATS_PIPELINE_REF = re.compile(r'\{<\s*IngestPipeline\s+"([^"]+)"\s*>\}')


@dataclass(frozen=True, slots=True)
class Source:
    """A transform module and where its upstream pipeline and fixtures live.

    Attributes:
        package: Integration package directory name.
        data_stream: Data stream directory name within the package.
        fixture_dir: Fixture directory, relative to ``tests/fixtures``.
        beats_module: Beats module holding the same source, if any.
        beats_fileset: Beats fileset within that module.
    """

    package: str
    data_stream: str
    fixture_dir: str
    beats_module: str | None = None
    beats_fileset: str | None = None


# Neither half of the mapping is derivable: fortinet's data stream is `log`
# while its fixtures sit under `fortigate`, and the azure package holds four of
# our modules as separate data streams.
SOURCES: dict[str, Source] = {
    "okta": Source("okta", "system", "okta/system", "okta", "system"),
    "azure_activitylogs": Source(
        "azure", "activitylogs", "azure/activitylogs", "azure", "activitylogs"
    ),
    "azure_auditlogs": Source(
        "azure", "auditlogs", "azure/auditlogs", "azure", "auditlogs"
    ),
    "azure_signinlogs": Source(
        "azure", "signinlogs", "azure/signinlogs", "azure", "signinlogs"
    ),
    "azure_platformlogs": Source(
        "azure", "platformlogs", "azure/platformlogs", "azure", "platformlogs"
    ),
    "crowdstrike": Source(
        "crowdstrike", "falcon", "crowdstrike/falcon", "crowdstrike", "falcon"
    ),
    "fortinet": Source("fortinet_fortigate", "log", "fortinet/fortigate"),
    "o365": Source("o365", "audit", "o365/audit", "o365", "audit"),
    "panw": Source("panw", "panos", "panw/panos", "panw", "panos"),
    "cisco_ios": Source("cisco_ios", "log", "cisco/ios"),
    "cisco_meraki": Source("cisco_meraki", "log", "cisco/meraki/logs"),
    "cisco_nexus": Source("cisco_nexus", "log", "cisco/nexus"),
}

# Build-time template the package build expands to the real pipeline name. We
# expand it to a flat name of our own so the set can be PUT without Fleet.
INGEST_PIPELINE_REF = re.compile(r'\{\{\s*IngestPipeline\s+"([^"]+)"\s*\}\}')

# Nondeterministic and information-free: stripped at capture rather than stored.
# Everything else is stored raw and excluded at comparison time by policy, so a
# change of mind about `tags` never costs a container run.
CAPTURE_STRIP_PATHS = (("_ingest", "timestamp"),)

log = logging.getLogger("compat")


class CompatError(Exception):
    """A step of the compat run could not be completed."""


@functools.cache
def sources_root() -> Path:
    """Locate the elastic clones, from the environment or the default.

    Returns:
        The directory holding ``integrations`` and ``beats``.

    Raises:
        CompatError: If it is absent, naming the variable that overrides it.
    """
    root = Path(os.environ.get(SOURCES_ENV, DEFAULT_SOURCES))
    if not root.is_dir():
        raise CompatError(
            f"no elastic sources at {root}. Clone elastic/integrations and "
            f"elastic/beats into one directory and set {SOURCES_ENV}, or pass "
            f"--sources."
        )
    return root


def integrations_root() -> Path:
    """Return the elastic/integrations clone."""
    return sources_root() / "integrations"


def beats_root() -> Path:
    """Return the elastic/beats clone."""
    return sources_root() / "beats"


def corpus_root() -> Path:
    """Return where confirmed output is written."""
    return Path(os.environ.get(CORPUS_ENV, CORPUS_ROOT))


def es_version() -> str:
    """Return the Elasticsearch version to run."""
    return os.environ.get(ES_VERSION_ENV, DEFAULT_ES_VERSION)


def es_image() -> str:
    """Return the Elasticsearch image to run."""
    return f"{ES_REPOSITORY}:{es_version()}"


def es_container() -> str:
    """Return the container name, which carries the version.

    Two versions must not share a container, or the second run silently reuses
    the first engine.
    """
    return f"dfe-compat-es-{es_version().replace('.', '-')}"


class PipelineLoader(yaml.SafeLoader):
    """SafeLoader that reads a bare ``=`` as the string a kv processor means."""


PipelineLoader.add_constructor(
    "tag:yaml.org,2002:value", lambda loader, node: loader.construct_scalar(node)
)


# --------------------------------------------------------------------------
# Container lifecycle
# --------------------------------------------------------------------------


def _run(cmd: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    """Run a command with UTF-8 decoding pinned.

    Args:
        cmd: Argument list. Never a shell string.
        check: Raise if the command exits non-zero.

    Returns:
        The completed process.
    """
    return subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=check,
    )


def _docker_available() -> None:
    """Raise unless the docker CLI is on PATH."""
    if shutil.which("docker") is None:
        raise CompatError("docker is not on PATH -- the compat run needs it for ES")


def _encode_mmdb_string(value: str) -> bytes:
    """Encode a string the way an mmdb metadata value is encoded.

    Lengths to 28 fit the control byte; 29 to 284 carry the remainder in one
    following byte.

    Args:
        value: The string to encode.

    Returns:
        The control bytes followed by the UTF-8 bytes.

    Raises:
        CompatError: If the string is longer than the two-byte form allows.
    """
    encoded = value.encode("utf-8")
    if len(encoded) < 29:
        return bytes([0x40 | len(encoded)]) + encoded
    if len(encoded) < 285:
        return bytes([0x40 | 29, len(encoded) - 29]) + encoded
    raise CompatError(f"{value!r} is too long for the two-byte length form")


def patch_geoip_type(source: Path, destination: Path, old: str, new: str) -> None:
    """Rewrite an mmdb's declared database type.

    Only the metadata section is touched, and it runs to end of file, so the
    length change cannot disturb anything that follows.

    Args:
        source: The database to read.
        destination: Where to write the patched copy.
        old: The type string currently declared.
        new: The type string Elasticsearch accepts.

    Raises:
        CompatError: If the metadata or the type string cannot be found.
    """
    raw = source.read_bytes()
    start = raw.rfind(MMDB_METADATA_MARKER)
    if start < 0:
        raise CompatError(f"{source} carries no mmdb metadata marker")
    head, metadata = raw[:start], raw[start:]
    needle = _encode_mmdb_string(old)
    if metadata.count(needle) != 1:
        raise CompatError(f"{source} does not declare the type {old!r} exactly once")
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(head + metadata.replace(needle, _encode_mmdb_string(new)))


def _run_flags(geoip: bool) -> list[str]:
    """Build the docker run flags, optionally mounting GeoIP databases.

    Args:
        geoip: Mount the local databases. Requires their metadata type to be
            one Elasticsearch accepts.

    Returns:
        Flags to place between the container name and the image.
    """
    flags = [
        "-p", "127.0.0.1:19200:9200",
        "-e", "discovery.type=single-node",
        "-e", "xpack.security.enabled=false",
        "-e", "xpack.license.self_generated.type=basic",
        # No reaching out for vendor databases: the mounts below are the source.
        "-e", "ingest.geoip.downloader.enabled=false",
        "-e", "ES_JAVA_OPTS=-Xms1g -Xmx1g",
    ]
    if not geoip:
        return flags
    for source_name, mounted_name in GEOIP_MOUNTS.items():
        patched = ensure_patched_geoip(source_name)
        if patched is None:
            log.warning("no %s -- processor-derived geo will be absent", source_name)
            continue
        flags += ["-v", f"{patched}:{GEOIP_CONTAINER_DIR}/{mounted_name}:ro"]
    return flags


def ensure_patched_geoip(source_name: str) -> Path | None:
    """Return a database Elasticsearch will accept, patching it if needed.

    Args:
        source_name: File name under the local GeoIP directory.

    Returns:
        The patched copy, or None when the source database is absent.
    """
    source = GEOIP_SOURCE / source_name
    if not source.is_file():
        return None
    patched = GEOIP_PATCHED / source_name
    if patched.is_file() and patched.stat().st_mtime >= source.stat().st_mtime:
        return patched
    old, new = GEOIP_TYPES[source_name]
    patch_geoip_type(source, patched, old, new)
    log.info("patched %s type %s -> %s", source_name, old, new)
    return patched


def container_up(timeout: float = 180.0, *, geoip: bool = False) -> None:
    """Start Elasticsearch and block until it answers a health check.

    Idempotent: an already-running container is reused, a stopped one restarted.

    Args:
        timeout: Seconds to wait for the cluster to answer.
        geoip: Mount the local GeoIP databases into the new container. Ignored
            when an existing container is reused.

    Raises:
        CompatError: If docker is missing or the cluster never comes up.
    """
    _docker_available()
    existing = _run(
        ["docker", "ps", "-aq", "-f", f"name=^{es_container()}$"], check=False
    ).stdout.strip()
    if existing:
        log.info("reusing container %s", es_container())
        restarted = _run(["docker", "start", es_container()], check=False)
        if restarted.returncode != 0:
            raise CompatError(
                f"could not start the existing {es_container()}: "
                f"{restarted.stderr.strip()[:160]}. Remove it with "
                f"`compat.py down` and run again."
            )
    else:
        log.info("starting %s", es_image())
        started = _run(
            ["docker", "run", "-d", "--name", es_container(), *_run_flags(geoip), es_image()],
            check=False,
        )
        if started.returncode != 0:
            raise CompatError(
                f"could not start {es_container()}: "
                f"{started.stderr.strip()[:160]}. Another compat container may "
                f"already hold {ES_URL} -- remove it with `compat.py down`."
            )

    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            _request("GET", "/_cluster/health")
            log.info("elasticsearch is up")
            return
        except (urllib.error.URLError, CompatError, OSError):
            time.sleep(2.0)
    raise CompatError(f"elasticsearch did not answer within {timeout:.0f}s")


def container_down() -> None:
    """Stop and remove the Elasticsearch container, ignoring absence."""
    _docker_available()
    _run(["docker", "rm", "-f", es_container()], check=False)
    log.info("removed container %s", es_container())


# --------------------------------------------------------------------------
# Elasticsearch HTTP
# --------------------------------------------------------------------------


def _request(method: str, path: str, body: Any = None) -> dict[str, Any]:
    """Call the Elasticsearch REST API.

    Args:
        method: HTTP verb.
        path: Path beginning with a slash.
        body: Optional JSON-serialisable request body.

    Returns:
        The decoded JSON response.

    Raises:
        CompatError: On a non-2xx response.
    """
    data = None if body is None else json.dumps(body).encode("utf-8")
    request = urllib.request.Request(
        f"{ES_URL}{path}",
        data=data,
        method=method,
        headers={"Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(request, timeout=60) as response:  # nosec B310
            return json.loads(response.read().decode("utf-8"))
    except urllib.error.HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")
        raise CompatError(f"{method} {path} -> {exc.code}: {detail}") from exc


# --------------------------------------------------------------------------
# Pipeline assembly
# --------------------------------------------------------------------------


@dataclass(frozen=True, slots=True)
class PipelineSet:
    """The ingest pipelines of one data stream, ready to install.

    Attributes:
        entry: Name of the pipeline an event enters through.
        definitions: Installed pipeline name to its definition.
        digests: Source file name to sha256 of its bytes.
    """

    entry: str
    definitions: dict[str, dict[str, Any]]
    digests: dict[str, str]


def read_at_ref(repo: Path, relative: str, ref: str | None) -> bytes:
    """Read a file from the working tree, or from a git ref.

    Reading through ``git show`` leaves the clone untouched, which matters
    because it is shared.

    Args:
        repo: Repository root.
        relative: Path within the repository.
        ref: Git ref, or None for the working tree.

    Returns:
        The file's bytes.

    Raises:
        CompatError: If the ref does not carry the file.
    """
    if ref is None:
        return (repo / relative).read_bytes()
    result = subprocess.run(
        ["git", "-C", str(repo), "show", f"{ref}:{relative}"],
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise CompatError(
            f"{relative} is absent at {ref}: "
            f"{result.stderr.decode('utf-8', 'replace').strip()[:80]}"
        )
    return result.stdout


def list_at_ref(repo: Path, directory: str, ref: str | None, suffix: str) -> list[str]:
    """List a directory's files, from the working tree or a git ref.

    Args:
        repo: Repository root.
        directory: Path within the repository.
        ref: Git ref, or None for the working tree.
        suffix: Only return names ending with this.

    Returns:
        Repository-relative paths, sorted.
    """
    if ref is None:
        found = sorted((repo / directory).glob(f"*{suffix}"))
        return [str(path.relative_to(repo)) for path in found]
    result = subprocess.run(
        ["git", "-C", str(repo), "ls-tree", "--name-only", ref, f"{directory}/"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if result.returncode != 0:
        return []
    return sorted(n for n in result.stdout.splitlines() if n.endswith(suffix))


def _flat_name(package: str, data_stream: str, pipeline: str) -> str:
    """Return the name a pipeline is installed under.

    Args:
        package: Integration package name.
        data_stream: Data stream name.
        pipeline: Pipeline stem, e.g. ``default``.

    Returns:
        A flat, collision-free pipeline name.
    """
    return f"compat-{package}-{data_stream}-{pipeline}"


def load_pipelines(
    package: str, data_stream: str, ref: str | None = None
) -> PipelineSet:
    """Read a data stream's pipelines and rewrite their cross-references.

    The package build expands ``{{ IngestPipeline "x" }}`` to a versioned name.
    We expand it to our own flat name instead, which is what lets the set be
    installed directly with no Fleet, Kibana or package registry involved.

    Args:
        package: Integration package name, e.g. ``okta``.
        data_stream: Data stream name, e.g. ``system``.
        ref: Git ref to read the pipelines at, or None for the working tree.

    Returns:
        The assembled pipeline set.

    Raises:
        CompatError: If the directory or the entry pipeline is missing.
    """
    repo = integrations_root()
    directory = (
        f"packages/{package}/data_stream/{data_stream}/elasticsearch/ingest_pipeline"
    )
    names = list_at_ref(repo, directory, ref, ".yml")
    if not names:
        where = ref or "the working tree"
        raise CompatError(f"no ingest_pipeline directory at {directory} in {where}")

    definitions: dict[str, dict[str, Any]] = {}
    digests: dict[str, str] = {}
    for name in names:
        stem = Path(name).stem
        raw = read_at_ref(repo, name, ref)
        digests[Path(name).name] = hashlib.sha256(raw).hexdigest()
        text = INGEST_PIPELINE_REF.sub(
            lambda m: _flat_name(package, data_stream, m.group(1)),
            raw.decode("utf-8"),
        )
        definitions[_flat_name(package, data_stream, stem)] = yaml.load(
            text, Loader=PipelineLoader
        )

    entry = _flat_name(package, data_stream, "default")
    if entry not in definitions:
        raise CompatError(f"no default.yml in {directory}")
    return PipelineSet(entry=entry, definitions=definitions, digests=digests)


def load_beats_pipelines(
    module: str, fileset: str, ref: str | None = None
) -> PipelineSet:
    """Read a beats module fileset's pipelines and rewrite its references.

    A fileset's own ``ingest/`` directory holds the entry pipeline; shared
    pipelines sit one level up beside the module manifest, so both are
    candidates when a reference is resolved.

    Args:
        module: Beats module name, e.g. ``azure``.
        fileset: Fileset name, e.g. ``activitylogs``.
        ref: Git ref to read the pipelines at, or None for the working tree.

    Returns:
        The assembled pipeline set.

    Raises:
        CompatError: If the module or its entry pipeline cannot be found.
    """
    repo = beats_root()
    for base in ("x-pack/filebeat", "filebeat"):
        module_dir = f"{base}/module/{module}"
        if list_at_ref(repo, f"{module_dir}/{fileset}/ingest", ref, ".yml"):
            break
    else:
        where = ref or "the working tree"
        raise CompatError(f"no beats module {module!r}/{fileset!r} in {where}")

    names = list_at_ref(repo, module_dir, ref, ".yml")
    names += list_at_ref(repo, f"{module_dir}/{fileset}/ingest", ref, ".yml")

    definitions: dict[str, dict[str, Any]] = {}
    digests: dict[str, str] = {}
    for name in names:
        raw = read_at_ref(repo, name, ref)
        if b"processors" not in raw:
            continue
        digests[name] = hashlib.sha256(raw).hexdigest()
        text = BEATS_PIPELINE_REF.sub(
            lambda m: _flat_name(module, fileset, m.group(1)), raw.decode("utf-8")
        )
        definitions[_flat_name(module, fileset, Path(name).stem)] = yaml.load(
            text, Loader=PipelineLoader
        )

    entry = _flat_name(module, fileset, "pipeline")
    if entry not in definitions:
        raise CompatError(f"no pipeline.yml for {module}/{fileset}")
    return PipelineSet(entry=entry, definitions=definitions, digests=digests)


def install_pipelines(pipelines: PipelineSet) -> None:
    """Install every pipeline of the set into Elasticsearch.

    Args:
        pipelines: The assembled set.
    """
    for name, definition in pipelines.definitions.items():
        _request("PUT", f"/_ingest/pipeline/{name}", definition)
        log.info("installed %s", name)


# --------------------------------------------------------------------------
# Input assembly
# --------------------------------------------------------------------------


@dataclass(frozen=True, slots=True)
class TestConfig:
    """The parts of an Elastic pipeline-test config that shape the input.

    Attributes:
        fields: Merged into every document before the pipeline runs.
        multiline_pattern: Regex marking the first line of an event.
        dynamic_fields: Field path to a regex its value need only match.
        numeric_keyword_fields: Fields whose numbers compare as strings.
    """

    fields: dict[str, Any]
    multiline_pattern: str | None
    dynamic_fields: dict[str, str]
    numeric_keyword_fields: list[str]


def load_test_config(path: Path | None) -> TestConfig:
    """Read a ``*-config.yml``, tolerating its absence.

    Args:
        path: Config path, or None when the fixture has no config.

    Returns:
        The parsed config, defaulted where keys are absent.
    """
    raw: dict[str, Any] = {}
    if path is not None and path.is_file():
        raw = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
    multiline = raw.get("multiline") or {}
    return TestConfig(
        fields=raw.get("fields") or {},
        multiline_pattern=multiline.get("first_line_pattern"),
        dynamic_fields=raw.get("dynamic_fields") or {},
        numeric_keyword_fields=raw.get("numeric_keyword_fields") or [],
    )


def split_events(text: str, pattern: str | None) -> list[str]:
    """Split raw log text into events.

    Args:
        text: Whole contents of the ``.log`` fixture.
        pattern: ``first_line_pattern`` regex, or None for one event per line.

    Returns:
        The events, in file order, with blank lines dropped.
    """
    lines = text.splitlines()
    if pattern is None:
        return [line for line in lines if line.strip()]

    first_line = re.compile(pattern)
    events: list[str] = []
    for line in lines:
        if not line.strip():
            continue
        if first_line.search(line) or not events:
            events.append(line)
        else:
            events[-1] += "\n" + line
    return events


def build_docs(events: list[str], config: TestConfig) -> list[dict[str, Any]]:
    """Wrap raw events as documents the way Beats delivers them.

    The vendor payload arrives as a string in ``message``, which is the shape
    these pipelines are written against, with the test config's ``fields``
    merged over the top. Some fixtures are already stored in that envelope, so
    a line that is itself an object carrying ``message`` is taken as the
    document rather than wrapped a second time. A source whose payload is bare
    JSON, such as okta, has no ``message`` key and is still wrapped.

    Args:
        events: Raw event texts.
        config: The fixture's test config.

    Returns:
        Documents ready for the simulate API.
    """
    docs: list[dict[str, Any]] = []
    for event in events:
        try:
            parsed = json.loads(event)
        except (json.JSONDecodeError, ValueError):
            parsed = None
        if isinstance(parsed, dict) and "message" in parsed:
            docs.append({"_source": {**parsed, **config.fields}})
        else:
            docs.append({"_source": {"message": event, **config.fields}})
    return docs


# --------------------------------------------------------------------------
# Simulation and capture
# --------------------------------------------------------------------------


def _strip_capture_paths(source: dict[str, Any]) -> dict[str, Any]:
    """Remove nondeterministic values that carry no information.

    Args:
        source: A document body, mutated in place.

    Returns:
        The same document, for chaining.
    """
    for path in CAPTURE_STRIP_PATHS:
        cursor: Any = source
        for key in path[:-1]:
            cursor = cursor.get(key) if isinstance(cursor, dict) else None
            if cursor is None:
                break
        if isinstance(cursor, dict):
            cursor.pop(path[-1], None)
    return source


def simulate(
    entry: str, docs: list[dict[str, Any]], *, verbose: bool
) -> tuple[list[dict[str, Any]], list[Any]]:
    """Run documents through an installed pipeline and capture the results.

    Args:
        entry: Installed name of the entry pipeline.
        docs: Documents to feed in.
        verbose: Also capture the per-processor trace as a debug artefact.

    Returns:
        A pair of (output documents, per-document processor traces). The trace
        list is empty unless ``verbose`` is set.

    Raises:
        CompatError: If the response shape is not what the API documents.
    """
    if not docs:
        return [], []
    query = "?verbose=true" if verbose else ""
    response = _request(
        "POST", f"/_ingest/pipeline/{entry}/_simulate{query}", {"docs": docs}
    )
    results = response.get("docs")
    if not isinstance(results, list):
        raise CompatError(f"unexpected simulate response: {response!r}")

    outputs: list[dict[str, Any]] = []
    traces: list[Any] = []
    for entry_result in results:
        if not isinstance(entry_result, dict):
            # A document the engine returned nothing for still occupies a slot,
            # so that the corpus stays aligned with the input.
            outputs.append({"_compat_error": "no result"})
            continue
        if verbose:
            steps = entry_result.get("processor_results", [])
            traces.append(steps)
            final = steps[-1].get("doc") if steps else None
        else:
            final = entry_result.get("doc")
        if final is None:
            # A document the pipeline failed outright still belongs in the
            # corpus -- silently dropping it would overstate our parity.
            outputs.append({"_compat_error": entry_result})
            continue
        outputs.append(_strip_capture_paths(final.get("_source", {})))
    return outputs, traces


# --------------------------------------------------------------------------
# Provenance and output
# --------------------------------------------------------------------------


def _git_sha(repo: Path) -> str | None:
    """Return the HEAD sha of a git repo, or None if it is not one.

    Args:
        repo: Repository root.

    Returns:
        The forty-character sha, or None.
    """
    result = _run(["git", "-C", str(repo), "rev-parse", "HEAD"], check=False)
    return result.stdout.strip() or None


def _package_version(package: str) -> str | None:
    """Read a package's declared version from its manifest.

    Args:
        package: Integration package name.

    Returns:
        The version string, or None if the manifest is unreadable.
    """
    manifest = integrations_root() / "packages" / package / "manifest.yml"
    if not manifest.is_file():
        return None
    parsed = yaml.safe_load(manifest.read_text(encoding="utf-8")) or {}
    version = parsed.get("version")
    return str(version) if version is not None else None


def write_corpus(
    package: str,
    data_stream: str,
    fixture: str,
    events: list[str],
    outputs: list[dict[str, Any]],
    traces: list[Any],
    pipelines: PipelineSet,
    config: TestConfig,
) -> Path:
    """Write one fixture's ground truth and its provenance to disk.

    Args:
        package: Integration package name.
        data_stream: Data stream name.
        fixture: Fixture stem, used as the directory name.
        events: The raw input events.
        outputs: The confirmed output documents.
        traces: Per-processor traces, empty when not captured.
        pipelines: The installed pipeline set, for its digests.
        config: The fixture's test config, carried into the corpus so the
            comparison side knows which fields are nondeterministic.

    Returns:
        The directory written.
    """
    out_dir = corpus_root() / package / data_stream / fixture
    out_dir.mkdir(parents=True, exist_ok=True)

    version = _request("GET", "/")
    meta = {
        "elasticsearch_version": version.get("version", {}).get("number"),
        "integrations_sha": _git_sha(integrations_root()),
        "package": package,
        "package_version": _package_version(package),
        "data_stream": data_stream,
        "fixture": fixture,
        "entry_pipeline": pipelines.entry,
        "pipeline_digests": pipelines.digests,
        "event_count": len(events),
        "dynamic_fields": config.dynamic_fields,
        "numeric_keyword_fields": config.numeric_keyword_fields,
    }
    (out_dir / "meta.json").write_text(
        json.dumps(meta, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    # Record the documents that were sent, not a second wrap of the raw text:
    # a fixture already in the Beats envelope would otherwise be stored with a
    # whole document inside `message`.
    _write_ndjson(
        out_dir / "input.ndjson", [doc["_source"] for doc in build_docs(events, config)]
    )
    _write_ndjson(out_dir / "expected.ndjson", outputs)
    if traces:
        trace_dir = TRACE_ROOT / package / data_stream / fixture
        trace_dir.mkdir(parents=True, exist_ok=True)
        (trace_dir / "trace.json").write_text(
            json.dumps(traces, indent=2) + "\n", encoding="utf-8"
        )
    return out_dir


def _write_ndjson(path: Path, records: list[Any]) -> None:
    """Write records one JSON document per line.

    Args:
        path: Destination file.
        records: Records to serialise.
    """
    with path.open("w", encoding="utf-8", newline="\n") as handle:
        for record in records:
            handle.write(json.dumps(record, sort_keys=True) + "\n")


# --------------------------------------------------------------------------
# Commands
# --------------------------------------------------------------------------


def list_fixtures(source: Source) -> list[Path]:
    """List a source's fixture logs that carry a committed expectation.

    A fixture with no expectation cannot answer the vintage question, so it is
    not returned.

    Args:
        source: The source to look under.

    Returns:
        Fixture log paths, in name order.
    """
    directory = REPO_ROOT / "tests" / "fixtures" / source.fixture_dir
    return [
        path
        for path in sorted(directory.glob("*.log"))
        if path.with_name(path.name + "-expected.json").is_file()
    ]


def config_for(log_path: Path) -> Path | None:
    """Return a fixture's config path, preferring its own over the shared one.

    Args:
        log_path: The fixture log.

    Returns:
        The config path, or None when neither exists.
    """
    own = log_path.with_name(log_path.name + "-config.yml")
    if own.is_file():
        return own
    shared = log_path.with_name("test-common-config.yml")
    return shared if shared.is_file() else None


# --------------------------------------------------------------------------
# Vintage audit
# --------------------------------------------------------------------------

@dataclass(frozen=True, slots=True)
class Policy:
    """Which differences are defects, loaded from `tests/compare-policy.yaml`.

    Attributes:
        nondeterministic: Paths that carry no information.
        not_emitted: Prefixes of Elastic plumbing DFE does not produce.
        known_different: Prefixes both sides produce, differently.
        unordered: ECS fields whose values are sets.
        reasons: Prefix to the reason it is excluded.
    """

    nondeterministic: tuple[str, ...]
    not_emitted: tuple[str, ...]
    known_different: tuple[str, ...]
    unordered: frozenset[str]
    reasons: dict[str, str]


@functools.cache
def load_policy() -> Policy:
    """Read the comparison policy.

    Returns:
        The parsed policy.

    Raises:
        CompatError: If the file is missing.
    """
    if not POLICY_PATH.is_file():
        raise CompatError(f"no comparison policy at {POLICY_PATH}")
    raw = yaml.safe_load(POLICY_PATH.read_text(encoding="utf-8")) or {}
    reasons: dict[str, str] = {}
    groups: dict[str, list[str]] = {}
    for group in ("nondeterministic", "not_emitted", "known_different"):
        entries = raw.get(group) or []
        keys = [entry.get("path") or entry.get("prefix") for entry in entries]
        groups[group] = [key for key in keys if key]
        for entry, key in zip(entries, keys):
            if key:
                reasons[key] = entry.get("why", "")
    return Policy(
        nondeterministic=tuple(groups["nondeterministic"]),
        not_emitted=tuple(groups["not_emitted"]),
        known_different=tuple(groups["known_different"]),
        unordered=frozenset(raw.get("unordered") or []),
        reasons=reasons,
    )


def flatten(value: Any, prefix: str = "") -> dict[str, Any]:
    """Flatten nested JSON to dotted paths, list indices included.

    Args:
        value: Any JSON value.
        prefix: Path accumulated so far.

    Returns:
        Leaf path to leaf value.
    """
    out: dict[str, Any] = {}
    if isinstance(value, dict):
        for key, sub in value.items():
            out.update(flatten(sub, f"{prefix}.{key}" if prefix else key))
    elif isinstance(value, list):
        for index, sub in enumerate(value):
            out.update(flatten(sub, f"{prefix}[{index}]"))
    else:
        out[prefix] = value
    return out


def _base_path(path: str) -> str:
    """Strip list indices from a flattened path.

    Args:
        path: A flattened path such as ``event.category[1]``.

    Returns:
        The path with indices removed.
    """
    return re.sub(r"\[\d+\]", "", path)


def _same_members(left: Any, right: Any) -> bool:
    """Report whether two values are lists with the same members in any order.

    Args:
        left: First value.
        right: Second value.

    Returns:
        True when both are lists holding the same multiset of members.
    """
    if not isinstance(left, list) or not isinstance(right, list):
        return False
    key = lambda item: json.dumps(item, sort_keys=True)
    return sorted(map(key, left)) == sorted(map(key, right))


def _values_equal(left: Any, right: Any, base: str, config: TestConfig) -> bool:
    """Compare two leaf values under the fixture's own declarations.

    Args:
        left: Committed value.
        right: Produced value.
        base: Path with list indices removed.
        config: The fixture's test config.

    Returns:
        True when the values are equal, or equal enough by declaration.
    """
    if left == right:
        return True
    if base in config.numeric_keyword_fields:
        return str(left) == str(right)
    pattern = config.dynamic_fields.get(base)
    if pattern is not None:
        return bool(
            left is not None
            and right is not None
            and re.fullmatch(pattern, str(left))
            and re.fullmatch(pattern, str(right))
        )
    return False


def matching_rule(base: str) -> str | None:
    """Return the policy prefix that excludes a path, if any.

    Args:
        base: Path with list indices removed.

    Returns:
        The longest matching prefix, or None when nothing excludes it.
    """
    policy = load_policy()
    candidates = policy.not_emitted + policy.known_different + policy.nondeterministic
    matches = [prefix for prefix in candidates if base.startswith(prefix)]
    return max(matches, key=len) if matches else None


def categorise(
    committed: dict[str, Any],
    compat: dict[str, Any],
    config: TestConfig | None = None,
) -> dict[str, set[str]]:
    """Sort the differences between two documents into buckets.

    Args:
        committed: Elastic's shipped expectation.
        compat: What the current pipeline actually produced.
        config: The fixture's test config, for its own declarations.

    Returns:
        Bucket name to the set of differing field paths in it. The ``real``
        bucket is the signal: everything not explained away.
    """
    policy = load_policy()
    config = config or TestConfig({}, None, {}, [])
    buckets: dict[str, set[str]] = {
        "metadata": set(), "enrichment": set(), "order": set(), "real": set()
    }
    not_emitted = policy.not_emitted
    flat_committed, flat_compat = flatten(committed), flatten(compat)
    for path in set(flat_committed) | set(flat_compat):
        base = _base_path(path)
        if _values_equal(flat_committed.get(path), flat_compat.get(path), base, config):
            continue
        if base.startswith(policy.nondeterministic):
            continue
        if base.startswith(not_emitted):
            buckets["metadata"].add(base)
        elif base.startswith(policy.known_different):
            buckets["enrichment"].add(base)
        elif base in policy.unordered and _same_members(
            _resolve(committed, base.split(".")), _resolve(compat, base.split("."))
        ):
            buckets["order"].add(base)
        else:
            buckets["real"].add(base)
    return buckets


def _resolve(document: dict[str, Any], keys: list[str]) -> Any:
    """Walk a dotted key path into a document.

    Args:
        document: The document to walk.
        keys: Path segments.

    Returns:
        The value found, or None.
    """
    cursor: Any = document
    for key in keys:
        if not isinstance(cursor, dict):
            return None
        cursor = cursor.get(key)
    return cursor


BEATS_MARKERS = frozenset(
    {"fileset.name", "log.offset", "input.type", "event.module"}
)


def read_expectation(path: Path) -> tuple[list[dict[str, Any]], str]:
    """Read a committed expectation and identify which engine produced it.

    Both container shapes appear in the corpus (a bare array and an object
    under ``expected``), and independently of that the documents are either
    nested or flat with dotted keys. Only the beats marker fields identify the
    lineage, so container shape is not used for it.

    Args:
        path: The ``-expected.json`` file.

    Returns:
        A pair of (events, lineage) where lineage is ``beats`` or
        ``integrations``.
    """
    parsed = json.loads(path.read_text(encoding="utf-8"))
    events = parsed["expected"] if isinstance(parsed, dict) else parsed
    lineage = "integrations"
    if events and BEATS_MARKERS & set(events[0]):
        lineage = "beats"
    return events, lineage


def pipelines_for(
    lineage: str,
    source: Source,
    installed: dict[str, PipelineSet | str],
    ref: str | None = None,
) -> PipelineSet | str:
    """Install a source's pipelines for one lineage, once.

    Args:
        lineage: ``beats`` or ``integrations``.
        source: The source being audited.
        ref: Git ref to read the pipelines at, or None for the working tree.
        installed: Per-source cache, mutated. Maps lineage to the installed set
            or to the message explaining why it could not be installed.

    Returns:
        The pipeline set, or a failure message.
    """
    if lineage in installed:
        return installed[lineage]
    try:
        if lineage == "beats":
            if source.beats_module is None or source.beats_fileset is None:
                raise CompatError("no beats module recorded for this source")
            built = load_beats_pipelines(source.beats_module, source.beats_fileset, ref)
        else:
            built = load_pipelines(source.package, source.data_stream, ref)
        install_pipelines(built)
        installed[lineage] = built
    except (CompatError, yaml.YAMLError) as exc:
        installed[lineage] = str(exc).replace("\n", " ")[:90]
    return installed[lineage]


def cmd_audit(args: argparse.Namespace) -> int:
    """Report which committed expectations still match current upstream.

    Args:
        args: Parsed command-line arguments.

    Returns:
        A process exit code.
    """
    container_up(geoip=args.geoip)
    names = [args.source] if args.source else list(SOURCES)
    rows: list[tuple[str, str]] = []
    real_fields: dict[str, dict[str, int]] = {}
    rule_hits: dict[str, int] = {}

    for name in names:
        source = SOURCES[name]
        installed: dict[str, PipelineSet | str] = {}

        events_total = clean = 0
        totals = {"metadata": 0, "enrichment": 0, "order": 0, "real": 0}
        seen_real: dict[str, int] = {}
        lineages: set[str] = set()
        failure: str | None = None

        for log_path in list_fixtures(source):
            expected, lineage = read_expectation(
                log_path.with_name(log_path.name + "-expected.json")
            )
            lineages.add(lineage)
            pipelines = pipelines_for(lineage, source, installed, args.ref)
            if isinstance(pipelines, str):
                failure = pipelines
                continue

            config = load_test_config(config_for(log_path))
            events = split_events(
                log_path.read_text(encoding="utf-8"), config.multiline_pattern
            )
            try:
                outputs, _ = simulate(
                    pipelines.entry, build_docs(events, config), verbose=False
                )
            except CompatError as exc:
                log.warning("%s %s: %s", name, log_path.name, exc)
                continue
            for want, got in zip(expected, outputs):
                events_total += 1
                buckets = categorise(want, got, config)
                for bucket, paths in buckets.items():
                    totals[bucket] += len(paths)
                    for path in paths:
                        if bucket == "real":
                            seen_real[path] = seen_real.get(path, 0) + 1
                        elif bucket != "order":
                            rule = matching_rule(path) or path
                            rule_hits[rule] = rule_hits.get(rule, 0) + 1
                if not buckets["real"]:
                    clean += 1

        label = "+".join(sorted(lineages)) or "none"
        if events_total == 0:
            rows.append((name, f"{label:13s} NO COMPARISON -- {failure or 'no fixtures'}"))
            continue
        verdict = "CURRENT" if totals["real"] == 0 else "DRIFTED"
        summary = (
            f"{label:13s} {verdict:8s} {clean:4d}/{events_total:<4d} clean  "
            f"real={totals['real']:<6d} order={totals['order']:<4d} "
            f"meta={totals['metadata']:<5d} enrich={totals['enrichment']}"
        )
        rows.append((name, summary))
        real_fields[name] = seen_real

    print("\nvintage audit -- committed expectations vs current upstream pipelines\n")
    for name, line in rows:
        print(f"  {name:22s} {line}")
    print("\ntop real differences per drifted source:\n")
    for name, fields in real_fields.items():
        if not fields:
            continue
        top = sorted(fields.items(), key=lambda kv: -kv[1])[:6]
        print(f"  {name}:")
        for path, count in top:
            print(f"      {count:5d}x  {path}")

    policy = load_policy()
    excluded = sum(rule_hits.values())
    print(f"\nexcluded {excluded} field differences by {len(rule_hits)} rules:\n")
    for rule, count in sorted(rule_hits.items(), key=lambda kv: -kv[1]):
        print(f"  {count:6d}x  {rule:22s} {policy.reasons.get(rule, '')}")
    return 0


def cmd_generate(args: argparse.Namespace) -> int:
    """Run one package/data-stream fixture through the real engine.

    Args:
        args: Parsed command-line arguments.

    Returns:
        A process exit code.
    """
    container_up(geoip=args.geoip)
    source = SOURCES[args.source]
    pipelines = load_pipelines(source.package, source.data_stream, args.ref)
    install_pipelines(pipelines)

    fixtures = list_fixtures(source)
    if args.fixture:
        fixtures = [p for p in fixtures if args.fixture in p.name]
    if not fixtures:
        raise CompatError(f"no fixture with a committed expectation for {args.source}")

    for log_path in fixtures:
        config = load_test_config(config_for(log_path))
        events = split_events(
            log_path.read_text(encoding="utf-8"), config.multiline_pattern
        )
        docs = build_docs(events, config)
        outputs, traces = simulate(pipelines.entry, docs, verbose=args.trace)
        failures = sum(1 for doc in outputs if "_compat_error" in doc)
        out_dir = write_corpus(
            source.package,
            source.data_stream,
            log_path.stem,
            events,
            outputs,
            traces,
            pipelines,
            config,
        )
        note = f", {failures} unprocessable" if failures else ""
        print(f"{log_path.name}: {len(outputs)} documents -> {out_dir}{note}")
    return 0


def cmd_check(_args: argparse.Namespace) -> int:
    """Verify the environment before anyone waits on a container.

    Args:
        _args: Unused.

    Returns:
        Zero when every source resolves, one otherwise.
    """
    problems: list[str] = []
    if shutil.which("docker") is None:
        problems.append("docker is not on PATH")
    try:
        root = sources_root()
        print(f"elastic sources   {root}")
        for name, path in (("integrations", integrations_root()), ("beats", beats_root())):
            state = "ok" if path.is_dir() else "MISSING"
            print(f"  {name:14s} {state}  {path}")
            if not path.is_dir():
                problems.append(f"{name} clone missing at {path}")
    except CompatError as exc:
        # Nothing below can succeed, and repeating it per source buries the fix.
        print(f"\nproblem:\n  - {exc}")
        return 1

    print(f"corpus            {corpus_root()}")
    print(f"elasticsearch     {es_image()}\n")

    for name, source in SOURCES.items():
        fixtures = len(list_fixtures(source))
        try:
            load_pipelines(source.package, source.data_stream)
            integrations = "ok"
        except (CompatError, yaml.YAMLError) as exc:
            integrations = f"FAILED ({str(exc).splitlines()[0][:40]})"
            problems.append(f"{name}: integrations pipelines {integrations}")
        beats = "-"
        if source.beats_module and source.beats_fileset:
            try:
                load_beats_pipelines(source.beats_module, source.beats_fileset)
                beats = "ok"
            except (CompatError, yaml.YAMLError):
                beats = "unavailable"
        print(f"  {name:20s} fixtures={fixtures:<4d} integrations={integrations:<10s} beats={beats}")

    if problems:
        print("\nproblems:")
        for problem in problems:
            print(f"  - {problem}")
        return 1
    print("\nready")
    return 0


def cmd_up(args: argparse.Namespace) -> int:
    """Start Elasticsearch and wait for it.

    Args:
        args: Parsed command-line arguments.

    Returns:
        A process exit code.
    """
    container_up(geoip=args.geoip)
    return 0


def cmd_down(_args: argparse.Namespace) -> int:
    """Remove the Elasticsearch container.

    Args:
        _args: Unused.

    Returns:
        A process exit code.
    """
    container_down()
    return 0


def main(argv: list[str] | None = None) -> int:
    """Parse arguments and dispatch.

    Args:
        argv: Argument list, defaulting to ``sys.argv[1:]``.

    Returns:
        A process exit code.
    """
    for stream in (sys.stdout, sys.stderr):
        reconfigure = getattr(stream, "reconfigure", None)
        if callable(reconfigure):
            reconfigure(encoding="utf-8", errors="replace")

    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("-v", "--verbose", action="store_true", help="debug logging")
    parser.add_argument(
        "--sources",
        help=f"directory holding the integrations and beats clones (${SOURCES_ENV})",
    )
    parser.add_argument(
        "--corpus", help=f"where confirmed output is written (${CORPUS_ENV})"
    )
    parser.add_argument(
        "--es-version",
        help=f"Elasticsearch version to run (${ES_VERSION_ENV}, default "
        f"{DEFAULT_ES_VERSION})",
    )
    parser.add_argument(
        "--ref",
        help="git ref to read pipelines at, e.g. v7.17.9. Defaults to the "
        "clone's working tree. The clone is never modified.",
    )
    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser(
        "check", help="verify clones, fixtures and pipelines without starting ES"
    ).set_defaults(func=cmd_check)

    geoip_help = "mount local GeoIP databases (needs a GeoLite2 metadata type)"
    up = sub.add_parser("up", help="start Elasticsearch")
    up.add_argument("--geoip", action="store_true", help=geoip_help)
    up.set_defaults(func=cmd_up)
    sub.add_parser("down", help="remove the container").set_defaults(func=cmd_down)

    generate = sub.add_parser("generate", help="produce confirmed output")
    generate.add_argument("--source", required=True, choices=sorted(SOURCES))
    generate.add_argument("--fixture", help="substring, to narrow to one fixture")
    generate.add_argument("--geoip", action="store_true", help=geoip_help)
    generate.add_argument(
        "--trace",
        action="store_true",
        help="also capture the per-processor trace as a debug artefact",
    )
    generate.set_defaults(func=cmd_generate)

    audit = sub.add_parser(
        "audit", help="report which committed expectations match current upstream"
    )
    audit.add_argument("--source", choices=sorted(SOURCES), help="just this one")
    audit.add_argument("--geoip", action="store_true", help=geoip_help)
    audit.set_defaults(func=cmd_audit)

    args = parser.parse_args(argv)
    # Set before anything resolves a path, so the factories are the only reader.
    if args.sources:
        os.environ[SOURCES_ENV] = args.sources
    if args.corpus:
        os.environ[CORPUS_ENV] = args.corpus
    if args.es_version:
        os.environ[ES_VERSION_ENV] = args.es_version
    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(levelname)s %(message)s",
        stream=sys.stderr,
    )
    try:
        return int(args.func(args))
    except CompatError as exc:
        log.error("%s", exc)
        return 1


if __name__ == "__main__":
    sys.exit(main())
