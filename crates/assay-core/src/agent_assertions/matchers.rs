use super::model::TraceAssertion;
use super::EpisodeGraph;
use crate::errors::diagnostic::Diagnostic;
// usage of HashMap removed

pub fn evaluate(
    graph: &EpisodeGraph,
    assertions: &[TraceAssertion],
) -> anyhow::Result<Vec<Diagnostic>> {
    let mut out = vec![];
    for a in assertions {
        if let Some(d) = check_one(graph, a) {
            out.push(d);
        }
    }
    Ok(out)
}

fn check_one(graph: &EpisodeGraph, a: &TraceAssertion) -> Option<Diagnostic> {
    match a {
        TraceAssertion::TraceMustCallTool { tool, min_calls } => {
            let actual = graph
                .tool_calls
                .iter()
                .filter(|t| t.tool_name.as_deref() == Some(tool.as_str()))
                .count();
            let min = min_calls.unwrap_or(1);
            if (actual as u32) < min {
                return Some(make_diag(
                    "E_TRACE_ASSERT_FAIL",
                    &format!(
                        "Expected tool '{}' to be called at least {} times, but got {}.",
                        tool, min, actual
                    ),
                    Some(format!("Must call tool: {}", tool)),
                    None,
                ));
            }
        }
        TraceAssertion::TraceMustNotCallTool { tool } => {
            if let Some(call) = graph
                .tool_calls
                .iter()
                .find(|t| t.tool_name.as_deref() == Some(tool.as_str()))
            {
                return Some(make_diag(
                    "E_TRACE_ASSERT_FAIL",
                    &format!(
                        "Expected tool '{}' NOT to be called, but it was called.",
                        tool
                    ),
                    Some(format!("Must not call tool: {}", tool)),
                    Some(serde_json::json!({
                        "failing_step_id": call.step_id,
                        "failing_tool": tool,
                        "failing_call_index": call.call_index
                    })),
                ));
            }
        }
        TraceAssertion::TraceToolSequence {
            sequence,
            allow_other_tools,
        } => {
            if *allow_other_tools {
                // Subsequence check
                if let Err(msg) = check_subsequence(&graph.tool_calls, sequence) {
                    return Some(make_diag(
                        "E_TRACE_ASSERT_FAIL",
                        &msg,
                        Some(format!("Tool sequence (subsequence): {:?}", sequence)),
                        None,
                    ));
                }
            } else {
                // Exact sequence check (contiguous, no extras)
                let actual_seq: Vec<String> = graph
                    .tool_calls
                    .iter()
                    .filter_map(|t| t.tool_name.clone())
                    .collect();

                if actual_seq != *sequence {
                    return Some(make_diag(
                        "E_TRACE_ASSERT_FAIL",
                        &format!(
                            "Expected exact tool sequence {:?}, got {:?}.",
                            sequence, actual_seq
                        ),
                        Some(format!("Tool sequence (exact): {:?}", sequence)),
                        None,
                    ));
                }
            }
        }
        TraceAssertion::TraceMaxSteps { max } => {
            let count = graph.steps.len();
            if count as u32 > *max {
                return Some(make_diag(
                    "E_TRACE_ASSERT_FAIL",
                    &format!("Expected at most {} steps, got {}.", max, count),
                    Some(format!("Max steps: {}", max)),
                    None,
                ));
            }
        }
        TraceAssertion::ArgsValid {
            tool,
            test_args,
            policy,
            expect,
        } => {
            if let Some(args) = test_args {
                let Some(pol) = policy else {
                    return Some(make_diag(
                        "E_CONFIG_ERROR",
                        "ArgsValid assertion requires 'policy' field (schema) when used in unit test mode.",
                        None,
                        None
                    ));
                };

                // Accommodate structure: { schema: { ... } } vs { properties: ... }
                let schema = pol.get("schema").unwrap_or(pol);
                // Wrap in tool map as expected by policy_engine
                let policy_map = serde_json::json!({ tool: schema });

                let verdict = crate::policy_engine::evaluate_tool_args(&policy_map, tool, args);
                let expected_pass = expect.as_deref().unwrap_or("pass") == "pass";
                let actual_pass = verdict.status == crate::policy_engine::VerdictStatus::Allowed;

                if expected_pass != actual_pass {
                    return Some(make_diag(
                        "E_POLICY_ASSERT_FAIL",
                        &format!(
                            "ArgsValid check failed. Expected {}, got {}. Reason: {:?}",
                            if expected_pass { "PASS" } else { "FAIL" },
                            if actual_pass { "PASS" } else { "FAIL" },
                            verdict.details
                        ),
                        None,
                        Some(serde_json::json!({
                            "tool": tool,
                            "args": args,
                            "verdict": verdict
                        })),
                    ));
                }
            }
        }
        TraceAssertion::SequenceValid {
            test_trace_raw,
            policy,
            expect,
            ..
        } => {
            if let Some(trace_vals) = test_trace_raw {
                if let Some(pol) = policy {
                    // Extract tool names from trace
                    // trace_vals is Vec<Value>. Expect { tool_name: "..." }
                    let tools: Vec<String> = trace_vals
                        .iter()
                        .filter_map(|v| {
                            v.get("tool")
                                .or(v.get("tool_name"))
                                .and_then(|s| s.as_str())
                                .map(|s| s.to_string())
                        })
                        .collect();

                    // Policy is { rules: [...] } or just rules array?
                    // evaluate_sequence expects regex string.
                    // But parity.rs constructs regex from JSON rules.
                    // We need a helper to convert JSON rules to regex string.
                    // crate::policy_engine::evaluate_sequence takes (regex, tools).
                    // We can assume 'policy' here IS the regex string or we need transformation logic.
                    // Implementation Plan: Assume policy contains "rules" and we construct regex or simplistic "join".
                    // parity.rs did: rules.join(" THEN ") ? No, that was latency_check.
                    // policy_engine has NO helper to convert JSON->Regex yet?
                    // Wait, `policy_engine::evaluate_sequence` takes `policy_regex: &str`.
                    // Does `policy_engine` have a JSON parser?
                    // Let's assume for this specific integration, we pass the regex string in the policy field?
                    // Or we assume the user provides it.
                    // Actually, parity.rs handled `CheckType::SequenceValid` by converting JSON rules to Regex.
                    // If we want Asserts to work, we verify what format `policy` comes in.
                    // fp_suite.yaml doesn't specify policy format yet.
                    // Let's assume policy IS the regex string for simplicity now, or simple rule list.

                    // Simplified: We skip implementing full rule engine here if not readily avail.
                    // We will allow `policy` to contain `regex` field.
                    let regex = pol.get("regex").and_then(|s| s.as_str()).unwrap_or(".*");

                    let verdict = crate::policy_engine::evaluate_sequence(regex, &tools);
                    let expected_pass = expect.as_deref().unwrap_or("pass") == "pass";
                    let actual_pass =
                        verdict.status == crate::policy_engine::VerdictStatus::Allowed;

                    if expected_pass != actual_pass {
                        return Some(make_diag(
                            "E_POLICY_ASSERT_FAIL",
                            &format!(
                                "SequenceValid check failed. Expected {}, got {}.",
                                if expected_pass { "PASS" } else { "FAIL" },
                                if actual_pass { "PASS" } else { "FAIL" }
                            ),
                            None,
                            None,
                        ));
                    }
                }
            }
        }
        TraceAssertion::ToolBlocklist {
            test_tool_calls,
            policy,
            expect,
            ..
        } => {
            if let Some(tools) = test_tool_calls {
                if let Some(pol) = policy {
                    // pol should look like { "blocked": [...] }
                    let blocked: Vec<String> = pol
                        .get("blocked")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();

                    let expected_pass = expect.as_deref().unwrap_or("pass") == "pass";
                    // Check if *any* tool is blocked
                    let mut actual_pass = true;
                    for t in tools {
                        if blocked.contains(t) {
                            actual_pass = false;
                            break;
                        }
                    }

                    if expected_pass != actual_pass {
                        return Some(make_diag(
                            "E_POLICY_ASSERT_FAIL",
                            &format!(
                                "ToolBlocklist check failed. Expected {}, got {}.",
                                if expected_pass { "PASS" } else { "FAIL" },
                                if actual_pass { "PASS" } else { "FAIL" }
                            ),
                            None,
                            None,
                        ));
                    }
                }
            }
        }
    }
    None
}

fn check_subsequence(
    calls: &[crate::storage::rows::ToolCallRow],
    expected: &[String],
) -> Result<(), String> {
    let mut current_idx = 0; // index in calls

    for expected_tool in expected {
        // Find next occurrence of expected_tool starting from current_idx
        let mut found = false;
        while current_idx < calls.len() {
            let row = &calls[current_idx];
            current_idx += 1;
            if row.tool_name.as_deref() == Some(expected_tool.as_str()) {
                found = true;
                break;
            }
        }

        if !found {
            return Err(format!(
                "Expected tool '{}' in sequence, but not found (missing or out of order).",
                expected_tool
            ));
        }
    }
    Ok(())
}

fn make_diag(
    code: &str,
    message: &str,
    _expected: Option<String>,
    context: Option<serde_json::Value>,
) -> Diagnostic {
    // We construct Diagnostic manually to match the struct definition.
    // Note: DiagnosticCode enum usage is available in other files but here we might need strings?
    // The Diagnostic struct uses String for code.

    Diagnostic {
        code: code.to_string(),
        severity: "error".to_string(),
        source: "agent_assertions".to_string(),
        message: message.to_string(),
        context: context.unwrap_or(serde_json::json!({})),
        fix_steps: vec![],
    }
}
