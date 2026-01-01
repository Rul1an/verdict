# Release v1.0.0: The Protocol Validation Release

Assay v1.0.0 establishes the stable baseline for deterministic protocol validation of Model Context Protocol (MCP) clients.

## 🚀 Key Capabilities

### Deterministic Replay Engine
Eliminates non-deterministic factors (network I/O, model variance) from integration tests.
- **Latency**: P99 validation time < 2ms.
- **Throughput**: > 5,000 checks/sec per core.
- **Isolation**: 100% offline execution of recorded traces.

### Gateway Pattern (Runtime Enforcement)
Deploy `assay-mcp-server` as a sidecar to enforce policy boundaries in production traffic.
- **Fail-Safe Mode**: Configurable `on_error: allow` with structured "Protocol Feedback" warnings.
- **Observability**: Native tracing integration for Datadog/OTLP.

### Protocol Compliance
Full support for MCP v1.0 JSON-RPC payloads.
- **Schema Validation**: Strict JSON Schema enforcement for tool arguments.
- **Policy Engine**: Invariant checking for tool call sequences and blocklists.

## 📦 Distribution

### Rust (CLI)
```bash
cargo install assay-cli --locked
```
*   `assay-linux-x86_64.tar.gz` (Musl static)
*   `assay-macos-aarch64.tar.gz` (Apple Silicon)
*   `assay-macos-x86_64.tar.gz` (Intel)

### Python (SDK)
```bash
pip install assay-it
```

## 📋 Changelog

*   **feat(core)**: Re-architected replay engine for zero-io execution.
*   **feat(server)**: Added "Protocol Feedback" metadata to runtime responses.
*   **feat(telemetry)**: Structured logging for governance usage metering.
*   **polish**: Comprehensive documentation overhaul to "Industrial Specification" standard.
*   **fix**: Resolved look-around regex compatibility issues in schema validation.

**Full Changelog**: https://github.com/Rul1an/assay/compare/v0.9.0...v1.0.0
