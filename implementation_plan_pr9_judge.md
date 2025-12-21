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

## Verification Plan

### Automated Tests
- Unit tests for `cache_key` generation (ensure sensitivity).
- Unit tests for `voting` logic (e.g. 2 True, 1 False -> True).
- Middleware tests for CLI flag precedence.

### Manual Verification
- Run `verdict run --judge fake` on `live-test.yaml`.
- Verify `run.json` contains enriched judge metadata.
