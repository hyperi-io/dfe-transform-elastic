#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the lines raise_baseline.py reads out of a corpus run.

Every one of these patterns reads a line the Rust test prints. Reword the
print and the pattern stops matching -- which, before the guard in `main`,
would have stopped a ratchet silently rather than loudly.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import raise_baseline


class Reach(unittest.TestCase):
    """The `never_ran` count, lifted out of the reach line."""

    LINE = (
        "painless runtime reach: 59789 handled, 10873 skipped across 1270 "
        "distinct scripts, 523 of which NEVER ran"
    )

    def test_the_never_ran_count_is_read(self) -> None:
        found = raise_baseline.REACH.search(self.LINE)
        self.assertIsNotNone(found)
        self.assertEqual(found["never"], "523")

    def test_a_reworded_line_does_not_match(self) -> None:
        """The guard in main turns this into a failure rather than a silence."""
        self.assertIsNone(raise_baseline.REACH.search("reach: 523 never ran"))


class Entries(unittest.TestCase):
    """The per-source lines, in each of their three markings."""

    ENTRY = '"tanium": { "events": 6, "events_total": 54, "fields_wrong": 1968 },'

    def test_a_plain_line_is_a_raise(self) -> None:
        found = raise_baseline.scores_in(f"  {self.ENTRY}", raise_baseline.LINE)
        self.assertEqual(
            found, {"tanium": {"events": 6, "events_total": 54, "fields_wrong": 1968}}
        )

    def test_a_resize_needs_its_own_marking(self) -> None:
        """A resized entry must not be taken by the plain raise pattern."""
        text = f"RESIZE  {self.ENTRY}"
        self.assertEqual(raise_baseline.scores_in(text, raise_baseline.LINE), {})
        self.assertIn("tanium", raise_baseline.scores_in(text, raise_baseline.RESIZE))

    def test_a_new_source_needs_its_own_marking(self) -> None:
        text = f"NEW     {self.ENTRY}"
        self.assertEqual(raise_baseline.scores_in(text, raise_baseline.LINE), {})
        self.assertIn("tanium", raise_baseline.scores_in(text, raise_baseline.NEW))

    def test_prose_mentioning_a_source_is_not_an_entry(self) -> None:
        self.assertEqual(
            raise_baseline.scores_in("  tanium: 6/54 events, baseline 6", raise_baseline.LINE),
            {},
        )


if __name__ == "__main__":
    unittest.main()
