# What Assay Does (and Doesn't Do)

Assay is a **deterministic policy enforcement engine** for AI agents. This page clarifies our scope to help you understand when Assay is the right tool—and when you should look elsewhere.

## Core Principle

> **If it needs a classifier, we don't build it. We build gates.**

Assay enforces rules that can be evaluated with 100% determinism. No probability scores. No "maybe". Just Pass or Fail.

---

## ✅ In Scope: Tool-Call Enforcement

Assay validates **agent actions** (tool calls) against policies you define.

### Argument Validation (`args_valid`)

Enforce that tool arguments match a JSON Schema.

```yaml
assertions:
  - type: args_valid
    tool: ApplyDiscount
    schema:
      type: object
      properties:
        percent:
          type: number
          maximum: 30  # Block discounts > 30%
        reason:
          type: string
          minLength: 10
      required: [percent, reason]
```

**Use case:** Prevent agents from applying excessive discounts, sending malformed API requests, or passing invalid parameters.

### Sequence Enforcement (`sequence_valid`)

Ensure tools are called in the correct order.

```yaml
assertions:
  - type: sequence_valid
    rules:
      - type: require
        tool: VerifyIdentity
      - type: before
        first: VerifyIdentity
        then: DeleteAccount
```

**Use case:** Require verification before destructive actions. Enforce multi-step approval workflows.

### Tool Blocklists (`tool_blocklist`)

Prevent specific tools from being called.

```yaml
assertions:
  - type: tool_blocklist
    blocked:
      - DeleteDatabase
      - DropTable
      - ExecuteRawSQL
```

**Use case:** Hard blocks on dangerous operations. Defense in depth for agent sandboxing.

---

## ❌ Out of Scope: Classifier-Based Safety

The following capabilities are **explicitly out of scope** for Assay. They require probabilistic classifiers and introduce non-determinism.

| Capability | Why Not Assay | Alternative |
|------------|---------------|-------------|
| **Toxicity Detection** | Requires language model classifier | [Llama Guard](https://ai.meta.com/llama-guard/), [Perspective API](https://perspectiveapi.com/) |
| **Jailbreak Detection** | Arms race, adversarial by nature | Prompt gateways, [Rebuff](https://github.com/protectai/rebuff) |
| **Hallucination Detection** | Requires ground truth comparison | LLM-as-judge pipelines, RAG evaluation tools |
| **RAG Grounding** | Context-dependent, semantic matching | [RAGAS](https://github.com/explodinggradients/ragas), [TruLens](https://trulens.org/) |
| **Bias Detection** | Subjective, contested definitions | Academic research tools, human review |
| **PII Detection** | Pattern matching is incomplete | [Presidio](https://github.com/microsoft/presidio), cloud DLP APIs |

### Why This Boundary Matters

1. **Determinism:** Classifiers have variance. The same input can produce different outputs across runs. Assay guarantees: same trace + same policy = same result, always.

2. **Latency:** Classifier-based checks add 100ms-1000ms. Assay's pure-function checks run in <10ms p95.

3. **False Positives:** Classifiers trade off precision vs recall. Assay's rules are explicit—you control exactly what passes and fails.

4. **Auditability:** "The model said it was toxic" is not a compliance answer. "The discount exceeded 30% (schema violation)" is.

---

## The Integration Model

Assay is designed to **complement** classifier-based tools, not replace them.

```
┌─────────────────────────────────────────────────────────────┐
│                     YOUR AGENT RUNTIME                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │ Prompt      │    │   ASSAY     │    │  Response   │     │
│  │ Gateway     │───▶│  Preflight  │───▶│  Filter     │     │
│  │ (jailbreak) │    │ (tool args) │    │ (toxicity)  │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                                             │
│  Classifier-based   DETERMINISTIC     Classifier-based     │
│  ~200ms             <10ms p95         ~150ms               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Assay owns the middle:** the tool-call decision point where deterministic enforcement is both possible and critical.

---

## Decision Framework

Use this to decide if Assay is right for your check:

| Question | If Yes → Assay | If No → Other Tool |
|----------|----------------|-------------------|
| Can I express this as a JSON Schema? | ✅ `args_valid` | Use classifier |
| Can I express this as a tool sequence? | ✅ `sequence_valid` | Use workflow engine |
| Can I express this as a blocklist? | ✅ `tool_blocklist` | Use allowlist/RBAC |
| Does "maybe" ever make sense? | ❌ Not Assay | Use probabilistic check |
| Must the check be <10ms? | ✅ Assay | Async classifier OK |

---

## FAQ

### Can Assay detect prompt injection?

No. Prompt injection detection requires semantic understanding of adversarial inputs. Use a dedicated prompt gateway or input sanitization layer.

### Can Assay validate response quality?

No. Quality is subjective and requires LLM-as-judge or human evaluation. Assay validates *actions*, not *content*.

### Can Assay enforce rate limits?

No. Rate limiting is a runtime infrastructure concern. Use your API gateway or agent framework's built-in throttling.

### Can Assay replace my observability stack?

No. Assay produces audit events, but it's not a monitoring platform. Export Assay results to your existing observability tools (Datadog, Grafana, etc.) via the planned OTel integration.

---

## Summary

| Assay Is | Assay Is Not |
|----------|--------------|
| Deterministic | Probabilistic |
| Rule-based | Classifier-based |
| Tool-focused | Content-focused |
| Fast (<10ms) | Expensive (100ms+) |
| Auditable | "Trust the model" |

**Tagline:** If you can write it as a rule, Assay enforces it. If you need a model to decide, look elsewhere.
