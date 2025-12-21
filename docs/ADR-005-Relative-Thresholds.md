# ADR-005 v2: Relative Thresholds & Baselines

## Status
Accepted (v2)

## Context
Absolute thresholds cause friction ("why 0.85?"). Relative gating against a baseline ("don't regress from main") is the best default for teams adopting Verdict.

## Decision

### A. DX: Combined Baseline Workflow
**Decision**: Reduce command surface area.
*   **Generate Baseline**: `verdict ci --export-baseline baseline.json --strict`
*   **Gate with Baseline**: `verdict ci --baseline baseline.json --strict`
*   (Legacy): `verdict compare` remains as an advanced tool but not primary onboarding.

**Rationale**: Friction kills adoption; this 1-2 punch is the correct wedge.

### B. Config: Suite Defaults + Override
**Decision**: Allow centralized configuration in `settings`.

```yaml
settings:
  thresholding:
    mode: relative
    max_drop: 0.03
    min_floor: 0.80

tests:
  - id: "critical_rag"
    expected:
      type: semantic_similarity_to
      text: "..."
      # uses defaults

  - id: "experimental_feature"
    expected:
      type: semantic_similarity_to
      text: "..."
      thresholding:
        max_drop: 0.10
```
**Rationale**: 80% of tests do not need specific overrides.

### C. Missing Baseline Behavior
**Decision**: Warn by default, actionable message.
*   **Default**: `Warn` with clear instructions on how to generate baseline.
*   **Strict Mode**: `Warn` becomes `Fail` (exit 1), effectively making baseline mandatory if strict.
*   (Future): `--require-baseline` flag.

### D. Baseline JSON Schema & Compatibility
**Decision**: Strict schema versioning.
```json
{
  "schema_version": 1,
  "suite": "demo_suite",
  "verdict_version": "0.1.0",
  "entries": [...]
}
```
*   `schema_version` mismatch: **Config Error** (Exit 2).
*   `verdict_version` mismatch: **Warn**.

## Consequences
*   Baselines become "normal" workflow rather than advanced.
*   Configuration remains compact via defaults.
*   Compatibility issues are detected early.

## Roadmap
*   **PR10**: Relative Threshold Logic & Schema.
*   **PR11**: CLI integration (`--baseline`, `--export-baseline`).
*   **Deferred**: Statistical gating, auto-baseline updates.
