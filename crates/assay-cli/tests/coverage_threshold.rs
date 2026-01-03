use assert_cmd::Command;

use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_coverage_min_threshold_failure() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("eval.yaml");
    let policy_path = dir.path().join("policy.yaml");
    let trace_path = dir.path().join("trace.jsonl");

    // Write external policy
    fs::write(
        &policy_path,
        r#"
version: "1"
name: threshold_policy
tools:
    allow: [ToolA, ToolB]
"#,
    )
    .unwrap();

    // Config references policy using 'sequence_valid' type
    fs::write(
        &config_path,
        format!(
            r#"
version: 1
suite: threshold_test
model: "dummy-model"
tests:
  - id: test1
    input: "dummy"
    expected:
      type: sequence_valid
      policy: "{}"
"#,
            policy_path.display()
        ),
    )
    .unwrap();

    // Trace only calls ToolA (50% coverage)
    fs::write(
        &trace_path,
        r#"
{"type": "call_tool", "tool": "ToolA", "test_id": "test1"}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("assay").unwrap();
    cmd.arg("coverage")
        .arg("--config")
        .arg(&config_path)
        .arg("--trace-file")
        .arg(&trace_path)
        .arg("--min-coverage")
        .arg("80") // Expect failure
        .assert()
        .failure()
        .stderr(contains("Coverage threshold not met"));
}

#[test]
fn test_coverage_min_threshold_success() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("eval.yaml");
    let policy_path = dir.path().join("policy.yaml");
    let trace_path = dir.path().join("trace.jsonl");

    fs::write(
        &policy_path,
        r#"
version: "1"
name: threshold_policy
tools:
    allow: [ToolA, ToolB]
"#,
    )
    .unwrap();

    fs::write(
        &config_path,
        format!(
            r#"
version: 1
suite: threshold_test
model: "dummy-model"
tests:
  - id: test1
    input: "dummy"
    expected:
      type: sequence_valid
      policy: "{}"
"#,
            policy_path.display()
        ),
    )
    .unwrap();

    // Trace calls both (100% coverage)
    fs::write(
        &trace_path,
        r#"
{"type": "call_tool", "tool": "ToolA", "test_id": "test1"}
{"type": "call_tool", "tool": "ToolB", "test_id": "test1"}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("assay").unwrap();
    cmd.arg("coverage")
        .arg("--config")
        .arg(&config_path)
        .arg("--trace-file")
        .arg(&trace_path)
        .arg("--min-coverage")
        .arg("80")
        .assert()
        .success();
}

#[test]
fn test_coverage_baseline_regression_failure() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("eval.yaml");
    let policy_path = dir.path().join("policy.yaml");
    let trace_full = dir.path().join("trace_full.jsonl");
    let trace_partial = dir.path().join("trace_partial.jsonl");
    let baseline_path = dir.path().join("baseline.json");

    fs::write(
        &policy_path,
        r#"
version: "1"
name: regression_policy
tools:
    allow: [ToolA, ToolB]
"#,
    )
    .unwrap();

    fs::write(
        &config_path,
        format!(
            r#"
version: 1
suite: regression_test
model: "dummy-model"
tests:
  - id: test1
    input: "dummy"
    expected:
      type: sequence_valid
      policy: "{}"
"#,
            policy_path.display()
        ),
    )
    .unwrap();

    // 1. Generate Baseline (100% coverage)
    fs::write(
        &trace_full,
        r#"
{"type": "call_tool", "tool": "ToolA", "test_id": "test1"}
{"type": "call_tool", "tool": "ToolB", "test_id": "test1"}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("assay").unwrap();
    cmd.arg("coverage")
        .arg("--config")
        .arg(&config_path)
        .arg("--trace-file")
        .arg(&trace_full)
        .arg("--export-baseline")
        .arg(&baseline_path)
        .assert()
        .success();

    // 2. Run with Partial Trace (50% coverage) + Baseline Check
    fs::write(
        &trace_partial,
        r#"
{"type": "call_tool", "tool": "ToolA", "test_id": "test1"}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("assay").unwrap();
    cmd.arg("coverage")
        .arg("--config")
        .arg(&config_path)
        .arg("--trace-file")
        .arg(&trace_partial)
        .arg("--baseline")
        .arg(&baseline_path)
        .assert()
        .failure()
        .stderr(contains("REGRESSION DETECTED"));
}

#[test]
fn test_coverage_high_risk_gap_failure() {
    let dir = TempDir::new().unwrap();
    let config_path = dir.path().join("eval.yaml");
    let policy_path = dir.path().join("policy.yaml");
    let trace_path = dir.path().join("trace.jsonl");

    fs::write(
        &policy_path,
        r#"
version: "1"
name: strict_policy
tools:
  allow: [SafeTool]
  deny: [CriticalTool]
"#,
    )
    .unwrap();

    fs::write(
        &config_path,
        format!(
            r#"
version: 1
suite: high_risk_test
model: "dummy-model"
tests:
  - id: test1
    input: "dummy"
    expected:
      type: sequence_valid
      policy: "{}"
"#,
            policy_path.display()
        ),
    )
    .unwrap();

    // Trace only safe tool -> CriticalTool is UNSEEN -> High Risk Gap
    fs::write(
        &trace_path,
        r#"
{"type": "call_tool", "tool": "SafeTool", "test_id": "test1"}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("assay").unwrap();
    cmd.arg("coverage")
        .arg("--config")
        .arg(&config_path)
        .arg("--trace-file")
        .arg(&trace_path)
        .assert()
        .failure()
        .stderr(contains("High Risk Gaps Detected"));
}
