#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Tests for the pure functions in compat.py.

Covers the parts that fail silently rather than loudly: an input wrapped twice
produces a document the pipeline cannot parse, a misread lineage compares
against the wrong engine, and a comparison rule that stops matching quietly
turns exclusions into defects.

Run with: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import compat


class SplitEvents(unittest.TestCase):
    """split_events honours first_line_pattern and drops blank lines."""

    def test_one_event_per_line_without_a_pattern(self) -> None:
        self.assertEqual(compat.split_events("a\nb\n\nc\n", None), ["a", "b", "c"])

    def test_continuation_lines_join_the_previous_event(self) -> None:
        text = "Dec 13 first\n  continued\nDec 13 second\n"
        self.assertEqual(
            compat.split_events(text, r"^Dec 13 "),
            ["Dec 13 first\n  continued", "Dec 13 second"],
        )

    def test_leading_continuation_starts_an_event(self) -> None:
        self.assertEqual(compat.split_events("  orphan\n", r"^Dec 13 "), ["  orphan"])

    def test_empty_input_yields_no_events(self) -> None:
        self.assertEqual(compat.split_events("\n\n", None), [])


class BuildDocs(unittest.TestCase):
    """build_docs must not wrap an event that already carries `message`."""

    def test_a_raw_line_is_wrapped(self) -> None:
        docs = compat.build_docs(["Feb  8 04:00:48 host thing"], _config())
        self.assertEqual(docs[0]["_source"]["message"], "Feb  8 04:00:48 host thing")

    def test_an_enveloped_event_is_not_wrapped_again(self) -> None:
        line = json.dumps({"message": "Feb  8 04:00:48 host thing"})
        docs = compat.build_docs([line], _config())
        self.assertEqual(docs[0]["_source"]["message"], "Feb  8 04:00:48 host thing")

    def test_bare_json_without_message_is_wrapped(self) -> None:
        line = json.dumps({"actor": {"id": "abc"}, "eventType": "user.session.start"})
        docs = compat.build_docs([line], _config())
        self.assertEqual(docs[0]["_source"]["message"], line)

    def test_config_fields_are_merged(self) -> None:
        config = compat.TestConfig({"tags": ["preserve_original_event"]}, None, {}, [])
        docs = compat.build_docs(["raw"], config)
        self.assertEqual(docs[0]["_source"]["tags"], ["preserve_original_event"])


class ReadExpectation(unittest.TestCase):
    """Lineage comes from the beats marker fields, not the container shape."""

    def test_bare_array_of_nested_documents_is_integrations(self) -> None:
        events, lineage = self._write([{"event": {"dataset": "okta.system"}}])
        self.assertEqual(lineage, "integrations")
        self.assertEqual(len(events), 1)

    def test_expected_object_is_unwrapped(self) -> None:
        events, lineage = self._write({"expected": [{"event": {"kind": "event"}}]})
        self.assertEqual(lineage, "integrations")
        self.assertEqual(len(events), 1)

    def test_beats_markers_identify_the_lineage(self) -> None:
        _, lineage = self._write([{"fileset.name": "audit", "log.offset": 0}])
        self.assertEqual(lineage, "beats")

    def test_a_bare_array_can_still_be_integrations(self) -> None:
        _, lineage = self._write([{"cisco_nexus": {"log": {"time": "x"}}}])
        self.assertEqual(lineage, "integrations")

    def _write(self, payload: object) -> tuple[list[dict], str]:
        """Write an expectation to a temporary file and read it back."""
        with tempfile.TemporaryDirectory(prefix="compat-") as tmp:
            path = Path(tmp) / "x.log-expected.json"
            path.write_text(json.dumps(payload), encoding="utf-8")
            return compat.read_expectation(path)


class Flatten(unittest.TestCase):
    """Dotted and nested documents must flatten to the same paths."""

    def test_nested_and_dotted_agree(self) -> None:
        nested = compat.flatten({"event": {"dataset": "o365.audit"}})
        dotted = compat.flatten({"event.dataset": "o365.audit"})
        self.assertEqual(nested, dotted)

    def test_list_members_carry_an_index(self) -> None:
        self.assertEqual(
            compat.flatten({"event": {"category": ["a", "b"]}}),
            {"event.category[0]": "a", "event.category[1]": "b"},
        )


class Categorise(unittest.TestCase):
    """Differences land in the bucket the policy says they should."""

    def test_an_identical_document_has_no_differences(self) -> None:
        doc = {"event": {"kind": "event"}}
        self.assertEqual(_totals(compat.categorise(doc, dict(doc))), 0)

    def test_tags_are_not_emitted(self) -> None:
        buckets = compat.categorise({"tags": ["a"]}, {"tags": ["b"]})
        self.assertEqual(buckets["metadata"], {"tags"})
        self.assertFalse(buckets["real"])

    def test_beats_collector_metadata_is_not_emitted(self) -> None:
        buckets = compat.categorise({"input.type": "log"}, {"input.type": "filestream"})
        self.assertEqual(buckets["metadata"], {"input.type"})
        self.assertFalse(buckets["real"])

    def test_geo_is_a_known_difference(self) -> None:
        buckets = compat.categorise(
            {"source": {"geo": {"city_name": "Dublin"}}},
            {"source": {"geo": {"city_name": "Changchun"}}},
        )
        self.assertEqual(buckets["enrichment"], {"source.geo.city_name"})

    def test_a_declared_set_ignores_order(self) -> None:
        buckets = compat.categorise(
            {"event": {"category": ["authentication", "session"]}},
            {"event": {"category": ["session", "authentication"]}},
        )
        self.assertEqual(buckets["order"], {"event.category"})
        self.assertFalse(buckets["real"])

    def test_an_undeclared_array_reordering_is_a_real_difference(self) -> None:
        buckets = compat.categorise(
            {"vendor": {"list": ["a", "b"]}},
            {"vendor": {"list": ["b", "a"]}},
        )
        self.assertTrue(buckets["real"])

    def test_a_genuine_change_is_real(self) -> None:
        buckets = compat.categorise(
            {"user": {"name": "username"}},
            {"user": {"name": "username@elastic.co"}},
        )
        self.assertEqual(buckets["real"], {"user.name"})

    def test_a_missing_field_is_a_real_difference(self) -> None:
        buckets = compat.categorise({"user": {"email": "a@b.c"}}, {})
        self.assertEqual(buckets["real"], {"user.email"})


