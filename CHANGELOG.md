# Changelog

All notable changes to this project will be documented in this file.


## [v1.0.0] - 2025-12-29
### Added
-   **Structured Logging**: `assay-core` now uses `tracing` for fail-safe events (`assay.failsafe.triggered`), enabling direct Datadog/OTLP integration.
-   **Agent Awareness**: `assay-mcp-server` now includes a `warning` field in the response when `on_error: allow` is active and an error occurs, allowing agents to self-regulate.
-   **Documentation**: Added "Look-behind Workarounds" to `docs/guides/migration-regex.md`.

## [v1.0.0-rc.2] - 2025-12-28

### 🚀 Release Candidate 2
Rapid-response release addressing critical Design Partner feedback regarding MCP protocol compliance and operational visibility.

### ✨ Features
- **Structured Fail-Safe Logging**: Introduced `assay.failsafe.triggered` JSON event when `on_error: allow` is active, enabling machine-readable audit trails.
- **Fail-Safe UX**: Logging now occurs via standard `stderr` to avoid polluting piping outputs.

### 🐛 Fixes
- **MCP Compliance**: `assay-mcp-server` tool results are now wrapped in standard `CallToolResult` structure (`{ content: [...], isError: bool }`), enabling clients to parse error details and agents to self-correct.


### 🚀 Release Candidate 1
First Release Candidate for Assay v1.0.0, introducing the "One Engine, Two Modes" guarantee and unified policy enforcement.

### ✨ Features
- **Unified Policy Engine**: Centralized validation logic (`assay-core::policy_engine`) shared between CLI, SDK, and MCP Server.
- **Fail-Safe Configuration**: New `on_error: block | allow` settings for graceful degradation.
- **Parity Test Suite**: New `tests/parity_batch_streaming.rs` ensuring identical behavior between batch and streaming modes.
- **False Positive Suite**: `tests/fp_suite.yaml` validation for legitimate business flows.
- **Latency Benchmarks**: confirmed core decision latency <0.1ms (p95).

### 🐛 Fixes
- Resolved schema validation discrepancies between local CLI and MCP calls.
- Fixed `sequence_valid` assertions to support regex-based policy matching.

## [v0.9.0] - 2025-12-27

### 🚀 Hardened & Release Ready

This release marks the transition to a hardened, production-grade CLI. It introduces strict contract guarantees, robust migration checks, and full CI support.

### ✨ Features
- **Official CI Template**: `.github/workflows/assay.yml` for drop-in GitHub Actions support.
- **Assay Check**: New `assay migrate --check` command to guard against unmigrated configs in CI.
- **CLI Contract**: Formalized exit codes:
  - `0`: Success / Clean
  - `1`: Test Failure
  - `2`: Configuration / Migration Error
- **Soak Tested**: Validated with >50 consecutive runs for 0-flake guarantee.
- **Strict Mode Config**: `configVersion: 1` removes top-level `policies` in favor of inline declarations.

### ⚠️ Breaking Changes
- **Configuration**: Top-level `policies` field is no longer supported in `configVersion: 1`. You must run `assay migrate` to update your config.
- **Fail-Fast**: `assay migrate` and `validate` now fail hard (Exit 2) on unknown standard fields.

### 🐛 Fixes
- Fixed "Silent Drop" issue where unknown YAML fields were ignored during parsing.
- Resolved argument expansion bug in test scripts on generic shells.

## [v0.8.0] - 2025-12-27
### Added
- Soak test hardening for legacy configs
- Unit tests for backward compatibility
- `EvalConfig::validate()` method

### Changed
- Prepared `configVersion: 1` logic (opt-in)
