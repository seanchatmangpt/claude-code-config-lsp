#!/usr/bin/env bash
# session-start.sh — idempotently register this project's LSP front door in
# .claude/settings.json's `lspServers` array, so a fresh Claude Code session
# picks it up automatically.
#
# Two registrations, in a fixed ORDER (first registration wins exclusive
# authority over its extensionToLanguage claims — order is load-bearing):
#
#   1. lsp-max-compositor (FIRST — exclusive authority over the routed set)
#      The compositor (sibling repo /Users/sac/lsp-max,
#      crates/lsp-max-compositor) is the front door: one stdio session fans
#      out per-extension to child servers and merges diagnostics. It reads
#      its routing manifest lsp-max.toml from the session cwd (this repo's
#      root has one: routes .json/.md → claude-code-config-lsp,
#      priority=diagnostics-only).
#   2. claude-code-config-lsp direct (SECOND — fallback)
#      Superseded by the compositor for the routed set whenever the
#      compositor binary exists. When the compositor binary is ABSENT, the
#      hook removes any stale compositor entry and the direct entry is the
#      sole authority. The direct entry is never deleted.
#
# This is a project-native adaptation of lsp-max 26.7.1's CC-006 concept
# (auto-registering an LSP server on SessionStart). See
# docs/reference/compositor-deployment.md and docs/v26.7.3-PRD-ARD.md.
#
# Requires: jq
set -euo pipefail

SERVER_NAME="claude-code-config-lsp"
SETTINGS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.claude"
SETTINGS_FILE="${SETTINGS_DIR}/settings.json"

# Compositor binary: release build of the sibling repo's compositor crate
# (cargo build --release -p lsp-max-compositor). Must be executable — an
# absent binary means the direct registration below stays sole authority.
COMPOSITOR_NAME="lsp-max-compositor"
COMPOSITOR_BIN="/Users/sac/lsp-max/target/release/lsp-max-compositor"

mkdir -p "${SETTINGS_DIR}"

if [ ! -f "${SETTINGS_FILE}" ]; then
    echo '{"lspServers":[]}' > "${SETTINGS_FILE}"
fi

if ! jq empty "${SETTINGS_FILE}" 2>/dev/null; then
    echo "session-start.sh: ${SETTINGS_FILE} is not valid JSON — refusing to modify it" >&2
    exit 1
fi

# Canonical extension→language maps. Language IDs MUST match
# src/file_types.rs (SUPPORTED_FILE_TYPES, generated from
# schema/claude-code-config.ttl) — file_types.rs is the authority.
CANONICAL_LANG_MAP='{
    ".json": "claude-config-json",
    ".md": "claude-config-markdown"
}'

# ── Registration 2 (direct) ────────────────────────────────────────────────
# Registered second: when the compositor entry is present it holds exclusive
# authority over .json/.md (first-wins), so this entry is the fallback used
# only when the compositor entry is absent.

already_registered=$(jq --arg name "${SERVER_NAME}" \
    '(.lspServers // []) | any(.name == $name)' \
    "${SETTINGS_FILE}")

if [ "${already_registered}" != "true" ]; then
    tmp_file="$(mktemp)"
    jq --arg name "${SERVER_NAME}" \
       '.lspServers = ((.lspServers // []) + [{
            name: $name,
            command: $name,
            args: ["serve", "--stdio"],
            extensionToLanguage: {
                ".json": "claude-config-json",
                ".md": "claude-config-markdown"
            }
        }])' \
       "${SETTINGS_FILE}" > "${tmp_file}"
    mv "${tmp_file}" "${SETTINGS_FILE}"
    echo "session-start.sh: registered ${SERVER_NAME} in ${SETTINGS_FILE}"
