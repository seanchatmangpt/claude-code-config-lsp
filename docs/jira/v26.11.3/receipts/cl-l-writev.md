# cl-l-writev — StreamWriter writev / UDS witness (lane cl-l, v26.11.3)

Standing: **ALIVE** — `cargo test -p cgap-arrow --test stream_writev_witness` exit 0,
3 passed / 0 failed, this session, against the exact subject on disk.

## Subject

- Repo `/Users/sac/cgap`, crate `cgap-arrow`, new file
  `crates/cgap-arrow/tests/stream_writev_witness.rs` (sole owned file in cgap).
- Receipt: `claude-code-config-lsp/docs/jira/v26.11.3/receipts/cl-l-writev.md` (this file).
- `src/**` untouched (verified: witness exercises only the landed pub API
  `cgap_arrow::ipc::{emit, read, seal_meta, window_len, MetaHeader, CONTINUATION, EOS, MAGIC}`
  + `cgap_arrow::{blake3, slab::ArrowRecordSlab}`).

## What closed (GA3's residual)

GA3 declared the StreamWriter writev handoff to a live UDS socket unverified —
framing bytes were court-pinned (ipc_courts C1/C1b/C2) but `emit`'s output had
never crossed a real socket. This lane landed the witness:

1. **W1** `w1_writev_handoff_through_live_uds`: bind a LIVE Unix domain socket in
   the host tempdir, connect in-process, emit the 4-row sealed slab into a
   64-alignment IPC1 window, scatter it into four wire segments (prefix /
   metadata / body / pad+EOS), hand off via real `write_vectored` (kernel
   writev), `read_exact` on the listener. Asserts: full-window write, byte-exact
   window, and every IPC1 field (continuation, meta_span=120, magic IPC1,
   version 1, body_len, alignment=64, body bytes, zero padding, EOS) pinned on
   the SOCKET-VISIBLE bytes.
2. **W2** `w2_received_window_decodes_via_read`: the received window decodes via
   the real `ipc::read` with the BLAKE3 schema digest back to the exact slab
   body — the socket path feeds a lawful consumer, not just a byte diff.
3. **W4** `w4_mutated_window_flips_the_byte_diff_court` (anti-vacuity): one body
   byte of the lane's own in-memory window mutated pre-handoff; the mutation
   survives the socket byte-exact (writev fidelity) and the byte-diff court
   FLIPS against the pristine slab body. A court that cannot flip is FAIL.

## Commands + exits

```text
cargo test -p cgap-arrow --test stream_writev_witness   # exit 0, 3 passed / 0 failed
cargo test -p cgap-arrow                                # exit 0, all targets:
#   lib: 139/0, bitmask_swar 3/0, ipc_courts 9/0, mutation_kills 36/0,
#   property 1/0, stream_writev_witness 3/0, validate_minimal 4/0, validation 6/0
```

Note: first run of the witness binary under the Bash tool's sandbox died
SIGKILL (AF_UNIX bind refused). Rerun unsandboxed → exit 0. Local-execution
fact, not a crate defect.

## Findings for the coordinator

1. **Body-integrity gap (scoped, by design at the framing layer, src untouched)**:
   `ipc::read` compares the caller's `expected_schema` against the header's
   `schema_digest` field but never re-hashes the body, so a body-corrupted
   window passes a digest-bearing read (`w4_mutated_window_flips_the_byte_diff_court`
   asserts this explicitly so a future body-integrity fix in `ipc.rs` flips it).
   Body integrity is the consumer's duty at the current contract. If GA3 wants
   body-hash verification in `read`, that is a src edit for a src-owning lane.
2. **No StreamWriter type exists in cgap-arrow.** The pub API is buffer-based
   (`emit`/`read` over caller buffers); there is no socket/stream writer
   abstraction to witness against. The witness therefore drives `emit` +
   real `write_vectored` in the test — the writev handoff itself is what is
   witnessed. If a real `StreamWriter` type is wanted in the crate, that is a
   src-owning lane's work.
3. First-attempt long-file Write corrupted (~1-in-5, per repo CLAUDE.md);
   re-assembled via python chunked write and grep-verified clean before compile.

## Verification ladder

Narrow (byte-level layout asserts on received bytes) → unit (decode via
`ipc::read` + slab identity) → integration (live UDS, real kernel writev, no
mocks). Real collaborators only; the W4 mutation is on the lane's own buffer,
never the shared tree; socket paths are unique per-run in the host tempdir and
removed at scope end.
