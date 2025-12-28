# The Gateway Pattern: Enterprise Runtime Enforcement

This guide documents the reference architecture for the "Gateway Pattern" configuration, designed for high-stakes enterprise deployments requiring strict protocol validation.

## 1. Architecture Overview

The Gateway Pattern positions Assay as an authoritative, non-blocking "Decision Gateway" or sidecar in the runtime path.

```mermaid
graph TD
    User((Operator)) -->|Initiates Action| Client[MCP Client]

    subgraph "Policy Enforcement Layer"
        Client -->|1. Tool Call (JSON-RPC)| Assay{Assay Gateway}

        Assay -->|Policy Eval < 1ms| PolicyDB[(Ruleset)]

        Assay -- "❌ BLOCK (Violation)" --> Client
        Assay -- "✅ ALLOW" --> Backend[System of Record]
    end

    Client -->|2. Feedback Loop| User
    Assay -.->|3. Audit Trail| OTLP[Observability]

    style Assay fill:#00d97e,stroke:#333,stroke-width:2px,color:white
```

## 2. Configuration Strategy

For initial deployment, utilize a "Fail-Open with Warning" strategy to ensure business continuity while gathering telemetry.

### Fail-Safe Mode (`on_error: allow`)

Configure the MCP server to allow operations even if the policy engine experiences failure (e.g., config corruption), but explicitly warn the client.

**Client Request:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "approve_transaction",
    "arguments": {
      "amount": 500,
      "on_error": "allow",
      "policy": "finance_v1"
    }
  }
}
```

**System Response (on Internal Failure):**
```json
{
  "content": [],
  "isError": false,
  "warning": "FAIL-SAFE ACTIVE: Policy engine offline. Proceed with caution."
}
```

## 3. Telemetry & Accounting

Assay emits structured logs for both Operational Monitoring and Usage Accounting.

### Metered Usage Event
Ingest these logs to calculate governance usage volume.

```json
{
  "target": "assay_billing",
  "event": "assay.usage.metered",
  "usage_type": "policy_check",
  "count": 1
}
```

### Fail-Safe Alert
Trigger P1 alerts on this event id.

```json
{
  "event": "assay.failsafe.triggered",
  "error": "config_load_error",
  "fallback": "allow"
}
```
