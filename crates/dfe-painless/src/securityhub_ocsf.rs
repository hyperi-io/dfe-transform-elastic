// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! securityhub's ECS entity fields, read off the resource an OCSF finding names.
//!
//! ```painless
//! def res;
//! if (ctx.aws_securityhub.finding.resources.size() == 1) {
//!   res = ctx.aws_securityhub.finding.resources[0];
//! } else {
//!   for (def resource: ctx.aws_securityhub.finding.resources) {
//!     if (resource.labels?.contains('primary_resource') == true) { res = resource; break; }
//!   }
//! }
//! ctx.resource.type = res.type;
//! ctx.resource.id = res.uid;
//! ...
//! ctx.cloud.service.name = res.type;
//! ```
//!
//! The `finding` data stream carries AWS's OCSF form, where the older
//! `securityhub_findings` pair carries ASFF. [`crate::common`] already reads the
//! ASFF spelling, and it cannot serve this one: every member is renamed
//! (`res.Type` to `res.type`, `res.Id` to `res.uid`) and the per-type details
//! moved out of `res.Details[res.Type]` into `res.data.awsEc2InstanceDetails`.
//! So this is a second pattern rather than a widened reader.
//!
//! Unclaimed it costs `resource.id`, `resource.type` and `cloud.service.name` on
//! every finding, and the tag list stays a list where Elasticsearch indexes a
//! map.
//!
//! **The empty maps the script seeds are not written.** `ctx.user = ctx.user ?:
//! [:]` leaves an empty object on a finding whose resource is not an IAM user,
//! and Elasticsearch emits no such field -- writing them would be an extra per
//! namespace per event.

use serde_json::{Map, Value};

use dfe_core::Event;

/// The resource list a finding's ECS entity fields are read from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcsfResource {
    /// The list walked, as a ctx path.
    source: String,
}

