pub const CI_EVAL_YAML: &str = r#"version: 1
suite: "ci_smoke"
model: "trace"
tests:
  - id: "ci_smoke_regex"
    input:
      prompt: "ci_regex"
    expected:
      type: regex_match
      pattern: "Hello\\s+CI"
      flags: ["i"]
  - id: "ci_smoke_schema"
    input:
      prompt: "ci_schema"
    expected:
      type: json_schema
      json_schema: "{}"
      schema_file: "schemas/ci_answer.schema.json"
"#;

pub const CI_SCHEMA_JSON: &str = r#"{
  "type": "object",
  "required": ["answer"],
  "properties": {
    "answer": { "type": "string" }
  },
  "additionalProperties": false
}"#;

pub const CI_TRACES_JSONL: &str = r#"{"schema_version": 1, "type": "verdict.trace", "request_id": "ci_1", "prompt": "ci_regex", "response": "hello   ci", "model": "trace", "provider": "trace"}
{"schema_version": 1, "type": "verdict.trace", "request_id": "ci_2", "prompt": "ci_schema", "response": "{\"answer\":\"ok\"}", "model": "trace", "provider": "trace"}
"#;

pub const CI_WORKFLOW_YML: &str = r#"name: Verdict Gate
on: [push, pull_request]
jobs:
  verdict:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      # Note: In real usage, you would install verdict binary or use a container.
      # This example assumes verdict is available or you build it.
      - name: Run Verdict Smoke Test
        run: |
          # Example: download release or build
          # cargo install --git https://github.com/Rul1an/verdict.git verdict-cli
          # verdict ci --config ci-eval.yaml --trace-file traces/ci.jsonl --strict
          echo "Customize this workflow to install verdict!"
"#;

pub const GITIGNORE: &str = "/.eval/\n/out/\n*.db\n*.db-shm\n*.db-wal\n/verdict\n";
