# RFC 0004: v1.1 Implementation Plan

## Phased Rollout

### v1.1.0: Core Value (Low Risk)
Focus on less brittle policies and better developer tooling.
- **Scope**: `eventually`, `max_calls`, `aliases`, `coverage metrics`
- **Defer**: `phases`, `arg_constraints` (complex coercion), `after/never_after` (complexity)

### v1.1.1: Ops & DX Polish
- `assay explain` (HTML)
- Runtime flags (`--timeout-ms`, `--max-bytes`)
- Health endpoints

### v1.1.2: Enterprise Features
- Audit JSONL
- Rego Export

## GitHub Issues (Ready for Import)

### [Blocker] Sequence DSL v2: Core Operators
- Implement `eventually(tool, within)`
- Implement `max_calls(tool, max)`
- Implement `tool_aliases`

### [Feature] Coverage Metrics
- Output `coverage.json`
- Flag `--min-coverage`

### [Feature] Assay Explain
- Terminal/Markdown output for trace debugging

### [Ops] Runtime Hardening
- Timeouts, memory limits, and health checks
