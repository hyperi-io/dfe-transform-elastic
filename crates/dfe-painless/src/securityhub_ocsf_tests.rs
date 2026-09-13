// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Tests for securityhub's OCSF entity extraction.

use serde_json::json;

use super::{
    OcsfRemediation, OcsfResource, OcsfSeverity, OcsfVulnerability, parse_ocsf_remediation,
    parse_ocsf_resource, parse_ocsf_severity, parse_ocsf_vulnerability, run_ocsf_remediation,
    run_ocsf_resource, run_ocsf_severity, run_ocsf_vulnerability,
};
use crate::common::normalise;
use dfe_core::Event;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/aws_securityhub_finding/default.rs`,
/// quoted in the ESCAPED one-line form the site holds.
const OCSF_RESOURCE: &str = r#"// Arrays won't work in general in current UI of Cloud Security Posture workflow. In AWS SecurityHub, a finding may contain multiple resources, but rarely.\n// When a finding has single-resource, we extract ECS fields as single-value so that the Findings UI behaves as expected for almost all cases.\n// But in the rare multi-resource case, we extract the ECS fields from the primary resource. \n\ndef res;\n\nif (ctx.aws_securityhub.finding.resources.size() == 1) {\n  res = ctx.aws_securityhub.finding.resources[0];\n} else {\n  for (def resource: ctx.aws_securityhub.finding.resources) {\n    if (resource.labels?.contains('primary_resource') == true) {\n      res = resource;\n      break;\n    }\n  }\n}\n\n// Define fields to be extracted. \nctx.resource = ctx.resource ?: [:];\nctx.user = ctx.user ?: [:];\nctx.group = ctx.group ?: [:];\nctx.host = ctx.host ?: [:];\nctx.host.ip = ctx.host.ip ?: [];\nctx.orchestrator = ctx.orchestrator ?: [:];\nctx.orchestrator.cluster = ctx.orchestrator.cluster ?: [:];\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.service = ctx.cloud.service ?: [:];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\n\n// Extract resource field\nctx.resource.type = res.type;\nctx.resource.id = res.uid;\nif (res.tags instanceof List) {\n  for (def tag: res.tags) {\n    if (tag.name == 'Name') {\n      ctx.resource.name = tag.value;\n      break;\n    }\n  }\n}\n\n// Extract ECS user field\nif (res.type == 'AWS::IAM::User') {\n  ctx.user.id = res.uid;\n}\n\n// Extract ECS group field\nif (res.type == 'AWS::IAM::Group') {\n  ctx.group.id = res.uid;\n}\n\n// Extract ECS host field\nif (res.type == 'AWS::EC2::Instance' && res.uid != null) {\n  ctx.host.id = res.uid;\n  ctx.host.name = res.uid;\n}\nif (res.type == 'AWS::EC2::Instance' && res.data?.awsEc2InstanceDetails?.type != null) {\n  ctx.host.type = res.data.awsEc2InstanceDetails.type;\n}\nif (res.type == 'AWS::EC2::Instance' && res.data?.awsEc2InstanceDetails?.ipV4Addresses instanceof List) {\n  for (def ipv4: res.data.awsEc2InstanceDetails.ipV4Addresses) {\n    if (ipv4 instanceof String) {\n      ctx.host.ip.add(ipv4);\n    }\n  }\n}\nif (res.type == 'AWS::EC2::Instance' && res.data?.awsEc2InstanceDetails?.ipV6Addresses instanceof List) {\n  for (def ipv6: res.data.awsEc2InstanceDetails.ipV6Addresses) {\n    if (ipv6 instanceof String) {\n      ctx.host.ip.add(ipv6);\n    }\n  }\n}\n\n// Extract ECS orchestrator field\nif (res.tags instanceof List) {\n  for (def tag: res.tags) {\n    if (tag.name == 'aws:eks:cluster-name') {\n      ctx.orchestrator.cluster.name = tag.value;\n    }\n  }\n}\nif (res.type == 'AWS::EKS::Cluster') {\n  ctx.orchestrator.cluster.id = res.uid;\n  ctx.orchestrator.type = 'kubernetes';\n}\n\n// Extract ECS cloud field\nctx.cloud.region = res.region;\nctx.cloud.service.name = res.type;\nif (res.type == 'AWS::EC2::Instance' && res.data?.awsEc2InstanceDetails?.type != null) {\n  ctx.cloud.machine.type = res.data.awsEc2InstanceDetails.type;\n}\nif (res.type == 'AWS::EC2::Instance') {\n  ctx.cloud.instance.id = res.uid;\n  ctx.cloud.instance.name = res.name;\n}\n\n// Convert key:value tags into object for better searchability.\nfor (def resource: ctx.aws_securityhub.finding.resources) {\n  if (resource.tags instanceof List) {\n    def tags = [:];\n    for (def tag: resource.tags) {\n      tags[tag.name] = tag.value;\n    }\n    resource.tags = tags;\n  }\n}"#;

/// The parse, and the list path read off the script rather than named here.
#[test]
fn the_list_is_read_off_the_script() {
    let pattern = parse_ocsf_resource(&normalise(OCSF_RESOURCE)).expect("the script is recognised");
    assert_eq!(pattern.source(), "aws_securityhub.finding.resources");
}

/// The ASFF sibling opens on the same comment over a list of the same name, and
/// reads members this runner would not find.
#[test]
fn the_asff_spelling_is_declined() {
    let asff = "if (resources.size() == 1) {\n  def res = ctx.aws.securityhub_findings.resources[0];\n  \
                ctx.resource.type = res.Type;\n  ctx.resource.id = res.Id;\n  \
                if (res.Details != null && res.Details[res.Type]?.Name != null) {\n    \
                ctx.resource.name = res.Details[res.Type].Name;\n  }\n}";
    assert!(parse_ocsf_resource(asff).is_none());
}

/// Run the script through the pattern and hand back the event it wrote.
fn run(document: serde_json::Value) -> Event {
    let pattern: OcsfResource =
        parse_ocsf_resource(&normalise(OCSF_RESOURCE)).expect("the script is recognised");
    let mut event = Event::new(document);
    assert!(run_ocsf_resource(&mut event, &pattern));
    event
}

/// The three fields every finding carries, off its one resource.
///
/// The values are the ones `testdata/compat/aws_securityhub/finding/
/// test-findings/expected.ndjson` carries for its first event.
#[test]
fn a_single_resource_names_the_entity() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [{
            "cloud_partition": "aws",
            "region": "us-east-2",
            "type": "AWS::SQS::Queue",
            "uid": "https://sqs.us-east-2.amazonaws.com/123456789012/securityhubfinding",
        }] } }
    }));

    assert_eq!(event.get_str("resource.type"), Some("AWS::SQS::Queue"));
    assert_eq!(
        event.get_str("resource.id"),
        Some("https://sqs.us-east-2.amazonaws.com/123456789012/securityhubfinding")
    );
    assert_eq!(event.get_str("cloud.service.name"), Some("AWS::SQS::Queue"));
    assert_eq!(event.get_str("cloud.region"), Some("us-east-2"));
}

