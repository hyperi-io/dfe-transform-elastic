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
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.set("cloud.provider", json!("aws"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                event.rename("message", "event.original")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("bill_bill_type"))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if let Some(csv_str) = event.get_string("event.original") {
                let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .quote(b'\"')
                    .has_headers(false)
                    .from_reader(csv_str.as_bytes());
                if let Some(Ok(record)) = rdr.records().next() {
                    if let Some(val) = record.get(0) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_bill_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(1) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_billing_entity", val)?;
                        }
                    }
                    if let Some(val) = record.get(2) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_billing_period_end_date", val)?;
                        }
                    }
                    if let Some(val) = record.get(3) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_billing_period_start_date", val)?;
                        }
                    }
                    if let Some(val) = record.get(4) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_invoice_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(5) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_invoicing_entity", val)?;
                        }
                    }
                    if let Some(val) = record.get(6) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_payer_account_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(7) {
                        if !val.is_empty() {
                            event.set("_tmp.bill_payer_account_name", val)?;
                        }
                    }
                    if let Some(val) = record.get(8) {
                        if !val.is_empty() {
                            event.set("_tmp.cost_category", val)?;
                        }
                    }
                    if let Some(val) = record.get(9) {
                        if !val.is_empty() {
                            event.set("_tmp.discount", val)?;
                        }
                    }
                    if let Some(val) = record.get(10) {
                        if !val.is_empty() {
                            event.set("_tmp.discount_bundled_discount", val)?;
                        }
                    }
                    if let Some(val) = record.get(11) {
                        if !val.is_empty() {
                            event.set("_tmp.discount_total_discount", val)?;
                        }
                    }
                    if let Some(val) = record.get(12) {
                        if !val.is_empty() {
                            event.set("_tmp.identity_line_item_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(13) {
                        if !val.is_empty() {
                            event.set("_tmp.identity_time_interval", val)?;
                        }
                    }
                    if let Some(val) = record.get(14) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_availability_zone", val)?;
                        }
                    }
                    if let Some(val) = record.get(15) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_blended_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(16) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_blended_rate", val)?;
                        }
                    }
                    if let Some(val) = record.get(17) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_currency_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(18) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_legal_entity", val)?;
                        }
                    }
                    if let Some(val) = record.get(19) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_line_item_description", val)?;
                        }
                    }
                    if let Some(val) = record.get(20) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_line_item_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(21) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_net_unblended_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(22) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_net_unblended_rate", val)?;
                        }
                    }
                    if let Some(val) = record.get(23) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_normalization_factor", val)?;
                        }
                    }
                    if let Some(val) = record.get(24) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_normalized_usage_amount", val)?;
                        }
                    }
                    if let Some(val) = record.get(25) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_operation", val)?;
                        }
                    }
                    if let Some(val) = record.get(26) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_product_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(27) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_tax_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(28) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_unblended_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(29) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_unblended_rate", val)?;
                        }
                    }
                    if let Some(val) = record.get(30) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_usage_account_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(31) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_usage_account_name", val)?;
                        }
                    }
                    if let Some(val) = record.get(32) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_usage_amount", val)?;
                        }
                    }
                    if let Some(val) = record.get(33) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_usage_end_date", val)?;
                        }
                    }
                    if let Some(val) = record.get(34) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_usage_start_date", val)?;
                        }
                    }
                    if let Some(val) = record.get(35) {
                        if !val.is_empty() {
                            event.set("_tmp.line_item_usage_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(36) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_currency", val)?;
                        }
                    }
                    if let Some(val) = record.get(37) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_lease_contract_length", val)?;
                        }
                    }
                    if let Some(val) = record.get(38) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_offering_class", val)?;
                        }
                    }
                    if let Some(val) = record.get(39) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_public_on_demand_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(40) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_public_on_demand_rate", val)?;
                        }
                    }
                    if let Some(val) = record.get(41) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_purchase_option", val)?;
                        }
                    }
                    if let Some(val) = record.get(42) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_rate_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(43) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_rate_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(44) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_term", val)?;
                        }
                    }
                    if let Some(val) = record.get(45) {
                        if !val.is_empty() {
                            event.set("_tmp.pricing_unit", val)?;
                        }
                    }
                    if let Some(val) = record.get(46) {
                        if !val.is_empty() {
                            event.set("_tmp.product", val)?;
                        }
                    }
                    if let Some(val) = record.get(47) {
                        if !val.is_empty() {
                            event.set("_tmp.product_comment", val)?;
                        }
                    }
                    if let Some(val) = record.get(48) {
                        if !val.is_empty() {
                            event.set("_tmp.product_fee_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(49) {
                        if !val.is_empty() {
                            event.set("_tmp.product_fee_description", val)?;
                        }
                    }
                    if let Some(val) = record.get(50) {
                        if !val.is_empty() {
                            event.set("_tmp.product_from_location", val)?;
                        }
                    }
                    if let Some(val) = record.get(51) {
                        if !val.is_empty() {
                            event.set("_tmp.product_from_location_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(52) {
                        if !val.is_empty() {
                            event.set("_tmp.product_from_region_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(53) {
                        if !val.is_empty() {
                            event.set("_tmp.product_instance_family", val)?;
                        }
                    }
                    if let Some(val) = record.get(54) {
                        if !val.is_empty() {
                            event.set("_tmp.product_instance_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(55) {
                        if !val.is_empty() {
                            event.set("_tmp.product_instancesku", val)?;
                        }
                    }
                    if let Some(val) = record.get(56) {
                        if !val.is_empty() {
                            event.set("_tmp.product_location", val)?;
                        }
                    }
                    if let Some(val) = record.get(57) {
                        if !val.is_empty() {
                            event.set("_tmp.product_location_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(58) {
                        if !val.is_empty() {
                            event.set("_tmp.product_operation", val)?;
                        }
                    }
                    if let Some(val) = record.get(59) {
                        if !val.is_empty() {
                            event.set("_tmp.product_pricing_unit", val)?;
                        }
                    }
                    if let Some(val) = record.get(60) {
                        if !val.is_empty() {
                            event.set("_tmp.product_product_family", val)?;
                        }
                    }
                    if let Some(val) = record.get(61) {
                        if !val.is_empty() {
                            event.set("_tmp.product_region_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(62) {
                        if !val.is_empty() {
                            event.set("_tmp.product_servicecode", val)?;
                        }
                    }
                    if let Some(val) = record.get(63) {
                        if !val.is_empty() {
                            event.set("_tmp.product_sku", val)?;
                        }
                    }
                    if let Some(val) = record.get(64) {
                        if !val.is_empty() {
                            event.set("_tmp.product_to_location", val)?;
                        }
                    }
                    if let Some(val) = record.get(65) {
                        if !val.is_empty() {
                            event.set("_tmp.product_to_location_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(66) {
                        if !val.is_empty() {
                            event.set("_tmp.product_to_region_code", val)?;
                        }
                    }
                    if let Some(val) = record.get(67) {
                        if !val.is_empty() {
                            event.set("_tmp.product_usagetype", val)?;
                        }
                    }
                    if let Some(val) = record.get(68) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_amortized_upfront_cost_for_usage", val)?;
                        }
                    }
                    if let Some(val) = record.get(69) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.reservation_amortized_upfront_fee_for_billing_period",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(70) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_availability_zone", val)?;
                        }
                    }
                    if let Some(val) = record.get(71) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_effective_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(72) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_end_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(73) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_modification_status", val)?;
                        }
                    }
                    if let Some(val) = record.get(74) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.reservation_net_amortized_upfront_cost_for_usage",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(75) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.reservation_net_amortized_upfront_fee_for_billing_period",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(76) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_net_effective_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(77) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_net_recurring_fee_for_usage", val)?;
                        }
                    }
                    if let Some(val) = record.get(78) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_net_unused_amortized_upfront_fee_for_billing_period", val)?;
                        }
                    }
                    if let Some(val) = record.get(79) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_net_unused_recurring_fee", val)?;
                        }
                    }
                    if let Some(val) = record.get(80) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_net_upfront_value", val)?;
                        }
                    }
                    if let Some(val) = record.get(81) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_normalized_units_per_reservation", val)?;
                        }
                    }
                    if let Some(val) = record.get(82) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_number_of_reservations", val)?;
                        }
                    }
                    if let Some(val) = record.get(83) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_recurring_fee_for_usage", val)?;
                        }
                    }
                    if let Some(val) = record.get(84) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_reservation_a_r_n", val)?;
                        }
                    }
                    if let Some(val) = record.get(85) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_start_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(86) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_subscription_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(87) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_total_reserved_normalized_units", val)?;
                        }
                    }
                    if let Some(val) = record.get(88) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_total_reserved_units", val)?;
                        }
                    }
                    if let Some(val) = record.get(89) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_units_per_reservation", val)?;
                        }
                    }
                    if let Some(val) = record.get(90) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.reservation_unused_amortized_upfront_fee_for_billing_period",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(91) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_unused_normalized_unit_quantity", val)?;
                        }
                    }
                    if let Some(val) = record.get(92) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_unused_quantity", val)?;
                        }
                    }
                    if let Some(val) = record.get(93) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_unused_recurring_fee", val)?;
                        }
                    }
                    if let Some(val) = record.get(94) {
                        if !val.is_empty() {
                            event.set("_tmp.reservation_upfront_value", val)?;
                        }
                    }
                    if let Some(val) = record.get(95) {
                        if !val.is_empty() {
                            event.set("_tmp.resource_tags", val)?;
                        }
                    }
                    if let Some(val) = record.get(96) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.savings_plan_amortized_upfront_commitment_for_billing_period",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(97) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_end_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(98) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_instance_type_family", val)?;
                        }
                    }
                    if let Some(val) = record.get(99) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_net_amortized_upfront_commitment_for_billing_period", val)?;
                        }
                    }
                    if let Some(val) = record.get(100) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.savings_plan_net_recurring_commitment_for_billing_period",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(101) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_net_savings_plan_effective_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(102) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_offering_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(103) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_payment_option", val)?;
                        }
                    }
                    if let Some(val) = record.get(104) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_purchase_term", val)?;
                        }
                    }
                    if let Some(val) = record.get(105) {
                        if !val.is_empty() {
                            event.set(
                                "_tmp.savings_plan_recurring_commitment_for_billing_period",
                                val,
                            )?;
                        }
                    }
                    if let Some(val) = record.get(106) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_region", val)?;
                        }
                    }
                    if let Some(val) = record.get(107) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_savings_plan_a_r_n", val)?;
                        }
                    }
                    if let Some(val) = record.get(108) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_savings_plan_effective_cost", val)?;
                        }
                    }
                    if let Some(val) = record.get(109) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_savings_plan_rate", val)?;
                        }
                    }
                    if let Some(val) = record.get(110) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_start_time", val)?;
                        }
                    }
                    if let Some(val) = record.get(111) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_total_commitment_to_date", val)?;
                        }
                    }
                    if let Some(val) = record.get(112) {
                        if !val.is_empty() {
                            event.set("_tmp.savings_plan_used_commitment", val)?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.discount_total_discount") };
            if _cond {
                if event.has_value("_tmp.discount_total_discount") {
                    if let Some(val) = event.get("_tmp.discount_total_discount") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "_tmp.discount_total_discount".into(),
                                message,
                            }
                        })?;
                        event.set("aws_billing.cur.total_discount", converted)?;
                    }
                }
            }

            if event.has_value("_tmp.line_item_blended_cost") {
                if let Some(val) = event.get("_tmp.line_item_blended_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_blended_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.blended_cost", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_blended_rate") {
                if let Some(val) = event.get("_tmp.line_item_blended_rate") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_blended_rate".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.blended_rate", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_net_unblended_cost") {
                if let Some(val) = event.get("_tmp.line_item_net_unblended_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_net_unblended_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.net_unblended_cost", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_net_unblended_rate") {
                if let Some(val) = event.get("_tmp.line_item_net_unblended_rate") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_net_unblended_rate".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.net_unblended_rate", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_normalized_usage_amount") {
                if let Some(val) = event.get("_tmp.line_item_normalized_usage_amount") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_normalized_usage_amount".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.line_item.normalized_usage_amount",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.line_item_unblended_cost") {
                if let Some(val) = event.get("_tmp.line_item_unblended_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_unblended_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.unblended_cost", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_normalization_factor") {
                if let Some(val) = event.get("_tmp.line_item_normalization_factor") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_normalization_factor".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.normalization_factor", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_unblended_rate") {
                if let Some(val) = event.get("_tmp.line_item_unblended_rate") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_unblended_rate".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.unblended_rate", converted)?;
                }
            }

            if event.has_value("_tmp.line_item_usage_amount") {
                if let Some(val) = event.get("_tmp.line_item_usage_amount") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.line_item_usage_amount".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.line_item.usage_amount", converted)?;
                }
            }

            if event.has_value("_tmp.pricing_public_on_demand_cost") {
                if let Some(val) = event.get("_tmp.pricing_public_on_demand_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.pricing_public_on_demand_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.pricing.public_on_demand_cost", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_amortized_upfront_cost_for_usage") {
                if let Some(val) = event.get("_tmp.reservation_amortized_upfront_cost_for_usage") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_amortized_upfront_cost_for_usage".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.amortized_upfront_cost_for_usage",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.pricing_public_on_demand_rate") {
                if let Some(val) = event.get("_tmp.pricing_public_on_demand_rate") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.pricing_public_on_demand_rate".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.pricing.public_on_demand_rate", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_net_amortized_upfront_cost_for_usage") {
                if let Some(val) =
                    event.get("_tmp.reservation_net_amortized_upfront_cost_for_usage")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_net_amortized_upfront_cost_for_usage".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.net_amortized_upfront_cost_for_usage",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_unused_quantity") {
                if let Some(val) = event.get("_tmp.reservation_unused_quantity") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_unused_quantity".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.reservation.unused_quantity", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_net_effective_cost") {
                if let Some(val) = event.get("_tmp.reservation_net_effective_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_net_effective_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.reservation.net_effective_cost", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_net_amortized_upfront_fee_for_billing_period") {
                if let Some(val) =
                    event.get("_tmp.reservation_net_amortized_upfront_fee_for_billing_period")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_net_amortized_upfront_fee_for_billing_period"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.net_amortized_upfront_fee_for_billing_period",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_net_upfront_value") {
                if let Some(val) = event.get("_tmp.reservation_net_upfront_value") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_net_upfront_value".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.reservation.net_upfront_value", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_net_recurring_fee_for_usage") {
                if let Some(val) = event.get("_tmp.reservation_net_recurring_fee_for_usage") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_net_recurring_fee_for_usage".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.net_recurring_fee_for_usage",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_effective_cost") {
                if let Some(val) = event.get("_tmp.reservation_effective_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_effective_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.reservation.effective_cost", converted)?;
                }
            }

            if event
                .has_value("_tmp.reservation_net_unused_amortized_upfront_fee_for_billing_period")
            {
                if let Some(val) = event
                    .get("_tmp.reservation_net_unused_amortized_upfront_fee_for_billing_period")
                {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.reservation_net_unused_amortized_upfront_fee_for_billing_period".into(),
                            message,
                        })?;
                    event.set("aws_billing.cur.reservation.net_unused_amortized_upfront_fee_for_billing_period", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_unused_normalized_unit_quantity") {
                if let Some(val) = event.get("_tmp.reservation_unused_normalized_unit_quantity") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_unused_normalized_unit_quantity".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.unused_normalized_unit_quantity",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_unused_amortized_upfront_fee_for_billing_period") {
                if let Some(val) =
                    event.get("_tmp.reservation_unused_amortized_upfront_fee_for_billing_period")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "_tmp.reservation_unused_amortized_upfront_fee_for_billing_period"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.reservation.unused_amortized_upfront_fee_for_billing_period", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_recurring_fee_for_usage") {
                if let Some(val) = event.get("_tmp.reservation_recurring_fee_for_usage") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_recurring_fee_for_usage".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.recurring_fee_for_usage",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_net_unused_recurring_fee") {
                if let Some(val) = event.get("_tmp.reservation_net_unused_recurring_fee") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_net_unused_recurring_fee".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.net_unused_recurring_fee",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_amortized_upfront_fee_for_billing_period") {
                if let Some(val) =
                    event.get("_tmp.reservation_amortized_upfront_fee_for_billing_period")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_amortized_upfront_fee_for_billing_period"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.amortized_upfront_fee_for_billing_period",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_unused_recurring_fee") {
                if let Some(val) = event.get("_tmp.reservation_unused_recurring_fee") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_unused_recurring_fee".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.unused_recurring_fee",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.savings_plan_amortized_upfront_commitment_for_billing_period")
            {
                if let Some(val) =
                    event.get("_tmp.savings_plan_amortized_upfront_commitment_for_billing_period")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "_tmp.savings_plan_amortized_upfront_commitment_for_billing_period"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.savings_plan.amortized_upfront_commitment_for_billing_period", converted)?;
                }
            }

            if event.has_value("_tmp.savings_plan_savings_plan_effective_cost") {
                if let Some(val) = event.get("_tmp.savings_plan_savings_plan_effective_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_savings_plan_effective_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.savings_plan.effective_cost", converted)?;
                }
            }

            if event
                .has_value("_tmp.savings_plan_net_amortized_upfront_commitment_for_billing_period")
            {
                if let Some(val) = event
                    .get("_tmp.savings_plan_net_amortized_upfront_commitment_for_billing_period")
                {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "_tmp.savings_plan_net_amortized_upfront_commitment_for_billing_period".into(),
                            message,
                        })?;
                    event.set("aws_billing.cur.savings_plan.net_amortized_upfront_commitment_for_billing_period", converted)?;
                }
            }

            if event.has_value("_tmp.savings_plan_net_savings_plan_effective_cost") {
                if let Some(val) = event.get("_tmp.savings_plan_net_savings_plan_effective_cost") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_net_savings_plan_effective_cost".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.savings_plan.net_effective_cost", converted)?;
                }
            }

            if event.has_value("_tmp.savings_plan_net_recurring_commitment_for_billing_period") {
                if let Some(val) =
                    event.get("_tmp.savings_plan_net_recurring_commitment_for_billing_period")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_net_recurring_commitment_for_billing_period"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.savings_plan.net_recurring_commitment_for_billing_period",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.savings_plan_savings_plan_rate") {
                if let Some(val) = event.get("_tmp.savings_plan_savings_plan_rate") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_savings_plan_rate".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.savings_plan.rate", converted)?;
                }
            }

            if event.has_value("_tmp.savings_plan_recurring_commitment_for_billing_period") {
                if let Some(val) =
                    event.get("_tmp.savings_plan_recurring_commitment_for_billing_period")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_recurring_commitment_for_billing_period"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.savings_plan.recurring_commitment_for_billing_period",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.savings_plan_total_commitment_to_date") {
                if let Some(val) = event.get("_tmp.savings_plan_total_commitment_to_date") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_total_commitment_to_date".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.savings_plan.total_commitment_to_date",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.savings_plan_used_commitment") {
                if let Some(val) = event.get("_tmp.savings_plan_used_commitment") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.savings_plan_used_commitment".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.savings_plan.used_commitment", converted)?;
                }
            }

            if event.has_value("_tmp.reservation_upfront_value") {
                if let Some(val) = event.get("_tmp.reservation_upfront_value") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_upfront_value".into(),
                            message,
                        }
                    })?;
                    event.set("aws_billing.cur.reservation.upfront_value", converted)?;
                }
            }

            if event.has_value("_tmp.bill_bill_type") {
                event.rename("_tmp.bill_bill_type", "aws_billing.cur.bill.type")?;
            }

            if event.has_value("_tmp.bill_billing_entity") {
                event.rename(
                    "_tmp.bill_billing_entity",
                    "aws_billing.cur.bill.billing_entity",
                )?;
            }

            if event.has_value("_tmp.bill_billing_period_end_date") {
                event.rename(
                    "_tmp.bill_billing_period_end_date",
                    "aws_billing.cur.bill.billing_period_end_date",
                )?;
            }

            if event.has_value("_tmp.bill_billing_period_start_date") {
                event.rename(
                    "_tmp.bill_billing_period_start_date",
                    "aws_billing.cur.bill.billing_period_start_date",
                )?;
            }

            if event.has_value("_tmp.bill_invoice_id") {
                event.rename("_tmp.bill_invoice_id", "aws_billing.cur.bill.invoice_id")?;
            }

            if event.has_value("_tmp.bill_invoicing_entity") {
                event.rename(
                    "_tmp.bill_invoicing_entity",
                    "aws_billing.cur.bill.invoicing_entity",
                )?;
            }

            if event.has_value("_tmp.bill_payer_account_id") {
                event.rename(
                    "_tmp.bill_payer_account_id",
                    "aws_billing.cur.bill.payer_account_id",
                )?;
            }

            if event.has_value("_tmp.bill_payer_account_name") {
                event.rename(
                    "_tmp.bill_payer_account_name",
                    "aws_billing.cur.bill.payer_account_name",
                )?;
            }

            if event.has_value("_tmp.discount_bundled_discount") {
                event.rename(
                    "_tmp.discount_bundled_discount",
                    "aws_billing.cur.bundled_discount",
                )?;
            }

            if event.has_value("_tmp.identity_line_item_id") {
                event.rename(
                    "_tmp.identity_line_item_id",
                    "aws_billing.cur.identity.line_item_id",
                )?;
            }

            if event.has_value("_tmp.identity_time_interval") {
                event.rename(
                    "_tmp.identity_time_interval",
                    "aws_billing.cur.identity.time_interval",
                )?;
            }

            if event.has_value("_tmp.line_item_availability_zone") {
                event.rename(
                    "_tmp.line_item_availability_zone",
                    "aws_billing.cur.line_item.availability_zone",
                )?;
            }

            if event.has_value("_tmp.line_item_currency_code") {
                event.rename(
                    "_tmp.line_item_currency_code",
                    "aws_billing.cur.line_item.currency_code",
                )?;
            }

            if event.has_value("_tmp.line_item_legal_entity") {
                event.rename(
                    "_tmp.line_item_legal_entity",
                    "aws_billing.cur.line_item.legal_entity",
                )?;
            }

            if event.has_value("_tmp.line_item_line_item_description") {
                event.rename(
                    "_tmp.line_item_line_item_description",
                    "aws_billing.cur.line_item.description",
                )?;
            }

            if event.has_value("_tmp.line_item_line_item_type") {
                event.rename(
                    "_tmp.line_item_line_item_type",
                    "aws_billing.cur.line_item.type",
                )?;
            }

            if event.has_value("_tmp.line_item_operation") {
                event.rename(
                    "_tmp.line_item_operation",
                    "aws_billing.cur.line_item.operation",
                )?;
            }

            if event.has_value("_tmp.line_item_product_code") {
                event.rename(
                    "_tmp.line_item_product_code",
                    "aws_billing.cur.line_item.product_code",
                )?;
            }

            if event.has_value("_tmp.line_item_tax_type") {
                event.rename(
                    "_tmp.line_item_tax_type",
                    "aws_billing.cur.line_item.tax_type",
                )?;
            }

            if event.has_value("_tmp.line_item_usage_account_id") {
                event.rename(
                    "_tmp.line_item_usage_account_id",
                    "aws_billing.cur.line_item.usage_account_id",
                )?;
            }

            if event.has_value("_tmp.line_item_usage_account_name") {
                event.rename(
                    "_tmp.line_item_usage_account_name",
                    "aws_billing.cur.line_item.usage_account_name",
                )?;
            }

            if event.has_value("_tmp.line_item_usage_end_date") {
                event.rename(
                    "_tmp.line_item_usage_end_date",
                    "aws_billing.cur.line_item.usage_end_date",
                )?;
            }

            if event.has_value("_tmp.line_item_usage_start_date") {
                event.rename(
                    "_tmp.line_item_usage_start_date",
                    "aws_billing.cur.line_item.usage_start_date",
                )?;
            }

            if event.has_value("_tmp.line_item_usage_type") {
                event.rename(
                    "_tmp.line_item_usage_type",
                    "aws_billing.cur.line_item.usage_type",
                )?;
            }

            if event.has_value("_tmp.pricing_currency") {
                event.rename("_tmp.pricing_currency", "aws_billing.cur.pricing.currency")?;
            }

            if event.has_value("_tmp.pricing_lease_contract_length") {
                event.rename(
                    "_tmp.pricing_lease_contract_length",
                    "aws_billing.cur.pricing.lease_contract_length",
                )?;
            }

            if event.has_value("_tmp.pricing_offering_class") {
                event.rename(
                    "_tmp.pricing_offering_class",
                    "aws_billing.cur.pricing.offering_class",
                )?;
            }

            if event.has_value("_tmp.pricing_purchase_option") {
                event.rename(
                    "_tmp.pricing_purchase_option",
                    "aws_billing.cur.pricing.purchase_option",
                )?;
            }

            if event.has_value("_tmp.pricing_rate_code") {
                event.rename(
                    "_tmp.pricing_rate_code",
                    "aws_billing.cur.pricing.rate_code",
                )?;
            }

            if event.has_value("_tmp.pricing_rate_id") {
                event.rename("_tmp.pricing_rate_id", "aws_billing.cur.pricing.rate_id")?;
            }

            if event.has_value("_tmp.pricing_term") {
                event.rename("_tmp.pricing_term", "aws_billing.cur.pricing.term")?;
            }

            if event.has_value("_tmp.pricing_unit") {
                event.rename("_tmp.pricing_unit", "aws_billing.cur.pricing.unit")?;
            }

            if event.has_value("_tmp.product") {
                event.rename("_tmp.product", "aws_billing.cur.product.product")?;
            }

            if event.has_value("_tmp.product_comment") {
                event.rename("_tmp.product_comment", "aws_billing.cur.product.comment")?;
            }

            if event.has_value("_tmp.product_fee_code") {
                event.rename("_tmp.product_fee_code", "aws_billing.cur.product.fee_code")?;
            }

            if event.has_value("_tmp.product_fee_description") {
                event.rename(
                    "_tmp.product_fee_description",
                    "aws_billing.cur.product.fee_description",
                )?;
            }

            if event.has_value("_tmp.product_from_location") {
                event.rename(
                    "_tmp.product_from_location",
                    "aws_billing.cur.product.from_location",
                )?;
            }

            if event.has_value("_tmp.product_from_location_type") {
                event.rename(
                    "_tmp.product_from_location_type",
                    "aws_billing.cur.product.from_location_type",
                )?;
            }

            if event.has_value("_tmp.product_from_region_code") {
                event.rename(
                    "_tmp.product_from_region_code",
                    "aws_billing.cur.product.from_region_code",
                )?;
            }

            if event.has_value("_tmp.product_instance_family") {
                event.rename(
                    "_tmp.product_instance_family",
                    "aws_billing.cur.product.instance_family",
                )?;
            }

            if event.has_value("_tmp.product_instance_type") {
                event.rename(
                    "_tmp.product_instance_type",
                    "aws_billing.cur.product.instance_type",
                )?;
            }

            if event.has_value("_tmp.product_instancesku") {
                event.rename(
                    "_tmp.product_instancesku",
                    "aws_billing.cur.product.instancesku",
                )?;
            }

            if event.has_value("_tmp.product_location") {
                event.rename("_tmp.product_location", "aws_billing.cur.product.location")?;
            }

            if event.has_value("_tmp.product_location_type") {
                event.rename(
                    "_tmp.product_location_type",
                    "aws_billing.cur.product.location_type",
                )?;
            }

            if event.has_value("_tmp.product_operation") {
                event.rename(
                    "_tmp.product_operation",
                    "aws_billing.cur.product.operation",
                )?;
            }

            if event.has_value("_tmp.product_pricing_unit") {
                event.rename(
                    "_tmp.product_pricing_unit",
                    "aws_billing.cur.product.pricing_unit",
                )?;
            }

            if event.has_value("_tmp.product_product_family") {
                event.rename(
                    "_tmp.product_product_family",
                    "aws_billing.cur.product.family",
                )?;
            }

            if event.has_value("_tmp.product_region_code") {
                event.rename(
                    "_tmp.product_region_code",
                    "aws_billing.cur.product.region_code",
                )?;
            }

            if event.has_value("_tmp.product_servicecode") {
                event.rename(
                    "_tmp.product_servicecode",
                    "aws_billing.cur.product.servicecode",
                )?;
            }

            if event.has_value("_tmp.product_sku") {
                event.rename("_tmp.product_sku", "aws_billing.cur.product.sku")?;
            }

            if event.has_value("_tmp.product_to_location") {
                event.rename(
                    "_tmp.product_to_location",
                    "aws_billing.cur.product.to_location",
                )?;
            }

            if event.has_value("_tmp.product_to_location_type") {
                event.rename(
                    "_tmp.product_to_location_type",
                    "aws_billing.cur.product.to_location_type",
                )?;
            }

            if event.has_value("_tmp.product_to_region_code") {
                event.rename(
                    "_tmp.product_to_region_code",
                    "aws_billing.cur.product.to_region_code",
                )?;
            }

            if event.has_value("_tmp.product_usagetype") {
                event.rename(
                    "_tmp.product_usagetype",
                    "aws_billing.cur.product.usagetype",
                )?;
            }

            if event.has_value("_tmp.reservation_availability_zone") {
                event.rename(
                    "_tmp.reservation_availability_zone",
                    "aws_billing.cur.reservation.availability_zone",
                )?;
            }

            if event.has_value("_tmp.reservation_end_time") {
                event.rename(
                    "_tmp.reservation_end_time",
                    "aws_billing.cur.reservation.end_time",
                )?;
            }

            if event.has_value("_tmp.reservation_modification_status") {
                event.rename(
                    "_tmp.reservation_modification_status",
                    "aws_billing.cur.reservation.modification_status",
                )?;
            }

            if event.has_value("_tmp.reservation_normalized_units_per_reservation") {
                event.rename(
                    "_tmp.reservation_normalized_units_per_reservation",
                    "aws_billing.cur.reservation.normalized_units_per_reservation",
                )?;
            }

            if event.has_value("_tmp.reservation_number_of_reservations") {
                if let Some(val) = event.get("_tmp.reservation_number_of_reservations") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_number_of_reservations".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.number_of_reservations",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_units_per_reservation") {
                if let Some(val) = event.get("_tmp.reservation_units_per_reservation") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reservation_units_per_reservation".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "aws_billing.cur.reservation.units_per_reservation",
                        converted,
                    )?;
                }
            }

            if event.has_value("_tmp.reservation_reservation_a_r_n") {
                event.rename(
                    "_tmp.reservation_reservation_a_r_n",
                    "aws_billing.cur.reservation.a_r_n",
                )?;
            }

            if event.has_value("_tmp.reservation_start_time") {
                event.rename(
                    "_tmp.reservation_start_time",
                    "aws_billing.cur.reservation.start_time",
                )?;
            }

            if event.has_value("_tmp.reservation_subscription_id") {
                event.rename(
                    "_tmp.reservation_subscription_id",
                    "aws_billing.cur.reservation.subscription_id",
                )?;
            }

            if event.has_value("_tmp.reservation_total_reserved_normalized_units") {
                event.rename(
                    "_tmp.reservation_total_reserved_normalized_units",
                    "aws_billing.cur.reservation.total_reserved_normalized_units",
                )?;
            }

            if event.has_value("_tmp.reservation_total_reserved_units") {
                event.rename(
                    "_tmp.reservation_total_reserved_units",
                    "aws_billing.cur.reservation.total_reserved_units",
                )?;
            }

            if event.has_value("_tmp.savings_plan_end_time") {
                event.rename(
                    "_tmp.savings_plan_end_time",
                    "aws_billing.cur.savings_plan.end_time",
                )?;
            }

            if event.has_value("_tmp.savings_plan_instance_type_family") {
                event.rename(
                    "_tmp.savings_plan_instance_type_family",
                    "aws_billing.cur.savings_plan.instance_type_family",
                )?;
            }

            if event.has_value("_tmp.savings_plan_offering_type") {
                event.rename(
                    "_tmp.savings_plan_offering_type",
                    "aws_billing.cur.savings_plan.offering_type",
                )?;
            }

            if event.has_value("_tmp.savings_plan_payment_option") {
                event.rename(
                    "_tmp.savings_plan_payment_option",
                    "aws_billing.cur.savings_plan.payment_option",
                )?;
            }

            if event.has_value("_tmp.savings_plan_purchase_term") {
                event.rename(
                    "_tmp.savings_plan_purchase_term",
                    "aws_billing.cur.savings_plan.purchase_term",
                )?;
            }

            if event.has_value("_tmp.savings_plan_region") {
                event.rename(
                    "_tmp.savings_plan_region",
                    "aws_billing.cur.savings_plan.region",
                )?;
            }

            if event.has_value("_tmp.savings_plan_savings_plan_a_r_n") {
                event.rename(
                    "_tmp.savings_plan_savings_plan_a_r_n",
                    "aws_billing.cur.savings_plan.a_r_n",
                )?;
            }

            if event.has_value("_tmp.savings_plan_start_time") {
                event.rename(
                    "_tmp.savings_plan_start_time",
                    "aws_billing.cur.savings_plan.start_time",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "_tmp.resource_tags", "temp_tags_obj")?;
                Ok(())
            })();

            let _cond = { !event.has_value("aws_billing.cur.product.region_code") };
            if _cond {
                event.set("aws_billing.cur.product.region_code", json!("No region"))?;
            }

            let _cond = { event.has_value("temp_tags_obj") };
            if _cond {
                // Painless script
                // Source: Map sortedTags = new TreeMap(); sortedTags.putAll(ctx.temp_tags_obj);\nStringBuilder sb = new StringBuilder(); sb.append('{');\nIterator it = sortedTags.entrySet().iterator(); while (it.hasNext()) {\n  Map.Entry pair = (Map.Entry)it.next();\n  sb.append('\"');\n  sb.append(pair.getKey());\n  sb.append('\"');\n  sb.append(':');\n  sb.append('\"');\n  sb.append(pair.getValue());\n  sb.append('\"');\n  if (it.hasNext()) {\n    sb.append(',');\n  }\n}\nsb.append('}'); ctx.aws_billing.cur.resource_tags = sb.toString();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map sortedTags = new TreeMap(); sortedTags.putAll(ctx.temp_tags_obj);\nStringBuilder sb = new StringBuilder(); sb.append('{');\nIterator it = sortedTags.entrySet().iterator(); while (it.hasNext()) {\n  Map.Entry pair = (Map.Entry)it.next();\n  sb.append('\"');\n  sb.append(pair.getKey());\n  sb.append('\"');\n  sb.append(':');\n  sb.append('\"');\n  sb.append(pair.getValue());\n  sb.append('\"');\n  if (it.hasNext()) {\n    sb.append(',');\n  }\n}\nsb.append('}'); ctx.aws_billing.cur.resource_tags = sb.toString();"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("temp_tags_obj").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "temp_tags_obj".into(),
                    });
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "_tmp.cost_category", "temp_cost_category_obj")?;
                Ok(())
            })();

            let _cond = { event.has_value("temp_cost_category_obj") };
            if _cond {
                // Painless script
                // Source: Map sortedCostCategories = new TreeMap(); sortedCostCategories.putAll(ctx.temp_cost_category_obj);\nStringBuilder sb = new StringBuilder(); sb.append('{');\nIterator it = sortedCostCategories.entrySet().iterator(); while (it.hasNext()) {\n  Map.Entry pair = (Map.Entry)it.next();\n  sb.append('\"');\n  sb.append(pair.getKey());\n  sb.append('\"');\n  sb.append(':');\n  sb.append('\"');\n  sb.append(pair.getValue());\n  sb.append('\"');\n  if (it.hasNext()) {\n    sb.append(',');\n  }\n}\nsb.append('}'); ctx.aws_billing.cur.cost_category = sb.toString();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map sortedCostCategories = new TreeMap(); sortedCostCategories.putAll(ctx.temp_cost_category_obj);\nStringBuilder sb = new StringBuilder(); sb.append('{');\nIterator it = sortedCostCategories.entrySet().iterator(); while (it.hasNext()) {\n  Map.Entry pair = (Map.Entry)it.next();\n  sb.append('\"');\n  sb.append(pair.getKey());\n  sb.append('\"');\n  sb.append(':');\n  sb.append('\"');\n  sb.append(pair.getValue());\n  sb.append('\"');\n  if (it.hasNext()) {\n    sb.append(',');\n  }\n}\nsb.append('}'); ctx.aws_billing.cur.cost_category = sb.toString();"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("temp_cost_category_obj").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "temp_cost_category_obj".into(),
                    });
                }
                Ok(())
            })();

            event.remove("_tmp");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            let _cond = {
                event.get("event.original").is_some_and(|v| v.is_string())
                    && event
                        .get_as_string("event.original")
                        .is_some_and(|s| s.len() > 32766)
            };
            if _cond {
                // Painless script
                // Source: ctx.event.original = 'sha1-'+ctx.event.original.sha1()+':'+ctx.event.original.length().toString();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event.original = 'sha1-'+ctx.event.original.sha1()+':'+ctx.event.original.length().toString();\n"#
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
