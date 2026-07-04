#!/usr/bin/env node
// Drives the real claude-code-config-lsp binary over the actual LSP wire
// protocol (Content-Length-framed JSON-RPC over stdio) against every config
// fixture in this repo's own .claude/ and .claude-plugin/ trees, and prints
// every diagnostic that comes back.
//
// This is the project's own dogfooding harness: it's not a unit test, it
// drives the compiled server exactly the way Claude Code's LSP client does.
//
// Usage:
//   cargo build
//   node scripts/dogfood.mjs [path-to-binary]
//
// Exit code is 0 if every fixture came back clean, 1 otherwise. Pass
// --allow <CODE> (repeatable) to permit specific diagnostic codes without
// failing the run (useful for fixtures that are intentionally invalid).
import { spawn, execSync } from "node:child_process";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { pathToFileURL } from "node:url";
import path from "node:path";

const repoRoot = execSync("git rev-parse --show-toplevel", { cwd: import.meta.dirname })
  .toString()
  .trim();

const args = process.argv.slice(2);
const allowed = new Set();
const positional = [];
for (let i = 0; i < args.length; i++) {
  if (args[i] === "--allow") {
    allowed.add(args[++i]);
  } else {
    positional.push(args[i]);
  }
}

const BIN = positional[0] || path.join(repoRoot, "target/debug/claude-code-config-lsp");
if (!existsSync(BIN)) {
  console.error(`Binary not found at ${BIN} — run \`cargo build\` first.`);
  process.exit(1);
}

// Discover every real config fixture in the repo's own config trees.
function walk(dir, pred, out = []) {
  if (!existsSync(dir)) return out;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === "worktrees") continue; // stray git-worktree copies, not real fixtures
    const p = path.join(dir, entry.name);
    if (entry.isDirectory()) walk(p, pred, out);
    else if (pred(p)) out.push(p);
  }
  return out;
}

const isConfigFile = (p) =>
  /settings.*\.json$|\.mcp\.json$|SKILL\.md$|\.claude\/agents\/.*\.md$|plugin\.json$|marketplace\.json$|lsp-max-auto\.toml$/.test(p);

const fixtures = [
  ...walk(path.join(repoRoot, ".claude"), isConfigFile),
  ...walk(path.join(repoRoot, ".claude-plugin"), isConfigFile),
  path.join(repoRoot, ".mcp.json"),
].filter(existsSync).sort();

console.log(`Discovered ${fixtures.length} fixtures:`);
for (const f of fixtures) console.log(`  ${path.relative(repoRoot, f)}`);
console.log();

const child = spawn(BIN, ["serve", "--stdio"], { stdio: ["pipe", "pipe", "pipe"] });
let buf = Buffer.alloc(0);
const diagnosticsByUri = {};

child.stderr.on("data", (d) => process.stderr.write(`[stderr] ${d}`));

function send(msg) {
  const json = JSON.stringify(msg);
  const header = `Content-Length: ${Buffer.byteLength(json, "utf8")}\r\n\r\n`;
  child.stdin.write(header + json);
}

child.stdout.on("data", (chunk) => {
  buf = Buffer.concat([buf, chunk]);
  while (true) {
    const headerEnd = buf.indexOf("\r\n\r\n");
    if (headerEnd === -1) return;
    const header = buf.slice(0, headerEnd).toString("utf8");
    const m = /Content-Length: (\d+)/.exec(header);
    if (!m) return;
    const len = parseInt(m[1], 10);
    const bodyStart = headerEnd + 4;
    if (buf.length < bodyStart + len) return;
    const body = buf.slice(bodyStart, bodyStart + len).toString("utf8");
    buf = buf.slice(bodyStart + len);
    const msg = JSON.parse(body);
    if (msg.method === "textDocument/publishDiagnostics") {
      diagnosticsByUri[msg.params.uri] = msg.params.diagnostics;
    }
  }
});

send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { processId: process.pid, rootUri: pathToFileURL(repoRoot).toString(), capabilities: {} } });
send({ jsonrpc: "2.0", method: "initialized", params: {} });

for (const p of fixtures) {
  const uri = pathToFileURL(p).toString();
  const text = readFileSync(p, "utf8");
  const languageId = p.endsWith(".json") ? "json" : "markdown";
  send({ jsonrpc: "2.0", method: "textDocument/didOpen", params: { textDocument: { uri, languageId, version: 1, text } } });
}

setTimeout(() => {
  let failed = false;
  const seenCodes = new Set();
  for (const p of fixtures) {
    const uri = pathToFileURL(p).toString();
    const diags = diagnosticsByUri[uri] || [];
    const rel = path.relative(repoRoot, p);
    if (diags.length === 0) {
      console.log(`✅ ${rel}: clean`);
      continue;
    }
    const unexpected = diags.filter((d) => !allowed.has(d.code));
    for (const d of diags) seenCodes.add(d.code);
    if (unexpected.length > 0) {
      failed = true;
      console.log(`❌ ${rel}:`);
      for (const d of unexpected) console.log(`     ${d.code}: ${d.message}`);
    } else {
      console.log(`✅ ${rel}: only allowed codes (${diags.map((d) => d.code).join(", ")})`);
    }
  }
  console.log();
  console.log(`Diagnostic codes observed: ${[...seenCodes].sort().join(", ") || "(none)"}`);
  child.kill();
  process.exit(failed ? 1 : 0);
}, 2000);
