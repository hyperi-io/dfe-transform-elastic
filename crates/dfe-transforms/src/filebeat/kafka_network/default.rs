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
            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event
                        .get_str("jolokia.metrics.mbean")
                        .is_some_and(|s| s.starts_with("kafka.network:type=Acceptor"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=Acceptor,name=AcceptorBlockedPercent,listener=%{WORD:listener}
                        if !cached_grok!("kafka.network:type=Acceptor,name=AcceptorBlockedPercent,listener=%{WORD:listener}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("listener") };
            if _cond {
                if let Some(v) = event.get("listener").cloned() {
                    event.set("jolokia.metrics.acceptor_blocked.listener", v)?;
                }
            }

            event.remove("listener");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("IdlePercent,networkProcessor=")),
                        serde_json::Value::String(s) => s.contains("IdlePercent,networkProcessor="),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=Processor,name=IdlePercent,networkProcessor=%{WORD:network_processor}
                        if !cached_grok!("kafka.network:type=Processor,name=IdlePercent,networkProcessor=%{WORD:network_processor}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("network_processor") };
            if _cond {
                if let Some(v) = event.get("network_processor").cloned() {
                    event.set(
                        "jolokia.metrics.processor_idle_percent.network_processor",
                        v,
                    )?;
                }
            }

            event.remove("network_processor");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("ResponseQueueSize"))
                        }
                        serde_json::Value::String(s) => s.contains("ResponseQueueSize"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("jolokia.metrics.mbean") {
                        if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                            // Grok pattern: kafka.network:type=RequestChannel,name=ResponseQueueSize,processor=%{WORD:processor}
                            if !cached_grok!("kafka.network:type=RequestChannel,name=ResponseQueueSize,processor=%{WORD:processor}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("processor") };
            if _cond {
                if let Some(v) = event.get("processor").cloned() {
                    event.set(
                        "jolokia.metrics.request_channel.response_queue_size.processor",
                        v,
                    )?;
                }
            }

            event.remove("processor");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("type=RequestMetrics"))
                        }
                        serde_json::Value::String(s) => s.contains("type=RequestMetrics"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=ErrorsPerSec,request=%{WORD:request_type},error=%{WORD:error_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=ErrorsPerSec,request=%{WORD:request_type},error=%{WORD:error_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("request_type") };
            if _cond {
                if let Some(v) = event.get("request_type").cloned() {
                    event.set("jolokia.metrics.request_metrics.request_type", v)?;
                }
            }

            let _cond = { event.has_value("error_type") };
            if _cond {
                if let Some(v) = event.get("error_type").cloned() {
                    event.set("jolokia.metrics.request_metrics.error_type", v)?;
                }
            }

            event.remove("request_type");

            event.remove("error_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=LocalTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=LocalTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=LocalTimeMs,request=%{WORD:local_request_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=LocalTimeMs,request=%{WORD:local_request_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("local_request_type") };
            if _cond {
                if let Some(v) = event.get("local_request_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.local_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=LocalTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=LocalTimeMs"),
                        _ => false,
                    })
                    && event.has_value("local_request_type")
                    && event.get_str("local_request_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("local_request_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=MessageConversionsTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=MessageConversionsTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=MessageConversionsTimeMs,request=%{WORD:message_conversions_request_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=MessageConversionsTimeMs,request=%{WORD:message_conversions_request_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("message_conversions_request_type") };
            if _cond {
                if let Some(v) = event.get("message_conversions_request_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.message_conversions_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=MessageConversionsTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=MessageConversionsTimeMs"),
                        _ => false,
                    })
                    && event.has_value("message_conversions_request_type")
                    && event.get_str("message_conversions_request_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("message_conversions_request_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=RemoteTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=RemoteTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=RemoteTimeMs,request=%{WORD:remote_request_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=RemoteTimeMs,request=%{WORD:remote_request_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("remote_request_type") };
            if _cond {
                if let Some(v) = event.get("remote_request_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.remote_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=RemoteTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=RemoteTimeMs"),
                        _ => false,
                    })
                    && event.has_value("remote_request_type")
                    && event.get_str("remote_request_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("remote_request_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=RequestBytes"))
                        }
                        serde_json::Value::String(s) => s.contains("name=RequestBytes"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=RequestBytes,request=%{WORD:request_bytes_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=RequestBytes,request=%{WORD:request_bytes_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("request_bytes_type") };
            if _cond {
                if let Some(v) = event.get("request_bytes_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.request_bytes.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=RequestBytes"))
                        }
                        serde_json::Value::String(s) => s.contains("name=RequestBytes"),
                        _ => false,
                    })
                    && event.has_value("request_bytes_type")
                    && event.get_str("request_bytes_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("request_bytes_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=RequestQueueTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=RequestQueueTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=RequestQueueTimeMs,request=%{WORD:request_queue_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=RequestQueueTimeMs,request=%{WORD:request_queue_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("request_queue_type") };
            if _cond {
                if let Some(v) = event.get("request_queue_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.request_queue_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=RequestQueueTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=RequestQueueTimeMs"),
                        _ => false,
                    })
                    && event.has_value("request_queue_type")
                    && event.get_str("request_queue_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("request_queue_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=ResponseQueueTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=ResponseQueueTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=ResponseQueueTimeMs,request=%{WORD:response_queue_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=ResponseQueueTimeMs,request=%{WORD:response_queue_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("response_queue_type") };
            if _cond {
                if let Some(v) = event.get("response_queue_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.response_queue_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=ResponseQueueTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=ResponseQueueTimeMs"),
                        _ => false,
                    })
                    && event.has_value("response_queue_type")
                    && event.get_str("response_queue_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("response_queue_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=ResponseSendTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=ResponseSendTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=ResponseSendTimeMs,request=%{WORD:response_send_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=ResponseSendTimeMs,request=%{WORD:response_send_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("response_send_type") };
            if _cond {
                if let Some(v) = event.get("response_send_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.response_send_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=ResponseSendTimeMs")),
                        serde_json::Value::String(s) => s.contains("name=ResponseSendTimeMs"),
                        _ => false,
                    })
                    && event.has_value("response_send_type")
                    && event.get_str("response_send_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("response_send_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=TemporaryMemoryBytes")),
                        serde_json::Value::String(s) => s.contains("name=TemporaryMemoryBytes"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=TemporaryMemoryBytes,request=%{WORD:temp_memory_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=TemporaryMemoryBytes,request=%{WORD:temp_memory_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("temp_memory_type") };
            if _cond {
                if let Some(v) = event.get("temp_memory_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.temporary_memory_bytes.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("name=TemporaryMemoryBytes")),
                        serde_json::Value::String(s) => s.contains("name=TemporaryMemoryBytes"),
                        _ => false,
                    })
                    && event.has_value("temp_memory_type")
                    && event.get_str("temp_memory_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("temp_memory_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=ThrottleTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=ThrottleTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=ThrottleTimeMs,request=%{WORD:throttle_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=ThrottleTimeMs,request=%{WORD:throttle_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("throttle_type") };
            if _cond {
                if let Some(v) = event.get("throttle_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.throttle_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=ThrottleTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=ThrottleTimeMs"),
                        _ => false,
                    })
                    && event.has_value("throttle_type")
                    && event.get_str("throttle_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("throttle_type");

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=TotalTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=TotalTimeMs"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.network:type=RequestMetrics,name=TotalTimeMs,request=%{WORD:total_time_type}
                        if !cached_grok!("kafka.network:type=RequestMetrics,name=TotalTimeMs,request=%{WORD:total_time_type}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("total_time_type") };
            if _cond {
                if let Some(v) = event.get("total_time_type").cloned() {
                    event.set(
                        "jolokia.metrics.request_metrics.total_time_ms.request_type",
                        v,
                    )?;
                }
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event.get("jolokia.metrics.mbean").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("name=TotalTimeMs"))
                        }
                        serde_json::Value::String(s) => s.contains("name=TotalTimeMs"),
                        _ => false,
                    })
                    && event.has_value("total_time_type")
                    && event.get_str("total_time_type") != Some("Produce")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("total_time_type");

            event.set("ecs.version", json!("8.11.0"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics") {
                    event.rename("jolokia.metrics", "kafka.network")?;
                }
                Ok(())
            })();

            event.set("event.kind", json!("metric"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("service.type", json!("kafka"))?;

            // Painless script
            // Source: def queue = new ArrayList();\ndef fingerprint = new ArrayList();\n\nif (ctx.containsKey('kafka') && ctx.kafka.containsKey('network')) {\n    queue.add(['p': 'kafka.network', 'v': ctx.kafka.network]);\n\n    while (!queue.isEmpty()) {\n        def item = queue.remove(0);\n        def path = item.p;\n        def val = item.v;\n\n        if (val instanceof Map) {\n            for (entry in val.entrySet()) {\n                def key = entry.getKey();\n                def child = entry.getValue();\n                def childPath = path + '.' + key;\n                queue.add(['p': childPath, 'v': child]);\n            }\n        } else {\n            fingerprint.add(path);\n        }\n    }\n\n    ctx.kafka_network_metric_fingerprint = fingerprint;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def queue = new ArrayList();\ndef fingerprint = new ArrayList();\n\nif (ctx.containsKey('kafka') && ctx.kafka.containsKey('network')) {\n    queue.add(['p': 'kafka.network', 'v': ctx.kafka.network]);\n\n    while (!queue.isEmpty()) {\n        def item = queue.remove(0);\n        def path = item.p;\n        def val = item.v;\n\n        if (val instanceof Map) {\n            for (entry in val.entrySet()) {\n                def key = entry.getKey();\n                def child = entry.getValue();\n                def childPath = path + '.' + key;\n                queue.add(['p': childPath, 'v': child]);\n            }\n        } else {\n            fingerprint.add(path);\n        }\n    }\n\n    ctx.kafka_network_metric_fingerprint = fingerprint;\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("kafka_network_metric_fingerprint") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "kafka_network_metric_fingerprint".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "kafka.network.metric_fingerprint",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
                }
                Ok(())
            })();

            event.remove("kafka_network_metric_fingerprint");

            event.remove("kafka.network.mbean");

            event.remove("jolokia.metrics");

            event.remove("kafka.mbean");

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
                        "Processor '{}' {} failed with message '{}'",
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
                                " with tag '{}' ",
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
