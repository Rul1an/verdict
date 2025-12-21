# Contributing to Verdict

Thank you for your interest in contributing to Verdict! We welcome contributions from the community.

## Local Development

### Prerequisites
- Rust (latest stable)
- Cargo

### Setup
1. Clone the repository everywhere
   ```bash
   git clone https://github.com/Rul1an/verdict.git
   cd verdict
   ```
2. Build
   ```bash
   cargo build
   ```
3. Test
   ```bash
   cargo test
   ```

### Running Demos
See `examples/README.md` for instructions on running the demo suites.

## Pull Requests

1. Fork the repo and create your branch from `main`.
2. Ensure `cargo fmt` and `cargo clippy` are clean.
3. Verify with `cargo test`.
4. Open a PR using the template provided.

## Release Process
(Maintainers only)
- Releases are triggered by pushing `v*` tags.
