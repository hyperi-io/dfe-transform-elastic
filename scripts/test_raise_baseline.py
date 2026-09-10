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


class ResizeFall(unittest.TestCase):
    """A resize may not carry a fall it does not explain."""

    def test_a_collapse_behind_a_grown_denominator_is_refused(self) -> None:
        """symantec_endpoint_security, verbatim: 52/58 to 3/60.

        The denominator moved by two and 49 events fell, and the old path
        wrote it down without a word.
        """
        old = {"events": 52, "events_total": 58, "fields_wrong": 0}
        new = {"events": 3, "events_total": 60, "fields_wrong": 316}
        self.assertFalse(raise_baseline.resize_explains_the_fall(old, new))

    def test_events_lost_with_the_events_removed_is_accepted(self) -> None:
        old = {"events": 10, "events_total": 12, "fields_wrong": 0}
        new = {"events": 8, "events_total": 10, "fields_wrong": 0}
        self.assertTrue(raise_baseline.resize_explains_the_fall(old, new))

    def test_a_grown_denominator_losing_nothing_is_accepted(self) -> None:
        old = {"events": 10, "events_total": 10, "fields_wrong": 0}
        new = {"events": 10, "events_total": 13, "fields_wrong": 0}
        self.assertTrue(raise_baseline.resize_explains_the_fall(old, new))

    def test_losing_one_more_than_the_denominator_shrank_is_refused(self) -> None:
        old = {"events": 10, "events_total": 12, "fields_wrong": 0}
        new = {"events": 7, "events_total": 10, "fields_wrong": 0}
        self.assertFalse(raise_baseline.resize_explains_the_fall(old, new))


if __name__ == "__main__":
    unittest.main()
