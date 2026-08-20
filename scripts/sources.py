#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""`sources.yaml`, as every tool that needs the source list reads it.

One declaration per source. The compat capture, the pipeline vendoring in -dev,
the regeneration driver and the service registry all resolve against this, so
adding a source is one edit rather than four that silently drift apart.

    python3 scripts/sources.py            # print the table
"""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path

try:
    import yaml
except ImportError:  # pragma: no cover - environment probe
    sys.exit("PyYAML is required: apt install python3-yaml, or pip install pyyaml")

DECLARATION = Path(__file__).resolve().parent.parent / "sources.yaml"


@dataclass(frozen=True, slots=True)
class Source:
    """One transform module, and where its upstream and fixtures live.

    Attributes:
        name: Our module name, and the middle segment of every registry name.
        package: Integration package directory name.
        data_stream: Data stream directory name within the package.
        pipelines: Directory under -dev/pipelines/ holding the vendored copy.
        fixture_dir: Fixture directory, relative to ``tests/fixtures``.
        origin: ``api`` or ``syslog``.
        framing: ``line`` or ``body`` for a syslog source, else None.
        transforms: Generated transforms wired into the registry.
        beats_module: Beats module holding the same source, if any.
        beats_fileset: Beats fileset within that module.
    """

    name: str
    package: str
    data_stream: str
    pipelines: str
    fixture_dir: str
    origin: str
    framing: str | None
    transforms: tuple[str, ...]
    beats_module: str | None = None
    beats_fileset: str | None = None

    @property
    def registry_names(self) -> tuple[str, ...]:
        """Every source name the service registry accepts for this module."""
        return tuple(f"filebeat.{self.name}.{t}" for t in self.transforms)


def _source(name: str, entry: dict[str, object]) -> Source:
    """Build one source, rejecting a declaration that cannot be acted on."""
    missing = {"package", "data_stream", "pipelines", "fixtures", "origin"} - set(entry)
    if missing:
        raise SystemExit(f"{DECLARATION}: {name} is missing {sorted(missing)}")

    origin = entry["origin"]
    if origin not in ("api", "syslog"):
        raise SystemExit(f"{DECLARATION}: {name} has origin {origin!r}, not api/syslog")

    framing = entry.get("framing")
    # Framing decides what goes in `message`, and getting it wrong is silent:
    # a header handed to a body pipeline corrupts its first field.
    if origin == "syslog" and framing not in ("line", "body"):
        raise SystemExit(f"{DECLARATION}: syslog source {name} needs framing line/body")
    if origin == "api" and framing is not None:
        raise SystemExit(f"{DECLARATION}: {name} is api-origin and cannot have framing")

    beats = entry.get("beats") or {}
    return Source(
        name=name,
        package=str(entry["package"]),
        data_stream=str(entry["data_stream"]),
        pipelines=str(entry["pipelines"]),
        fixture_dir=str(entry["fixtures"]),
        origin=str(origin),
        framing=None if framing is None else str(framing),
        transforms=tuple(entry.get("transforms") or ("default",)),
        beats_module=beats.get("module"),
        beats_fileset=beats.get("fileset"),
    )


def load(path: Path | None = None) -> dict[str, Source]:
    """Read the declaration, keyed by module name.

    Args:
        path: Override for the declaration file.

    Returns:
        Every declared source, in declaration order.

    Raises:
        SystemExit: If the file is absent or an entry cannot be acted on.
    """
    declaration = path or DECLARATION
    if not declaration.is_file():
        raise SystemExit(f"no source declaration at {declaration}")

    document = yaml.safe_load(declaration.read_text(encoding="utf-8")) or {}
    entries = document.get("sources") or {}
    if not entries:
        raise SystemExit(f"{declaration} declares no sources")

    return {name: _source(name, entry) for name, entry in entries.items()}


SOURCES: dict[str, Source] = load()


def main() -> int:
    rows = [
        (
            name,
            f"{s.package}/{s.data_stream}",
            s.origin + (f"/{s.framing}" if s.framing else ""),
            f"{len(s.transforms)} transform(s)",
        )
        for name, s in SOURCES.items()
    ]
    widths = [max(len(row[i]) for row in rows) for i in range(3)]
    for row in rows:
        print(f"{row[0]:<{widths[0]}}  {row[1]:<{widths[1]}}  {row[2]:<{widths[2]}}  {row[3]}")

    entries = sum(len(s.transforms) for s in SOURCES.values())
    print(f"\n{len(SOURCES)} sources, {entries} registry entries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
