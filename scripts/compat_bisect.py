#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Find the upstream commit that changed what Elasticsearch produces for a source.

`compat.py --ref` answers *what did Elastic produce at this ref?*. This answers
the one after it: *which ref changed it?* -- which is what nothing said when
cisco_ios fell from 23 clean fixtures to 5 on a re-vendor. Across hundreds of
sources with periodic re-vendoring that question is constant, and answering it
by reading diffs does not scale.

Only a change to the package's own ingest pipelines can move the output, so the
candidate list is the commits touching that directory between the two refs --
usually a handful, not the thousands in the range. The FIXTURES are held at the
worktree's commit throughout, so what is being bisected is the pipeline change
alone.

The search is binary, so it costs log2(N) Elasticsearch runs rather than N.

    scripts/compat_bisect.py --source cisco_ios --good v8.15.0 --bad main
    scripts/compat_bisect.py --source okta --good <sha> --bad <sha> --list

Set DFE_COMPAT_SOURCES / DFE_COMPAT_ES_VERSION exactly as for compat.py.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import logging
import subprocess
import sys
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parent))

# Imported after the path insert above, which is what makes it importable.
import compat

log = logging.getLogger("compat_bisect")

# Fields the engine rewrites on every call, so they must not count as a change.
# The same list `tests/compare-policy.yaml` calls nondeterministic.
VOLATILE = ("_ingest", "event.ingested", "event.created", "@timestamp")


