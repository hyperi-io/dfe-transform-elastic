#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Write src/detect/labels.rs and src/detect/markers.rs from the pinned dfe-schemas catalogue.

The catalogue names each source (the `_source` dfe-loader routes on), the
transform variant that produces it, and the `catalogue.detect` markers agentless
mode names it by. `scripts/dfe-schemas.toml` pins the wheel by version and
sha256, and the tables are generated from that wheel and nothing else:

    scripts/gen_detect_tables.py            # rewrite both files
    scripts/gen_detect_tables.py --check    # exit 1 when either is stale
    scripts/gen_detect_tables.py --from /projects/dfe-schemas   # an unreleased checkout

Output from `--from` names no release, so `--check` refuses it.

The wheel is read from `DFE_SCHEMAS_WHEEL` when set, else from
`$XDG_CACHE_HOME/dfe-schemas-wheels/<sha256>.whl` (`~/.cache` by default), else
downloaded from PyPI into that cache. Each is held to the pinned sha256. Exit 3
means none was available.
"""

import argparse
import hashlib
import io
import json
import os
import re
import sys
import tomllib
import urllib.request
import zipfile
from pathlib import Path

import yaml

REPO = Path(__file__).resolve().parent.parent
PIN = REPO / "scripts" / "dfe-schemas.toml"
LABELS = REPO / "src" / "detect" / "labels.rs"
MARKERS = REPO / "src" / "detect" / "markers.rs"

# Where the wheel carries the catalogue tree.
WHEEL_CATALOGUE = "dfe_schemas/data/catalogue/"

PYPI = "https://pypi.org/pypi/dfe-schemas/{version}/json"

# Past this the line holds one literal per line, rustfmt's own max_width.
WIDTH = 100

VERIFY_NAME = re.compile(r"^[a-z_][a-z0-9_]*$")

# The detector marks an all marker's literals as bits of one u8.
MAX_ALL_LITERALS = 8

# The detector numbers SIGNATURES entries with a u16.
MAX_SIGNATURES = 65_534

# Exit status when the wheel is in no cache and PyPI cannot be reached.
EXIT_UNAVAILABLE = 3


class CatalogueError(Exception):
    """The catalogue cannot be turned into the tables."""


def read_pin() -> tuple[str, str]:
    """The pinned dfe-schemas version and wheel sha256."""
    pin = tomllib.loads(PIN.read_text(encoding="utf-8"))
    return pin["version"], pin["sha256"]


class WheelUnavailable(CatalogueError):
    """The pinned wheel is in no cache and PyPI cannot be reached."""


def cache_path(sha256: str) -> Path:
    """Where a downloaded wheel is kept, named by its digest."""
    root = os.environ.get("XDG_CACHE_HOME") or Path.home() / ".cache"
    return Path(root) / "dfe-schemas-wheels" / f"{sha256}.whl"


def _checked(*, body: bytes, sha256: str, where: str) -> bytes:
    """*body*, refused unless its sha256 is *sha256*."""
    digest = hashlib.sha256(body).hexdigest()
    if digest != sha256:
        raise CatalogueError(f"{where} is sha256 {digest}, pinned {sha256}")
    return body


def _https_get(url: str, *, timeout: int) -> bytes:
    """The body at *url*, refused unless it is https, so no file:// or other scheme is read."""
    if not url.startswith("https://"):
        raise CatalogueError(f"{url!r} is not an https URL")
    # Risk accepted: the scheme is checked above and the body is held to the pinned sha256.
    # nosemgrep: python.lang.security.audit.dynamic-urllib-use-detected.dynamic-urllib-use-detected
    response = urllib.request.urlopen(url, timeout=timeout)
    with response:
        return response.read()


def fetch_wheel(*, version: str, sha256: str) -> bytes:
    """The pinned wheel from `DFE_SCHEMAS_WHEEL`, else the cache, else PyPI.

    Every source is held to *sha256*, so a cached or supplied file cannot stand
    in for another release.
    """
    supplied = os.environ.get("DFE_SCHEMAS_WHEEL")
    if supplied:
        body = Path(supplied).read_bytes()
        return _checked(body=body, sha256=sha256, where=f"DFE_SCHEMAS_WHEEL {supplied}")
    cached = cache_path(sha256)
    if cached.is_file():
        return _checked(body=cached.read_bytes(), sha256=sha256, where=str(cached))
    try:
        release = json.loads(_https_get(PYPI.format(version=version), timeout=60))
        wheels = [f for f in release["urls"] if f["packagetype"] == "bdist_wheel"]
        if len(wheels) != 1:
            raise CatalogueError(
                f"dfe-schemas {version} has {len(wheels)} wheels, not one"
            )
        body = _https_get(wheels[0]["url"], timeout=120)
    except OSError as exc:
        raise WheelUnavailable(
            f"dfe-schemas {version} is not cached at {cached} and PyPI is unreachable ({exc}). "
            "Set DFE_SCHEMAS_WHEEL to a downloaded copy."
        ) from exc
    _checked(body=body, sha256=sha256, where=f"dfe-schemas {version} wheel")
    try:
        cached.parent.mkdir(parents=True, exist_ok=True)
        partial = cached.with_suffix(".part")
        partial.write_bytes(body)
        partial.replace(cached)
    except OSError:
        pass  # A read-only cache only costs the next run a download.
    return body


