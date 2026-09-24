# Signing-key rotation (v26.9.24)

Recorded 2026-09-24 (fleet key scan after the single-repo migration). Base `900d5a9e7561` of `claude-code-config-lsp`.
Every private key listed here was committed to this repository and is therefore compromised: every receipt or
attestation signed with it carries no signing authority (standing REFUSED, broken_term R_missing_authority).
The keys leave the tree (history is not rewritten; no force-push), and each key directory's `.gitignore` now
covers both halves. Every checkout keeps its own pair: ggen generates one on first use, and a tracked public
half without its private half would make that first `ggen sync` refuse [FM-KEY-010/011]. The canonical
checkout's new public key is published below for anyone verifying its future receipts.

| key dir | removed private key sha256 | removed public key sha256 | new public key (canonical checkout) |
|---|---|---|---|
| `.ggen/keys` | `1e9652ba853de6f514dd48afcdda3d7f7fb456e2dfe1a9b4f25e57f78125d3c6` | `0a21a28bd73628b0e1dc9be4453880ce7bcca76540fbf2674a6fd02e3337977b` | `32bbee2d83c21f378b46d737b9ff212a15ae703255277991132ce79e7de39ac6` |