impl OcsfResource {
    /// The list this pattern reads, for a generator emitting the runner call.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Read the resource list the script walks, or decline.
///
/// The gate is the OCSF entity writes themselves, not the vendor's comment or
/// the word `resources`: the ASFF sibling opens on the same comment and walks a
/// list of the same name, and claiming it here would write an entity off
/// members it does not carry.
#[must_use]
pub fn parse_ocsf_resource(script: &str) -> Option<OcsfResource> {
    if !script.contains("ctx.resource.type = res.type;")
        || !script.contains("ctx.resource.id = res.uid;")
        || !script.contains("ctx.cloud.service.name = res.type;")
        || !script.contains("tags[tag.name] = tag.value;")
    {
        return None;
    }

    let head = script.split_once(".size() == 1")?.0;
    let at = head.rfind("ctx.")? + "ctx.".len();
    let source = crate::params::clean_path(head[at..].trim());
    (!source.is_empty()).then_some(OcsfResource { source })
}

/// The primary resource's members, lifted before the list is rewritten.
#[derive(Default)]
struct Primary {
    kind: Option<String>,
    uid: Option<String>,
    region: Option<String>,
    name: Option<String>,
    /// The `Name` tag's value, which names the resource.
    tag_name: Option<String>,
    /// The `aws:eks:cluster-name` tag's value.
    cluster_name: Option<String>,
    /// `res.data.awsEc2InstanceDetails.type`, the instance's machine type.
    machine_type: Option<String>,
    /// Every v4 then v6 address the instance details carry, strings only.
    ips: Vec<Value>,
}

/// One string member, owned, or `None` where it is absent or another type.
fn text(value: Option<&Value>) -> Option<String> {
    value.and_then(Value::as_str).map(str::to_owned)
}

/// Whether a resource's own labels name it the primary one.
fn is_primary(resource: &Value) -> bool {
    resource
        .get("labels")
        .and_then(Value::as_array)
        .is_some_and(|labels| {
            labels
                .iter()
                .any(|label| label.as_str() == Some("primary_resource"))
        })
}

/// Lift the members the entity writes read, in one pass over the resource.
///
/// The script walks the tag list twice; one pass answers both because the two
/// loops differ only in which tag they stop at -- `Name` breaks on the first,
/// the cluster name has no break and so takes the last.
fn lift(resource: &Value) -> Primary {
    let mut lifted = Primary::default();
    let Some(record) = resource.as_object() else {
        return lifted;
    };

    lifted.kind = text(record.get("type"));
    lifted.uid = text(record.get("uid"));
    lifted.region = text(record.get("region"));
    lifted.name = text(record.get("name"));

    if let Some(tags) = record.get("tags").and_then(Value::as_array) {
        for tag in tags {
            let Some(name) = tag.get("name").and_then(Value::as_str) else {
                continue;
            };
            if name == "Name" && lifted.tag_name.is_none() {
                lifted.tag_name = text(tag.get("value"));
            }
            if name == "aws:eks:cluster-name" {
                lifted.cluster_name = text(tag.get("value"));
            }
        }
    }

    if let Some(details) = record
        .get("data")
        .and_then(|data| data.get("awsEc2InstanceDetails"))
    {
        lifted.machine_type = text(details.get("type"));
        for key in ["ipV4Addresses", "ipV6Addresses"] {
            let Some(addresses) = details.get(key).and_then(Value::as_array) else {
                continue;
            };
            lifted.ips.extend(
                addresses
                    .iter()
                    .filter(|address| address.is_string())
                    .cloned(),
            );
        }
    }

    lifted
}

/// Write the ECS entity fields the chosen resource carries.
///
/// A member the resource does not carry writes nothing rather than null: the
/// script assigns Painless's null there, which Elasticsearch does not index,
/// and `cloud.region` has a later `set` behind a null test that a written null
/// would still satisfy.
fn write_entity(event: &mut Event, lifted: &Primary) {
    let ec2 = lifted.kind.as_deref() == Some("AWS::EC2::Instance");

    if let Some(kind) = &lifted.kind {
        let _ = event.set("resource.type", kind.clone());
        let _ = event.set("cloud.service.name", kind.clone());
    }
    if let Some(uid) = &lifted.uid {
        let _ = event.set("resource.id", uid.clone());
        match lifted.kind.as_deref() {
            Some("AWS::IAM::User") => {
                let _ = event.set("user.id", uid.clone());
            }
            Some("AWS::IAM::Group") => {
                let _ = event.set("group.id", uid.clone());
            }
            Some("AWS::EKS::Cluster") => {
                let _ = event.set("orchestrator.cluster.id", uid.clone());
                let _ = event.set("orchestrator.type", "kubernetes");
            }
            _ => {}
        }
        if ec2 {
            let _ = event.set("host.id", uid.clone());
            let _ = event.set("host.name", uid.clone());
            let _ = event.set("cloud.instance.id", uid.clone());
        }
    }
    if let Some(name) = &lifted.tag_name {
        let _ = event.set("resource.name", name.clone());
    }
    if let Some(cluster) = &lifted.cluster_name {
        let _ = event.set("orchestrator.cluster.name", cluster.clone());
    }
    if let Some(region) = &lifted.region {
        let _ = event.set("cloud.region", region.clone());
    }
    if !ec2 {
        return;
    }
    if let Some(machine) = &lifted.machine_type {
        let _ = event.set("host.type", machine.clone());
        let _ = event.set("cloud.machine.type", machine.clone());
    }
    if let Some(name) = &lifted.name {
        let _ = event.set("cloud.instance.name", name.clone());
    }
    for address in &lifted.ips {
        let _ = event.append("host.ip", address.clone());
    }
}

/// Rebuild every resource's `tags` list into the map the vendor indexes.
fn fold_tags(resources: &mut [Value]) {
    for resource in resources {
        let Some(tags) = resource.get("tags").and_then(Value::as_array) else {
            continue;
        };
        let mut folded = Map::with_capacity(tags.len());
        for tag in tags {
            let (Some(name), Some(value)) =
                (tag.get("name").and_then(Value::as_str), tag.get("value"))
            else {
                continue;
            };
            folded.insert(name.to_owned(), value.clone());
        }
        if let Some(record) = resource.as_object_mut() {
            record.insert("tags".to_owned(), Value::Object(folded));
        }
    }
}

/// Extract the ECS entity fields, then fold every resource's tag list.
///
/// Where several resources are named and none is labelled primary the script
/// leaves `res` null and throws on the first read, which the vendor's
/// `on_failure` turns into an appended `error.message`. Writing no entity is
/// the closest this gets without putting one finding's identity on another's
/// resource; the tag fold still runs, because the script reaches it either way.
pub fn run_ocsf_resource(event: &mut Event, pattern: &OcsfResource) -> bool {
    let lifted = event.get_array(&pattern.source).and_then(|resources| {
        if resources.len() == 1 {
            resources.first()
        } else {
            resources.iter().find(|resource| is_primary(resource))
        }
        .map(lift)
    });

    if let Some(lifted) = lifted {
        write_entity(event, &lifted);
    }
    if let Some(Value::Array(resources)) = crate::params::pointer_mut(event, &pattern.source) {
        fold_tags(resources);
    }
    true
}

/// The remediation a finding carries, or the one its first vulnerability does.
///
/// ```painless
/// def remediation = ctx.aws_securityhub?.finding?.remediation;
/// if (remediation == null && ctx.aws_securityhub?.finding?.vulnerabilities instanceof List && ...) {
///   remediation = ctx.aws_securityhub.finding.vulnerabilities[0].remediation;
/// }
/// def desc = remediation?.desc;
/// if (remediation?.references instanceof List) {
///   def separator = String.valueOf((char)10);
///   for (def ref: remediation.references) { desc += separator + ref; }
/// }
/// ctx.rule.remediation = desc;
/// ctx.rule.reference = remediation?.references;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcsfRemediation {
    /// The finding root the two sources hang off, as a ctx path.
    finding: String,
}

