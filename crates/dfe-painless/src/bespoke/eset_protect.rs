// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `eset_protect`'s vulnerability lift: whichever of the three ONEOF
//! vulnerability objects a record carries, flattened onto
//! `eset_protect.device_vulnerability` under `snake_case` names.
//!
//! The same pass fills the ECS `vulnerability.*` fields and the `package.*`
//! the transform later keys its title on, so a record whose lift is skipped
//! loses its whole vendor namespace as well as those.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The subtree every lifted member lands under.
const DEVICE_VULNERABILITY: &str = "eset_protect.device_vulnerability";

/// The ECS severity each vendor level maps to.
const ECS_SEVERITY: [(&str, &str); 5] = [
    ("SEVERITY_LEVEL_HIGH", "high"),
    ("SEVERITY_LEVEL_MEDIUM", "medium"),
    ("SEVERITY_LEVEL_LOW", "low"),
    ("SEVERITY_LEVEL_INFORMATIONAL", "low"),
    ("SEVERITY_LEVEL_DIAGNOSTIC", "low"),
];

/// The members carried across whichever vulnerability object is present.
const COMMON: [(&str, &str); 7] = [
    ("cveNumber", "cve_number"),
    ("firstDetectTime", "first_detect_time"),
    ("lastDetectTime", "last_detect_time"),
    ("patchAvailable", "patch_available"),
    ("riskScore", "risk_score"),
    ("severity", "severity"),
    ("vulnerabilityId", "vulnerability_id"),
];

/// The three vendor keys, in the order the script tests them, with the type
/// each one names.
const ONEOF: [(&str, &str); 3] = [
    ("applicationVulnerability", "application"),
    ("osVulnerability", "os"),
    ("packageVulnerability", "package"),
];

/// `script_extract_vulnerability` in
/// `compat-eset_protect-device_vulnerability-default`: the vulnerability
/// object onto the vendor namespace, plus `vulnerability.*` and `package.*`.
fn extract_vulnerability(event: &mut Event, _params: &Value) {
    let Some((vuln_type, vuln)) = held_vulnerability(event) else {
        return;
    };

    let _ = event.set(&format!("{DEVICE_VULNERABILITY}.type"), vuln_type);
    for (from, to) in COMMON {
        if let Some(value) = vuln.get(from) {
            let _ = event.set(&format!("{DEVICE_VULNERABILITY}.{to}"), value.clone());
        }
    }

    match vuln_type {
        "application" => lift_application(event, &vuln),
        "os" => {
            if let Some(family) = vuln.get("osFamilyId") {
                let _ = event.set(
                    &format!("{DEVICE_VULNERABILITY}.os_family_id"),
                    family.clone(),
                );
            }
        }
        _ => lift_package(event, &vuln),
    }

    // Read back off the namespace rather than the vendor object, which is
    // what the script does -- the cast is of the value it just wrote.
    let cve = event.get_string(&format!("{DEVICE_VULNERABILITY}.cve_number"));
    let severity = event.get_string(&format!("{DEVICE_VULNERABILITY}.severity"));
    if let Some(cve) = cve {
        let _ = event.set("vulnerability.id", cve);
        let _ = event.set("vulnerability.enumeration", "CVE");
    }
    if let Some(severity) = severity
        && let Some((_, ecs)) = ECS_SEVERITY.iter().find(|(level, _)| *level == severity)
    {
        let _ = event.set("vulnerability.severity", *ecs);
    }
    let _ = event.set("vulnerability.scanner.vendor", "ESET");

    for (key, _) in ONEOF {
        event.remove(&format!("json.{key}"));
    }
}

/// The vulnerability object the record carries, and the type naming it.
///
/// The three are a ONEOF, and the script takes the FIRST that is a map.
fn held_vulnerability(event: &Event) -> Option<(&'static str, Map<String, Value>)> {
    ONEOF.into_iter().find_map(|(key, vuln_type)| {
        event
            .get_object(&format!("json.{key}"))
            .map(|held| (vuln_type, held.clone()))
    })
}

/// The application arm: the application members under `application`, and the
/// ECS package fields the title is built from.
fn lift_application(event: &mut Event, vuln: &Map<String, Value>) {
    let Some(src) = vuln.get("application").and_then(Value::as_object) else {
        return;
    };

    let mut app = Map::new();
    if let Some(uuid) = src.get("uuid") {
        app.insert("uuid".to_owned(), uuid.clone());
    }
    if let Some(name) = src.get("displayName") {
        app.insert("display_name".to_owned(), name.clone());
        let _ = event.set("package.name", name.clone());
    }
    if let Some(developer) = src.get("developerDisplayName") {
        app.insert("developer_display_name".to_owned(), developer.clone());
        let _ = event.set("package.vendor", developer.clone());
    }
    if let Some(version) = src.get("version").and_then(Value::as_object) {
        app.insert("version".to_owned(), Value::Object(version.clone()));
        if let Some(name) = version.get("name")
            && !name.is_null()
            && name.as_str() != Some("")
        {
            let _ = event.set("package.version", name.clone());
        }
    }

    let _ = event.set(
        &format!("{DEVICE_VULNERABILITY}.application"),
        Value::Object(app),
    );
}

/// The package arm, which names the ECS package from the vendor's own names
/// rather than its display ones.
fn lift_package(event: &mut Event, vuln: &Map<String, Value>) {
    let Some(src) = vuln.get("package").and_then(Value::as_object) else {
        return;
    };

    let mut pkg = Map::new();
    if let Some(display) = src.get("displayName") {
        pkg.insert("display_name".to_owned(), display.clone());
    }
    if let Some(name) = src.get("name") {
        pkg.insert("name".to_owned(), name.clone());
        let _ = event.set("package.name", name.clone());
    }
    if let Some(manager) = src.get("packageManagerType") {
        pkg.insert("package_manager_type".to_owned(), manager.clone());
    }
    if let Some(version) = src.get("versionName") {
        pkg.insert("version_name".to_owned(), version.clone());
        let _ = event.set("package.version", version.clone());
    }

    let _ = event.set(
        &format!("{DEVICE_VULNERABILITY}.package"),
        Value::Object(pkg),
    );
}

/// Every `eset_protect` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "87be0ad55d312b2b2c23be38f80ac28dea5fa6a1e2efcdc1f76008ec218fd871",
    source: "eset_protect",
    name: "extract_vulnerability",
    run: extract_vulnerability,
}];
