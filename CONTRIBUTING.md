# Contributing

This repository uses Conventional Commits for commit messages and pull request titles.

## Commit Convention

Use this format for commits:

```text
type(scope): subject
```

Examples:

```text
feat(move): add CLAMM whitelist validation
fix(types): correct account abstraction encoding
docs(readme): update build instructions
test(types): cover serialize edge cases
chore(deps): bump initia to latest
```

Rules:

- use lowercase `type` and `scope`
- keep the subject short and imperative
- do not end the subject with a period
- if the change is breaking, add `!` in the Conventional Commit prefix

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, `ci`

## Branch Naming

Format:

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

PR titles should follow the same Conventional Commit format.

Follow the PR template in [.github/PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md).
At minimum, every PR should clearly describe what changed, why it changed, how it was validated, and whether the change is breaking.

## Validation

Common commands:

- build: `make build`
- Rust tests: `cargo test --all`
- Go tests: `go test ./...`
- lint: `make lint`

## Formatting and Generated Files

For Rust code:
- `cargo fmt --all`

For Go code:
- `go fmt ./...`

## Scope Discipline

Keep each PR focused on one logical change. Avoid mixing unrelated refactors, formatting-only edits, and behavior changes unless they must land together.
