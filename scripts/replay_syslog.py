#!/usr/bin/env python3
# SPDX-License-Identifier: BUSL-1.1
# Copyright (c) 2026 HYPERI PTY LIMITED
"""Replay committed vendor log fixtures at a dfe-receiver syslog listener.

Feeds agentless mode: one stream, many device types. Lines come from
tests/fixtures/unencumbered/<package>/*.log and are sent as syslog over UDP
(one datagram per line) or TCP (newline framed). Standard library only.

    replay_syslog.py --list
    replay_syslog.py --target udp://receiver:514 --sources cisco-asa,fortinet,panw
    replay_syslog.py --target tcp://receiver:514 --sources cisco-ios --rate 200 --loop --duration 60

--sources takes catalogue labels, read from src/detect/labels.rs (the table the
detector stamps `_source` from). dfe-receiver's HTTP /ingest takes JSON, not
syslog, so there is no http:// form.
"""

import argparse
import itertools
import re
import socket
import sys
import time
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parent.parent
LABELS = ROOT / "src" / "detect" / "labels.rs"
FIXTURES = ROOT / "tests" / "fixtures" / "unencumbered"

# local0.info, for lines whose fixture carries no PRI of its own.
DEFAULT_PRI = "<134>"

PRI = re.compile(r"<\d{1,3}>")
BSD_STAMP = re.compile(r"[A-Z][a-z]{2} [ \d]\d \d\d:\d\d:\d\d")
BARE_ASA_ID = re.compile(r"ASA-\d-\d{6}:")
LABEL_ROW = re.compile(r'\(\s*"filebeat\.([a-z0-9_]+)\.default",\s*"([^"]+)"\s*,?\s*\)')


def label_table() -> dict[str, str]:
    """Catalogue label to fixture package, from the generated detector table."""
    if not LABELS.is_file():
        sys.exit(f"label table not found at {LABELS}")
    text = LABELS.read_text(encoding="utf-8")
    return {label: package for package, label in LABEL_ROW.findall(text)}


def fixture_files(package: str) -> list[Path]:
    return sorted((FIXTURES / package).glob("*.log"))


def labels_with_fixtures(table: dict[str, str]) -> dict[str, list[Path]]:
    return {
        label: files
        for label, pkg in sorted(table.items())
        if (files := fixture_files(pkg))
    }


def wrap(line: str, host: str = "replay") -> str:
    """Give `line` a syslog header only if the fixture lacks one."""
    if BARE_ASA_ID.match(line):
        # A device writes %ASA-n-id and the logstash fixtures dropped the percent sign.
        line = "%" + line
    if PRI.match(line):
        return line
    if BSD_STAMP.match(line):
        return DEFAULT_PRI + line
    stamp = time.strftime("%b %e %H:%M:%S")
    return f"{DEFAULT_PRI}{stamp} {host} replay: {line}"


def read_lines(files: list[Path]) -> list[str]:
    lines = []
    for path in files:
        with path.open(encoding="utf-8", errors="replace") as handle:
            lines.extend(line.rstrip("\r\n") for line in handle if line.strip())
    return lines


def pick_sources(
    spec: str, available: dict[str, list[Path]], table: dict[str, str]
) -> list[str]:
    chosen = [s.strip() for s in spec.split(",") if s.strip()]
    if not chosen:
        sys.exit("--sources is empty")
    for label in chosen:
        if label not in table:
            sys.exit(f"'{label}' is not a catalogue label in {LABELS.name}")
        if label not in available:
            sys.exit(
                f"label '{label}' has no fixtures under {FIXTURES}. "
                f"Labels with fixtures: {', '.join(available)}"
            )
    return chosen


def stream(sources: list[str], available: dict[str, list[Path]], loop: bool):
    """Interleave the sources one line each, so a short run still sees every one."""
    pools = [read_lines(available[s]) for s in sources]
    cycles = [itertools.cycle(p) if loop else iter(p) for p in pools]
    live = list(range(len(cycles)))
    while live:
        for i in list(live):
            line = next(cycles[i], None)
            if line is None:
                live.remove(i)
            else:
                yield wrap(line)


class Sender:
    def __init__(self, target: str):
        url = urlparse(target)
        if url.scheme not in ("udp", "tcp") or not url.hostname or not url.port:
            sys.exit(
                f"--target must be udp://host:port or tcp://host:port, got '{target}'"
            )
        self.tcp = url.scheme == "tcp"
        addr = (url.hostname, url.port)
        if self.tcp:
            self.sock = socket.create_connection(addr, timeout=10)
        else:
            self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            self.addr = addr

    def send(self, line: str) -> None:
        data = line.encode("utf-8", errors="replace")
        if self.tcp:
            self.sock.sendall(data + b"\n")
        else:
            self.sock.sendto(data, self.addr)

    def close(self) -> None:
        self.sock.close()


def main() -> None:
    p = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    p.add_argument(
        "--target", help="udp://host:port or tcp://host:port (newline framed)"
    )
    p.add_argument(
        "--sources", help="comma list of catalogue labels, e.g. cisco-asa,fortinet,panw"
    )
    p.add_argument(
        "--rate", type=float, default=20.0, help="lines per second (default 20)"
    )
    p.add_argument("--count", type=int, help="stop after this many lines")
    p.add_argument("--duration", type=float, help="stop after this many seconds")
    p.add_argument(
        "--loop",
        action="store_true",
        help="cycle the fixtures until --count or --duration",
    )
    p.add_argument(
        "--list",
        action="store_true",
        help="list the labels that have fixtures and exit",
    )
    p.add_argument(
        "--dry-run", action="store_true", help="print the lines instead of sending them"
    )
    args = p.parse_args()

    table = label_table()
    available = labels_with_fixtures(table)
    if args.list:
        for label, files in available.items():
            print(
                f"{label}\t{len(read_lines(files))} lines\t{', '.join(f.name for f in files)}"
            )
        return
    if not args.sources or not (args.target or args.dry_run):
        p.error("--sources and --target are required")
    if args.loop and args.count is None and args.duration is None:
        p.error("--loop needs --count or --duration")
    if args.rate <= 0:
        p.error("--rate must be positive")

    sources = pick_sources(args.sources, available, table)
    sender = None if args.dry_run else Sender(args.target)
    started = time.monotonic()
    sent = 0
    try:
        for line in stream(sources, available, args.loop):
            if args.count is not None and sent >= args.count:
                break
            if (
                args.duration is not None
                and time.monotonic() - started >= args.duration
            ):
                break
            if sender:
                sender.send(line)
            else:
                print(line)
            sent += 1
            delay = started + sent / args.rate - time.monotonic()
            if delay > 0:
                time.sleep(delay)
    except KeyboardInterrupt:
        pass
    finally:
        if sender:
            sender.close()
    elapsed = time.monotonic() - started
    print(
        f"sent {sent} lines from {', '.join(sources)} in {elapsed:.1f}s",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