/// The seeded empty maps are NOT written: a queue finding has no user, group,
/// host or orchestrator, and Elasticsearch emits none of them.
#[test]
fn a_resource_with_no_entity_leaves_those_namespaces_absent() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [{
            "region": "global", "type": "AWS::IAM::Policy", "uid": "AJKSDBVKJJKVBJDSVBWKURGBFWK",
        }] } }
    }));

    for absent in [
        "user",
        "group",
        "host",
        "host.ip",
        "orchestrator",
        "cloud.instance",
    ] {
        assert!(!event.has(absent), "{absent} must not be written");
    }
    assert_eq!(event.get_str("cloud.region"), Some("global"));
}

/// An EC2 instance carries the host, machine and instance halves at once, and
/// both address families land in one `host.ip`.
#[test]
fn an_ec2_instance_writes_every_half_its_details_carry() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [{
            "region": "us-east-2",
            "type": "AWS::EC2::Instance",
            "uid": "i-0abcdef012345678b",
            "tags": [
                { "name": "aws:eks:cluster-name", "value": "demo_prod" },
                { "name": "Name", "value": "worker" },
            ],
            "data": { "awsEc2InstanceDetails": {
                "type": "t3.medium",
                "ipV4Addresses": ["10.90.1.245", "10.90.1.45"],
                "ipV6Addresses": ["fd00::1"],
            } },
        }] } }
    }));

    assert_eq!(event.get_str("host.id"), Some("i-0abcdef012345678b"));
    assert_eq!(event.get_str("host.name"), Some("i-0abcdef012345678b"));
    assert_eq!(event.get_str("host.type"), Some("t3.medium"));
    assert_eq!(event.get_str("cloud.machine.type"), Some("t3.medium"));
    assert_eq!(
        event.get_str("cloud.instance.id"),
        Some("i-0abcdef012345678b")
    );
    assert_eq!(
        event.get_str("orchestrator.cluster.name"),
        Some("demo_prod")
    );
    assert_eq!(event.get_str("resource.name"), Some("worker"));
    assert_eq!(
        event.get("host.ip"),
        Some(&json!(["10.90.1.245", "10.90.1.45", "fd00::1"]))
    );
}

