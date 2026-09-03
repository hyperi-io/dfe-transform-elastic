#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the join shape_reach.py makes between the two dumps.

The join is the whole tool: match the wrong halves and it reports a defect
list that is really a list of shapes doing their job, or misses the ones that
are not. Every case here is one the join has to get right for the number to
mean anything.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import shape_reach


def dumps(scripts: list[dict], reach: list[dict]) -> tuple[Path, Path]:
    """Write a census and a reach dump, returning both paths."""
    directory = Path(tempfile.mkdtemp())
    binding = directory / "binding.json"
    runtime = directory / "reach.json"
    binding.write_text(json.dumps({"scripts": scripts}), encoding="utf-8")
    runtime.write_text(
        json.dumps(
            {
                "distinct": len(reach),
                "never_ran": sum(1 for row in reach if row["ran"] == 0),
                "scripts": reach,
            }
        ),
        encoding="utf-8",
    )
    return binding, runtime


class ShapeOf(unittest.TestCase):
    """The family name is what ranks; the payload the census prints is not."""

    def test_a_payload_is_not_part_of_the_name(self) -> None:
        self.assertEqual(shape_reach.shape_of("GuardedCopy(Program { .. })"), "GuardedCopy")

    def test_a_braced_payload_is_not_part_of_the_name(self) -> None:
        self.assertEqual(shape_reach.shape_of("DropEmpty { policy: .. }"), "DropEmpty")

    def test_a_bare_name_survives_intact(self) -> None:
        self.assertEqual(shape_reach.shape_of("Basename"), "Basename")


class Load(unittest.TestCase):
    """A missing dump says which command produces it rather than tracebacks."""

    def test_a_missing_dump_exits_with_the_command_to_run(self) -> None:
        with self.assertRaises(SystemExit) as raised:
            shape_reach.load(Path("/nonexistent/binding.json"), "binding census")
        self.assertIn("DFE_BINDING_DUMP", str(raised.exception))


class Join(unittest.TestCase):
    """What the two dumps agree on, and what each excludes."""

    def setUp(self) -> None:
        self.argv = sys.argv

    def tearDown(self) -> None:
        sys.argv = self.argv

    def run_tool(self, scripts: list[dict], reach: list[dict], *flags: str) -> str:
        binding, runtime = dumps(scripts, reach)
        sys.argv = [
            "shape_reach.py",
            "--binding",
            str(binding),
            "--reach",
            str(runtime),
            *flags,
        ]
        from contextlib import redirect_stdout
        from io import StringIO

        out = StringIO()
        with redirect_stdout(out):
            shape_reach.main()
        return out.getvalue()

    def test_a_script_that_never_ran_and_is_bound_is_counted(self) -> None:
        scripts = [{"head": "if (a)", "binding": ["Basename"], "uses": [{}, {}]}]
        reach = [{"script": "if (a)", "ran": 0, "skipped": 7}]
        report = self.run_tool(scripts, reach)
        self.assertIn("7 invocations      2 sites  Basename", report)

    def test_a_script_that_sometimes_ran_is_excluded(self) -> None:
        """Declining on events lacking the source field is ordinary, not a defect."""
        scripts = [{"head": "if (a)", "binding": ["Basename"], "uses": [{}]}]
        reach = [{"script": "if (a)", "ran": 3, "skipped": 9}]
        report = self.run_tool(scripts, reach)
        self.assertIn("sometimes declined (ordinary, excluded): 1", report)
        self.assertNotIn("Basename", report.split("by the shape")[1])

    def test_a_script_nothing_binds_is_not_blamed_on_a_shape(self) -> None:
        reach = [{"script": "if (a)", "ran": 0, "skipped": 4}]
        report = self.run_tool([], reach)
        self.assertIn("honestly unbound:  4 invocations", report)

    def test_solo_drops_a_script_something_else_may_handle(self) -> None:
        """A second binding ahead of it may be doing the work correctly."""
        scripts = [
            {"head": "if (a)", "binding": ["Basename", "GuardedCopy"], "uses": [{}]},
            {"head": "if (b)", "binding": ["JoinOptional"], "uses": [{}]},
        ]
        reach = [
            {"script": "if (a)", "ran": 0, "skipped": 5},
            {"script": "if (b)", "ran": 0, "skipped": 2},
        ]
        self.assertIn("Basename", self.run_tool(scripts, reach))
        self.assertNotIn("Basename", self.run_tool(scripts, reach, "--solo"))

    def test_the_join_keys_on_the_shorter_run(self) -> None:
        """The census truncates a script; the reach dump keeps more of it."""
        head = "x" * shape_reach.KEY
        scripts = [{"head": head, "binding": ["Basename"], "uses": [{}]}]
        reach = [{"script": head + " and a much longer tail", "ran": 0, "skipped": 1}]
        self.assertIn("Basename", self.run_tool(scripts, reach))


if __name__ == "__main__":
    unittest.main()
