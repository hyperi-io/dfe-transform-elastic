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

import yaml

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
    """What counts as already a document, and what still gets wrapped.

    `elastic-package` wraps every entry of a `.log` fixture into `message`
    without looking at it, so the only lines a wrap must not be applied to are
    the two forms `tests/fixtures/` holds that upstream never had: a published
    Beats document, marked by libbeat's `@metadata` envelope, and a line the
    vendoring already wrapped, whose only key is `message`.
    """

    def test_a_raw_line_is_wrapped(self) -> None:
        docs = compat.build_docs(["Feb  8 04:00:48 host thing"], _config())
        self.assertEqual(docs[0]["_source"]["message"], "Feb  8 04:00:48 host thing")

    def test_a_published_beats_document_is_not_wrapped_again(self) -> None:
        line = json.dumps(
            {
                "@metadata": {"beat": "filebeat", "type": "_doc", "version": "8.13.2"},
                "message": "Oct 10 2018 12:34:56 localhost CiscoASA[999]: built",
                "fileset": {"name": "asa"},
            }
        )
        docs = compat.build_docs([line], _config())
        self.assertEqual(
            docs[0]["_source"]["message"],
            "Oct 10 2018 12:34:56 localhost CiscoASA[999]: built",
        )

    def test_a_line_the_vendoring_already_wrapped_is_not_wrapped_again(self) -> None:
        """cisco_ios, as `tests/fixtures/` holds it.

        Upstream's `test-cisco-ios.log` is raw syslog text; our copy is that
        text with the wrap applied at vendoring time. 36 fixtures across
        cisco_ios, cisco_meraki, cisco_nexus, cisco_umbrella, crowdstrike and
        fortinet are like this, and wrapping one again nests a wrap in a wrap.
        """
        raw = (
            "Feb  8 04:00:48 192.168.100.2 585917: %SEC-6-IPACCESSLOGRP: "
            "list 177 denied igmp 192.168.100.197 -> 224.0.0.22, 1 packet"
        )
        docs = compat.build_docs([json.dumps({"message": raw})], _config())
        self.assertEqual(docs[0]["_source"], {"message": raw})

    def test_bare_json_without_message_is_wrapped(self) -> None:
        line = json.dumps({"actor": {"id": "abc"}, "eventType": "user.session.start"})
        docs = compat.build_docs([line], _config())
        self.assertEqual(docs[0]["_source"]["message"], line)

    def test_config_fields_are_merged(self) -> None:
        config = compat.TestConfig({"tags": ["preserve_original_event"]}, None, {}, [])
        docs = compat.build_docs(["raw"], config)
        self.assertEqual(docs[0]["_source"]["tags"], ["preserve_original_event"])

    def test_the_recorded_input_is_what_was_sent(self) -> None:
        """input.ndjson holds the document bodies, not a second wrap.

        Storing `{"message": <raw text>}` puts a whole Beats document inside
        `message` for every already-enveloped fixture, and the comparison side
        then feeds JSON text to a parser expecting a vendor line.
        """
        line = json.dumps(
            {
                "@metadata": {"beat": "filebeat"},
                "message": "csv,fields,here",
                "agent": {"type": "filebeat"},
            }
        )
        recorded = [doc["_source"] for doc in compat.build_docs([line], _config())]
        self.assertEqual(recorded[0]["message"], "csv,fields,here")
        self.assertEqual(recorded[0]["agent"], {"type": "filebeat"})


class VendorPayloadWithAMessageMember(unittest.TestCase):
    """A vendor payload carrying `message` is still the thing inside the wrap.

    Reading `message` as the envelope marker cost five onboarded sources their
    whole capture: the payload was passed through unwrapped, `event.original`
    was built from the prose line inside it, and the pipeline's `json`
    processor threw a Jackson `Unrecognized token` on every event. One case per
    source, each the first line of the real fixture.
    """

    def test_ece_keeps_the_whole_json_document_in_message(self) -> None:
        """ece's pipeline json-parses `event.original` into `tmp.ece.log`.

        `Unrecognized token 'Created'` -- the second word of the prose line.
        """
        line = json.dumps(
            {
                "@timestamp": "2025-05-09T07:43:09.031238Z",
                "message": "201 Created - philipp - POST /api/v1/deployments (1711 ms)",
                "log": {"logger": "no.found.adminconsole.http.requests"},
                "request_method": "POST",
            }
        )
        self._assert_wrapped(line)

    def test_nextron_thor_keeps_the_whole_json_document_in_message(self) -> None:
        """`Unrecognized token 'At'`."""
        line = json.dumps(
            {
                "message": "At jobs are configured on this system",
                "MODULE": "AtJobs",
                "SCANID": "S-9GyHzKfVRog",
            }
        )
        self._assert_wrapped(line)

    def test_backstage_keeps_the_whole_json_document_in_message(self) -> None:
        """`Unrecognized token 'catalog'`."""
        line = json.dumps(
            {
                "level": "info",
                "message": "catalog processing completed",
                "service": "backstage",
            }
        )
        self._assert_wrapped(line)

    def test_elastic_security_keeps_the_whole_json_document_in_message(self) -> None:
        """`Unrecognized token 'Malicious'`. Note the `agent` member, which is
        vendor data here and not an envelope marker -- beyondtrust_epm has one
        on 2 of its 9 events for the same reason."""
        line = json.dumps(
            {
                "message": "Malicious Behavior Prevention Alert: Suspicious PowerShell",
                "kibana.alert.rule.name": "Endpoint Security",
                "agent": {"id": "abc", "type": "endpoint"},
            }
        )
        self._assert_wrapped(line)

    def test_github_keeps_the_whole_json_document_in_message(self) -> None:
        """The fixture the skip message named, and not one of the five."""
        line = json.dumps(
            {
                "action": "business.add_organization",
                "message": "Organization added to enterprise",
                "@timestamp": 1723570362707,
            }
        )
        self._assert_wrapped(line)

    def _assert_wrapped(self, line: str) -> None:
        """The whole line is the value of `message`, and nothing is hoisted."""
        source = compat.build_docs([line], _config())[0]["_source"]
        self.assertEqual(source, {"message": line})
        self.assertEqual(json.loads(source["message"]), json.loads(line))


