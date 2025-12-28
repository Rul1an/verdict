# Migrating Regex Patterns

Assay's policy engine (v1.0+) is powered by Rust's `regex` crate. This offers guaranteed linear time execution `O(n)`, preventing "ReDoS" (Regular Expression Denial of Service) attacks common in Python/JS implementations.

However, this safety comes with a trade-off: **Look-around features are limited.**

## Key Differences

| Feature | Python `re` | Rust `regex` | Workaround |
| :--- | :--- | :--- | :--- |
| **Look-behind** | `(?<=...)` | ❌ Not Supported | Use capture groups |
| **Negative Look-behind** | `(?<!...)` | ❌ Not Supported | Match broader, filter later |
| **Look-ahead** | `(?=...)` | ❌ Not Supported | Match to end or capture |
| **Backreferences** | `\1` | ❌ Not Supported | Use named capture groups (mostly) |

## Common Migration Patterns

### 1. Extracting values (e.g., IBAN)

**Legacy (Python):** using negative lookahead `(?!...)` to exclude "9999" test range.
```regex
^DE\d{2}\s?(?!9999)\d{4}\s?\d{4}\s?\d{4}\s?\d{2}$
```
*Intent: Match valid IBANs but safeguard against test numbers.*

**Assay (Rust):** Flatten the logic. Rust `regex` doesn't support `(?!...)`.
```regex
^DE\d{2}\s?[0-8]\d{3}\s?\d{4}\s?\d{4}\s?\d{2}$
```
*Refactor: Use an allow-list logic (e.g. `[0-8]`) or handle exclusions in a separate blocklist policy.*

### 2. Password Validation (Look-aheads)

**Legacy:** Ensuring a digit exists anywhere.
```regex
(?=.*\d)
```

**Assay:** Use two separate rules.
*   Rule 1: `.*` (matches everything)
*   Rule 2 (Blocklist): Block if `^\D*$` (no digits).
*   *Alternatively in v1.0:* Use multiple simple regexes: `[0-9]` must match.

### 3. Look-behind Workarounds
Users encountering issues migrating patterns relying on `(?<=...)`:

**Legacy:** Match digits preceded by "DE".
```regex
(?<=DE)\d+
```

**Assay:** Use **Capture Groups**. Match the prefix, but only extract/validate the group.
```regex
DE(\d+)
```
*Tip: If you cannot modify the extraction logic, use a broader match and filter the result in a subsequent policy step.*

## Why this change?

Legacy regex engines use backtracking, which can be exponential `O(2^n)`. A malicious agent output string of 50 chars could hang your CI for minutes. Rust's `regex` is strictly linear, ensuring our **<0.1ms latency guarantee** even on massive traces.
