# Quick Start

Run your first Assay test in 60 seconds.

---

## Prerequisites

- Assay installed ([installation guide](installation.md))
- An MCP session log (or use our example below)

---

# Quick Start

Initialize and run a protocol validation test.

## Prerequisites

- Assay installed ([installation guide](installation.md))
- A representative MCP session log (JSON-RPC trace)

---

## 1. Import Session Trail

Normalize a raw MCP session into a deterministic trace file. Use the `--init` flag to auto-generate a baseline policy configuration.

```bash
# Import from local file
assay import --format mcp-inspector session.json --init
```

**Artifacts Generated:**
- `traces/session.jsonl`: Normalized execution trail.
- `mcp-eval.yaml`: Test runner configuration.
- `policies/default.yaml`: Baseline schema constraints derived from the session.

## 2. Execute Validation

Run the replay engine against the generated policy.

```bash
assay run --config mcp-eval.yaml --strict
```

**Output:**
```
Assay v1.0.0

Suite: mcp-basics
Trace: traces/session.jsonl

┌───────────────────┬────────┬─────────────────────────┐
│ Metric            │ Status │ Details                 │
├───────────────────┼────────┼─────────────────────────┤
│ args_valid        │ ✅ PASS │ Schema compliance OK    │
│ sequence_valid    │ ✅ PASS │ Order invariant OK      │
│ tool_blocklist    │ ✅ PASS │ No blocked tools        │
└───────────────────┴────────┴─────────────────────────┘

Total: 2ms | 3 passed, 0 failed
Exit code: 0
```

## 3. Refine Constraint Policy

Edit `policies/default.yaml` to enforce stricter schema boundaries.

```yaml
# policies/default.yaml
tools:
  apply_discount:
    arguments:
      percent:
        type: number
        min: 0
        max: 30  # Constraint: Block values > 30
```

## 4. Verify Violation

Re-run the test with a trace containing an invalid value (e.g., 50).

```bash
assay run --config mcp-eval.yaml
```

**Output:**
```
❌ FAIL: args_valid

   Tool: apply_discount
   Argument: percent = 50
   Violation: Value exceeds maximum (max: 30)
```

## CI Integration

Add the validation step to your pipeline manifest.

```yaml
# .github/workflows/protocol-check.yml
- name: Protocol Validation
  run: assay run --config mcp-eval.yaml --strict --junit report.xml
```

This ensures strict protocol compliance for every commit.


---

## Next Steps

<div class="grid cards" markdown>

-   :material-file-document:{ .lg .middle } __Your First Test__

    ---

    Write a custom policy from scratch.

    [:octicons-arrow-right-24: First test](first-test.md)

-   :material-github:{ .lg .middle } __CI Integration__

    ---

    Add Assay to GitHub Actions, GitLab, or Azure.

    [:octicons-arrow-right-24: CI guide](ci-integration.md)

-   :material-shield-check:{ .lg .middle } __Sequence Rules__

    ---

    Enforce tool call order (e.g., "verify before delete").

    [:octicons-arrow-right-24: Sequences](../config/sequences.md)

-   :material-protocol:{ .lg .middle } __MCP Deep Dive__

    ---

    Advanced MCP integration patterns.

    [:octicons-arrow-right-24: MCP guide](../mcp/index.md)

</div>

---

## Troubleshooting

### "No trace file found"

Make sure you ran `assay import` first:

```bash
assay import --format mcp-inspector session.json --init
```

### "Config version mismatch"

Run the migration command:

```bash
assay migrate --config mcp-eval.yaml
```

### "Unknown tool in policy"

The tool name in your policy must match exactly what's in the trace. Check with:

```bash
assay inspect --trace traces/session.jsonl --tools
```

---

## Video Walkthrough

*Coming soon — 60-second demo: Import → Run → CI*

<!-- Uncomment when video is ready
<video controls width="100%">
  <source src="/assets/quickstart-demo.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>
-->