class TheCaptureRewritesNothing(unittest.TestCase):
    """The capture neither shortens a value nor escapes one a second time.

    Both have been read off the corpus as harness defects and neither is one.
    ` (truncated)` is the vendor pipelines' own `filterMassive` helper, which
    qualys_vmdr and servicenow run over the whole document; a `message` holding
    four backslashes against an `event.original` holding two is ti_custom's
    `script_unscape_values` walking the document and replacing `\\\\` with `\\`.
    Teaching the capture to compensate for either would put the corpus out of
    step with upstream's own committed expectation, which it currently matches.
    """

    def test_a_long_value_reaches_the_document_at_full_length(self) -> None:
        line = "x" * 40_000
        source = compat.build_docs([line], _config())[0]["_source"]
        self.assertEqual(len(source["message"]), 40_000)
        self.assertNotIn(" (truncated)", source["message"])

    def test_a_long_value_survives_the_round_trip_to_disk(self) -> None:
        line = "x" * 40_000
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "input.ndjson"
            compat._write_ndjson(path, [{"message": line}])
            recorded = json.loads(path.read_text(encoding="utf-8").splitlines()[0])
        self.assertEqual(recorded["message"], line)

    def test_backslash_runs_reach_the_document_unchanged(self) -> None:
        line = r'{"pattern":"HKEY_LOCAL_MACHINE\\\\System"}'
        source = compat.build_docs([line], _config())[0]["_source"]
        self.assertEqual(source["message"], line)
        self.assertEqual(source["message"].count("\\"), 4)

    def test_backslash_runs_survive_the_round_trip_to_disk(self) -> None:
        line = r'{"pattern":"HKEY_LOCAL_MACHINE\\\\System"}'
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "input.ndjson"
            compat._write_ndjson(path, [{"message": line}])
            raw = path.read_text(encoding="utf-8").splitlines()[0]
            recorded = json.loads(raw)
        # Eight on the line and four in the value: one JSON encoding, not two.
        self.assertIn("HKEY_LOCAL_MACHINE" + "\\" * 8 + "System", raw)
        self.assertEqual(recorded["message"], line)


