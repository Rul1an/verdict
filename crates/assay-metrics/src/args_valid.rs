use assay_core::metrics_api::{Metric, MetricResult};
use assay_core::model::{Expected, LlmResponse, TestCase, ToolCallRecord};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct ArgsValidMetric;

#[async_trait]
impl Metric for ArgsValidMetric {
    fn name(&self) -> &'static str {
        "args_valid"
    }

    async fn evaluate(
        &self,
        _tc: &TestCase,
        expected: &Expected,
        resp: &LlmResponse,
    ) -> anyhow::Result<MetricResult> {
        let (policy_path, inline_schema) = match expected {
            Expected::ArgsValid { policy, schema } => (policy, schema),
            _ => return Ok(MetricResult::pass(1.0)),
        };

        let schemas: HashMap<String, serde_json::Value> = if let Some(schema) = inline_schema {
            serde_json::from_value(schema.clone()).map_err(|e| {
                anyhow::anyhow!("config error: invalid inline args_valid schema: {}", e)
            })?
        } else if let Some(path) = policy_path {
            static WARN_ONCE: std::sync::Once = std::sync::Once::new();
            WARN_ONCE.call_once(|| {
                 if std::env::var("MCP_CONFIG_LEGACY").is_err() {
                     eprintln!("WARN: Deprecated policy file '{}' detected. Please migrate to inline usage.", path);
                     eprintln!("      To suppress this, set MCP_CONFIG_LEGACY=1 or run 'assay migrate'.");
                 }
             });

            let policy_content = std::fs::read_to_string(path).map_err(|e| {
                anyhow::anyhow!(
                    "config error: failed to read args_valid policy '{}': {}",
                    path,
                    e
                )
            })?;
            serde_yaml::from_str(&policy_content).map_err(|e| {
                anyhow::anyhow!("config error: invalid args_valid policy YAML: {}", e)
            })?
        } else {
            return Ok(MetricResult::pass(1.0));
        };

        let tool_calls: Vec<ToolCallRecord> = if let Some(val) = resp.meta.get("tool_calls") {
            serde_json::from_value(val.clone()).unwrap_or_default()
        } else {
            Vec::new() // No calls -> valid args (vacuously true)
        };

        let mut errors: Vec<serde_json::Value> = Vec::new();

        for call in tool_calls {
            let policy_val = serde_json::Value::Object(
                schemas
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            );

            let verdict = assay_core::policy_engine::evaluate_tool_args(
                &policy_val,
                &call.tool_name,
                &call.args,
            );

            if verdict.status == assay_core::policy_engine::VerdictStatus::Blocked {
                // For ArgsValid, we only care about schema violations (E_ARG_SCHEMA).
                // Missing tool (E_POLICY_MISSING_TOOL) is policy-dependent.
                // The old code did: "if tool not in policy -> skip".
                // policy_engine returns Blocked for missing tool.
                // So we need to match reason_code.
                if verdict.reason_code == "E_ARG_SCHEMA" {
                    if let Some(violations) =
                        verdict.details.get("violations").and_then(|v| v.as_array())
                    {
                        errors.extend(violations.clone());
                    }
                } else if verdict.reason_code == "E_POLICY_MISSING_TOOL" {
                    // Legacy behavior: ignore.
                } else {
                    // Other errors (e.g. compile fail)
                    errors.push(serde_json::json!({
                         "message": format!("Policy error for {}: {} ({})", call.tool_name, verdict.reason_code, verdict.details)
                     }));
                }
            }
        }

        if errors.is_empty() {
            Ok(MetricResult::pass(1.0))
        } else {
            let mut details = serde_json::Map::new();
            details.insert(
                "message".to_string(),
                serde_json::Value::String(format!("args_valid failed: {} errors", errors.len())),
            );
            details.insert("violations".to_string(), serde_json::Value::Array(errors));

            Ok(MetricResult {
                passed: false,
                score: 0.0,
                details: serde_json::Value::Object(details),
                unstable: false,
            })
        }
    }
}
