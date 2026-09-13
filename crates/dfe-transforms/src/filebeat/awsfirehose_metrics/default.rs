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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "", "*")?;
                Ok(())
            })();

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/ApiGateway") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.apigateway_metrics"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/DynamoDB") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.dynamodb"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/EBS") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.ebs"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/EC2") };
            if _cond {
                event.set("event.dataset", json!("aws.ec2_metrics"))?;
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/ECS") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.ecs_metrics"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/ELB") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.elb_metrics"))?;
                    Ok(())
                })();
            }

            let _cond =
                { event.get_str("aws.cloudwatch.namespace") == Some("AWS/ElasticMapReduce") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.emr_metrics"))?;
                    Ok(())
                })();
            }

            let _cond =
                { event.get_str("aws.cloudwatch.namespace") == Some("AWS/NetworkFirewall") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.firewall_metrics"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/Kafka") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.kafka_metrics"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/Kinesis") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.kinesis"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/Lambda") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.lambda"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/NATGateway") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.natgateway"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/RDS") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.rds"))?;
                    Ok(())
                })();
            }

            let _cond =
                { event.get_str("aws.cloudwatch.namespace") == Some("AWS/S3/Storage-Lens") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.s3_storage_lens"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/SNS") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.sns"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/SQS") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.sqs"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/TransitGateway") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.transitgateway"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/Usage") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.usage"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/VPN") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.vpn"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("aws.cloudwatch.namespace") == Some("AWS/S3")
                    && (event.has_value("aws.s3.metrics.BucketSizeBytes")
                        || event.has_value("aws.s3.metrics.NumberOfObjects"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.s3_daily_storage"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("aws.cloudwatch.namespace") == Some("AWS/S3")
                    && !event.has_value("aws.s3.metrics.BucketSizeBytes")
                    && !event.has_value("aws.s3.metrics.NumberOfObjects")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws.s3_request"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("aws.cloudwatch.namespace") == Some("AWS/Bedrock") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws_bedrock.runtime"))?;
                    Ok(())
                })();
            }

            let _cond =
                { event.get_str("aws.cloudwatch.namespace") == Some("AWS/Bedrock/Guardrails") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws_bedrock.guardrails"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("aws.cloudwatch.namespace") == Some("AWS/AmazonMQ")
                    && (event.has_value("aws.amazonmq.metrics.AmqpMaximumConnections")
                        || event.has_value("aws.amazonmq.metrics.MqttMaximumConnections")
                        || event.has_value("aws.amazonmq.metrics.OpenwireMaximumConnections")
                        || event.has_value("aws.amazonmq.metrics.StompMaximumConnections")
                        || event.has_value("aws.amazonmq.metrics.WsMaximumConnections")
                        || event.has_value("aws.amazonmq.metrics.CurrentConnectionsCount")
                        || event.has_value("aws.amazonmq.metrics.EstablishedConnectionsCount")
                        || event
                            .has_value("aws.amazonmq.metrics.InactiveDurableTopicSubscribersCount")
                        || event.has_value("aws.amazonmq.metrics.JournalFilesForFastRecovery")
                        || event.has_value("aws.amazonmq.metrics.JournalFilesForFullRecovery")
                        || event.has_value("aws.amazonmq.metrics.NetworkConnectorConnectionCount")
                        || event.has_value("aws.amazonmq.metrics.NetworkIn")
                        || event.has_value("aws.amazonmq.metrics.NetworkOut")
                        || event.has_value("aws.amazonmq.metrics.OpenTransactionCount")
                        || event.has_value("aws.amazonmq.metrics.TotalConsumerCount")
                        || event.has_value("aws.amazonmq.metrics.TotalMessageCount")
                        || event.has_value("aws.amazonmq.metrics.TotalProducerCount")
                        || event.has_value("aws.amazonmq.metrics.VolumeReadOps")
                        || event.has_value("aws.amazonmq.metrics.VolumeWriteOps")
                        || event.has_value("aws.amazonmq.metrics.ReceiveCount")
                        || event.has_value("aws.amazonmq.metrics.ProducerCount")
                        || event.has_value("aws.amazonmq.metrics.QueueSize")
                        || event.has_value("aws.amazonmq.metrics.TotalEnqueueCount")
                        || event.has_value("aws.amazonmq.metrics.TotalDequeueCount"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws_mq.activemq_metrics"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("aws.cloudwatch.namespace") == Some("AWS/AmazonMQ")
                    && (event.has_value("aws.amazonmq.metrics.ExchangeCount")
                        || event.has_value("aws.amazonmq.metrics.QueueCount")
                        || event.has_value("aws.amazonmq.metrics.ConnectionCount")
                        || event.has_value("aws.amazonmq.metrics.ChannelCount")
                        || event.has_value("aws.amazonmq.metrics.MessageCount")
                        || event.has_value("aws.amazonmq.metrics.MessageReadyCount")
                        || event.has_value("aws.amazonmq.metrics.MessageUnacknowledgedCount")
                        || event.has_value("aws.amazonmq.metrics.PublishRate")
                        || event.has_value("aws.amazonmq.metrics.ConfirmRate")
                        || event.has_value("aws.amazonmq.metrics.AckRate")
                        || event.has_value("aws.amazonmq.metrics.SystemCpuUtilization")
                        || event.has_value("aws.amazonmq.metrics.RabbitMQMemLimit")
                        || event.has_value("aws.amazonmq.metrics.RabbitMQMemUsed")
                        || event.has_value("aws.amazonmq.metrics.RabbitMQDiskFreeLimit")
                        || event.has_value("aws.amazonmq.metrics.RabbitMQFdUsed")
                        || event.has_value("aws.amazonmq.metrics.RabbitMQIOReadAverageTime")
                        || event.has_value("aws.amazonmq.metrics.RabbitMQIOWriteAverageTime"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("aws_mq.rabbitmq_metrics"))?;
                    Ok(())
                })();
            }

            // Painless script
            // Source: List metricNames = new ArrayList(); if (ctx.aws != null && ctx.aws instanceof Map) {\n    for (entry in ctx.aws.entrySet()) {\n        def nestedMap = entry.getValue();\n        if (nestedMap instanceof Map) { // Get the top-level key (firehose, cloudwatch, etc.)\n            def metricsMap = nestedMap.get(\"metrics\"); // Get aws.*.metrics.*\n            if (metricsMap instanceof Map) {\n                metricNames.addAll(metricsMap.keySet());\n                break;\n            }\n        }\n    }\n} Collections.sort(metricNames); ctx.aws.metrics_names = metricNames;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"List metricNames = new ArrayList(); if (ctx.aws != null && ctx.aws instanceof Map) {\n    for (entry in ctx.aws.entrySet()) {\n        def nestedMap = entry.getValue();\n        if (nestedMap instanceof Map) { // Get the top-level key (firehose, cloudwatch, etc.)\n            def metricsMap = nestedMap.get(\"metrics\"); // Get aws.*.metrics.*\n            if (metricsMap instanceof Map) {\n                metricNames.addAll(metricsMap.keySet());\n                break;\n            }\n        }\n    }\n} Collections.sort(metricNames); ctx.aws.metrics_names = metricNames;"#
                ),
            )?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("aws.metrics_names") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "aws.metrics_names_fingerprint",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            event.remove("aws.metrics_names");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
