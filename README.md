<h1 align="center">
  <br>
  <img src="docs/assets/logo.svg" alt="Assay Logo" width="200">
  <br>
  Assay
  <br>
</h1>

<h4 align="center">Deterministic Integration Testing for MCP</h4>

<p align="center">
  <a href="https://github.com/Rul1an/assay/actions/workflows/assay.yml">
    <img src="https://github.com/Rul1an/assay/actions/workflows/assay.yml/badge.svg" alt="CI Status">
  </a>
  <a href="https://crates.io/crates/assay">
    <img src="https://img.shields.io/crates/v/assay.svg" alt="Crates.io">
  </a>
  <a href="https://docs.assay.dev">
    <img src="https://img.shields.io/badge/docs-assay.dev-blue" alt="Documentation">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  </a>
</p>

---

## Overview

Assay is a local Testing & Policy Engine for the **Model Context Protocol (MCP)**. It validates JSON-RPC `call_tool` payloads against strict schema policies with sub-millisecond overhead.

Designed for high-compliance environments, Assay operates in two modes:
1.  **CI Validation**: Deterministic replay of recorded JSON-RPC sessions.
2.  **Runtime Gateway**: Sidecar proxy for enforcing protocol constraints in production.

## Core Capabilities

*   **Protocol Compliance**: Validates tool arguments against strict JSON Schemas.
*   **Policy Enforcement**: Blocks disallowed tool sequences and argument values.
*   **Deterministic Replay**: Re-runs recorded sessions without network I/O or model inference.
*   **Zero-Overhead**: P99 latency <1ms for policy evaluation.

## Installation

**CLI (Rust)**:
```bash
cargo install assay-cli --locked
```

**SDK (Python)**:
```bash
pip install assay-it
```

## Quick Start

### 1. Verification (CI)

Validate a recorded session against a policy configuration.

```bash
# 1. Initialize policy from a recorded session (e.g., via MCP Inspector)
assay import --format mcp-inspector session.json --init

# 2. Execute validation (Offline, <5ms)
assay run --config mcp-eval.yaml --strict
```

### 2. Enforcement (Runtime)

Run Assay as an MCP Server to validate tool calls before execution.

```bash
# Start the policy server
assay mcp-server --port 3001 --policy policies/
```

**Client Request:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "assay_check_args",
    "arguments": {
      "target_tool": "deploy_prod",
      "args": { "force": true }
    }
  }
}
```

**Server Response (Policy Violation):**
```json
{
  "content": [{ "type": "text", "text": "Policy violation: 'force' is not allowed in production." }],
  "isError": true
}
```

## Architecture

Assay is distributed as a Rust workspace:

| Crate | Function |
|-------|----------|
| `assay-core` | Policy evaluation engine and replay logic. |
| `assay-cli` | Command-line interface for CI integration. |
| `assay-mcp-server` | MCP-compliant server implementation for runtime hooks. |
| `assay-metrics` | Core validation logic (args, sequence, blocklist). |

## Documentation

Full technical documentation is available at [docs.assay.dev](https://docs.assay.dev).

*   [Configuration Schema](https://docs.assay.dev/config/)
*   [CLI Reference](https://docs.assay.dev/cli/)
*   [MCP Integration Guide](https://docs.assay.dev/mcp/)

## License

MIT License. See [LICENSE](LICENSE).
