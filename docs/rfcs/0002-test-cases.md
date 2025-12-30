# RFC 0002: Assay Policy DSL v1.1 - Test Cases

This document defines the acceptance tests for the v1.1 DSL implementation.

## 1. Static Constraints

### Allowlist (ALLOW)
- `ALLOW-001`: Trace `[AllowedTool]` -> PASS
- `ALLOW-002`: Trace `[ForbiddenTool]` -> DENY
- `ALLOW-003`: Trace `[]` -> PASS

### Denylist (DENY)
- `DENY-001`: Trace `[SafeTool]` -> PASS
- `DENY-002`: Trace `[DeniedTool]` -> DENY
- `DENY-003`: Trace `[Safe, Denied]` -> DENY at index 1

### Required Args (ARGS)
- `ARGS-001`: Call with all required args -> PASS
- `ARGS-002`: Call missing required arg -> DENY

### Arg Constraints (CONST)
- `CONST-001`: Numeric value within min/max -> PASS
- `CONST-002`: Numeric value outside range -> DENY
- `CONST-003`: Enum value match -> PASS
- `CONST-004`: Regex pattern match -> PASS
- `CONST-005`: Regex pattern mismatch -> DENY

## 2. Temporal Constraints

### Eventually (EVEN)
- `EVEN-001`: Tool called at index 0 (within 3) -> PASS
- `EVEN-002`: Tool called at index 2 (within 3) -> PASS
- `EVEN-003`: Tool called at index 3 (within 3) -> DENY (exceeds limit if using 0-based index < 3, check implementation spec)
- `EVEN-004`: Tool never called -> DENY at end

### Max Calls (MAX)
- `MAX-001`: Calls <= max -> PASS
- `MAX-002`: Calls > max -> DENY on (max+1)th call

### Before (BEF)
- `BEF-001`: First then Second -> PASS
- `BEF-002`: Second without First -> DENY

### After (AFT)
- `AFT-001`: Trigger then Followup (within N) -> PASS
- `AFT-002`: Trigger then no Followup -> DENY at end
- `AFT-003`: Trigger then Followup (too late) -> DENY

### Never After (NEV)
- `NEV-001`: Forbidden then Trigger -> PASS
- `NEV-002`: Trigger then Forbidden -> DENY

### Sequence (SEQ)
- `SEQ-001`: Standard sequence (sparse) -> PASS
- `SEQ-002`: Standard sequence (wrong order) -> DENY
- `SEQ-003`: Strict sequence (interleaved) -> DENY
