# M6 — Request Arbitration Tests

- **Status**: PARTIAL (ALIVE witnesses at the executable seams; loop-level execution BLOCKED)
- **Lane**: IMPLEMENTATION (m6)
- **Repo**: `/Users/sac/lsp-max` (canonical checkout)
- **Owned file**: `crates/lsp-max-compositor/tests/request_arbitration.rs` (new, only file touched)
- **Receipt**: `claude-code-config-lsp/docs/jira/v26.11.3/receipts/m6-arbitration.md`

## Subject

`crates/lsp-max-compositor` request arbitration: how hover/completion-style requests are
arbitrated across child LSP servers that claim the same document.

## Real semantics found (vs the contract's premises)

Read from `src/fanout.rs`, `src/server.rs`, and the client transport
(`lsp-max/src/client/builder.rs`, `src/client/server_handle.rs`), 2026-11-03:

1. **FirstSuccess is a sequential short-circuit, not a concurrent fan-out.**
   `CompositorServer::hover`/`completion` (`src/server.rs:269-323`) iterate Primary-tier
   candidates in tier order and `return` on the first `Ok(Some)`. Once child A succeeds,
   child B is never dispatched — suppression is structural (sequential `return`).
2. **Child errors flatten to null.** The client transport read loop
   (`lsp-max/src/client/builder.rs:144-151`) extracts only `result`
   (`msg.get("result").cloned().unwrap_or(Value::Null)`) and **drops the JSON-RPC `error`
   field**. A child answering `-32601` resolves the pending request with null →
   `Ok(None)` at the loop; the loop's `Err` branch is unreachable through this transport.
   A child answering a wrongly-shaped result is likewise flattened to `Ok(None)`
   (deserialization failure → `None`), witnessed by the first completion test run.
3. **All children fail → `Ok(None)`, not a typed failure.** The contract's item (3)
   ("typed failure surfaces to the client") is **false for hover/completion**: the loop
   folds every failure class (error → None, dead → None, null → None, undeserializable →
   None) into a single `Ok(None)` — the client sees a null result, no error object.
4. **Single-claimant methods = PrimaryOnly, tier-based not capability-based.** There is
   no per-child capability claiming: unlisted methods (incl. `workspace/executeCommand`)
   classify as `DispatchStrategy::PrimaryOnly` and deliver to ALL Primary-tier children
   registered on the doc's extension (Secondary/DiagnosticsOnly excluded).
5. **Crashed-child stall finding.** The first request after a child process dies pays the
   full 30 s pending timeout (`src/client/server_handle.rs:79`): the proxy write loop only
   observes the broken pipe when it attempts the write, and the pending entry is never
   resolved on that path. Subsequent requests fail fast via the mpsc send-failure path.
   Consequence: one crashed child can stall a FirstSuccess request up to 30 s even when a
   healthy sibling could have answered instantly. (Open improvement item, not fixed here —
   this lane owns tests only.)

## Tests landed (9, all passing)

`cargo test -p lsp-max-compositor --test request_arbitration` — exit 0,
9 passed / 0 failed in 30.01 s (the 30 s is the dead-child phase-1 witness, expected):

| # | Test | Witness |
|---|---|---|
| 1 | `both_children_claiming_doc_are_candidates_in_tier_order` | both Primary children are candidates, tier order |
| 2 | `strategy_classification_pins_request_arbitration` | hover/completion → FirstSuccess; executeCommand → PrimaryOnly |
| 3 | `first_child_success_is_the_arbitration_outcome` | first candidate's result wins, content-falsified (`hover-from-child-answer`) |
| 4 | `child_error_falls_through_to_next_child` | `-32601` flattens to `Ok(None)`; next child's success wins (fallback) |
| 5 | `dead_child_folds_to_none_first_request_slow_then_fast` | crashed `lsp-echo-server` subprocess: 30 s first hit, then fast send-fail |
| 6 | `all_children_failing_surfaces_null_result_not_typed_error` | all-fail shape is `Ok(None)`, no typed error |
| 7 | `second_child_receives_no_dispatch_after_first_success` | losing child's request counter stays 0 (see BLOCKED note) |
| 8 | `completion_first_success_same_as_hover` | completion shares hover's arbitration |
| 9 | `single_claimant_method_delivers_only_to_primary_tier` | PrimaryOnly delivers only to the Primary claimant |

## Honest-scoping note (PARTIAL, not ALIVE at loop level)

`CompositorServer` has private fields, no public constructor, and is only built inside
`run_stdio` via `LspService` — the arbitration loop itself (`for (id, handle) in
self.primary_handles(&uri) { … return Ok(Some(h)) … }`) is **not executable from an
integration test**. BLOCKED: `crates/lsp-max-compositor/src/server.rs` (private struct
fields; lane is forbidden from modifying compositor src). The tests therefore pin the two
seams the loop consumes, with real collaborators:

- candidate resolution (`fanout::servers_for_uri` + Primary filter — exactly what
  `CompositorServer::primary_handles` computes),
- per-child request semantics over the REAL `ClientBuilder` transport (LSP framing, id
  correlation, pending map, null-flattening) driven against hand-written in-process stub
  children. Stated Chicago exception: no binary in the repo answers hover/completion
  (`lsp-echo-server` replies `-32601` to everything), so the success-path stub is a simple
  hand-written fake, not a mock; the dead-child case uses the REAL `lsp-echo-server`
  subprocess (M4 harness pattern).

Loop-level aggregation (sequential `return` suppression of the second response, final
`Ok(None)` fold) is audited from `src/server.rs:269-323` + `src/fanout.rs:20-39` and is
4 lines of trivially-auditable code, but is asserted only at seam level, not executed.

## Commands / exits

```
cargo test -p lsp-max-compositor --test request_arbitration   # exit 0, 9 passed, 30.01 s
cargo test -p lsp-max-compositor --test request_arbitration dead_child  # exit 0
cargo clippy -p lsp-max-compositor --test request_arbitration # exit 0, no warnings
```

Session-introduced failures: none. One shared-compile freeze from another lane's in-flight
`src/client/builder.rs` edit (syntax error at :209) — resolved by the owning lane mid-run;
my lane never touched compositor/lsp-max src. The dead-child exit poll was widened 5s→15s
after a flake under parallel load.

## Standing

- Arbitration seam semantics: ADMITTED (9/9 real-execution witnesses).
- Loop-level FirstSuccess execution: BLOCKED (undrivable `CompositorServer` construction;
  would need a pub test constructor or an in-src unit test — out of lane scope).
- No mocks: transport, framing, id correlation, and the crashed-child subprocess are real;
  one stated-exception fake (in-process responder stub).
