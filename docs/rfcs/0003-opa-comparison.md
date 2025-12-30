# RFC 0003: Assay DSL vs OPA/Rego

## 1. The Fundamental Difference

### OPA/Rego: Stateless Decisions
OPA evaluates single requests in isolation. It excels at:
*   Static Argument Validation
*   RBAC/Identity Checks
*   Infrastructure Policy (K8s)

OPA **cannot** natively handle sequential constraints (e.g., "Tool A must be called before Tool B") without external state management.

### Assay: Stateful Sequence Validation
Assay evaluates **traces** (sequences of calls). It excels at:
*   Temporal Ordering (`before`, `after`, `sequence`)
*   Session Rate Limiting (`max_calls`)
*   Workflow Invariants (`never_after`)

## 2. Hybrid Architecture (Recommended)

Use **OPA** for identity and **Assay** for behavior.

```
Request -> [OPA (Identity/Auth)] -> [Assay (Sequence/Safety)] -> Execution
```

## 3. Rego Export

Assay v1.1 supports exporting static constraints to Rego.

**Exported:** `allow`, `deny`, `require_args`, `arg_constraints`
**Not Exported:** `sequences`, `aliases`