/// The tag list becomes the map the vendor indexes, on every resource.
#[test]
fn the_tag_list_is_folded_into_a_map() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [{
            "type": "AWS::EC2::Instance",
            "uid": "i-1",
            "tags": [
                { "name": "aws:eks:cluster-name", "value": "demo_prod" },
                { "name": "eks:nodegroup-name", "value": "demo_prod_linux" },
            ],
        }] } }
    }));

    assert_eq!(
        event.get("aws_securityhub.finding.resources"),
        Some(&json!([{
            "type": "AWS::EC2::Instance",
            "uid": "i-1",
            "tags": {
                "aws:eks:cluster-name": "demo_prod",
                "eks:nodegroup-name": "demo_prod_linux",
            },
        }]))
    );
}

/// An IAM user's uid is its ECS user id, and a group's its group id.
#[test]
fn an_iam_resource_names_the_principal_its_type_says() {
    let user = run(json!({
        "aws_securityhub": { "finding": { "resources": [
            { "type": "AWS::IAM::User", "uid": "AIDA1" }
        ] } }
    }));
    assert_eq!(user.get_str("user.id"), Some("AIDA1"));
    assert!(!user.has("group"));

    let group = run(json!({
        "aws_securityhub": { "finding": { "resources": [
            { "type": "AWS::IAM::Group", "uid": "AGPA1" }
        ] } }
    }));
    assert_eq!(group.get_str("group.id"), Some("AGPA1"));
    assert!(!group.has("user"));
}

/// An EKS cluster names the orchestrator it is.
#[test]
fn an_eks_cluster_names_the_orchestrator() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [
            { "type": "AWS::EKS::Cluster", "uid": "arn:aws:eks:::cluster/demo" }
        ] } }
    }));

    assert_eq!(
        event.get_str("orchestrator.cluster.id"),
        Some("arn:aws:eks:::cluster/demo")
    );
    assert_eq!(event.get_str("orchestrator.type"), Some("kubernetes"));
}

/// Several resources take the one its own labels call primary.
#[test]
fn several_resources_take_the_one_labelled_primary() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [
            { "type": "AWS::SQS::Queue", "uid": "queue" },
            { "type": "AWS::IAM::User", "uid": "AIDA1", "labels": ["primary_resource"] },
        ] } }
    }));

    assert_eq!(event.get_str("resource.id"), Some("AIDA1"));
    assert_eq!(event.get_str("user.id"), Some("AIDA1"));
}

