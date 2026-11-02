#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the variant export in emit_variants.py, which dfe-schemas checks its catalogue against."""

import sys
import unittest
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))

import emit_variants
import sources


def _source(
    *, intakes: tuple[str, ...], name: str, transforms: tuple[str, ...]
) -> sources.Source:
    """A declared source carrying only what the export reads."""
    return sources.Source(
        data_stream="log",
        fixture_dir=f"{name}/log",
        framing=None,
        intakes=intakes,
        name=name,
        package=name,
        pipelines=f"{name}/log",
        transforms=transforms,
    )


class Render(unittest.TestCase):
    """The export's order, format and coverage of the declaration."""

    def test_variants_and_intakes_are_sorted(self) -> None:
        declared = {
            "zeta": _source(
                intakes=("fetcher", "beats"), name="zeta", transforms=("default",)
            ),
            "acme": _source(
                intakes=("receiver", "beats"),
                name="acme",
                transforms=("json", "default"),
            ),
        }
        body = emit_variants.render(declared=declared).removeprefix(
            emit_variants.HEADER
        )
        self.assertEqual(
            body,
            "filebeat.acme.default: [beats, receiver]\n"
            "filebeat.acme.json: [beats, receiver]\n"
            "filebeat.zeta.default: [beats, fetcher]\n",
        )

    def test_the_declaration_exports_every_registry_name_with_its_intakes(self) -> None:
        expected = {}
        for source in sources.SOURCES.values():
            for name in source.registry_names:
                expected[name] = sorted(source.intakes)
        exported = yaml.safe_load(emit_variants.render(declared=sources.SOURCES))
        self.assertEqual(exported, expected)

    def test_the_declaration_renders_the_same_twice(self) -> None:
        first = emit_variants.render(declared=sources.SOURCES)
        self.assertEqual(first, emit_variants.render(declared=sources.SOURCES))


if __name__ == "__main__":
    unittest.main()