impl OcsfRemediation {
    /// The finding root this pattern reads.
    #[must_use]
    pub fn finding(&self) -> &str {
        &self.finding
    }
}

/// Read the finding root the script walks, or decline.
#[must_use]
pub fn parse_ocsf_remediation(script: &str) -> Option<OcsfRemediation> {
    if !script.contains("ctx.rule.remediation = desc;")
        || !script.contains("ctx.rule.reference = remediation?.references;")
        || !script.contains("desc += separator + ref;")
    {
        return None;
    }

    let head = script.split_once("?.remediation;")?.0;
    let at = head.rfind("ctx.")? + "ctx.".len();
    let finding = crate::params::clean_path(head[at..].trim());
    (!finding.is_empty()).then_some(OcsfRemediation { finding })
}

/// Join the remediation's description and its references, and write both.
///
/// The join is Java's string concatenation, so an absent description under a
/// list of references renders the word `null` ahead of the first one -- that is
/// what Elasticsearch writes, and matching it is the parity. A reference that is
/// not a string renders as its JSON text, which no vendored finding carries.
pub fn run_ocsf_remediation(event: &mut Event, pattern: &OcsfRemediation) -> bool {
    let direct = format!("{}.remediation", pattern.finding);
    let fallback = format!("{}.vulnerabilities.0.remediation", pattern.finding);
    let source = if event.has_value(&direct) {
        direct
    } else {
        fallback
    };

    let Some(remediation) = event.get_object(&source).cloned() else {
        return true;
    };
    let references = remediation.get("references").and_then(Value::as_array);
    let desc = remediation.get("desc").and_then(Value::as_str);

    let joined = references.map(|references| {
        let mut out = desc.unwrap_or("null").to_owned();
        for reference in references {
            out.push('\n');
            match reference.as_str() {
                Some(text) => out.push_str(text),
                None => out.push_str(&reference.to_string()),
            }
        }
        out
    });

    match (joined, desc) {
        (Some(joined), _) => {
            let _ = event.set("rule.remediation", joined);
        }
        (None, Some(desc)) => {
            let _ = event.set("rule.remediation", desc.to_owned());
        }
        (None, None) => {}
    }
    if let Some(references) = references {
        let _ = event.set("rule.reference", Value::Array(references.clone()));
    }
    true
}

/// The ECS vulnerability and package fields off a finding's first entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcsfVulnerability {
    /// The finding root, as a ctx path.
    finding: String,
}

impl OcsfVulnerability {
    /// The finding root this pattern reads.
    #[must_use]
    pub fn finding(&self) -> &str {
        &self.finding
    }
}

/// Read the finding root the script walks, or decline.
#[must_use]
pub fn parse_ocsf_vulnerability(script: &str) -> Option<OcsfVulnerability> {
    if !script.contains("ctx.vulnerability.id = vuln.cve?.uid;")
        || !script.contains("ctx.package.fixed_version.add(pkg.fixed_in_version);")
        || !script.contains("ctx.vulnerability.classification = 'CVSS';")
    {
        return None;
    }

    let head = script.split_once(".vulnerabilities[0];")?.0;
    let at = head.rfind("ctx.")? + "ctx.".len();
    let finding = crate::params::clean_path(head[at..].trim());
    (!finding.is_empty()).then_some(OcsfVulnerability { finding })
}

