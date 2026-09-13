#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the package attribution skipped_debt.py ranks by.

The join is the whole tool: attribute a generated directory to the wrong
source and the ranking points at the wrong generator gap. A directory is
`<package>_<data stream>` with no separator saying where the package stops,
which is the only hard part.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import skipped_debt


class PackageOf(unittest.TestCase):
    """Which baseline source a generated directory belongs to."""

    KNOWN = {"tanium", "microsoft_dhcp", "google_workspace", "ti_opencti"}

    def package(self, directory: str) -> str | None:
        return skipped_debt.package_of(directory, self.KNOWN)

    def test_the_package_is_the_directorys_prefix(self) -> None:
        self.assertEqual(self.package("tanium_threat_response"), "tanium")

    def test_the_longest_known_prefix_wins(self) -> None:
        """`microsoft_dhcp_log` is the dhcp package, not a `microsoft` one."""
        self.assertEqual(self.package("microsoft_dhcp_log"), "microsoft_dhcp")

    def test_a_directory_that_is_the_package_matches_itself(self) -> None:
        self.assertEqual(self.package("tanium"), "tanium")

    def test_an_unscored_source_attributes_to_nothing(self) -> None:
        """Counted as unattributed rather than guessed onto a neighbour."""
        self.assertIsNone(self.package("kolide_issues"))

    def test_a_shared_first_word_does_not_capture_another_package(self) -> None:
        """`ti_opencti_indicator` is ti_opencti; `ti_anomali` is neither."""
        self.assertEqual(self.package("ti_opencti_indicator"), "ti_opencti")
        self.assertIsNone(self.package("ti_anomali_threatstream"))


class Marker(unittest.TestCase):
    """The condition text lifted out of the generated marker."""

    def test_the_condition_is_everything_after_the_marker(self) -> None:
        line = "            // SKIPPED: condition not transpiled: ctx.a['b c'] != null"
        found = skipped_debt.CONDITION.search(line)
        self.assertIsNotNone(found)
        self.assertEqual(found.group(1).strip(), "ctx.a['b c'] != null")

    def test_an_ordinary_comment_is_not_a_marker(self) -> None:
        self.assertIsNone(skipped_debt.CONDITION.search("// TODO: Transpile Painless"))


if __name__ == "__main__":
    unittest.main()
