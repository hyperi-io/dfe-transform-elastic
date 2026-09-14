// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `qualys_vmdr`'s asset pass: the cloud provider's own instance metadata
//! collated onto ECS `cloud.*`.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// Where each provider's attribute list sits.
const EC2: &str = "qualys_vmdr.asset_host_detection.metadata.ec2.attribute";
const GOOGLE: &str = "qualys_vmdr.asset_host_detection.metadata.google.attribute";
const AZURE: &str = "qualys_vmdr.asset_host_detection.metadata.azure.attribute";

/// Every list the script collates into, in one place so a provider arm names
/// the target rather than carrying its own variable.
#[derive(Default)]
struct Collated {
    account_ids: Vec<Value>,
    project_ids: Vec<Value>,
    project_names: Vec<Value>,
    regions: Vec<Value>,
    machine_types: Vec<Value>,
    zones: Vec<Value>,
}

/// `qualys_vmdr/asset_host_detection`, processor `cloud_provider_attributes`:
/// `cloud.account.id`, `cloud.project.id`, `cloud.project.name`,
/// `cloud.region`, `cloud.machine.type` and `cloud.availability_zone`.
///
/// Each provider names its instance metadata differently and the script reads
/// all three lists, so an asset seen by more than one provider collates into
/// the same ECS fields. Every value stays a LIST, which is what the vendor's
/// own expectations carry.
fn cloud_provider_attributes(event: &mut Event, _params: &Value) {
    let mut collated = Collated::default();
    collate_ec2(event, &mut collated);
    collate_google(event, &mut collated);
    collate_azure(event, &mut collated);

    if !collated.account_ids.is_empty() {
        let _ = event.set("cloud.account.id", Value::Array(collated.account_ids));
    }
    if !collated.project_ids.is_empty() {
        let _ = event.set(
            "cloud.project.id",
            Value::Array(collated.project_ids.clone()),
        );
        if !event.has_value("cloud.account.id") {
            let _ = event.set("cloud.account.id", Value::Array(collated.project_ids));
        }
    }
    if !collated.project_names.is_empty() {
        let _ = event.set(
            "cloud.project.name",
            Value::Array(collated.project_names.clone()),
        );
        if !event.has_value("cloud.account.name") {
            let _ = event.set("cloud.account.name", Value::Array(collated.project_names));
        }
    }
    if !collated.regions.is_empty() {
        let _ = event.set("cloud.region", Value::Array(collated.regions));
    }
    if !collated.machine_types.is_empty() {
        let _ = event.set("cloud.machine.type", Value::Array(collated.machine_types));
    }
    if !collated.zones.is_empty() {
        let _ = event.set("cloud.availability_zone", Value::Array(collated.zones));
    }
}

/// The EC2 instance-identity document, whose attribute names are the metadata
/// service's own URL paths.
fn collate_ec2(event: &Event, collated: &mut Collated) {
    for (name, value) in attributes(event, EC2) {
        match name.as_str() {
            "latest/dynamic/instance-identity/document/accountId" => {
                collated.account_ids.push(value);
            }
            "latest/dynamic/instance-identity/document/instanceType" => {
                collated.machine_types.push(value);
            }
            "latest/dynamic/instance-identity/document/region" => collated.regions.push(value),
            "latest/dynamic/instance-identity/document/availabilityZone" => {
                collated.zones.push(value);
            }
            _ => {}
        }
    }
}

/// Google's instance metadata, which names both a project id and its number.
fn collate_google(event: &Event, collated: &mut Collated) {
    for (name, value) in attributes(event, GOOGLE) {
        match name.as_str() {
            "projectId" => collated.project_names.push(value),
            "projectIdNo" => collated.project_ids.push(value),
            "machineType" => collated.machine_types.push(value),
            "location" => collated.regions.push(value),
            "zone" => collated.zones.push(value),
            _ => {}
        }
    }
}

/// Azure's instance metadata, which carries a subscription rather than a
/// project.
fn collate_azure(event: &Event, collated: &mut Collated) {
    for (name, value) in attributes(event, AZURE) {
        match name.as_str() {
            "subscriptionId" => collated.project_ids.push(value),
            "location" => collated.regions.push(value),
            _ => {}
        }
    }
}

/// One provider's `{name, value}` attributes, where it sent a list of them.
///
/// The vendor guards each attribute with `value == null && value == ""`, which
/// no value satisfies, so nothing is filtered out here.
fn attributes(event: &Event, path: &str) -> Vec<(String, Value)> {
    let Some(listed) = event.get_array(path) else {
        return Vec::new();
    };
    listed
        .iter()
        .filter_map(|attribute| {
            let name = attribute.get("name")?.as_str()?.to_owned();
            let value = attribute.get("value").cloned().unwrap_or(Value::Null);
            Some((name, value))
        })
        .collect()
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "5ae52753a2f671408ff9df1e1d85adc90e313dab3a92dbdc7b280d13ba2dd1db",
    source: "qualys_vmdr",
    name: "cloud_provider_attributes",
    run: cloud_provider_attributes,
}];
