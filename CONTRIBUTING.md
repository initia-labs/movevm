# Contributing to Initia MoveVM

Thank you for your interest in contributing to Initia MoveVM.

## Prerequisites

- **Rust 1.86+** (install via [rustup](https://rustup.rs/))
- **Go 1.24+**
- **Cargo** (comes with rustup)

## Commit Convention

This repository uses [Conventional Commits](https://www.conventionalcommits.org/) for commit messages and pull request titles.

```text
type(scope): subject
```

Examples:

```text
feat(move): add CLAMM whitelist validation
fix(types): correct account abstraction encoding
docs(readme): update build instructions
test(storage): cover serialize edge cases
chore(deps): bump initia to latest
```

Rules:

- use lowercase `type` and `scope`
- keep the subject short and imperative
- do not end the subject with a period
- if the change is breaking, add `!` after the type/scope (e.g. `feat(api)!: remove legacy endpoint`)

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, `ci`

## Branch Naming

```text
type/short-description
```

Examples:

```text
fix/move-clamm-fee-swap
feat/cancel-proposal
docs/contributing-guide
test/serialize-edge-cases
```

## Pull Requests

- PR titles must follow the same Conventional Commit format.
- Follow the PR template in [.github/PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md).
- Every PR should clearly describe what changed, why it changed, how it was validated, and whether the change is breaking.
- Keep each PR focused on one logical change. Avoid mixing unrelated refactors, formatting-only edits, and behavior changes unless they must land together.

## Building

```bash
# Build everything (precompile + Rust + Go)
make build

# Build Rust libraries only (release)
make build-rust

# Build Go code only
make build-go

# Quick debug build for Rust (faster compile, larger binary)
make build-rust-debug
```

## Testing

```bash
# Run all tests (precompile + Rust + Go)
make test

# Rust tests only
make test-rust

# Individual Rust test targets
make test-compiler    # initia-move-compiler
make test-lib         # initia-move-vm
make test-movevm      # movevm FFI crate
make test-json        # initia-move-json
make test-storage     # initia-move-storage
make test-e2e         # e2e-move-tests
make test-unit        # Move stdlib unit tests

# Go tests only
make test-go

# Go tests with race detector and cgo safety checks
make test-safety
```

## Linting and Formatting

```bash
# Lint (clippy, warnings as errors)
make lint

# Format everything (Rust + Go + Move)
make fmt

# Format individually
make rust-fmt     # clippy --fix + cargo fmt
make go-fmt       # gofmt + goimports
make move-fmt     # movefmt (requires nightly toolchain)
```

## Code Generation

Some files are generated and must be kept in sync:

```bash
# Regenerate precompile contracts
make precompile

# Regenerate BCS Go bindings
make bcs-go-gen

# Update C header bindings after Rust changes
make update-bindings
```

Do not manually edit generated files. Re-run the appropriate generator and commit the result.
