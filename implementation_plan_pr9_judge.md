# Implementation Plan - PR9: Judge Metrics (LLM-as-a-Judge)

Based on [ADR-004 v2](docs/ADR-004-Judge-Metrics.md).

## User Review Required
> [!IMPORTANT]
> **Exit Code Policy**: We are strictly adhering to "No Exit Code 3". All "unstable" or "disagreement" states will result in `WARN` status (Exit 0) by default, or `FAIL` (Exit 1) under `--strict`.
> **Cache Invalidation**: This PR changes the cache key structure (adding temp/max_tokens/samples). Old caches will be effectively invalidated/ignored.

## Proposed Changes

### 1. CLI & Config (`verdict-cli`, `verdict-core`)
Implement new flags and environment variable precedence.

#### [MODIFY] [main.rs](file:///crates/verdict-cli/src/main.rs)
- Add `JudgeArgs` flattened struct to `RunArgs` and `CiArgs`.
- Flags: `--judge`, `--judge-model`, `--judge-samples`, `--judge-refresh`, `--judge-api-key`.
- implement `load_judge_config` with precedence (Flag > Env > Default).

#### [MODIFY] [model.rs](file:///crates/verdict-core/src/model.rs)
- Update `Expected` enum to support `faithfulness`, `relevance` (or generic mapping).
- Add `JudgeConfig` to `Settings` (runtime settings).

### 2. Core Engine & Providers
Implement the "Enrichment Pattern".

#### [MODIFY] [runner.rs](file:///crates/verdict-core/src/engine/runner.rs)
- Inject `JudgeService` into `Runner`.
- In `run_test`, if judge is enabled and metric requires it:
    - Call `judge.evaluate(prompt, response, context, criteria)`.
    - Inject result into `resp.meta.verdict.judge`.

#### [NEW] [judge.rs](file:///crates/verdict-core/src/providers/judge.rs)
- `JudgeService` struct.
- Handling of `openai` provider (using `async-openai` or generic client).
- Helper for "voting" (k=3 means 3 calls -> majority vote).

### 3. Caching
#### [MODIFY] [vcr.rs](file:///crates/verdict-core/src/cache/vcr.rs)
- Update `generate_key` to include:
    - `provider`, `model`
    - `rubric_id` (metric type)
    - `temperature`, `max_tokens`
    - `samples` (k)
    - `input_hash`

### 4. Metrics
#### [MODIFY] [judge.rs](file:///crates/verdict-metrics/src/judge.rs)
- Implement `FaithfulnessMetric`, `RelevanceMetric` (or generic `LlmJudgeMetric`).
- Logic: Read from `resp.meta.verdict.judge`.
- assert `passed` based on `min_score` (was `threshold`).

## Reference: Copy/Paste Ready Text

### CLI Help Text (clap attributes)
```rust
// --judge
help = "Enable or disable LLM-as-judge evaluation.
- none: judge calls disabled (replay/trace-only)
- openai: live judge calls via OpenAI
- fake: deterministic fake judge (tests/dev)"

// --judge-model
help = "Judge model identifier (provider-specific). Example: gpt-4o-mini"

// --judge-samples
help = "Number of judge samples per test (majority vote). Default: 3. Tip: for critical production gates consider: --judge-samples 5"

// --judge-refresh
help = "Ignore judge cache and re-run judge calls (live mode only)."

// --judge-temperature
help = "Temperature used for judge calls (affects cache key). Default: 0.0"

// --judge-max-tokens
help = "Max tokens for judge response (affects cache key). Default: 800"
```

### Error Messages
| Scenario | Exit | Message | Hint |
| :--- | :--- | :--- | :--- |
| **Missing API Key** | 2 | `config error: judge 'openai' requires OPENAI_API_KEY.` | `set OPENAI_API_KEY in your environment` |
| **Unknown Provider** | 2 | `config error: unknown judge provider '<VALUE>'.` | `valid values: none \| openai \| fake` |
| **Judge Disabled** | 2 | `config error: test '<TEST_ID>' requires judge results ... but judge is disabled.` | `options: 1) run live ... 2) run replay/CI offline ...` |
| **Timeout** | 1 | `error: judge call timed out after <SECONDS>s for test '<TEST_ID>'.` | `increase timeout via settings.timeout_seconds` |
| **Invalid Schema** | 2 | `config error: judge response for ... is invalid` | `try upgrading Verdict or re-running with --judge-refresh` |

### Warnings / Notes
*   **Disagreement**: `warning: judge samples disagreed for test '<TEST_ID>' (<PASS_COUNT>/<K> passed). marking as unstable (status=Warn).`
*   **Cache Hit**: `note: judge(faithfulness:v1) cache hit (test='<TEST_ID>')`

### Trace Schema (Meta)
```json
"meta": {
  "verdict": {
    "judge": {
      "faithfulness": {
        "rubric_version": "v1",
        "passed": true,
        "score": 0.92,
        "source": "cache", // or "live" or "trace"
        "samples": [true, true, false],
        "agreement": 0.67,
        "citations": [],
        "rationale": "Supported by chunk:2.",
        "cached_at": "2025-12-20T10:00:00Z"
      }
    }
  }
}
```

## Verification Plan

### Automated Tests
- Unit tests for `cache_key` generation (ensure sensitivity).
- Unit tests for `voting` logic (e.g. 2 True, 1 False -> True).
- Middleware tests for CLI flag precedence.

### Manual Verification
- Run `verdict run --judge fake` on `live-test.yaml`.
- Verify `run.json` contains enriched judge metadata.

## Future: PR11 Baseline Reference (Copy/Paste Ready)

### CLI Help Text (Baseline)
```rust
// --baseline
help = "Load a baseline.json and compare this run against it (relative thresholds). If a baseline entry is missing for a test, Verdict emits a warning by default."

// --export-baseline
help = "Export a baseline.json from this run (for main branch baseline publishing)."

// --require-baseline
help = "Fail (config error) if any test is missing a baseline entry. Note: --strict will also treat baseline-missing warnings as blocking."
```

### Baseline Messages
| Scenario | Message | Hint |
| :--- | :--- | :--- |
| **Missing Entry** | `warning: no baseline entry found for test '<TEST_ID>' (metric='<METRIC>'). warning: regression check skipped for this test.` | `hint: to create a baseline run: verdict ci --export-baseline baseline.json` |
| **Schema Mismatch** | `config error: unsupported baseline schema_version <X> (supported: <Y>).` | `hint: regenerate the baseline with: verdict ci --export-baseline baseline.json` |