/// Several resources and none labelled primary writes NO entity: the script
/// throws there, and an entity taken off an arbitrary resource would be one
/// finding wearing another's identity.
#[test]
fn several_resources_with_no_primary_write_no_entity() {
    let event = run(json!({
        "aws_securityhub": { "finding": { "resources": [
            { "type": "AWS::SQS::Queue", "uid": "queue", "tags": [{ "name": "Env", "value": "dev" }] },
            { "type": "AWS::IAM::User", "uid": "AIDA1" },
        ] } }
    }));

    assert!(!event.has("resource.id"));
    assert!(!event.has("cloud.service.name"));
    // The fold still runs, because the script reaches it either way.
    assert_eq!(
        event.get("aws_securityhub.finding.resources"),
        Some(&json!([
            { "type": "AWS::SQS::Queue", "uid": "queue", "tags": { "Env": "dev" } },
            { "type": "AWS::IAM::User", "uid": "AIDA1" },
        ]))
    );
}

/// An absent list is the call site's own guard doing its job.
#[test]
fn an_absent_list_writes_nothing() {
    let event = run(json!({ "aws_securityhub": { "finding": {} } }));
    assert_eq!(
        event.as_value(),
        &json!({ "aws_securityhub": { "finding": {} } })
    );
}

/// Verbatim from the generated call site, in the escaped one-line form.
const OCSF_REMEDIATION: &str = r#"ctx.rule = ctx.rule ?: [:];\n\ndef remediation = ctx.aws_securityhub?.finding?.remediation;\nif (remediation == null && ctx.aws_securityhub?.finding?.vulnerabilities instanceof List && ctx.aws_securityhub.finding.vulnerabilities.size() > 0) {\n  remediation = ctx.aws_securityhub.finding.vulnerabilities[0].remediation;\n}\n\ndef desc = remediation?.desc;\nif (remediation?.references instanceof List) {\n  def separator = String.valueOf((char)10);\n  for (def ref: remediation.references) {\n    desc += separator + ref;\n  }\n}\n\nctx.rule.remediation = desc;\nctx.rule.reference = remediation?.references;"#;

/// Run the remediation script through its pattern.
fn run_remediation(document: serde_json::Value) -> Event {
    let pattern: OcsfRemediation =
        parse_ocsf_remediation(&normalise(OCSF_REMEDIATION)).expect("the script is recognised");
    let mut event = Event::new(document);
    assert!(run_ocsf_remediation(&mut event, &pattern));
    event
}

/// The finding root is read off the script rather than named here.
#[test]
fn the_finding_root_is_read_off_the_remediation_script() {
    let pattern =
        parse_ocsf_remediation(&normalise(OCSF_REMEDIATION)).expect("the script is recognised");
    assert_eq!(pattern.finding(), "aws_securityhub.finding");
}

/// The description carries its references on their own lines, and the list is
/// written whole beside it.
///
/// The values are the ones `testdata/compat/aws_securityhub/finding/
/// test-findings/expected.ndjson` carries for its first event.
#[test]
fn a_remediation_joins_its_references_onto_the_description() {
    let event = run_remediation(json!({
        "aws_securityhub": { "finding": { "remediation": {
            "desc": "For information on how to correct this issue, consult the AWS Security Hub controls documentation.",
            "references": ["https://docs.aws.amazon.com/console/securityhub/SQS.3/remediation"],
        } } }
    }));

    assert_eq!(
        event.get_str("rule.remediation"),
        Some(
            "For information on how to correct this issue, consult the AWS Security Hub controls documentation.\nhttps://docs.aws.amazon.com/console/securityhub/SQS.3/remediation"
        )
    );
    assert_eq!(
        event.get("rule.reference"),
        Some(&json!([
            "https://docs.aws.amazon.com/console/securityhub/SQS.3/remediation"
        ]))
    );
}

/// A finding with no remediation of its own takes the one its first
/// vulnerability carries.
#[test]
fn a_finding_falls_back_to_its_first_vulnerabilitys_remediation() {
    let event = run_remediation(json!({
        "aws_securityhub": { "finding": { "vulnerabilities": [
            { "remediation": { "desc": "upgrade", "references": ["https://example.test/cve"] } }
        ] } }
    }));

    assert_eq!(
        event.get_str("rule.remediation"),
        Some("upgrade\nhttps://example.test/cve")
    );
}

