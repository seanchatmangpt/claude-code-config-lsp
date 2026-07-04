# Project Plan: Implement CLI Commands in `claude-code-config-lsp`

This plan details the steps required to implement the three new CLI commands: `conformance`, `receipt`, and `fix`, along with verification and testing strategies.

## Architecture & Codebase Design
The project is an LSP server for Claude Code configuration.
We will add standard CLI commands to the existing Rust binary structure.
- **Binary CLI parsing**: We will integrate CLI arguments in `src/main.rs` (likely using `clap` or standard argument parsing).
- **Subcommands**:
  - `conformance [PATH] [--format json|yaml|table]`
  - `receipt chain [PATH]`
  - `receipt verify [PATH]`
  - `fix [PATH] [--apply|-y]`

## Milestones

### Milestone 1: codebase Exploration (Read-Only)
- Target: Understand existing CLI parsing, workspace scanning, conformance checking, receipt chain implementation, and auto-fix capabilities.
- Verification: Explorer handoff report documenting:
  - CLI parser location and implementation details.
  - Conformance calculation APIs.
  - Merkle receipt chain storage/structure and verification logic.
  - Auto-fix logic and diagnostic representation.

### Milestone 2: Implement Conformance CLI Command
- Target: Add `conformance` command.
  - Query workspace conformance score.
  - Print summary of violations.
  - Support format flag (`json`, `yaml`, `table`).
- Verification: Unit/integration test for conformance command, format matching `claude-config://health`.

### Milestone 3: Implement Receipt Chain Commands
- Target: Add `receipt chain` and `receipt verify` subcommands.
  - Read/print the sequence of snapshots in the Merkle receipt chain.
  - Perform integrity verification on hashes and checksums.
  - Return exit code 0 if valid, non-zero if invalid/tampered.
- Verification: Test suite with mock receipt chains (valid and tampered/modified).

### Milestone 4: Implement Auto-Fix Command
- Target: Add `fix` CLI command.
  - Dry-run mode by default (print diff/description).
  - Apply flag (`--apply` / `-y`) to modify files.
- Verification: Test auto-fix on repairable configuration diagnostics.

### Milestone 5: E2E and Quality Gates
- Target: Run full verification, including Clippy, rustfmt, and cargo tests.
- Verification: Clean runs of all checks.
