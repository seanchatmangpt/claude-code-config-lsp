# Original User Request

## Initial Request — 2026-07-03T18:24:33Z

Implement CLI commands in the `claude-code-config-lsp` Rust repository: a `conformance` command to output conformance scores/violations directly, a `receipt verify` / `receipt chain` command to validate the Merkle chain of config snapshots, and a `fix` command to automatically resolve repairable config violations.

Working directory: /Users/sac/claude-code-config-lsp
Integrity mode: development

## Requirements

### R1. Conformance Command
Add a `conformance` CLI command to `claude-code-config-lsp`.
- Syntax: `cargo run -- conformance [PATH]` (defaulting PATH to `.`).
- It must query the workspace conformance score and display a summary of violations.
- It must support the standard output options (e.g. `--format json` or `--format yaml` or `--format table`).

### R2. Receipt Chain Commands
Provide subcommands to verify and view the configuration Merkle receipt chain.
- Syntax: `cargo run -- receipt chain [PATH]` and `cargo run -- receipt verify [PATH]`.
- `receipt chain` must read and print the sequence of snapshots in the Merkle receipt chain.
- `receipt verify` must perform a full integrity check on the prior hashes and diagnostic checksums of the receipt chain, returning exit code 0 if valid and non-zero if tampering is detected.

### R3. Auto-Fix Command
Add a `fix` CLI command to repair basic, auto-fixable config issues.
- Syntax: `cargo run -- fix [PATH]`.
- By default, it must run in dry-run mode (outputting a diff/description of the proposed changes without writing to disk).
- It must accept an `--apply` or `-y` flag to write the fixes back to the files.

## Verification Mechanisms
Every command must have programmatic unit/integration tests added under `tests/` or in the relevant modules to verify:
1. `conformance` outputs correct score and structures matching LSP `claude-config://health`.
2. `receipt verify` succeeds on a valid chain and fails when files or receipts in the history are modified.
3. `fix` in dry-run mode does not modify files but shows changes, and `fix --apply` successfully modifies files.

## Acceptance Criteria

### CLI Commands Verification
- [ ] Running `cargo run -- conformance` successfully parses the current workspace and prints a conformance report.
- [ ] Running `cargo run -- receipt verify` validates the receipt chain without errors.
- [ ] Running `cargo run -- fix` lists proposed changes without editing target files.
- [ ] Running `cargo run -- fix --apply` updates the config files and resolves repairable diagnostics.
- [ ] All tests build and pass via `cargo test`.
- [ ] Code passes formatting and clippy (`cargo fmt --check` and `cargo clippy -- -D warnings`).
