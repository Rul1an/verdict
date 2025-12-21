# Handoff: Verdict (Evals ML)

**Date:** 2025-12-21
**Status:** Stable / `main` verified
**Focus:** Semantic Similarity Precision & Interactive UX

## 1. Executive Summary
The **Semantic Similarity** metric (`semantic_similarity_to`) has been hardened for production robustness. We migrated the core math backend to `f64` to eliminate floating-point non-determinism and added an interactive CLI experience for live API keys. Codebase is clean ("swept") and regression-tested.

## 2. Technical State

### 2.1 Core Math & Precision
*   **Location**: `crates/verdict-core/src/embeddings/util.rs`
*   **Change**: Replaced `f32` cosine similarity with `cosine_similarity_f64`.
*   **Guardrail**: Added `EPSILON` (1e-6) in `crates/verdict-metrics/src/semantic.rs`.
    *   *Logic*: `score + EPSILON >= threshold` -> Pass.
    *   *Why*: Prevents flaky failures where score is `0.7999999` vs threshold `0.8`.

### 2.2 CLI & UX
*   **Location**: `crates/verdict-cli/src/main.rs`
*   **Feature**: Interactive `OPENAI_API_KEY` prompt.
    *   If key is missing from env, CLI prompts user via `stdin` (masked input not implemented, strictly standard stdin).
    *   Output artifacts: `run.json` is now generated for `run` commands (previously only `ci`), enabling inspection of exact scores in `details.score`.

### 2.3 Reporting
*   **Console**: Enhanced to print `message` for FAIL/ERRORS (e.g. "401 Unauthorized").

## 3. Architecture Overview (ML Focus)

| Crate | Purpose | Key Files for ML |
| :--- | :--- | :--- |
| `verdict-metrics` | Metric Implementations | `semantic.rs` (Embedding logic), `regex_match.rs` |
| `verdict-core` | Math, Providers, Engine | `embeddings/util.rs` (Math), `providers/embedder/` |
| `verdict-cli` | Entrypoint & Wiring | `main.rs` (`build_runner` wires the Embedder) |

## 4. Verification & Usage

### Live Test (OpenAI)
We created `live-test.yaml` for smoke testing.
```bash
# Requires text-embedding-3-small access
cargo run -- run \
  --config live-test.yaml \
  --embedder openai \
  --embedding-model text-embedding-3-small
```
*Expected Result*: `pass=1`. Reference text matched to "dummy" output to guarantee 1.0 similarity.

### Unit Tests
```bash
cargo test -p verdict-metrics
```
*   `test_boundary_pass`: Verifies exact match.
*   `test_boundary_epsilon_guard`: Verifies epsilon tolerance.

## 5. Known Cleanups (Codesweep)
*   Removed unused `run.json`, `junit.xml`, `.eval` DB from git tracking.
*   `cargo clippy` is clean.
*   `TODO`s removed (except intentional output strings).

## 6. Next Steps for ML Dev
1.  **Local Embeddings**: Integrate `Candle` or `ort` for local inference (remove OpenAI dependency).
2.  **Reranking Metric**: Add `rerank_score` metric using Cross-Encoders.
3.  **Hybrid Search**: Implement sparse + dense retrieval metrics.
