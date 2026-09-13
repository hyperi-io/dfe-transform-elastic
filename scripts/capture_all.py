#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Capture Elasticsearch's output for every declared source with no corpus yet.

`compat.py generate` takes ONE source and a bucket is ninety-odd, so this
drives it per source. It keeps going past a failure and reports the failures at
the end, because one pipeline Elasticsearch will not install must not cost the
rest of the bucket its capture.

    scripts/capture_all.py             # every source with no capture
    scripts/capture_all.py zeek qualys # only those name prefixes
"""

import json
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
CORPUS = REPO / "testdata" / "compat"


def has_corpus(entry: dict) -> bool:
    """Whether the corpus already holds a capture for this source."""
    directory = CORPUS / entry["package"] / entry["data_stream"]
    return directory.is_dir() and any(directory.iterdir())


def main() -> int:
    import yaml

    raw = yaml.safe_load((REPO / "sources.yaml").read_text(encoding="utf-8"))
    wanted = [name for name, entry in raw["sources"].items() if not has_corpus(entry)]
    if len(sys.argv) > 1:
        prefixes = tuple(sys.argv[1:])
        wanted = [name for name in wanted if name.startswith(prefixes)]

    print(f"{len(wanted)} source(s) to capture", flush=True)
    failed: dict[str, str] = {}
    for i, name in enumerate(wanted, 1):
        print(f"[{i}/{len(wanted)}] {name}", flush=True)
        done = subprocess.run(
            [sys.executable, str(REPO / "scripts" / "compat.py"), "generate", "--source", name],
            capture_output=True,
            text=True,
            encoding="utf-8",
            check=False,
        )
        if done.returncode != 0:
            tail = (done.stderr or done.stdout).strip().splitlines()
            failed[name] = tail[-1] if tail else f"exit {done.returncode}"
            print(f"    FAILED {failed[name][:300]}", flush=True)

    print(f"\n{len(wanted) - len(failed)} captured, {len(failed)} failed")
    if failed:
        print(json.dumps(failed, indent=2)[:8000])
    return 0


if __name__ == "__main__":
    sys.exit(main())
