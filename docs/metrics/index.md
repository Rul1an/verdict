# Assertion Types (V1)

In Assay v0.9.0+, behavioral checks are defined using **inline assertions** on the test case, rather than separate policy files.

These assertions map to underlying metrics but provide a cleaner, schema-validated syntax.

---

## Tool Assertions

### `trace_must_call_tool`
Passes if the trace contains at least one successful call to the specified tool.

```yaml
type: trace_must_call_tool
tool_name: "get_weather"
```

### `trace_no_tool_call`
Passes if the trace contains **zero** calls to the specified tool. Replaces the legacy `tool_blocklist` policy.

```yaml
type: trace_no_tool_call
tool_name: "delete_database"
```

### `trace_tool_args_match`
Passes if **every** call to the specified tool matches the provided argument values.

```yaml
type: trace_tool_args_match
tool_name: "apply_discount"
args:
  percent: 10
  code: "SUMMER"
```

### `trace_tool_args_schema`
Passes if **every** call to the tool matches the provided JSON Schema.

```yaml
type: trace_tool_args_schema
tool_name: "search"
schema:
  required: ["query"]
  properties:
    query: { type: "string", minLength: 3 }
```

### `trace_tool_call_count`
Passes if the tool call count is within the specified range.

```yaml
type: trace_tool_call_count
tool_name: "retry"
min: 1
max: 3
```

### `trace_no_tool_errors`
Passes only if the trace contains zero tool execution errors (e.g., exceptions raised by the tool).

```yaml
type: trace_no_tool_errors
```

---

## Sequence Assertions

### `trace_tool_sequence`
Enforces a strict order of tool calls. Other tools can be called in between, but the specified sequence must appear in that relative order.

```yaml
type: trace_tool_sequence
sequence: ["login", "view_balance", "logout"]
```

---

## Comparison Table

| V1 Assertion | Legacy V0 Policy |
|---|---|
| `trace_tool_args_match` | `args_valid` metric |
| `trace_tool_sequence` | `sequence_valid` metric |
| `trace_no_tool_call` | `tool_blocklist` metric |
