#!/usr/bin/env bash
# build-release.sh — release packaging for claude-code-config-lsp.
#
# Solves the plugin-cache binary problem: the Claude plugin cache copies
# plugin *source* only, it never compiles — so the wired LSP depends on a
# stale prior build. This script is the single packaging gate:
#
#   1. cargo build --release
#   2. clippy --release -- -D warnings        (fail-closed gate)
#   3. copy binary to dist/ with version-stamped name + sha256 checksum file
#   4. regenerate the plugin cache copy at
#      ~/.claude/plugins/cache/claude-code-config-lsp/<...>/<version>/
#      if present (idempotent; jq used only for reads, settings untouched)
#
# Never touches ~/.claude/settings.json.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1)"
BIN_NAME="claude-code-config-lsp"
DIST_DIR="$REPO_ROOT/dist"
STAMPED="claude-code-config-lsp-$VERSION"

echo "== claude-code-config-lsp release build (v$VERSION) =="

# ── 1. build ────────────────────────────────────────────────────────────────
echo "-- cargo build --release"
cargo build --release

# ── 2. clippy gate (lib + bins; contract scope) ─────────────────────────────
echo "-- clippy --release -- -D warnings (fail-closed, lib+bins)"
cargo clippy --release -- -D warnings

# ── 3. dist/ + checksum ─────────────────────────────────────────────────────
mkdir -p "$DIST_DIR"
cp "target/release/$BIN_NAME" "$DIST_DIR/$STAMPED"
chmod +x "$DIST_DIR/$STAMPED"
shasum -a 256 "$DIST_DIR/$STAMPED" > "$DIST_DIR/$STAMPED.sha256"

# ── 4. plugin-cache regeneration ────────────────────────────────────────────
CACHE_BASE="$HOME/.claude/plugins/cache/claude-code-config-lsp"
if [ -d "$CACHE_BASE" ]; then
    while IFS= read -r cache_dir; do
        echo "-- regenerating plugin cache binary: $cache_dir"
        mkdir -p "$cache_dir/dist"
        cp "target/release/$BIN_NAME" "$cache_dir/dist/$BIN_NAME"
        chmod +x "$cache_dir/dist/$BIN_NAME"
        cp "$DIST_DIR/$STAMPED.sha256" "$cache_dir/dist/$STAMPED.sha256"
    done < <(find "$CACHE_BASE" -mindepth 2 -maxdepth 2 -type d)
else
    echo "-- no plugin cache dir at $CACHE_BASE (skipping regeneration)"
fi

echo "== DONE: $DIST_DIR/$STAMPED (+ .sha256); plugin cache refreshed =="