def wheel_catalogue(body: bytes) -> list[dict]:
    """Every catalogue document the wheel carries, in path order."""
    with zipfile.ZipFile(io.BytesIO(body)) as wheel:
        names = sorted(
            n
            for n in wheel.namelist()
            if n.startswith(WHEEL_CATALOGUE) and n.endswith(".yaml")
        )
        if not names:
            raise CatalogueError(f"the wheel carries no {WHEEL_CATALOGUE}*.yaml")
        return [yaml.safe_load(wheel.read(n).decode("utf-8")) for n in names]


def checkout_catalogue(root: Path) -> list[dict]:
    """Every catalogue document under a dfe-schemas checkout, in path order."""
    paths = sorted((root / "catalogue").rglob("*.yaml"))
    if not paths:
        raise CatalogueError(f"no catalogue/*.yaml under {root}")
    return [yaml.safe_load(p.read_text(encoding="utf-8")) for p in paths]


def elastic_entries(docs: list[dict]) -> dict[str, dict]:
    """Each elastic entry by its transform variant."""
    entries: dict[str, dict] = {}
    for doc in docs:
        transform = doc.get("transform") or {}
        if transform.get("engine") != "elastic":
            continue
        variant = transform["variant"]
        if variant in entries:
            raise CatalogueError(f"{variant} is named by two catalogue entries")
        entries[variant] = doc
    return entries


def rust_str(text: str) -> str:
    """*text* as a Rust string literal, refusing anything outside printable ASCII."""
    if not text.isascii() or not text.isprintable():
        raise CatalogueError(f"{text!r} is not printable ASCII")
    return '"' + text.replace("\\", "\\\\").replace('"', '\\"') + '"'


def rust_comment(text: str) -> str:
    """*text* as one `//` line, refusing a newline that would end the comment."""
    if "\n" in text or not text.isascii():
        raise CatalogueError(f"note {text!r} is not one line of ASCII")
    return f"    // {text}"


def detect_rows(entries: dict[str, dict]) -> tuple[list[dict], list[dict]]:
    """The literal and CEF markers, each in the order the detector breaks ties by.

    Literal markers go by their source's `detect.order`, then file order. CEF
    markers go by source name, then file order, and always lose a tie to a
    literal marker because the detector numbers them after every literal one.
    """
    literal: list[tuple[int, int, dict]] = []
    cef: list[tuple[str, int, dict]] = []
    orders: dict[int, str] = {}
    for variant, doc in entries.items():
        detect = (doc.get("catalogue") or {}).get("detect")
        if detect is None:
            continue
        order = detect.get("order")
        for position, marker in enumerate(detect["markers"]):
            row = {"source": variant, **marker}
            if "cef" in marker:
                cef.append((variant, position, row))
                continue
            if order is None:
                raise CatalogueError(
                    f"{variant} has a literal marker and no detect.order"
                )
            literal.append((order, position, row))
        if order is not None:
            if order in orders:
                raise CatalogueError(
                    f"{variant} and {orders[order]} share detect.order {order}"
                )
            orders[order] = variant
    literal.sort(key=lambda r: (r[0], r[1]))
    cef.sort(key=lambda r: (r[0], r[1]))
    return [r[2] for r in literal], [r[2] for r in cef]


def literal_line(row: dict) -> list[str]:
    """One SIGNATURES entry, on one line where it fits."""
    source = rust_str(row["source"])
    priority = int(row["priority"])
    verify = row.get("verify")
    if not 0 <= priority <= 255:
        raise CatalogueError(f"{row['source']}: priority {priority} does not fit a u8")
    if "all" in row:
        if verify is not None:
            raise CatalogueError(f"{row['source']}: verify is only for an any marker")
        if len(row["all"]) > MAX_ALL_LITERALS:
            raise CatalogueError(
                f"{row['source']}: an all marker holds {len(row['all'])} literals, at most "
                f"{MAX_ALL_LITERALS} fit the detector's one-byte mask"
            )
        call, literals, tail = "every", row["all"], ""
    else:
        call, literals = ("checked", row["any"]) if verify else ("one_of", row["any"])
        if verify is not None and not VERIFY_NAME.match(verify):
            raise CatalogueError(
                f"{row['source']}: verify {verify!r} is not a function name"
            )
        tail = f", verify::{verify}" if verify else ""
    quoted = [rust_str(text) for text in literals]
    flat = f"    {call}({source}, {priority}, &[{', '.join(quoted)}]{tail}),"
    if len(flat) <= WIDTH:
        return [flat]
    lines = [f"    {call}({source}, {priority}, &["]
    lines += [f"        {q}," for q in quoted]
    lines.append(f"    ]{tail}),")
    return lines


