# Changelog

All notable changes to this project will be documented in this file.

## [v0.9.0] - 2025-12-27

### 🚀 Hardened & Release Ready

This release marks the transition to a hardened, production-grade CLI. It introduces strict contract guarantees, robust migration checks, and full CI support.

### ✨ Features
- **Official CI Template**: `.github/workflows/assay.yml` for drop-in GitHub Actions support.
- **Assay Check**: New `assay migrate --check` command to guard against unmigrated configs in CI.
- **CLI Contract**: Formalized exit codes:
  - `0`: Success / Clean
  - `1`: Test Failure
  - `2`: Configuration / Migration Error
- **Soak Tested**: Validated with >50 consecutive runs for 0-flake guarantee.
- **Strict Mode Config**: `configVersion: 1` removes top-level `policies` in favor of inline declarations.

### ⚠️ Breaking Changes
- **Configuration**: Top-level `policies` field is no longer supported in `configVersion: 1`. You must run `assay migrate` to update your config.
- **Fail-Fast**: `assay migrate` and `validate` now fail hard (Exit 2) on unknown standard fields.

### 🐛 Fixes
- Fixed "Silent Drop" issue where unknown YAML fields were ignored during parsing.
- Resolved argument expansion bug in test scripts on generic shells.
