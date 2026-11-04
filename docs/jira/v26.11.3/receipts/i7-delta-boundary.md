# I7 — Delta-Encoding Boundary Fix (encode_abs continuation seed)

Lane: I7 (IMPLEMENTATION). Owned surfaces: `src/tokens_delta.rs`,
`tests/tokens_delta.rs` (new), this receipt. Disclosed edit in P8's
`tests/tokens_color.rs` (SLA, same-checkout-fanout): removed the four
synthetic delta LAW tests + `stok` helper + masking NOTE — replacements run
over real `build_tokens` output in `tests/tokens_delta.rs`. No git commands.

## Diagnosis

`tokens_delta::encode_abs` (src/tokens_delta.rs:55-73 pre-fix) seeded its
delta encoding at absolute `(line=0, char=0)` for every edit. LSP delta
semantics require an edit's `data` to CONTINUE the array's encoding stream
from the token immediately preceding the edit region in the previous array
(the client splices and decodes the full array as one continuous stream).
The (0,0) seed coincides with correct continuation only when the boundary
token sits at the origin — exactly what the synthetic `stok` fixtures in
tests/tokens_color.rs guaranteed (boundary token at (0,0)), which is how the
defect was masked.

I3's receipt (docs/jira/v26.11.3-witness pair `{"model": "opus"}` →
`{"model": "sonnet"}`) cited the failing pair as `[dl0, ds9]` emitted vs
`[dl0, ds8]` required. Re-measured on the current substrate (post-I3), the
real geometry is: prev data `[dl0, ds1, len7, tt3]`, `[dl0, ds9, len6, tt4]`;
next value token abs (0,10,8). Correct continuation from the boundary token
(abs col 1) is `[dl0, ds9]`; the (0,0) seed emitted `[dl0, ds10]` (absolute
column, off by the boundary token's column offset of 1). I3's ds8 figure was
computed against the pre-I3 0-token substrate and is off by one against real
geometry; the defect class is identical.

## Fix

- `encode_abs(tokens, seed_line, seed_ch)` — seed now passed explicitly; for
  an edit region starting at token index `p > 0`, seed =
  `prev_abs[p-1].(line, start_char)`; for `p == 0` the (0,0) origin seed is
  the legitimate continuation (no preceding token).
- `AbsToken` fields made `pub` so integration tests can assert continuation
  against decoded real arrays.
- `semantic_tokens_delta` computes the seed from `prev_abs[p-1]`.

## Tests

- tests/tokens_delta.rs (new, 5 tests, all over real `build_tokens` output):
  - `i3_witnessed_pair_continues_encoding_from_boundary_token` — pinned
    regression; expectation computed from the real decoded arrays, not a
    hand-copied constant.
  - `pretty_printed_one_key_edit_continues_from_same_line_boundary` — second
    I3 witness: origin seeding emitted `[dl1, ds11]`; continuation requires
    `[dl0, ds11]` (same-line boundary).
  - `edit_at_array_start_keeps_absolute_zero_seed` — `{"m": ...}` →
    `{"theme": ...}`; prefix trims to 0, origin seed legitimate, asserted.
  - `append_after_late_token_continues_from_last_cached_token` — the (0,0)
    seed also corrupted late appends' line deltas; now continues from the
    last cached token.
  - `real_edits_reconstruct_next_across_boundary_positions` — 5 real pairs
    incl. nested permissions allow-list and multi-line statusLine.
- tests/tokens_color.rs: removed the 4 synthetic delta LAW tests (`stok`
  fixtures with boundary tokens at the origin — the masking vectors),
  `stok` helper, and the stale substrate NOTE. Kept: substrate witness and
  `integration_delta_over_real_build_tokens_output_reconstructs_next`, both
  green against the fixed substrate (11 passed, 0 failed).

## Verification (ALIVE — commands + exits this session)

- `cargo test --test tokens_delta` → 5 passed, 0 failed.
- `cargo test --test tokens_color` → 11 passed, 0 failed (was 14 passed /
  1 failed pre-wave; the red law test is now green WITH the real substrate,
  synthetic masking gone).
- `cargo test --lib tokens_delta` → 7 passed, 0 failed (in-file witness).
- `cargo clippy --test tokens_delta --test tokens_color --lib` → finished,
  zero findings on owned surfaces.
- `cargo fmt` applied.
- Pre-fix emitted values (re-measured this session, real substrate):
  prev `[dl0, ds1, len7, tt3], [dl0, ds9, len6, tt4]`; next value token
  `[dl0, ds9, len8, tt4]` absolute (0,10). Bug emitted ds=10 (absolute);
  continuation requires ds=9 (relative to boundary col 1).

result: encode_abs boundary defect fixed — delta edit data now continues the
LSP encoding stream from the token preceding the edit region; synthetic
masking tests replaced with real build_tokens boundary tests; tokens_color
14→11 tests, all green (was 1 red), tokens_delta 5/0, lib 7/0.
