use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VerdictStatus {
    Allowed,
    Blocked,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Verdict {
    pub status: VerdictStatus,
    pub reason_code: String, // e.g., "OK", "E_ARG_SCHEMA", "E_TOOL_NOT_ALLOWED"
    pub details: Value,      // JSON details, violations, etc.
}

/// Evaluates tool arguments against a policy (JSON/YAML Value).
/// The policy is expected to be a map of tool_name -> schema.
pub fn evaluate_tool_args(policy: &Value, tool_name: &str, tool_args: &Value) -> Verdict {
    // 1. Check if tool exists in policy
    let schema_val = match policy.get(tool_name) {
        Some(s) => s,
        None => {
            // Implicit block if not in policy? Or allow?
            // "only validate if schema is present" vs "blocklist".
            // If we are strictly validating "Must match schema", missing schema = unknown tool = potentially unsafe.
            // But usually this means "no restrictions".
            // User requirement: "tool actions te valideren ... blocklist"
            // If policy engine is for "Validation", missing policy for a tool usually means "Unchecked" (Allowed) OR "Untrusted" (Blocked).
            // Given "prevent Excessive Agency", "allowed list" is better.
            // But let's assume if the policy is provided, it acts as an authority.
            // Helper `args_valid` metric assumed "if not in policy, skipped".
            // MCP check_args assumed "Tool not defined -> Error (E_POLICY_MISSING_TOOL)".
            // Let's stick to MCP strictness: If checking against a policy, the tool MUST be in it.
            return Verdict {
                status: VerdictStatus::Blocked,
                reason_code: "E_POLICY_MISSING_TOOL".to_string(),
                details: serde_json::json!({
                    "message": format!("Tool '{}' not defined in policy", tool_name)
                }),
            };
        }
    };

    // 2. Compile Schema
    // In a real high-perf scenario, we'd cache this (Compilation is expensive).
    // For this core function, we compile on the fly or need a cached compilation context.
    // User Step 1.2: "Compile JSON Schema validators één keer bij policy load".
    // Since this function takes `&Value`, it implies per-call.
    // To support caching, we'd need a `PolicyState` struct.
    // For now, I'll compile on the fly (parity correctness first).

    let compiled = match jsonschema::JSONSchema::compile(schema_val) {
        Ok(c) => c,
        Err(e) => {
            return Verdict {
                status: VerdictStatus::Blocked,
                reason_code: "E_SCHEMA_COMPILE".to_string(),
                details: serde_json::json!({
                    "message": format!("Invalid schema for tool '{}': {}", tool_name, e)
                }),
            };
        }
    };

    // 3. Validate
    evaluate_schema(&compiled, tool_args)
}

/// Evaluates tool arguments against a compiled schema.
pub fn evaluate_schema(compiled: &jsonschema::JSONSchema, tool_args: &Value) -> Verdict {
    let result = compiled.validate(tool_args);
    match result {
        Ok(_) => Verdict {
            status: VerdictStatus::Allowed,
            reason_code: "OK".to_string(),
            details: serde_json::json!({}),
        },
        Err(errors) => {
            let violations: Vec<Value> = errors
                .map(|e| {
                    serde_json::json!({
                        "path": e.instance_path.to_string(),
                        "constraint": e.to_string(),
                        "message": e.to_string()
                    })
                })
                .collect();
            Verdict {
                status: VerdictStatus::Blocked,
                reason_code: "E_ARG_SCHEMA".to_string(),
                details: serde_json::json!({
                    "violations": violations
                }),
            }
        }
    }
}

/// Evaluates a sequence of tool calls against a sequence policy (regex-like).
/// For v0.9, simplified: the policy is just a string (regex) of tool names.
/// E.g. "^search (analyze )*report$"
/// The input is a list of tool names invoked in order.
pub fn evaluate_sequence(policy_regex: &str, tool_names: &[String]) -> Verdict {
    // 1. Construct the sequence string
    // We join tool names with space. Note: tool names should not contain spaces ideally.
    // If they do, this simple approach might be ambiguous, but standard tools usually don't.
    let trace_str = tool_names.join(" ");

    // 2. Compile Regex
    // Again, efficiency concern: compile once.
    let re = match regex::Regex::new(policy_regex) {
        Ok(r) => r,
        Err(e) => {
            return Verdict {
                status: VerdictStatus::Blocked,
                reason_code: "E_POLICY_REGEX_INVALID".to_string(),
                details: serde_json::json!({
                    "message": format!("Invalid regex policy '{}': {}", policy_regex, e)
                }),
            };
        }
    };

    // 3. Match
    if re.is_match(&trace_str) {
        Verdict {
            status: VerdictStatus::Allowed,
            reason_code: "OK".to_string(),
            details: serde_json::json!({}),
        }
    } else {
        Verdict {
            status: VerdictStatus::Blocked,
            reason_code: "E_SEQUENCE_VIOLATION".to_string(),
            details: serde_json::json!({
                "expected": policy_regex,
                "found": trace_str
            }),
        }
    }
}