def git(*args: str) -> str:
    """Run a git command in the integrations clone and return its stdout."""
    result = subprocess.run(
        ["git", "-C", str(compat.integrations_root()), *args],
        capture_output=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if result.returncode != 0:
        raise compat.CompatError(f"git {' '.join(args)}: {result.stderr.strip()}")
    return result.stdout


def strip_volatile(doc: Any, prefix: str = "") -> Any:
    """Return `doc` without the fields the engine rewrites per call."""
    if not isinstance(doc, dict):
        return doc
    out = {}
    for key, value in doc.items():
        path = f"{prefix}.{key}" if prefix else key
        if any(path == v or path.startswith(f"{v}.") for v in VOLATILE):
            continue
        out[key] = strip_volatile(value, path)
    return out


def candidates(package: str, good: str, bad: str) -> list[tuple[str, str]]:
    """The commits between `good` and `bad` that touched the package's pipelines.

    Args:
        package: The integrations package name.
        good: The ref known to produce the wanted output.
        bad: The ref known not to.

    Returns:
        `(sha, subject)` pairs, oldest first.
    """
    # `:(glob)` magic, because a plain pathspec's `*` does not cross a `/` and
    # the data-stream name sits in the middle -- without it this silently
    # matches nothing and reports a clean range.
    path = f":(glob)packages/{package}/data_stream/*/elasticsearch/ingest_pipeline/**"
    output = git(
        "log", "--reverse", "--format=%H%x00%s", f"{good}..{bad}", "--", path
    )
    rows = []
    for line in output.splitlines():
        if "\0" in line:
            sha, subject = line.split("\0", 1)
            rows.append((sha, subject))
    return rows


def capture(source: compat.Source, ref: str) -> tuple[str, dict[str, list[Any]]]:
    """Run every fixture of `source` through Elasticsearch with pipelines at `ref`.

    Args:
        source: The source to run.
        ref: The git ref to read the pipelines at.

    Returns:
        A digest of the whole capture, and the per-fixture output documents.

    Raises:
        CompatError: If the pipelines at `ref` cannot be installed at all.
    """
    installed: dict[str, compat.PipelineSet | str] = {}
    outputs: dict[str, list[Any]] = {}
    failure: str | None = None

    for log_path in compat.list_fixtures(source):
        _, lineage = compat.read_expectation(
            log_path.with_name(log_path.name + "-expected.json")
        )
        pipelines = compat.pipelines_for(lineage, source, installed, ref)
        if isinstance(pipelines, str):
            failure = pipelines
            continue

        config = compat.load_test_config(compat.config_for(log_path))
        events = compat.split_events(
            log_path.read_text(encoding="utf-8"), config.multiline_pattern
        )
        try:
            produced, _ = compat.simulate(
                pipelines.entry, compat.build_docs(events, config), verbose=False
            )
        except compat.CompatError as exc:
            log.warning("%s at %s: %s", log_path.name, ref[:12], exc)
            continue
        outputs[log_path.name] = [strip_volatile(doc) for doc in produced]

    if not outputs:
        raise compat.CompatError(f"nothing ran at {ref[:12]}: {failure or 'no fixtures'}")

    canonical = json.dumps(outputs, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest(), outputs


def first_difference(
    good: dict[str, list[Any]], bad: dict[str, list[Any]], limit: int = 15
) -> list[str]:
    """Describe what moved between two captures, aggregated by field.

    One field changing across every event of a fixture is ONE finding, not
    forty: the per-event listing buries the shape of the change under its
    volume.

    Args:
        good: Per-fixture output documents before the change.
        bad: The same after it.
        limit: How many distinct changes to report.

    Returns:
        Lines, most frequent change first.
    """
    changes: dict[tuple[str, str, str], int] = {}
    counts: list[str] = []

    for fixture in sorted(set(good) | set(bad)):
        before, after = good.get(fixture, []), bad.get(fixture, [])
        if before == after:
            continue
        if len(before) != len(after):
            counts.append(f"{fixture}: {len(before)} events became {len(after)}")
            continue
        for was, now in zip(before, after):
            if was == now:
                continue
            was_paths, now_paths = _paths(was), _paths(now)
            for path in sorted(set(was_paths) | set(now_paths)):
                old = was_paths.get(path, "(absent)")
                new = now_paths.get(path, "(absent)")
                if old != new:
                    changes[(path, old, new)] = changes.get((path, old, new), 0) + 1

    lines = list(counts)
    ranked = sorted(changes.items(), key=lambda kv: (-kv[1], kv[0]))
    for (path, old, new), count in ranked[:limit]:
        lines.append(f"{count:5d}x  {path}: {old[:40]} -> {new[:40]}")
    if len(ranked) > limit:
        lines.append(f"        ... and {len(ranked) - limit} more distinct changes")
    return lines


def _paths(doc: Any, prefix: str = "") -> dict[str, str]:
    """Every leaf path in `doc`, mapped to its value as text."""
    out: dict[str, str] = {}
    if isinstance(doc, dict):
        for key, value in doc.items():
            out.update(_paths(value, f"{prefix}.{key}" if prefix else key))
    elif isinstance(doc, list):
        for index, value in enumerate(doc):
            out.update(_paths(value, f"{prefix}[{index}]"))
    else:
        out[prefix] = str(doc)
    return out


def bisect(source: compat.Source, good: str, bad: str, commits: list[tuple[str, str]]) -> int:
    """Binary search the commits for the first one that changes the output.

    Args:
        source: The source to run.
        good: The ref whose output is the reference.
        bad: The ref known to differ.
        commits: Candidates, oldest first.

    Returns:
        A process exit code.
    """
    good_digest, good_outputs = capture(source, good)
    bad_digest, _ = capture(source, bad)
    print(f"  {good[:12]} (good) digest {good_digest[:16]}")
    print(f"  {bad[:12]} (bad)  digest {bad_digest[:16]}")

    if good_digest == bad_digest:
        print("\nthe two refs produce the SAME output -- nothing upstream moved it.")
        print("look at our own history instead: the change is on this side.")
        return 0

    # Invariant: `lo` matches good, `hi` does not. `hi` starts past the end so
    # that a single candidate is testable.
    lo, hi = -1, len(commits)
    while hi - lo > 1:
        mid = (lo + hi) // 2
        sha, subject = commits[mid]
        digest, _ = capture(source, sha)
        verdict = "same" if digest == good_digest else "MOVED"
        print(f"  {sha[:12]} {verdict}  {subject[:70]}")
        if digest == good_digest:
            lo = mid
        else:
            hi = mid

    if hi == len(commits):
        print(
            "\nno candidate commit changed the output, yet the endpoints differ.\n"
            "the change is outside this package's ingest pipelines -- a shared\n"
            "pipeline, a fixture, or the package's own field definitions."
        )
        return 1

    sha, subject = commits[hi]
    _, culprit_outputs = capture(source, sha)
    print(f"\nfirst commit that moved the output:\n\n  {sha}\n  {subject}\n")
    # Scoped to the package: a sweeping upstream commit touches hundreds of
    # files and the stat for all of them says nothing about this source.
    print(
        git(
            "show",
            "--stat",
            "--format=  %an, %ad%n",
            sha,
            "--",
            f"packages/{source.package}",
        ).strip()
    )
    print("\nwhat moved:")
    for line in first_difference(good_outputs, culprit_outputs):
        print(f"  {line}")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--source", required=True, choices=sorted(compat.SOURCES))
    parser.add_argument("--good", required=True, help="ref whose output is wanted")
    parser.add_argument("--bad", required=True, help="ref whose output is not")
    parser.add_argument(
        "--list",
        action="store_true",
        help="just list the candidate commits, without running Elasticsearch",
    )
    parser.add_argument("--geoip", action="store_true", help="mount local GeoIP databases")
    parser.add_argument("-v", "--verbose", action="store_true")
    args = parser.parse_args(argv)

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(levelname)s %(message)s",
        stream=sys.stderr,
    )

    source = compat.SOURCES[args.source]
    try:
        commits = candidates(source.package, args.good, args.bad)
    except compat.CompatError as exc:
        log.error("%s", exc)
        return 2

    print(
        f"\n{args.source}: {len(commits)} commit(s) touched "
        f"packages/{source.package}'s ingest pipelines between "
        f"{args.good[:12]} and {args.bad[:12]}\n"
    )
    if args.list or not commits:
        for sha, subject in commits:
            print(f"  {sha[:12]}  {subject}")
        if not commits:
            print("  (none -- nothing upstream can have moved this source's output)")
        return 0

    try:
        compat.container_up(geoip=args.geoip)
        return bisect(source, args.good, args.bad, commits)
    except compat.CompatError as exc:
        log.error("%s", exc)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