def header(*, origin: str, what: str) -> list[str]:
    """The licence lines and module doc every generated file opens with."""
    return [
        "// SPDX-License-Identifier: BUSL-1.1",
        "// Copyright (c) 2026 HYPERI PTY LIMITED",
        "",
        f"//! {what}",
        "//!",
        f"//! Generated by `scripts/gen_detect_tables.py` from {origin}. Do not edit by hand.",
        "",
    ]


def render_labels(*, entries: dict[str, dict], origin: str) -> str:
    """src/detect/labels.rs for *entries*."""
    lines = header(
        origin=origin,
        what="The `_source` label each transform's output carries, from the dfe-schemas catalogue.",
    )
    lines += [
        "/// Transform variant to catalogue source, sorted by variant.",
        "#[rustfmt::skip]",
        "static LABELS: &[(&str, &str)] = &[",
    ]
    lines += [
        f"    ({rust_str(v)}, {rust_str(entries[v]['source'])}),"
        for v in sorted(entries, key=str.encode)
    ]
    lines += [
        "];",
        "",
        "/// The catalogue source a transform's events are stamped with.",
        "#[must_use]",
        "pub fn label(variant: &str) -> Option<&'static str> {",
        "    LABELS",
        "        .binary_search_by_key(&variant, |&(v, _)| v)",
        "        .ok()",
        "        .map(|i| LABELS[i].1)",
        "}",
    ]
    return "\n".join(lines) + "\n"


def render_markers(*, entries: dict[str, dict], origin: str) -> str:
    """src/detect/markers.rs for *entries*."""
    literal, cef = detect_rows(entries)
    if len(literal) > MAX_SIGNATURES:
        raise CatalogueError(
            f"{len(literal)} literal markers, at most {MAX_SIGNATURES}"
        )
    verified = any(row.get("verify") for row in literal)
    every = sum("all" in row for row in literal)
    lines = header(
        origin=origin,
        what="The marker table, from the `catalogue.detect` blocks of the dfe-schemas catalogue.",
    )
    lines.append(
        "use super::signatures::{CefMarker, Signature, cef, checked, every, one_of};"
    )
    if verified:
        lines.append("use super::verify;")
    lines += [
        "",
        "/// The `every` entries in [`SIGNATURES`], which sizes the detector's per-line",
        "/// scratch.",
        f"pub(super) const MAX_EVERY: usize = {every};",
        "",
        "/// Every literal-marker entry, in tie-break order.",
        "#[rustfmt::skip]",
        "pub(super) static SIGNATURES: &[Signature] = &[",
    ]
    for row in literal:
        if row.get("note"):
            lines.append(rust_comment(row["note"]))
        lines += literal_line(row)
    lines += [
        "];",
        "",
        "/// Every CEF vendor and product entry.",
        "#[rustfmt::skip]",
        "pub(super) static CEF: &[CefMarker] = &[",
    ]
    for row in cef:
        if row.get("note"):
            lines.append(rust_comment(row["note"]))
        pair = row["cef"]
        lines.append(
            f"    cef({rust_str(row['source'])}, {int(row['priority'])}, "
            f"{rust_str(pair['vendor'])}, {rust_str(pair['product'])}),"
        )
    lines.append("];")
    return "\n".join(lines) + "\n"


def render(*, docs: list[dict], origin: str) -> dict[Path, str]:
    """Both generated files, by path."""
    entries = elastic_entries(docs)
    return {
        LABELS: render_labels(entries=entries, origin=origin),
        MARKERS: render_markers(entries=entries, origin=origin),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--check", action="store_true", help="exit 1 when a file is stale"
    )
    parser.add_argument(
        "--from",
        dest="checkout",
        type=Path,
        help="read an unreleased dfe-schemas checkout instead of the pinned wheel",
    )
    args = parser.parse_args(argv)
    if args.checkout and args.check:
        parser.error("--check compares against the pinned wheel, so it takes no --from")
    try:
        if args.checkout:
            docs, origin = (
                checkout_catalogue(args.checkout),
                "a local dfe-schemas checkout",
            )
        else:
            version, sha256 = read_pin()
            docs, origin = (
                wheel_catalogue(fetch_wheel(version=version, sha256=sha256)),
                (f"dfe-schemas {version}"),
            )
        files = render(docs=docs, origin=origin)
    except CatalogueError as exc:
        print(f"gen_detect_tables: {exc}", file=sys.stderr)
        return EXIT_UNAVAILABLE if isinstance(exc, WheelUnavailable) else 2
    stale = [
        p for p, text in files.items() if not p.exists() or p.read_text("utf-8") != text
    ]
    if args.check:
        for path in stale:
            print(
                f"{path.relative_to(REPO)} is stale against scripts/dfe-schemas.toml: "
                "run scripts/gen_detect_tables.py",
                file=sys.stderr,
            )
        return 1 if stale else 0
    for path in stale:
        path.write_text(files[path], encoding="utf-8", newline="\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
