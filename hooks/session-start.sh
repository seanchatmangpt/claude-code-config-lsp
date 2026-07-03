#!/usr/bin/env bash
# session-start.sh — idempotently register this project's own
# claude-code-config-lsp binary in .claude/settings.json's `lspServers`
# array, so a fresh Claude Code session picks it up automatically.
#
# This is a project-native adaptation of lsp-max 26.7.1's CC-006 concept
# (auto-registering an LSP server on SessionStart). Unlike upstream CC-006,
# this script does NOT wire a compositor/fan-out endpoint — it registers
# this project's binary directly, since composition/fan-out is out of scope
# for claude-code-config-lsp. See docs/v26.7.3-PRD-ARD.md.
#
# Requires: jq
set -euo pipefail

SERVER_NAME="claude-code-config-lsp"
SETTINGS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.claude"
SETTINGS_FILE="${SETTINGS_DIR}/settings.json"

mkdir -p "${SETTINGS_DIR}"

if [ ! -f "${SETTINGS_FILE}" ]; then
    echo '{"lspServers":[]}' > "${SETTINGS_FILE}"
fi

if ! jq empty "${SETTINGS_FILE}" 2>/dev/null; then
    echo "session-start.sh: ${SETTINGS_FILE} is not valid JSON — refusing to modify it" >&2
    exit 1
fi

already_registered=$(jq --arg name "${SERVER_NAME}" \
    '(.lspServers // []) | any(.name == $name)' \
    "${SETTINGS_FILE}")

if [ "${already_registered}" = "true" ]; then
    echo "session-start.sh: ${SERVER_NAME} already registered in ${SETTINGS_FILE}"
    exit 0
fi

tmp_file="$(mktemp)"
jq --arg name "${SERVER_NAME}" \
   '.lspServers = ((.lspServers // []) + [{
        name: $name,
        command: $name,
        args: [],
        extensionToLanguage: {
            ".json": "claude-code-config",
            ".md": "claude-code-config"
        }
    }])' \
   "${SETTINGS_FILE}" > "${tmp_file}"
mv "${tmp_file}" "${SETTINGS_FILE}"

echo "session-start.sh: registered ${SERVER_NAME} in ${SETTINGS_FILE}"
