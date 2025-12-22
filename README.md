# Verdict ⚖️

**Determinism at the speed of Rust.**
Local-first evaluation, regression gating, and active monitoring for AI Agents & RAG.

[![CI](https://github.com/Rul1an/verdict/actions/workflows/verdict-ci.yml/badge.svg)](https://github.com/Rul1an/verdict/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Why Verdict?

*   **⚡ Blazing Fast**: Native Rust binary (`verdict`). No Python runtime overhead for evaluation logic.
*   **🛡️ CI-Native**: strict regression gating with **deterministic replay**. Zero network calls in CI.
*   **🖥️ SOTA TUI**: High-fidelity terminal UI for real-time agent observation and attack simulation.
*   **🔍 OTel-Ready**: Seamlessly ingest OpenTelemetry spans from any language.

---

## 🚀 Quick Start

### 1. Install
```bash
# Build from source (Recommended for v0.4.0)
cargo install --path crates/verdict-cli
```

### 2. Run a Regression Gate
Compare a new trace against a known-good baseline with **zero flakiness**.
```bash
verdict ci \
  --config examples/ci-regression-gate/eval.yaml \
  --trace-file examples/ci-regression-gate/traces/main.jsonl \
  --baseline examples/ci-regression-gate/baseline.json \
  --strict
```

### 3. Experience the SOTA TUI
Run our flagship Agent Demo (`examples/agent-demo-2`) to see the TUI in action:
```bash
cd examples/agent-demo-2
uv pip install -r requirements.txt
python demo_tui.py
```

---

## 📂 Repository Structure

The repository is organized for clarity and separation of concerns:

*   **[`crates/`](./crates)**: The Rust Core.
    *   `verdict-core`: Business logic, SQLite storage, OTel ingestion.
    *   `verdict-cli`: The binary entrypoint.
    *   `verdict-metrics`: Shared metric definitions (Semantic, Regex, JSON Schema).
*   **[`examples/`](./examples)**: **Start Here.** Self-contained reference implementations.
    *   **[`agent-demo-2/`](./examples/agent-demo-2)**: 🌟 **SOTA Agent Demo**. Rich TUI, attack sim, live dashboard.
    *   **[`ci-regression-gate/`](./examples/ci-regression-gate)**: 🤖 **CI/CD Blueprints**. GitHub Actions workflows.
    *   `rag-grounding/`, `negation-safety/`: Focused metric examples.

---

## 🛠️ GitHub Actions Integration

Use **Verdict** directly in your CI pipelines to block regressions before they merge.

```yaml
# .github/workflows/ci.yml
jobs:
  verdict-gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Verdict
        run: cargo build --release
      - name: Run Gate
        env:
          VERDICT_BIN: ./target/release/verdict
        run: |
          $VERDICT_BIN ci --config eval.yaml --trace-file traces/pr.jsonl --baseline baseline.json --strict
```
*(See [`examples/ci-regression-gate`](./examples/ci-regression-gate) for the full production setup)*

---

## 📐 Philosophy (The "Senior Dev" Way)

1.  **Strictness over flakiness**: If a test isn't deterministic, it's a bug in the test, not the code.
2.  **Local-first**: Debug on your laptop with the exact same binary running in CI.
3.  **No fluff**: Minimal configuration, maximum type safety (Rust).

## 🤝 Contributing
We enforce **strict** code quality.
```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