class TestConfigLayering(unittest.TestCase):
    """The shared config and the fixture's own BOTH apply, shared first.

    `elastic-package` unpacks `test-common-config.yml` and then the fixture's
    own `-config.yml` into the same struct. Discarding the shared one whenever
    an own one exists cost microsoft_dhcp its whole capture.
    """

    def test_the_shared_config_survives_an_own_config(self) -> None:
        """microsoft_dhcp, exactly as upstream ships it.

        Its own config names only `log.file.path`. Dropping the shared file
        took `_conf.tz_offset` with it, the date processor resolved an empty
        zone, and all 32 events came back
        `Invalid ID for ZoneOffset, invalid format: ""`.
        """
        config = self._layer(
            {"fields": {"tags": ["preserve_original_event"],
                        "_conf": {"tz_offset": "America/New_York"}}},
            {"fields": {"log": {"file": {"path": "DhcpSrvLog-Thu.txt"}}}},
        )
        self.assertEqual(config.fields["_conf"], {"tz_offset": "America/New_York"})
        self.assertEqual(config.fields["tags"], ["preserve_original_event"])
        self.assertEqual(config.fields["log"], {"file": {"path": "DhcpSrvLog-Thu.txt"}})

    def test_the_own_config_wins_a_shared_key(self) -> None:
        config = self._layer(
            {"fields": {"_conf": {"ioc_expiration_duration": "5d"}}},
            {"fields": {"_conf": {"ioc_expiration_duration": ""}}},
        )
        self.assertEqual(config.fields["_conf"], {"ioc_expiration_duration": ""})

    def test_a_nested_map_layers_key_by_key(self) -> None:
        """symantec_endpoint's own `_conf` adds a key without losing the
        shared one."""
        config = self._layer(
            {"fields": {"_conf": {"tz_offset": "UTC"}}},
            {"fields": {"_conf": {"remove_mapped_fields": True}}},
        )
        self.assertEqual(
            config.fields["_conf"], {"tz_offset": "UTC", "remove_mapped_fields": True}
        )

    def test_a_shorter_list_keeps_the_shared_tail(self) -> None:
        """go-ucfg merges a list by POSITION, and two fixtures depend on it.

        symantec_endpoint declares `tags: [forwarded]` over a shared
        `[forwarded, preserve_original_event]`, and upstream's expectation
        carries BOTH -- with `event.kind: event`, so nothing appended it on
        failure.
        """
        config = self._layer(
            {"fields": {"tags": ["forwarded", "preserve_original_event"]}},
            {"fields": {"tags": ["forwarded"]}},
        )
        self.assertEqual(config.fields["tags"], ["forwarded", "preserve_original_event"])

    def test_a_longer_list_takes_every_member(self) -> None:
        """suricata, the same rule the other way about."""
        config = self._layer(
            {"fields": {"tags": ["preserve_original_event"]}},
            {"fields": {"tags": ["forwarded", "preserve_original_event"]}},
        )
        self.assertEqual(config.fields["tags"], ["forwarded", "preserve_original_event"])

    def test_declarations_outside_fields_layer_too(self) -> None:
        """citrix_adc and haproxy declare `dynamic_fields` only in the shared
        file, and it reaches the corpus through `meta.json`."""
        config = self._layer(
            {"dynamic_fields": {"url.extension": "^.*$"},
             "numeric_keyword_fields": ["zoom.meeting.id"]},
            {"fields": {"tags": ["preserve_original_event"]}},
        )
        self.assertEqual(config.dynamic_fields, {"url.extension": "^.*$"})
        self.assertEqual(config.numeric_keyword_fields, ["zoom.meeting.id"])

    def test_a_multiline_pattern_survives_an_own_config(self) -> None:
        config = self._layer(
            {"multiline": {"first_line_pattern": "^Dec 13 "}},
            {"fields": {"tags": ["forwarded"]}},
        )
        self.assertEqual(config.multiline_pattern, "^Dec 13 ")

    def test_absent_configs_default(self) -> None:
        config = compat.load_test_config(None)
        self.assertEqual(config.fields, {})
        self.assertIsNone(config.multiline_pattern)

    def _layer(self, shared: dict, own: dict) -> compat.TestConfig:
        """Write both configs beside a fixture and read them back through
        `configs_for`, so the ORDER under test is the one production uses."""
        with tempfile.TemporaryDirectory(prefix="compat-") as tmp:
            fixture = Path(tmp) / "test-x.log"
            fixture.write_text("a line\n", encoding="utf-8")
            fixture.with_name("test-common-config.yml").write_text(
                yaml.safe_dump(shared), encoding="utf-8"
            )
            fixture.with_name("test-x.log-config.yml").write_text(
                yaml.safe_dump(own), encoding="utf-8"
            )
            paths = compat.configs_for(fixture)
            self.assertEqual(
                [p.name for p in paths],
                ["test-common-config.yml", "test-x.log-config.yml"],
            )
            return compat.load_test_config(paths, fixture)


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
    """No more sources point at a fixture directory than already do."""

    @property
    def SOURCES_WITHOUT_FIXTURES(self) -> int:  # noqa: N802 - a ratchet, named as one
        """Streams with no fixtures upstream, which COMPAT.md states as expected.

        A ceiling rather than zero, so a mistyped fixture_dir still fails. Read
        from the same `tests/ratchets.json` the Rust ratchets use, which is
        what makes that file the source of truth rather than one language's
        copy of a number.
        """
        ratchets = json.loads(
            (compat.REPO_ROOT / "tests/ratchets.json").read_text(encoding="utf-8")
        )
        return ratchets["sources_without_fixtures"]

    # The table has to be loaded for the ceiling below to mean anything: an
    # empty SOURCES gives no missing entries and clears the ceiling on nothing.
    MIN_SOURCES = 1_000

    def test_the_source_table_is_loaded(self) -> None:
        self.assertGreaterEqual(len(compat.SOURCES), self.MIN_SOURCES)

    def test_no_more_sources_lack_a_fixture_directory(self) -> None:
        self.assertGreaterEqual(len(compat.SOURCES), self.MIN_SOURCES)
        missing = [
            name
            for name, source in compat.SOURCES.items()
            if not (compat.REPO_ROOT / "tests" / "fixtures" / source.fixture_dir).is_dir()
        ]
        self.assertLessEqual(
            len(missing),
            self.SOURCES_WITHOUT_FIXTURES,
            f"{len(missing)} sources name a fixture directory that does not exist, "
            f"up from {self.SOURCES_WITHOUT_FIXTURES}. A new one is either a "
            f"mistyped fixture_dir or a stream upstream has no fixtures for: "
            f"{sorted(set(missing))[:5]}",
        )


def _config() -> compat.TestConfig:
    """Return a config with nothing declared."""
    return compat.TestConfig({}, None, {}, [])


def _totals(buckets: dict[str, set[str]]) -> int:
    """Return the number of differing paths across all buckets."""
    return sum(len(paths) for paths in buckets.values())


if __name__ == "__main__":
    unittest.main()
