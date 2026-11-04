# Receipt: cl-m-dogfood-fix — dogfood_compositor.mjs arg-parse + rebuild gate + de-stale BLOCKED path

- **Subject**: `/Users/sac/claude-code-config-lsp/scripts/dogfood_compositor.mjs`
- **Lane**: IMPLEMENTATION (M12/F4 named residuals)
- **Date**: 2026-11-03

## Diff (what changed)

1. **Arg-parse fix (the named residual)**: replaced the `args.indexOf`/twin-slot
   parsing with a proper ordered argv walk. `--allow CODE` is repeatable and
   refuses a missing value; `--fixtures JSON [MD]` consumes up to two
   positional values, stopping at the next flag (the old code's
   `args[indexOf("--fixtures") + 1]` re-read the JSON slot for the md fixture —
   `--fixtures A B` gave md=A); `--timeout MS` is now parsed at all (the old
   script accepted the flag in its usage string but never read it); `--compositor`/`--child`
   parsed in the same walk; unknown flags are a typed BLOCKED (exit 2).
2. **Rebuild gate**: before spawning, both binaries (compositor and
   config-lsp child) are checked for existence AND freshness (newest
   `.rs`/`Cargo.toml` mtime vs binary mtime, `src/` + manifest). Stale or
   missing → bounded release build with recorded duration:
   `cargo build --release -p lsp-max-compositor` in `/Users/sac/lsp-max`, and
   `cargo build --release` in the repo. Build failure = BLOCKED exit 2.
3. **De-staled BLOCKED text**: the old `reportBlocked` (exit 2) hardcoded the
   pre-F4 root cause ("notification dispatch is an unimplemented stub") as
   permanent text. Now a no-diagnostics timeout is a **FAIL (exit 1)** — F4's
   dispatch fix landed, so front-door silence is a regression, not an
   environment blocker — with regression language instead of stale
   root-cause prose.
4. **buf.slice sweep**: `dogfood_compositor.mjs` has zero `Buffer.slice`
   sites; the frame reader uses `subarray` throughout (3 sites). Remaining
   `.slice` matches are Array.slice (`process.argv.slice(2)`, wire-log cap) —
   not deprecated. `scripts/dogfood.mjs` still has 3 Buffer.slice sites;
   out of lane scope.

## Verification (real runs, exit codes)

Build state at run time: both binaries existed and were fresh; the gate
correctly took the no-build path and reported it (`Rebuild gate: compositor
fresh` / `child fresh`).

Contract invocation (exit 0):

```
$ node scripts/dogfood_compositor.mjs --allow CCC-JSON-001 --fixtures tests/fixtures/claude-config/settings.json
[compositor] lsp-max-compositor: 2 server(s) registered
  claude-code-config-lsp (diagnostics-only): primary=[".json", ".md"] secondary=[]
  justfile-lsp (diagnostics-only): primary=[".just"] secondary=[]
merge context: 2 andon prefix(es)
Compositor front door: /Users/sac/lsp-max/target/release/lsp-max-compositor
Child (via PATH):      /Users/sac/claude-code-config-lsp/target/release/claude-code-config-lsp
Rebuild gate: compositor fresh
Rebuild gate: child fresh
JSON fixture: tests/fixtures/claude-config/settings.json -> 1 diagnostic(s)
MD fixture:   tests/fixtures/claude-config/claude-global-CLAUDE.md -> 0 diagnostic(s)
Diagnostic codes observed through the compositor: CCC-JSON-001
CCC-* codes (child multiplexed at the wire level): CCC-JSON-001
  wire: <- (response) id=1
  wire: <- textDocument/publishDiagnostics
  wire: <- lspMax/admissionChanged
  wire: <- textDocument/publishDiagnostics
EXIT=0
```

Any-order + twin-fixture proof (the exact order that tripped the old parser):

```
$ node scripts/dogfood_compositor.mjs --fixtures settings.json claude-global-CLAUDE.md --timeout 15000 --allow CCC-JSON-001
Rebuild gate: compositor fresh / child fresh
JSON fixture: ... -> 1 diagnostic(s)
MD fixture:   ... -> 0 diagnostic(s)
CCC-* codes: CCC-JSON-001
EXIT=0
```

## Standing: ALIVE

Both invocations exit 0 with CCC-JSON-001 observed through the compositor
front door over the real wire; F4's notification dispatch is witnessed
working. No git commands run; `scripts/dogfood.mjs` untouched.

## Residuals (honest)

- The compositor emits a broken-pipe panic on shutdown
  (`failed printing to stdout: Broken pipe (os error 32)` on stderr) when the
  harness kills it after the exit notification. Harmless to the verdict
  (exit 0, all assertions before it), but it is a real compositor-side
  shutdown-path defect in `/Users/sac/lsp-max` — out of lane scope.
