<h1 align="center">
  <br>
  <img src="assets/logo.svg" alt="Assay Logo" width="200">
  <br>
  Assay
  <br>
</h1>

<p class="subtitle">Deterministic Integration Testing for MCP</p>

Assay is a specialized toolchain for ensuring strict protocol compliance in Model Context Protocol (MCP) implementations. It decouples testing from non-deterministic components (LLMs), enabling deterministic validation of tool execution and argument schemas.

---

## Operational Modes

Assay operates in two primary contexts:

<div class="grid cards" markdown>

-   :material-clock-fast:{ .lg .middle } __Offline Validation (CI)__

    ---

    Replays recorded JSON-RPC sessions against a policy set. Used to detect regression in tool usage patterns without invoking external APIs.

    [:octicons-code-24: CLI Reference](cli/index.md)

-   :material-server-network:{ .lg .middle } __Runtime Enforcement__

    ---

    Acts as a policy sidecar or gateway, intercepting tool calls to enforce safety constraints before execution.

    [:octicons-server-24: Gateway Pattern](guides/gateway-pattern.md)

</div>

## Key Metrics

| Metric | Specification |
|--------|---------------|
| **Latency** | < 1ms (P99) per evaluation |
| **Throughput** | > 10,000 checks/sec locally |
| **Determinism** | 100% (No network IO in replay) |
| **Protocol** | Model Context Protocol (v1.0) |

## Integration

### 1. Define Policies

Policies are defined in YAML and describe the valid state space for tool arguments and sequences.

```yaml
# policies/deployment_safety.yaml
- metric: args_valid
  tool: deploy_service
  constraints:
    replicas: { max: 3 }
    env: { regex: "^(dev|staging)$" }
```

### 2. Execute Validation

Use the CLI to validate recorded traces against these policies.

```bash
assay run --config mcp-eval.yaml --strict
```

---

## Component Architecture

*   **Policy Engine (`assay-core`)**: The stateless validation kernel.
*   **Replay Engine**: Ingests `session.json` (MCP Inspector format) and reconstructs the tool call sequence.
*   **MCP Server**: Exposes the key `check_args` and `check_sequence` tools via JSON-RPC.

[:octicons-arrow-right-24: View Crate Architecture](architecture/index.md)
