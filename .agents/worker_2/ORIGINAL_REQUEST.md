## 2026-07-03T18:31:22-07:00
You are a codebase Worker. Your task is to implement the Conformance CLI Command (Milestone 2) according to the requirements and acceptance criteria in ORIGINAL_REQUEST.md.

**MANDATORY INTEGRITY WARNING**:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A Forensic Auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

### Task Details:
1. Define the `WorkspaceConformance` struct in `src/inventory.rs` or `src/conformance.rs`. It must contain:
   - `score`: f64 (between 0.0 and 1.0)
   - `surfaces_checked`: usize
   - `surfaces_admitted`: usize
   - `violations`: Vec<String> (strings of format `"{code}: {relative_path} {message}"`)
2. Implement a `conformance_report(root: &str) -> WorkspaceConformance` function in `src/inventory.rs`. It must:
   - Discover all config files under `root` (reusing `scan::discover`).
   - Run analyzers on each file (using `scan::analyze_document`).
   - For each finding, determine if it is an error or a warning (see `docs/reference.md` for severities, or map it using a helper function).
   - Calculate the conformance score using the formula `(100 - 10 * errors - 3 * warnings) / 100.0`, clamped to `[0.0, 1.0]`.
   - Track `surfaces_checked` (number of config files analyzed) and `surfaces_admitted` (number of config files with zero errors and warnings).
3. Register the `conformance` CLI command as a verb in `src/nouns/config.rs`:
   - Syntax: `cargo run -- conformance [PATH]` (defaulting PATH to `.`).
   - It must return the `WorkspaceConformance` struct (which derives `serde::Serialize`).
4. Update the LSP virtual document rendering for `claude-config://health` in `src/virtual_docs.rs` or `src/backend.rs` to return the JSON representation of `WorkspaceConformance` matching the workspace root (which can be `.` or the current directory `std::env::current_dir()`).
5. Write unit/integration tests (AAA pattern) in `tests/chicago_tdd_inventory.rs` or a new test file under `tests/` to verify that `conformance` outputs the correct score and structures matching the LSP `claude-config://health`.
6. Run `cargo test` and ensure all tests compile and pass.
7. Return a detailed handoff report in `handoff.md` inside your working directory.

Your working directory is: `/Users/sac/claude-code-config-lsp/.agents/worker_2`
