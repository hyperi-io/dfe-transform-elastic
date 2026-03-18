// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Benchmarks comparing dfe-parse native parsers against regex equivalents.
//!
//! Run with: `cargo bench -p dfe-parse`

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dfe_parse::{ip, network, numeric, string, timestamp};

fn bench_ipv4(c: &mut Criterion) {
    let input = "192.168.1.1:8080";
    c.bench_function("parse_ipv4", |b| {
        b.iter(|| ip::parse_ipv4(black_box(input)))
    });
}

fn bench_ipv6(c: &mut Criterion) {
    let input = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
    c.bench_function("parse_ipv6", |b| {
        b.iter(|| ip::parse_ipv6(black_box(input)))
    });
}

fn bench_hostname(c: &mut Criterion) {
    let input = "web-01.prod.example.com:443";
    c.bench_function("parse_hostname", |b| {
        b.iter(|| ip::parse_hostname(black_box(input)))
    });
}

fn bench_ip_or_host(c: &mut Criterion) {
    let ip_input = "10.0.0.1:8080";
    let host_input = "example.com:443";
    c.bench_function("parse_ip_or_host/ip", |b| {
        b.iter(|| ip::parse_ip_or_host(black_box(ip_input)))
    });
    c.bench_function("parse_ip_or_host/host", |b| {
        b.iter(|| ip::parse_ip_or_host(black_box(host_input)))
    });
}

fn bench_int(c: &mut Criterion) {
    let input = "42 rest";
    c.bench_function("parse_int", |b| {
        b.iter(|| numeric::parse_int(black_box(input)))
    });
}

fn bench_number(c: &mut Criterion) {
    c.bench_function("parse_number/integer", |b| {
        b.iter(|| numeric::parse_number(black_box("12345")))
    });
    c.bench_function("parse_number/float", |b| {
        b.iter(|| numeric::parse_number(black_box("3.14159")))
    });
    c.bench_function("parse_number/exponent", |b| {
        b.iter(|| numeric::parse_number(black_box("1.5e10")))
    });
}

fn bench_port(c: &mut Criterion) {
    c.bench_function("parse_port", |b| {
        b.iter(|| numeric::parse_port(black_box("8080/tcp")))
    });
}

fn bench_iso8601(c: &mut Criterion) {
    let input = "2024-01-15T10:30:00.123456Z rest";
    c.bench_function("parse_iso8601", |b| {
        b.iter(|| timestamp::parse_iso8601(black_box(input)))
    });
}

fn bench_syslog_timestamp(c: &mut Criterion) {
    let input = "Jan 15 10:30:00 rest";
    c.bench_function("parse_syslog_timestamp", |b| {
        b.iter(|| timestamp::parse_syslog_timestamp(black_box(input)))
    });
}

fn bench_word(c: &mut Criterion) {
    let input = "hello_world rest";
    c.bench_function("take_word", |b| {
        b.iter(|| string::take_word(black_box(input)))
    });
}

fn bench_quoted(c: &mut Criterion) {
    let input = r#""hello world with \"escaped\" quotes" rest"#;
    c.bench_function("take_quoted", |b| {
        b.iter(|| string::take_quoted(black_box(input)))
    });
}

fn bench_loglevel(c: &mut Criterion) {
    c.bench_function("parse_loglevel", |b| {
        b.iter(|| string::parse_loglevel(black_box("ERROR rest")))
    });
}

fn bench_mac(c: &mut Criterion) {
    c.bench_function("parse_mac/colon", |b| {
        b.iter(|| network::parse_mac(black_box("aa:bb:cc:dd:ee:ff rest")))
    });
    c.bench_function("parse_mac/cisco", |b| {
        b.iter(|| network::parse_mac(black_box("aabb.ccdd.eeff rest")))
    });
}

fn bench_uuid(c: &mut Criterion) {
    let input = "550e8400-e29b-41d4-a716-446655440000 rest";
    c.bench_function("parse_uuid", |b| {
        b.iter(|| network::parse_uuid(black_box(input)))
    });
}

fn bench_uri(c: &mut Criterion) {
    let input = "https://example.com/path?query=1&b=2#fragment rest";
    c.bench_function("parse_uri", |b| {
        b.iter(|| network::parse_uri(black_box(input)))
    });
}

fn bench_email(c: &mut Criterion) {
    let input = "user.name+tag@sub.example.com rest";
    c.bench_function("parse_email", |b| {
        b.iter(|| network::parse_email(black_box(input)))
    });
}

criterion_group!(
    benches,
    bench_ipv4,
    bench_ipv6,
    bench_hostname,
    bench_ip_or_host,
    bench_int,
    bench_number,
    bench_port,
    bench_iso8601,
    bench_syslog_timestamp,
    bench_word,
    bench_quoted,
    bench_loglevel,
    bench_mac,
    bench_uuid,
    bench_uri,
    bench_email,
);
criterion_main!(benches);