/// No remediation anywhere writes neither field, which is what Elasticsearch
/// indexes for the null the script assigns.
#[test]
fn a_finding_with_no_remediation_writes_neither_field() {
    let event = run_remediation(json!({ "aws_securityhub": { "finding": { "severity": "Low" } } }));
    assert!(!event.has("rule.remediation"));
    assert!(!event.has("rule.reference"));
}

/// A description with no references is written alone, unjoined.
#[test]
fn a_remediation_with_no_references_writes_the_description_alone() {
    let event = run_remediation(json!({
        "aws_securityhub": { "finding": { "remediation": { "desc": "consult the docs" } } }
    }));

    assert_eq!(event.get_str("rule.remediation"), Some("consult the docs"));
    assert!(!event.has("rule.reference"));
}

/// References under no description render Java's own `null`, which is the value
/// Elasticsearch writes rather than one this runner may improve on.
#[test]
fn references_with_no_description_render_the_word_java_renders() {
    let event = run_remediation(json!({
        "aws_securityhub": { "finding": { "remediation": {
            "references": ["https://example.test/a"]
        } } }
    }));

    assert_eq!(
        event.get_str("rule.remediation"),
        Some("null\nhttps://example.test/a")
    );
}

/// Verbatim from the generated call site, in the escaped one-line form.
const OCSF_VULNERABILITY: &str = r#"def vuln = ctx.aws_securityhub.finding.vulnerabilities[0];\n\n// Define fields to be extracted. \nctx.package = ctx.package ?: [:];\nctx.package.name = ctx.package.name ?: [];\nctx.package.version = ctx.package.version ?: [];\nctx.package.fixed_version = ctx.package.fixed_version ?: [];\nctx.vulnerability = ctx.vulnerability ?: [:];\nctx.vulnerability.scanner = ctx.vulnerability.scanner ?: [:];\nctx.vulnerability.score = ctx.vulnerability.score ?: [:];\n\nif (vuln.affected_packages instanceof List) {\n  for (def pkg: vuln.affected_packages) {\n    if (pkg.name != null) {\n      ctx.package.name.add(pkg.name);\n    }\n    if (pkg.version != null) {\n      ctx.package.version.add(pkg.version);\n    }\n    if (pkg.fixed_in_version != null) {\n      ctx.package.fixed_version.add(pkg.fixed_in_version);\n    }\n  }\n}\n\nctx.vulnerability.description = vuln.cve?.desc;\nctx.vulnerability.id = vuln.cve?.uid;\nctx.vulnerability.reference = vuln.references;\nctx.vulnerability.published_date = vuln.cve?.created_time_dt;\nif (ctx.vulnerability.published_date == null) {\n  ctx.vulnerability.published_date = vuln.cve?.created_time;\n}\n\nif (vuln.cve?.uid != null) {\n  String[] tokenList = vuln.cve.uid.splitOnToken(\"-\");\n  ctx.vulnerability.enumeration = tokenList[0];\n  if (tokenList[0] == 'CVE') {\n    ctx.vulnerability.cve = vuln.cve.uid;\n  }\n}\n\nif (vuln.cve?.cvss instanceof List && vuln.cve.cvss.size() > 0) {\n  def cvss = vuln.cve.cvss[0];\n  ctx.vulnerability.score.base = cvss.base_score;\n  ctx.vulnerability.score.version = cvss.version;\n  ctx.vulnerability.classification = 'CVSS';\n}\n\nif (ctx.aws_securityhub.finding.finding_info?.title != null) {\n  ctx.vulnerability.title = ctx.aws_securityhub.finding.finding_info.title;\n}\nif (ctx.aws_securityhub.finding.metadata?.product?.name != null) {\n  ctx.vulnerability.scanner.vendor = ctx.aws_securityhub.finding.metadata.product.name;\n}"#;

