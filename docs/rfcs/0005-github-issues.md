# RFC 0005: GitHub Issues for v1.1

## Epic: Sequence DSL v2

**Label:** `epic`, `v1.1-blocker`, `dsl`

### Description

Implement the Assay Policy DSL v1.1 as specified in the RFC. This epic covers all temporal constraint operators and supporting infrastructure.

### Acceptance Criteria

- [ ] All v1.0 policies continue to work without modification
- [ ] New operators: `eventually`, `max_calls`, `after`, `never_after`
- [ ] Enhanced `sequence` operator with `strict` mode
- [ ] Alias resolution in all contexts
- [ ] JSON Schema for policy validation
- [ ] Migration command: `assay migrate --to 1.1`

---

## Issue 1: `eventually` Operator

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Implement the `eventually` temporal constraint operator.

```yaml
sequences:
  - type: eventually
    tool: SearchKnowledgeBase
    within: 3
```

### Acceptance Criteria

- [ ] Tool must be called within first `within` calls
- [ ] Failure detected at `within`th call or trace end
- [ ] Works with aliases
- [ ] Error message includes: rule_id, event_index, expected tool

---

## Issue 2: `max_calls` Operator

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Implement the `max_calls` rate limiting operator.

```yaml
sequences:
  - type: max_calls
    tool: ExternalAPI
    max: 3
```

### Acceptance Criteria

- [ ] Track call count per tool
- [ ] Deny on `max + 1` call
- [ ] Counter resets per trace
- [ ] Works with aliases

---

## Issue 3: `after` Operator

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Implement the `after` post-condition operator.

```yaml
sequences:
  - type: after
    trigger: CreateRecord
    then: AuditLog
    within: 2
```

### Acceptance Criteria

- [ ] After `trigger`, `then` must occur within `within` calls
- [ ] Multiple triggers reset the counter
- [ ] Trace end without `then` is failure
- [ ] Default `within: 1`

---

## Issue 4: `never_after` Operator

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Implement the `never_after` forbidden sequence operator.

```yaml
sequences:
  - type: never_after
    trigger: ArchiveRecord
    forbidden: DeleteRecord
```

### Acceptance Criteria

- [ ] Once `trigger` called, `forbidden` is permanently denied
- [ ] `forbidden` before `trigger` is allowed
- [ ] State is permanent (no reset)

---

## Issue 5: Enhanced `sequence` Operator

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Enhance the existing `sequence` operator with `strict` mode.

```yaml
sequences:
  - type: sequence
    tools: [Search, Analyze, Create]
    strict: false  # default
```

### Acceptance Criteria

- [ ] `strict: false` (default): other tools allowed between
- [ ] `strict: true`: no tools between sequence members
- [ ] Backwards compatible with v1.0

---

## Issue 6: Alias Resolution

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Implement tool alias resolution across all constraint types.

```yaml
aliases:
  Search:
    - SearchKnowledgeBase
    - SearchWeb
```

### Acceptance Criteria

- [ ] Aliases resolve in `tools.allow`, `tools.deny`
- [ ] Aliases resolve in all sequence operators
- [ ] Aliases are NOT recursive
- [ ] Original tool names remain valid
- [ ] Case-sensitive matching

---

## Issue 7: Argument Constraints

**Labels:** `v1.1-blocker`, `dsl`, `core`

### Description

Implement argument value validation.

```yaml
tools:
  arg_constraints:
    TransferMoney:
      amount:
        min: 1
        max: 10000
```

### Acceptance Criteria

- [ ] `min`/`max` for numeric values
- [ ] `enum` for allowed values
- [ ] `pattern` for regex validation (Rust `regex` crate)
- [ ] String-to-number coercion for min/max
- [ ] Missing argument with constraint = DENY

---

## Issue 8: JSON Schema Validation

**Labels:** `v1.1-blocker`, `dsl`, `dx`

### Description

Create JSON Schema for policy validation and IDE support.

### Acceptance Criteria

- [ ] Schema validates all DSL constructs
- [ ] Conditional validation for each sequence type
- [ ] Published at `https://assay.dev/schema/policy-v1.1.json`
- [ ] VS Code / JetBrains integration via `$schema`

---

## Issue 9: Error Message Structure

**Labels:** `v1.1-blocker`, `dsl`, `dx`

### Description

Implement structured error messages for all denial reasons.

```json
{
  "verdict": "deny",
  "rule_id": "search-first",
  "rule_type": "before",
  "event_index": 0,
  "tool": "CreateTicket",
  "reason": "..."
}
```

### Acceptance Criteria

- [ ] All denials include: verdict, rule_id, rule_type, event_index, tool, reason
- [ ] Optional context with rule-specific details
- [ ] Human-readable `reason` string
- [ ] Machine-parseable JSON structure

---

## Issue 10: Migration Command

**Labels:** `v1.1-blocker`, `dsl`, `cli`

### Description

Implement policy migration from v1.0 to v1.1.

```bash
assay migrate policy-v1.0.yaml --to 1.1 > policy-v1.1.yaml
```

### Acceptance Criteria

- [ ] `type: require` → `type: eventually` with appropriate `within`
- [ ] `type: blocklist` → `tools.deny`
- [ ] Preserve all other rules unchanged
- [ ] Warning for constructs that can't be migrated

---

## Issue 11: Rego Export

**Labels:** `v1.2`, `dsl`, `enterprise`

### Description

Export static constraints to OPA/Rego format.

### Acceptance Criteria

- [ ] Export `tools.allow` → Rego allow rules
- [ ] Export `tools.deny` → Rego deny rules
- [ ] Export `tools.require_args` → Rego argument checks
- [ ] Export `tools.arg_constraints` → Rego validation
- [ ] Warning comment for non-exportable temporal constraints

---

## Issue 12: Documentation

**Labels:** `v1.1-blocker`, `docs`

### Description

Comprehensive documentation for DSL v1.1.

### Deliverables

- [ ] `docs/dsl-reference.md` - Complete DSL reference
- [ ] `docs/dsl-migration.md` - v1.0 → v1.1 migration guide
- [ ] `docs/dsl-vs-opa.md` - Comparison with OPA/Rego
- [ ] `docs/dsl-examples.md` - Real-world examples
