# Changelog

All notable changes to Assay will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-01-02

### Added

#### Policy DSL v2 - Temporal Constraints

New sequence operators for complex agent workflow validation:

- **`max_calls`** - Rate limiting per tool
  ```yaml
  sequences:
    - type: max_calls
      tool: FetchURL
      max: 10  # Deny on 11th call
  ```

- **`after`** - Post-condition enforcement
  ```yaml
  sequences:
    - type: after
      trigger: ModifyData
      then: AuditLog
      within: 3  # AuditLog must appear within 3 calls after ModifyData
  ```

- **`never_after`** - Forbidden sequences
  ```yaml
  sequences:
    - type: never_after
      trigger: Logout
      forbidden: AccessData  # Once logged out, cannot access data
  ```

- **`sequence`** - Exact ordering with strict mode
  ```yaml
  sequences:
    - type: sequence
      tools: [Authenticate, Authorize, Execute]
      strict: true  # Must be consecutive, no intervening calls
  ```

#### Aliases

Define tool groups for cleaner policies:

```yaml
aliases:
  Search:
    - SearchKnowledgeBase
    - SearchWeb
    - SearchDatabase

sequences:
  - type: eventually
    tool: Search  # Matches any alias member
    within: 5
```

#### Coverage Metrics

New `assay coverage` command for CI/CD integration:

```bash
# Check tool and rule coverage
assay coverage --policy policy.yaml --traces traces.jsonl --min-coverage 80

# Output formats: summary, json, markdown, github
assay coverage --policy policy.yaml --traces traces.jsonl --format github
```

Features:
- Tool coverage: which policy tools were exercised
- Rule coverage: which rules were triggered
- High-risk gaps: blocklisted tools never tested
- Unexpected tools: tools in traces but not in policy
- Exit codes: 0 (pass), 1 (fail), 2 (error)
- GitHub Actions annotations for PR feedback

#### GitHub Action

```yaml
- uses: assay-dev/assay-action@v1
  with:
    policy: policies/agent.yaml
    traces: traces/
    min-coverage: 80
```

#### One-liner Installation

```bash
curl -sSL https://assay.dev/install.sh | sh
```

### Changed

- Policy version bumped to `1.1`
- Improved error messages with actionable hints
- Better alias resolution performance

### Experimental

The following features are available but not yet stable:

- `assay explain` - Trace debugging and visualization (use `--experimental` flag)

### Migration from v1.0

v1.1 is fully backwards compatible with v1.0 policies. To use new features:

1. Update `version: "1.0"` to `version: "1.1"` in your policy files
2. Add `aliases` section if using tool groups
3. Add new sequence rules as needed

Existing v1.0 policies will continue to work without modification.

---

## [1.0.0] - 2025-12-30

### Added

- Initial stable release
- Policy DSL v1.0 with allow/deny lists
- Sequence rules: `require`, `eventually`, `before`, `blocklist`
- MCP server integration
- CLI: `assay check` command
- Deterministic, local-first evaluation

### Documentation

- Installation guide
- Policy reference
- MCP integration guide
- CI/CD examples

---

## [0.9.0] - 2025-12-28

### Added

- Pre-release candidate
- Rebrand from "Verdict" to "Assay"
- Migration tooling for existing users

---

[1.1.0]: https://github.com/assay-dev/assay/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/assay-dev/assay/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/assay-dev/assay/releases/tag/v0.9.0