/// Run the vulnerability script through its pattern.
fn run_vulnerability(document: serde_json::Value) -> Event {
    let pattern: OcsfVulnerability =
        parse_ocsf_vulnerability(&normalise(OCSF_VULNERABILITY)).expect("the script is recognised");
    let mut event = Event::new(document);
    assert!(run_ocsf_vulnerability(&mut event, &pattern));
    event
}

/// The finding root is read off the script rather than named here.
#[test]
fn the_finding_root_is_read_off_the_vulnerability_script() {
    let pattern =
        parse_ocsf_vulnerability(&normalise(OCSF_VULNERABILITY)).expect("the script is recognised");
    assert_eq!(pattern.finding(), "aws_securityhub.finding");
}

/// Every half the first entry carries, with the three package lists in the
/// order the packages are named.
///
/// The values are the ones `testdata/compat/aws_securityhub/finding/
/// test-findings/expected.ndjson` carries for its fifth event.
#[test]
fn a_vulnerability_writes_its_packages_and_its_cve() {
    let event = run_vulnerability(json!({
        "aws_securityhub": { "finding": {
            "finding_info": { "title": "CVE-2023-44487 - golang.org/x/net, google.golang.org/grpc" },
            "metadata": { "product": { "name": "Inspector" } },
            "vulnerabilities": [{
                "affected_packages": [
                    { "name": "golang.org/x/net", "version": "v0.1.0", "fixed_in_version": "0.17.0" },
                    { "name": "google.golang.org/grpc", "version": "v1.31.0", "fixed_in_version": "1.58.3" },
                ],
                "references": ["https://example.test/a"],
                "cve": {
                    "uid": "CVE-2023-44487",
                    "desc": "The HTTP/2 protocol allows a denial of service.",
                    "created_time_dt": "2023-10-10T14:15:10.000Z",
                    "cvss": [{ "base_score": 7.5, "version": "3.1" }],
                },
            }],
        } }
    }));

    assert_eq!(
        event.get("package.name"),
        Some(&json!(["golang.org/x/net", "google.golang.org/grpc"]))
    );
    assert_eq!(
        event.get("package.version"),
        Some(&json!(["v0.1.0", "v1.31.0"]))
    );
    assert_eq!(
        event.get("package.fixed_version"),
        Some(&json!(["0.17.0", "1.58.3"]))
    );
    assert_eq!(event.get_str("vulnerability.id"), Some("CVE-2023-44487"));
    assert_eq!(event.get_str("vulnerability.cve"), Some("CVE-2023-44487"));
    assert_eq!(event.get_str("vulnerability.enumeration"), Some("CVE"));
    assert_eq!(event.get_f64("vulnerability.score.base"), Some(7.5));
    assert_eq!(event.get_str("vulnerability.score.version"), Some("3.1"));
    assert_eq!(event.get_str("vulnerability.classification"), Some("CVSS"));
    assert_eq!(
        event.get_str("vulnerability.published_date"),
        Some("2023-10-10T14:15:10.000Z")
    );
    assert_eq!(
        event.get_str("vulnerability.scanner.vendor"),
        Some("Inspector")
    );
}

/// An enumeration other than CVE names itself and writes no `vulnerability.cve`.
#[test]
fn a_non_cve_enumeration_writes_no_cve_field() {
    let event = run_vulnerability(json!({
        "aws_securityhub": { "finding": { "vulnerabilities": [
            { "cve": { "uid": "GHSA-1234-5678" } }
        ] } }
    }));

    assert_eq!(event.get_str("vulnerability.enumeration"), Some("GHSA"));
    assert!(!event.has("vulnerability.cve"));
}

/// The undated form falls back to the epoch member the vendor also ships.
#[test]
fn a_vulnerability_with_no_dated_creation_takes_the_epoch_one() {
    let event = run_vulnerability(json!({
        "aws_securityhub": { "finding": { "vulnerabilities": [
            { "cve": { "uid": "CVE-1", "created_time": 1_758_273_439_594_i64 } }
        ] } }
    }));

    assert_eq!(
        event.get_i64("vulnerability.published_date"),
        Some(1_758_273_439_594)
    );
}

