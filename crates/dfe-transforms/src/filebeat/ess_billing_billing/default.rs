// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { !event.has_value("ess.billing") && !event.has_value("error.message") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.remove("ess.billing.quantities");

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            if let Some(date_str) = event.get_as_string("ess.billing.from") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ess.billing.from".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("ess.billing.deployment_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.from") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.sku") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.to") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("ess.billing.total_ecu") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(v) = event
                .get("ess.billing.organization_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if let Some(v) = event
                .get("ess.billing.deployment_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if let Some(v) = event.get("ess.billing.deployment_id").cloned() {
                event.set("cloud.instance.name", v)?;
            }

            if let Some(v) = event
                .get("ess.billing.deployment_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.name", v)?;
            }

            let _cond = {
                event.get_str("ess.billing.type") == Some("capacity")
                    && event.get_str("ess.billing.deployment_type") == Some("deployment")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("ess.billing.sku") {
                        // Grok pattern: (?:Cloud-Enterprise_)?(?P<cloud_provider>(?:aws|gcp|azure|global))\\.%{NOTSPACE:cloud.machine.type}_%{NOTSPACE:cloud.region}_%{NUMBER:ess.billing.ram_per_zone:int}_%{NUMBER:ess.billing.zone_count:int}
                        let _ = cached_grok_mapped!("(?:Cloud-Enterprise_)?(?P<cloud_provider>(?:aws|gcp|azure|global))\\.%{NOTSPACE:cloud.machine.type}_%{NOTSPACE:cloud.region}_%{NUMBER:ess.billing.ram_per_zone:int}_%{NUMBER:ess.billing.zone_count:int}", [("cloud_provider", "cloud.provider")]).extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("ess.billing.type") != Some("capacity")
                    && event.get_str("ess.billing.deployment_type") == Some("deployment")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("ess.billing.sku") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(".") else {
                                break 'dissect false;
                            };
                            captured.push(("cloud.provider", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(".") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("ess.billing.deployment_type") != Some("deployment") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("ess.billing.sku") {
                        // Grok pattern: %{DATA}\\.(?P<ess_billing_cloud_service_type>[^_]+)(?:_(?P<cloud_region>.+))?
                        let _ = cached_grok_mapped!("%{DATA}\\.(?P<ess_billing_cloud_service_type>[^_]+)(?:_(?P<cloud_region>.+))?", [("ess_billing_cloud_service_type", "ess.billing.cloud.service.type"), ("cloud_region", "cloud.region")]).extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("ess.billing.deployment_type") != Some("deployment") };
            if _cond {
                event.set("cloud.provider", json!("serverless"))?;
            }

            if event.has_value("ess.billing.ram_per_zone") {
                if let Some(val) = event.get("ess.billing.ram_per_zone") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "ess.billing.ram_per_zone".into(),
                            message,
                        }
                    })?;
                    event.set("ess.billing.ram_per_zone", converted)?;
                }
            }

            if event.has_value("ess.billing.zone_count") {
                if let Some(val) = event.get("ess.billing.zone_count") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "ess.billing.zone_count".into(),
                            message,
                        }
                    })?;
                    event.set("ess.billing.zone_count", converted)?;
                }
            }

            event.set("cloud.geo", cached_params!("{}").clone())?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: Map regionGeo = [\n  'gcp-us-central1': ['location': ['lon': -93.2650, 'lat': 44.9778], 'name': 'gcp-us-central1'],\n  'gcp-us-east4': ['location': ['lon': -80.1918, 'lat': 25.7617], 'name': 'gcp-us-east4'],\n  'gcp-us-west1': ['location': ['lon': -122.4194, 'lat': 37.7749], 'name': 'gcp-us-west1'],\n  'gcp-us-west2': ['location': ['lon': -118.2437, 'lat': 34.0522], 'name': 'gcp-us-west2'],\n  'gcp-us-east1': ['location': ['lon': -77.0369, 'lat': 38.9072], 'name': 'gcp-us-east1'],\n  'gcp-northamerica-northeast1': ['location': ['lon': -73.6148, 'lat': 45.5059], 'name': 'gcp-northamerica-northeast1'],\n  'gcp-southamerica-east1': ['location': ['lon': -46.6333, 'lat': -23.5505], 'name': 'gcp-southamerica-east1'],\n  'gcp-europe-west1': ['location': ['lon': -6.2603, 'lat': 53.3498], 'name': 'gcp-europe-west1'],\n  'gcp-europe-west2': ['location': ['lon': -0.1276, 'lat': 51.5074], 'name': 'gcp-europe-west2'],\n  'gcp-europe-west3': ['location': ['lon': 11.5820, 'lat': 48.1351], 'name': 'gcp-europe-west3'],\n  'gcp-europe-west4': ['location': ['lon': 19.0402, 'lat': 47.4979], 'name': 'gcp-europe-west4'],\n  'gcp-europe-north1': ['location': ['lon': 24.9458, 'lat': 60.1921], 'name': 'gcp-europe-north1'],\n  'gcp-asia-east1': ['location': ['lon': 121.5654, 'lat': 25.0330], 'name': 'gcp-asia-east1'],\n  'gcp-asia-northeast1': ['location': ['lon': 139.6917, 'lat': 35.6895], 'name': 'gcp-asia-northeast1'],\n  'gcp-asia-south1': ['location': ['lon': 72.8777, 'lat': 19.0760], 'name': 'gcp-asia-south1'],\n  'gcp-asia-southeast1': ['location': ['lon': 103.8198, 'lat': 1.3521], 'name': 'gcp-asia-southeast1'],\n  'gcp-australia-southeast1': ['location': ['lon': 151.2093, 'lat': -33.8688], 'name': 'gcp-australia-southeast1'],\n  'gcp-asia-northeast3': ['location': ['lon': 126.9780, 'lat': 37.5665], 'name': 'gcp-asia-northeast3'],\n  'gcp-asia-southeast2': ['location': ['lon': 106.8650, 'lat': -6.2088], 'name': 'gcp-asia-southeast2'],\n  'gcp-europe-west9': ['location': ['lon': 4.9041, 'lat': 52.3676], 'name': 'gcp-europe-west9'],\n  'gcp-me-west1': ['location': ['lon': 34.8516, 'lat': 31.0461], 'name': 'gcp-me-west1'],\n  'aws-us-east-2': ['location': ['lon': -82.9988, 'lat': 39.9612], 'name': 'aws-us-east-2'],\n  'aws-us-east-1': ['location': ['lon': -77.0369, 'lat': 39.9612], 'name': 'aws-us-east-2'],\n  'us-east-1': ['location': ['lon': -77.0369, 'lat': 38.9072], 'name': 'us-east-1'],\n  'eu-west-1': ['location': ['lon': -6.2603, 'lat': 53.3498], 'name': 'eu-west-1'],\n  'us-west-1': ['location': ['lon': -122.4194, 'lat': 37.7749], 'name': 'us-west-1'],\n  'us-west-2': ['location': ['lon': -122.6765, 'lat': 45.5235], 'name': 'us-west-2'],\n  'aws-ca-central-1': ['location': ['lon': -75.6972, 'lat': 45.4215], 'name': 'aws-ca-central-1'],\n  'sa-east-1': ['location': ['lon': -46.6333, 'lat': -23.5505], 'name': 'sa-east-1'],\n  'aws-eu-west-2': ['location': ['lon': -0.1276, 'lat': 51.5074], 'name': 'aws-eu-west-2'],\n  'aws-eu-west-3': ['location': ['lon': 2.3522, 'lat': 48.8566], 'name': 'aws-eu-west-3'],\n  'aws-eu-central-1': ['location': ['lon': 8.6821, 'lat': 50.1109], 'name': 'aws-eu-central-1'],\n  'aws-ap-south-1': ['location': ['lon': 72.8777, 'lat': 19.0760], 'name': 'aws-ap-south-1'],\n  'ap-northeast-1': ['location': ['lon': 139.6917, 'lat': 35.6895], 'name': 'ap-northeast-1'],\n  'aws-ap-northeast-2': ['location': ['lon': 126.9780, 'lat': 37.5665], 'name': 'aws-ap-northeast-2'],\n  'aws-ap-east-1': ['location': ['lon': 114.1095, 'lat': 22.3964], 'name': 'aws-ap-east-1'],\n  'ap-southeast-1': ['location': ['lon': 103.8198, 'lat': 1.3521], 'name': 'ap-southeast-1'],\n  'ap-southeast-2': ['location': ['lon': 151.2093, 'lat': -33.8688], 'name': 'ap-southeast-2'],\n  'aws-af-south-1': ['location': ['lon': 18.4241, 'lat': -33.9249], 'name': 'aws-af-south-1'],\n  'aws-me-south-1': ['location': ['lon': 55.2708, 'lat': 25.2048], 'name': 'aws-me-south-1'],\n  'aws-eu-south-1': ['location': ['lon': 9.1900, 'lat': 45.4642], 'name': 'aws-eu-south-1'],\n  'aws-eu-north-1': ['location': ['lon': 18.0686, 'lat': 59.3293], 'name': 'aws-eu-north-1'],\n  'aws-eu-central-2': ['location': ['lon': 16.3738, 'lat': 48.2082], 'name': 'aws-eu-central-2'],\n  'azure-eastus2': ['location': ['lon': -80.1918, 'lat': 25.7617], 'name': 'azure-eastus2'],\n  'azure-eastus': ['location': ['lon': -77.0369, 'lat': 38.9072], 'name': 'azure-eastus'],\n  'azure-centralus': ['location': ['lon': -93.2650, 'lat': 44.9778], 'name': 'azure-centralus'],\n  'azure-southcentralus': ['location': ['lon': -97.7431, 'lat': 30.2672], 'name': 'azure-southcentralus'],\n  'azure-westus2': ['location': ['lon': -122.6765, 'lat': 45.5235], 'name': 'azure-westus2'],\n  'azure-northeurope': ['location': ['lon': -6.2603, 'lat': 53.3498], 'name': 'azure-northeurope'],\n  'azure-uksouth': ['location': ['lon': -0.1276, 'lat': 51.5074], 'name': 'azure-uksouth'],\n  'azure-westeurope': ['location': ['lon': 4.9041, 'lat': 52.3676], 'name': 'azure-westeurope'],\n  'azure-francecentral': ['location': ['lon': 2.3522, 'lat': 48.8566], 'name': 'azure-francecentral'],\n  'azure-japaneast': ['location': ['lon': 139.6917, 'lat': 35.6895], 'name': 'azure-japaneast'],\n  'azure-southeastasia': ['location': ['lon': 103.8198, 'lat': 1.3521], 'name': 'azure-southeastasia'],\n  'azure-australiaeast': ['location': ['lon': 151.2093, 'lat': -33.8688], 'name': 'azure-australiaeast'],\n  'azure-canadacentral': ['location': ['lon': -75.6972, 'lat': 45.4215], 'name': 'azure-canadacentral'],\n  'azure-brazilsouth': ['location': ['lon': -46.6333, 'lat': -23.5505], 'name': 'azure-brazilsouth'],\n  'azure-southafricanorth': ['location': ['lon': 18.4241, 'lat': -33.9249], 'name': 'azure-southafricanorth'],\n  'azure-centralindia': ['location': ['lon': 77.5946, 'lat': 12.9716], 'name': 'azure-centralindia']\n];\nString region = ctx.cloud.region;\nif (regionGeo.containsKey(region)) {\n  ctx.cloud.geo = regionGeo[region];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map regionGeo = [\n  'gcp-us-central1': ['location': ['lon': -93.2650, 'lat': 44.9778], 'name': 'gcp-us-central1'],\n  'gcp-us-east4': ['location': ['lon': -80.1918, 'lat': 25.7617], 'name': 'gcp-us-east4'],\n  'gcp-us-west1': ['location': ['lon': -122.4194, 'lat': 37.7749], 'name': 'gcp-us-west1'],\n  'gcp-us-west2': ['location': ['lon': -118.2437, 'lat': 34.0522], 'name': 'gcp-us-west2'],\n  'gcp-us-east1': ['location': ['lon': -77.0369, 'lat': 38.9072], 'name': 'gcp-us-east1'],\n  'gcp-northamerica-northeast1': ['location': ['lon': -73.6148, 'lat': 45.5059], 'name': 'gcp-northamerica-northeast1'],\n  'gcp-southamerica-east1': ['location': ['lon': -46.6333, 'lat': -23.5505], 'name': 'gcp-southamerica-east1'],\n  'gcp-europe-west1': ['location': ['lon': -6.2603, 'lat': 53.3498], 'name': 'gcp-europe-west1'],\n  'gcp-europe-west2': ['location': ['lon': -0.1276, 'lat': 51.5074], 'name': 'gcp-europe-west2'],\n  'gcp-europe-west3': ['location': ['lon': 11.5820, 'lat': 48.1351], 'name': 'gcp-europe-west3'],\n  'gcp-europe-west4': ['location': ['lon': 19.0402, 'lat': 47.4979], 'name': 'gcp-europe-west4'],\n  'gcp-europe-north1': ['location': ['lon': 24.9458, 'lat': 60.1921], 'name': 'gcp-europe-north1'],\n  'gcp-asia-east1': ['location': ['lon': 121.5654, 'lat': 25.0330], 'name': 'gcp-asia-east1'],\n  'gcp-asia-northeast1': ['location': ['lon': 139.6917, 'lat': 35.6895], 'name': 'gcp-asia-northeast1'],\n  'gcp-asia-south1': ['location': ['lon': 72.8777, 'lat': 19.0760], 'name': 'gcp-asia-south1'],\n  'gcp-asia-southeast1': ['location': ['lon': 103.8198, 'lat': 1.3521], 'name': 'gcp-asia-southeast1'],\n  'gcp-australia-southeast1': ['location': ['lon': 151.2093, 'lat': -33.8688], 'name': 'gcp-australia-southeast1'],\n  'gcp-asia-northeast3': ['location': ['lon': 126.9780, 'lat': 37.5665], 'name': 'gcp-asia-northeast3'],\n  'gcp-asia-southeast2': ['location': ['lon': 106.8650, 'lat': -6.2088], 'name': 'gcp-asia-southeast2'],\n  'gcp-europe-west9': ['location': ['lon': 4.9041, 'lat': 52.3676], 'name': 'gcp-europe-west9'],\n  'gcp-me-west1': ['location': ['lon': 34.8516, 'lat': 31.0461], 'name': 'gcp-me-west1'],\n  'aws-us-east-2': ['location': ['lon': -82.9988, 'lat': 39.9612], 'name': 'aws-us-east-2'],\n  'aws-us-east-1': ['location': ['lon': -77.0369, 'lat': 39.9612], 'name': 'aws-us-east-2'],\n  'us-east-1': ['location': ['lon': -77.0369, 'lat': 38.9072], 'name': 'us-east-1'],\n  'eu-west-1': ['location': ['lon': -6.2603, 'lat': 53.3498], 'name': 'eu-west-1'],\n  'us-west-1': ['location': ['lon': -122.4194, 'lat': 37.7749], 'name': 'us-west-1'],\n  'us-west-2': ['location': ['lon': -122.6765, 'lat': 45.5235], 'name': 'us-west-2'],\n  'aws-ca-central-1': ['location': ['lon': -75.6972, 'lat': 45.4215], 'name': 'aws-ca-central-1'],\n  'sa-east-1': ['location': ['lon': -46.6333, 'lat': -23.5505], 'name': 'sa-east-1'],\n  'aws-eu-west-2': ['location': ['lon': -0.1276, 'lat': 51.5074], 'name': 'aws-eu-west-2'],\n  'aws-eu-west-3': ['location': ['lon': 2.3522, 'lat': 48.8566], 'name': 'aws-eu-west-3'],\n  'aws-eu-central-1': ['location': ['lon': 8.6821, 'lat': 50.1109], 'name': 'aws-eu-central-1'],\n  'aws-ap-south-1': ['location': ['lon': 72.8777, 'lat': 19.0760], 'name': 'aws-ap-south-1'],\n  'ap-northeast-1': ['location': ['lon': 139.6917, 'lat': 35.6895], 'name': 'ap-northeast-1'],\n  'aws-ap-northeast-2': ['location': ['lon': 126.9780, 'lat': 37.5665], 'name': 'aws-ap-northeast-2'],\n  'aws-ap-east-1': ['location': ['lon': 114.1095, 'lat': 22.3964], 'name': 'aws-ap-east-1'],\n  'ap-southeast-1': ['location': ['lon': 103.8198, 'lat': 1.3521], 'name': 'ap-southeast-1'],\n  'ap-southeast-2': ['location': ['lon': 151.2093, 'lat': -33.8688], 'name': 'ap-southeast-2'],\n  'aws-af-south-1': ['location': ['lon': 18.4241, 'lat': -33.9249], 'name': 'aws-af-south-1'],\n  'aws-me-south-1': ['location': ['lon': 55.2708, 'lat': 25.2048], 'name': 'aws-me-south-1'],\n  'aws-eu-south-1': ['location': ['lon': 9.1900, 'lat': 45.4642], 'name': 'aws-eu-south-1'],\n  'aws-eu-north-1': ['location': ['lon': 18.0686, 'lat': 59.3293], 'name': 'aws-eu-north-1'],\n  'aws-eu-central-2': ['location': ['lon': 16.3738, 'lat': 48.2082], 'name': 'aws-eu-central-2'],\n  'azure-eastus2': ['location': ['lon': -80.1918, 'lat': 25.7617], 'name': 'azure-eastus2'],\n  'azure-eastus': ['location': ['lon': -77.0369, 'lat': 38.9072], 'name': 'azure-eastus'],\n  'azure-centralus': ['location': ['lon': -93.2650, 'lat': 44.9778], 'name': 'azure-centralus'],\n  'azure-southcentralus': ['location': ['lon': -97.7431, 'lat': 30.2672], 'name': 'azure-southcentralus'],\n  'azure-westus2': ['location': ['lon': -122.6765, 'lat': 45.5235], 'name': 'azure-westus2'],\n  'azure-northeurope': ['location': ['lon': -6.2603, 'lat': 53.3498], 'name': 'azure-northeurope'],\n  'azure-uksouth': ['location': ['lon': -0.1276, 'lat': 51.5074], 'name': 'azure-uksouth'],\n  'azure-westeurope': ['location': ['lon': 4.9041, 'lat': 52.3676], 'name': 'azure-westeurope'],\n  'azure-francecentral': ['location': ['lon': 2.3522, 'lat': 48.8566], 'name': 'azure-francecentral'],\n  'azure-japaneast': ['location': ['lon': 139.6917, 'lat': 35.6895], 'name': 'azure-japaneast'],\n  'azure-southeastasia': ['location': ['lon': 103.8198, 'lat': 1.3521], 'name': 'azure-southeastasia'],\n  'azure-australiaeast': ['location': ['lon': 151.2093, 'lat': -33.8688], 'name': 'azure-australiaeast'],\n  'azure-canadacentral': ['location': ['lon': -75.6972, 'lat': 45.4215], 'name': 'azure-canadacentral'],\n  'azure-brazilsouth': ['location': ['lon': -46.6333, 'lat': -23.5505], 'name': 'azure-brazilsouth'],\n  'azure-southafricanorth': ['location': ['lon': 18.4241, 'lat': -33.9249], 'name': 'azure-southafricanorth'],\n  'azure-centralindia': ['location': ['lon': 77.5946, 'lat': 12.9716], 'name': 'azure-centralindia']\n];\nString region = ctx.cloud.region;\nif (regionGeo.containsKey(region)) {\n  ctx.cloud.geo = regionGeo[region];\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { !event.has_value("cloud.geo.name") };
            if _cond {
                event.set(
                    "cloud.geo",
                    cached_params!(
                        "{\"name\":\"unknown\",\"location\":{\"lon\":135.0,\"lat\":90.0}}"
                    )
                    .clone(),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