/// Write the vulnerability and package halves the first entry carries.
///
/// The three package lists are built only where a member exists, so a finding
/// whose packages carry no version leaves `package.version` absent rather than
/// holding an empty list Elasticsearch does not index.
pub fn run_ocsf_vulnerability(event: &mut Event, pattern: &OcsfVulnerability) -> bool {
    let root = format!("{}.vulnerabilities.0", pattern.finding);
    let Some(vuln) = event.get(&root).cloned() else {
        return true;
    };

    if let Some(packages) = vuln.get("affected_packages").and_then(Value::as_array) {
        for (member, target) in [
            ("name", "package.name"),
            ("version", "package.version"),
            ("fixed_in_version", "package.fixed_version"),
        ] {
            for package in packages {
                if let Some(value) = package.get(member) {
                    let _ = event.append(target, value.clone());
                }
            }
        }
    }

    let cve = vuln.get("cve");
    for (value, target) in [
        (
            cve.and_then(|cve| cve.get("desc")),
            "vulnerability.description",
        ),
        (cve.and_then(|cve| cve.get("uid")), "vulnerability.id"),
        (vuln.get("references"), "vulnerability.reference"),
    ] {
        if let Some(value) = value {
            let _ = event.set(target, value.clone());
        }
    }

    if let Some(published) = cve
        .and_then(|cve| cve.get("created_time_dt"))
        .or_else(|| cve.and_then(|cve| cve.get("created_time")))
    {
        let _ = event.set("vulnerability.published_date", published.clone());
    }

    if let Some(uid) = cve.and_then(|cve| cve.get("uid")).and_then(Value::as_str) {
        let enumeration = uid.split('-').next().unwrap_or(uid);
        let _ = event.set("vulnerability.enumeration", enumeration.to_owned());
        if enumeration == "CVE" {
            let _ = event.set("vulnerability.cve", uid.to_owned());
        }
    }

    if let Some(cvss) = cve
        .and_then(|cve| cve.get("cvss"))
        .and_then(Value::as_array)
        .and_then(|entries| entries.first())
    {
        if let Some(base) = cvss.get("base_score") {
            let _ = event.set("vulnerability.score.base", base.clone());
        }
        if let Some(version) = cvss.get("version") {
            let _ = event.set("vulnerability.score.version", version.clone());
        }
        let _ = event.set("vulnerability.classification", "CVSS");
    }

    for (source, target) in [
        ("finding_info.title", "vulnerability.title"),
        ("metadata.product.name", "vulnerability.scanner.vendor"),
    ] {
        if let Some(value) = event.get(&format!("{}.{source}", pattern.finding)).cloned() {
            let _ = event.set(target, value);
        }
    }
    true
}

/// A finding's severity id banded onto ECS, and onto its vulnerability.
///
/// ```painless
/// String severity = ctx.aws_securityhub.finding.severity_id;
/// String vulnerability_severity;
/// if (severity == "1" || severity == "2") { ctx.event.severity = 21; vulnerability_severity = 'Low'; }
/// ...
/// if (ctx.aws_securityhub?.finding?.vulnerabilities instanceof List && ...) {
///   ctx.vulnerability.severity = vulnerability_severity;
/// }
/// ```
///
/// The generic equality ladder reads the `event.severity` half and stops there:
/// the band's WORD goes to a local the ladder has no notion of, and is written
/// out only where the finding carries a vulnerability. That second half is the
/// whole reason this is its own pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcsfSeverity {
    /// The finding root, as a ctx path.
    finding: String,
}

impl OcsfSeverity {
    /// The finding root this pattern reads.
    #[must_use]
    pub fn finding(&self) -> &str {
        &self.finding
    }
}

/// Read the finding root the severity bands hang off, or decline.
#[must_use]
pub fn parse_ocsf_severity(script: &str) -> Option<OcsfSeverity> {
    if !script.contains("String vulnerability_severity;")
        || !script.contains("ctx.vulnerability.severity = vulnerability_severity;")
        || !script.contains("ctx.event.severity = ")
    {
        return None;
    }

    let head = script.split_once(".severity_id;")?.0;
    let at = head.rfind("ctx.")? + "ctx.".len();
    let finding = crate::params::clean_path(head[at..].trim());
    (!finding.is_empty()).then_some(OcsfSeverity { finding })
}

/// Band the severity id, and name the vulnerability's severity with it.
pub fn run_ocsf_severity(event: &mut Event, pattern: &OcsfSeverity) -> bool {
    let Some(id) = event.get_as_string(&format!("{}.severity_id", pattern.finding)) else {
        return true;
    };
    let Some((score, word)) = (match id.as_str() {
        "1" | "2" => Some((21, "Low")),
        "3" => Some((47, "Medium")),
        "4" => Some((73, "High")),
        "5" | "6" => Some((99, "Critical")),
        _ => None,
    }) else {
        return true;
    };

    let _ = event.set("event.severity", score);
    let vulnerabilities = format!("{}.vulnerabilities", pattern.finding);
    if event
        .get_array(&vulnerabilities)
        .is_some_and(|list| !list.is_empty())
    {
        let _ = event.set("vulnerability.severity", word);
    }
    true
}

#[cfg(test)]
#[path = "securityhub_ocsf_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