/// The seeded empty lists are not written where the entry names no packages.
#[test]
fn a_vulnerability_with_no_packages_leaves_the_lists_absent() {
    let event = run_vulnerability(json!({
        "aws_securityhub": { "finding": { "vulnerabilities": [{ "cve": { "uid": "CVE-1" } }] } }
    }));

    for absent in [
        "package",
        "package.name",
        "package.version",
        "package.fixed_version",
    ] {
        assert!(!event.has(absent), "{absent} must not be written");
    }
}

/// Run the severity script through its pattern.
fn run_severity(document: serde_json::Value) -> Event {
    let pattern: OcsfSeverity =
        parse_ocsf_severity(&normalise(OCSF_SEVERITY)).expect("the script is recognised");
    let mut event = Event::new(document);
    assert!(run_ocsf_severity(&mut event, &pattern));
    event
}

/// Verbatim from the generated call site, in the escaped one-line form.
const OCSF_SEVERITY: &str = r#"String severity = ctx.aws_securityhub.finding.severity_id;\nString vulnerability_severity;\nif (severity == \"1\" || severity == \"2\") { // Informational and Low\n  ctx.event.severity = 21;\n  vulnerability_severity = 'Low';\n} else if (severity == \"3\") { // Medium\n  ctx.event.severity = 47;\n  vulnerability_severity = 'Medium';\n} else if (severity == \"4\") { // High\n  ctx.event.severity = 73;\n  vulnerability_severity = 'High';\n} else if (severity == \"5\" || severity == \"6\") { // Critical and Fatal\n  ctx.event.severity = 99;\n  vulnerability_severity = 'Critical';\n}\nctx.vulnerability = ctx.vulnerability ?: [:];\nif (ctx.aws_securityhub?.finding?.vulnerabilities instanceof List && ctx.aws_securityhub.finding.vulnerabilities.size() > 0) {\n  ctx.vulnerability.severity = vulnerability_severity;\n}"#;

/// Every id bands onto the ECS number the vendor names.
#[test]
fn every_severity_id_takes_its_band() {
    for (id, score) in [
        ("1", 21),
        ("2", 21),
        ("3", 47),
        ("4", 73),
        ("5", 99),
        ("6", 99),
    ] {
        let event = run_severity(json!({
            "aws_securityhub": { "finding": { "severity_id": id } }
        }));
        assert_eq!(event.get_i64("event.severity"), Some(score), "id {id}");
    }
}

/// A finding carrying a vulnerability takes the band's WORD as well, which is
/// the half the generic equality ladder cannot reach.
#[test]
fn a_finding_with_a_vulnerability_also_takes_the_band_word() {
    let event = run_severity(json!({
        "aws_securityhub": { "finding": {
            "severity_id": "4",
            "vulnerabilities": [{ "cve": { "uid": "CVE-1" } }],
        } }
    }));

    assert_eq!(event.get_i64("event.severity"), Some(73));
    assert_eq!(event.get_str("vulnerability.severity"), Some("High"));
}

/// A finding with no vulnerabilities takes the number alone.
#[test]
fn a_finding_with_no_vulnerability_takes_the_number_alone() {
    let event = run_severity(json!({
        "aws_securityhub": { "finding": { "severity_id": "1" } }
    }));

    assert_eq!(event.get_i64("event.severity"), Some(21));
    assert!(!event.has("vulnerability.severity"));
}

/// An id outside the six writes nothing at all.
#[test]
fn an_unnamed_severity_id_writes_neither_half() {
    let event = run_severity(json!({
        "aws_securityhub": { "finding": {
            "severity_id": "0",
            "vulnerabilities": [{ "cve": { "uid": "CVE-1" } }],
        } }
    }));

    assert!(!event.has("event.severity"));
    assert!(!event.has("vulnerability.severity"));
}