else
    # Repair an already-registered direct entry whose extensionToLanguage map
    # predates the language-ID reconciliation (file_types.rs is authority).
    current_map=$(jq --arg name "${SERVER_NAME}" -c \
        '(.lspServers // [])[] | select(.name == $name) | .extensionToLanguage // {}' \
        "${SETTINGS_FILE}")
    if [ "${current_map}" != "$(echo "${CANONICAL_LANG_MAP}" | jq -c .)" ]; then
        tmp_file="$(mktemp)"
        jq --arg name "${SERVER_NAME}" --argjson lang "${CANONICAL_LANG_MAP}" \
           '(.lspServers // []) |= map(if .name == $name then .extensionToLanguage = $lang else . end)' \
           "${SETTINGS_FILE}" > "${tmp_file}"
        mv "${tmp_file}" "${SETTINGS_FILE}"
        echo "session-start.sh: repaired stale extensionToLanguage for ${SERVER_NAME} in ${SETTINGS_FILE}"
    else
        echo "session-start.sh: ${SERVER_NAME} already registered in ${SETTINGS_FILE}"
    fi
fi

# ── Registration 1 (compositor — MUST occupy the first array position) ─────
# ORDER LAW: first registration wins exclusive authority over its claimed
# extensions. The compositor entry is therefore placed at index 0; the direct
# entry stays in the array as fallback but never receives the routed set
# while the compositor is first. If the compositor binary is not present on
# disk, any stale compositor entry is removed so the direct entry regains
# sole authority (settings never reference a missing binary).

if [ -x "${COMPOSITOR_BIN}" ]; then
    comp_present=$(jq --arg name "${COMPOSITOR_NAME}" \
        '(.lspServers // []) | any(.name == $name)' \
        "${SETTINGS_FILE}")
    comp_index=$(jq --arg name "${COMPOSITOR_NAME}" \
        '(.lspServers // []) | map(.name) | index($name) // -1' \
        "${SETTINGS_FILE}")
    comp_map=$(jq --arg name "${COMPOSITOR_NAME}" -c \
        '(.lspServers // [])[] | select(.name == $name) | .extensionToLanguage // {}' \
        "${SETTINGS_FILE}")

    if [ "${comp_present}" = "true" ] \
       && [ "${comp_index}" = "0" ] \
       && [ "${comp_map}" = "$(echo "${CANONICAL_LANG_MAP}" | jq -c .)" ]; then
        echo "session-start.sh: ${COMPOSITOR_NAME} already registered first in ${SETTINGS_FILE}"
    else
        tmp_file="$(mktemp)"
        jq --arg name "${COMPOSITOR_NAME}" --arg bin "${COMPOSITOR_BIN}" \
           --argjson lang "${CANONICAL_LANG_MAP}" \
           '(.lspServers // []) |= (
                map(select(.name != $name))
                + [{
                    name: $name,
                    command: $bin,
                    args: [],
                    extensionToLanguage: $lang
                }]
                | sort_by(if .name == $name then 0 else 1 end)
            )' \
           "${SETTINGS_FILE}" > "${tmp_file}"
        mv "${tmp_file}" "${SETTINGS_FILE}"
        echo "session-start.sh: registered/refreshed ${COMPOSITOR_NAME} as first entry (exclusive authority) in ${SETTINGS_FILE}"
    fi
else
    if jq --arg name "${COMPOSITOR_NAME}" \
          '(.lspServers // []) | any(.name == $name)' \
          "${SETTINGS_FILE}" | grep -q true; then
        tmp_file="$(mktemp)"
        jq --arg name "${COMPOSITOR_NAME}" \
           '(.lspServers // []) |= map(select(.name != $name))' \
           "${SETTINGS_FILE}" > "${tmp_file}"
        mv "${tmp_file}" "${SETTINGS_FILE}"
        echo "session-start.sh: compositor binary missing — removed ${COMPOSITOR_NAME} entry; ${SERVER_NAME} is sole authority"
    else
        echo "session-start.sh: compositor binary ${COMPOSITOR_BIN} not present — ${SERVER_NAME} direct entry remains sole authority"
    fi
fi