class FixtureDeclarations(unittest.TestCase):
    """The declarations Elastic ships are honoured, not merely recorded."""

    def test_numeric_keyword_field_compares_as_a_string(self) -> None:
        config = compat.TestConfig({}, None, {}, ["zoom.meeting.id"])
        buckets = compat.categorise(
            {"zoom": {"meeting": {"id": 123}}},
            {"zoom": {"meeting": {"id": "123"}}},
            config,
        )
        self.assertFalse(buckets["real"])

    def test_dynamic_field_matches_by_regex(self) -> None:
        config = compat.TestConfig({}, None, {"vendor.code": r"\d+"}, [])
        buckets = compat.categorise(
            {"vendor": {"code": "123"}},
            {"vendor": {"code": "456"}},
            config,
        )
        self.assertFalse(buckets["real"])

    def test_a_dynamic_field_still_fails_when_the_regex_does_not_match(self) -> None:
        config = compat.TestConfig({}, None, {"vendor.code": r"\d+"}, [])
        buckets = compat.categorise(
            {"vendor": {"code": "123"}},
            {"vendor": {"code": "abc"}},
            config,
        )
        self.assertEqual(buckets["real"], {"vendor.code"})


class PolicyFile(unittest.TestCase):
    """The shipped policy must load and carry a reason for every rule."""

    def test_every_rule_has_a_reason(self) -> None:
        policy = compat.load_policy()
        rules = policy.not_emitted + policy.known_different + policy.nondeterministic
        missing = [rule for rule in rules if not policy.reasons.get(rule)]
        self.assertEqual(missing, [])

    def test_the_unordered_set_is_populated(self) -> None:
        self.assertIn("event.category", compat.load_policy().unordered)

    def test_matching_rule_prefers_the_longest_prefix(self) -> None:
        self.assertEqual(compat.matching_rule("source.geo.city_name"), "source.geo")

    def test_matching_rule_returns_nothing_for_a_vendor_field(self) -> None:
        self.assertIsNone(compat.matching_rule("okta.actor.id"))


class GeoipTypePatch(unittest.TestCase):
    """The mmdb type string is rewritten without disturbing the data section."""

    def test_short_strings_use_one_control_byte(self) -> None:
        self.assertEqual(compat._encode_mmdb_string("GeoLite2-City"), b"\x4dGeoLite2-City")

    def test_long_strings_carry_the_remainder_in_a_second_byte(self) -> None:
        value = "DBIP-ASN-Lite (compat=GeoLite2-ASN)"
        encoded = compat._encode_mmdb_string(value)
        self.assertEqual(encoded[0], 0x40 | 29)
        self.assertEqual(encoded[1], len(value) - 29)
        self.assertEqual(encoded[2:], value.encode())

    def test_patching_replaces_the_type_and_keeps_the_data_section(self) -> None:
        data = b"DATA-SECTION-UNTOUCHED"
        original = (
            data + compat.MMDB_METADATA_MARKER + compat._encode_mmdb_string("DBIP-City-Lite")
        )
        with tempfile.TemporaryDirectory(prefix="compat-") as tmp:
            source = Path(tmp) / "in.mmdb"
            destination = Path(tmp) / "out.mmdb"
            source.write_bytes(original)
            compat.patch_geoip_type(source, destination, "DBIP-City-Lite", "GeoLite2-City")
            patched = destination.read_bytes()
        self.assertTrue(patched.startswith(data))
        self.assertIn(b"GeoLite2-City", patched)
        self.assertNotIn(b"DBIP-City-Lite", patched)

    def test_a_file_without_metadata_is_refused(self) -> None:
        with tempfile.TemporaryDirectory(prefix="compat-") as tmp:
            source = Path(tmp) / "in.mmdb"
            source.write_bytes(b"not an mmdb")
            with self.assertRaises(compat.CompatError):
                compat.patch_geoip_type(source, Path(tmp) / "out.mmdb", "a", "b")


class SourceTable(unittest.TestCase):
    """Every source names a fixture directory that exists."""

    def test_fixture_directories_exist(self) -> None:
        missing = [
            name
            for name, source in compat.SOURCES.items()
            if not (compat.REPO_ROOT / "tests" / "fixtures" / source.fixture_dir).is_dir()
        ]
        self.assertEqual(missing, [])


def _config() -> compat.TestConfig:
    """Return a config with nothing declared."""
    return compat.TestConfig({}, None, {}, [])


def _totals(buckets: dict[str, set[str]]) -> int:
    """Return the number of differing paths across all buckets."""
    return sum(len(paths) for paths in buckets.values())


if __name__ == "__main__":
    unittest.main()
