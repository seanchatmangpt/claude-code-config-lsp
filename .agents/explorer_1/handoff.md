# Handoff Report — Explorer 1

## 1. Observation

Direct code observations in `/Users/sac/claude-code-config-lsp`:

- **CLI Argument Parsing (`src/main.rs` & `src/nouns/config.rs`)**:
  - In `src/main.rs` lines 10-21:
    ```rust
    fn main() -> clap_noun_verb::Result<()> {
        if std::env::args().nth(1).is_none() {
            let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
            rt.block_on(nouns::config::start_stdio_server());
            return Ok(());
        }
        clap_noun_verb::run()
    }
    ```
  - In `src/nouns/config.rs` lines 15-46, verbs are registered as functions marked with the attribute `#[verb("verb_name", "root")]`, such as:
    ```rust
    #[verb("scan", "root")]
    pub fn scan(#[arg(index = 1)] path: Option<String>) -> Result<ScanReport> { ... }
    ```

- **Workspace Conformance Score (`src/virtual_docs.rs`, `src/coverage.rs`)**:
  - In `src/virtual_docs.rs` lines 10-26, the `health` virtual document is rendered via `crate::coverage::coverage_report()`:
    ```rust
    pub fn render() -> String {
        render_with_coverage(&crate::coverage::coverage_report())
    }
    ```
  - `src/coverage.rs` defines `CoverageReport::coverage_percent()` which calculates implemented LSP method percentage, but there is no file or workspace conformance validation score module.
  - In `docs/v26.7.3-PRD-ARD.md` lines 125-133, a weighted conformance score model is listed as a planned refinement:
    ```
    #### 6. Conformance Scoring Refinement
    - **Current State**: Conformance score is (100 - 10*errors - 3*warnings); no weighting
    - **Goal**: Nuanced scoring that reflects severity and category
    ```

- **Receipt Chain (`praxis/src/lib.rs`, `src/backend.rs`)**:
  - In `praxis/src/lib.rs` lines 42-70, `AdmittedReceipt` and its blake3 signature function are defined:
    ```rust
    pub struct AdmittedReceipt {
        pub hash: String,
        pub timestamp: String,
        pub verdict: String,
        pub proof: String,
    }
    impl AdmittedReceipt {
        pub fn sign<T>(evidence: Evidence<T, Admitted, ()>) -> Self
        ...
        let hash = blake3::hash(evidence_json.as_bytes()).to_hex().to_string();
    ```
  - In `src/backend.rs` lines 255-270, the `claude-code-config/conformanceAudit` command executes `praxis_retrofit::audit_workspace()`, wraps the LSP conformance vector as evidence, and returns an `AdmittedReceipt` JSON.
  - No hashing verification or chain-linking functionality exists in the codebase today.

- **Diagnostics & Auto-Fixes (`src/scan.rs`, `src/conformance.rs`)**:
  - `src/scan.rs` line 68 dispatches diagnostics using `classify()` and specific analyzer modules in `src/analyzers/`.
  - In `src/conformance.rs` line 24, `textDocument/codeAction` is explicitly refused:
    ```rust
    LawAxis::Custom("textDocument/codeAction".to_string()),
    ```

---

## 2. Logic Chain

1. **CLI Commands Implementation**: Since the entrypoint routes to `clap_noun_verb::run()`, implementing the new commands (`conformance`, `receipt verify`, `receipt chain`, `fix`) requires adding corresponding `#[verb("...", "root")]` functions in `src/nouns/config.rs` (or similar modules under `nouns`).
2. **Conformance Score Implementation**: Since `src/virtual_docs.rs` currently renders the LSP method coverage markdown rather than the `WorkspaceConformance` JSON structure, the implementer needs to define a scoring structure mapping (surface, severity) to weights and update the `health` document rendering.
3. **Receipt Chain Commands Implementation**: Since the `AdmittedReceipt` is signed but not chained, the implementer must:
   - Extend the receipt metadata to track `previous_receipt_hash`.
   - Implement storage persistence (e.g. to `~/.claude/receipts/`).
   - Implement Merkle tree validation walking the hashes.
4. **Auto-Fixes**: Since there is no auto-fix execution module and the LSP refuses code actions, a mechanism for analyzing the spans returned by `RawFinding` and writing corrected content back to disk must be implemented for the `fix` command.

---

## 3. Caveats

- We did not evaluate the external binary dependencies (e.g., how the MCP server interacts with the filesystem or the exact behavior of `wasm4pm-compat`).
- We assume that the `clap-noun-verb` framework can serialize any structure returned by the verbs as long as it derives `serde::Serialize`.

---

## 4. Conclusion

The `claude-code-config-lsp` codebase has structured CLI argument parsing, file scanner classification, and evidence/receipt structs. However, the core logic for the requested CLI commands (`conformance`, `receipt`, `fix`) is unimplemented and must be built by linking scanner diagnostics, extending `AdmittedReceipt` with hash-chain links, and building file-modification algorithms for auto-fixes.

---

## 5. Verification Method

To verify the codebase setup and run checks:
1. Run `cargo test` in `/Users/sac/claude-code-config-lsp`.
2. Inspect the analysis output file `/Users/sac/claude-code-config-lsp/.agents/explorer_1/analysis.md` to review the structural mapping.
3. Verify that the LSP tests compile and execute within < 1 second.
