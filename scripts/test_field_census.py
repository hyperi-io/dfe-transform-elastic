#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the leaf-path walk field_census.py counts with.

The walk is the whole tool: count the wrong things and the width figures that
decide the schema's shape are wrong. Every case here is one the walk has to get
right for the numbers to mean anything.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import field_census


class Leaves(unittest.TestCase):
    """What counts as a field path, and what does not."""

    def paths(self, value) -> set[str]:
        return set(field_census.leaves(value))

    def test_a_nested_object_yields_dotted_paths(self) -> None:
        self.assertEqual(
            self.paths({"a": {"b": 1, "c": {"d": 2}}}),
            {"a.b", "a.c.d"},
        )

    def test_a_list_contributes_its_items_paths_not_an_index(self) -> None:
        """A column holds the list, so the index is not part of the name."""
        self.assertEqual(
            self.paths({"dns": {"answers": [{"data": "a"}, {"data": "b"}]}}),
            {"dns.answers.data"},
        )

    def test_a_scalar_at_the_root_has_no_path_and_is_dropped(self) -> None:
        self.assertEqual(self.paths("bare"), set())

    def test_an_empty_container_yields_nothing(self) -> None:
        self.assertEqual(self.paths({"a": {}, "b": []}), set())

    def test_a_null_leaf_still_counts_as_a_field(self) -> None:
        """Elasticsearch emitted the key, so a column has to hold it."""
        self.assertEqual(self.paths({"a": None}), {"a"})


class Census(unittest.TestCase):
    """Grouping by package and by data stream."""

    def corpus(self, captures: list[tuple[str, str, list[dict]]]) -> Path:
        """Write a corpus of (package, data_stream, events) and return its root."""
        # Registered for cleanup rather than left behind: mkdtemp alone leaked
        # a tree per test, of a suite that now runs in the gate.
        holder = tempfile.TemporaryDirectory()
        self.addCleanup(holder.cleanup)
        root = Path(holder.name)
        for index, (package, stream, events) in enumerate(captures):
            directory = root / package / stream / f"fixture{index}"
            directory.mkdir(parents=True)
            (directory / "meta.json").write_text(
                json.dumps({"package": package, "data_stream": stream}), encoding="utf-8"
            )
            (directory / "expected.ndjson").write_text(
                "\n".join(json.dumps(event) for event in events), encoding="utf-8"
            )
        return root

    def test_paths_group_by_package_and_by_stream(self) -> None:
        root = self.corpus(
            [
                ("aws", "cloudtrail", [{"aws": {"a": 1}}]),
                ("aws", "vpcflow", [{"aws": {"b": 2}}]),
                ("okta", "system", [{"okta": {"c": 3}}]),
            ]
        )
        events, everything, per_package, per_stream = field_census.census(root)

        self.assertEqual(events, 3)
        self.assertEqual(len(everything), 3)
        # One package spanning two streams counts both its paths once.
        self.assertEqual(per_package["aws"], {"aws.a", "aws.b"})
        self.assertEqual(per_stream["aws.cloudtrail"], {"aws.a"})
        self.assertEqual(len(per_stream), 3)

    def test_a_capture_with_no_expected_output_is_skipped(self) -> None:
        """A capture Elasticsearch never answered has nothing to measure."""
        root = self.corpus([("aws", "cloudtrail", [{"a": 1}])])
        (root / "aws/cloudtrail/fixture0/expected.ndjson").unlink()

        events, everything, _, per_stream = field_census.census(root)
        self.assertEqual((events, everything, per_stream), (0, set(), {}))

    def test_the_same_field_across_events_counts_once(self) -> None:
        root = self.corpus([("aws", "cloudtrail", [{"a": 1}, {"a": 2}, {"a": 3}])])
        events, everything, _, _ = field_census.census(root)
        self.assertEqual((events, everything), (3, {"a"}))


if __name__ == "__main__":
    unittest.main()
