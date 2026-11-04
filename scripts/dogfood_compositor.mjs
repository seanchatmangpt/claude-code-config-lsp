#!/usr/bin/env node
// Drives the real lsp-max-compositor binary over the actual LSP wire protocol
// (Content-Length-framed JSON-RPC over stdio) with this repo's lsp-max.toml as
// the fan-out manifest. The compositor spawns the claude-code-config-lsp child
// itself and merges its diagnostics into the front-door stream, so diagnostics
// arriving here carry the child's own CCC-* codes — multiplexing witnessed at
// the wire level, not in-process.
//
// Derived from scripts/dogfood.mjs (which drives the child directly); this
// variant drives the compositor front door instead. Unlike dogfood.mjs it does
// not walk fixture trees: it opens the specific contract fixtures (one config
// JSON, one markdown) and asserts on the CCC-* codes that come back.
//
// Usage:
//   node scripts/dogfood_compositor.mjs [--allow <CODE>]... \
//        [--compositor <path>] [--child <path>] \
//        [--fixtures <json> <md>] [--timeout <ms>]
//
// The rebuild gate below builds stale binaries itself (bounded), so a plain
// `node scripts/dogfood_compositor.mjs` is self-sufficient on a fresh clone.
//
// Exit 0 = child CCC-* codes observed through the compositor over the wire and
// every non-allowed diagnostic accounted for. Exit 1 = assertion failed
// (including: compositor spawned the child but no publishDiagnostics crossed
// the front door within the timeout — a regression, F4's notification-dispatch
// fix landed and must not silently revert). Exit 2 = BLOCKED (build failed or
// binaries unavailable).
import { spawn, execFileSync } from "node:child_process";
import { readFileSync, existsSync, statSync, readdirSync } from "node:fs";
import { pathToFileURL } from "node:url";
import path from "node:path";

const repoRoot = path.resolve(import.meta.dirname, "..");
const lspMaxRoot = "/Users/sac/lsp-max";

// ---------------------------------------------------------------------------
// (1) Argv walk: parse --allow X (repeatable), --fixtures JSON MD,
// --timeout MS, --compositor P, --child P in any order.
// ---------------------------------------------------------------------------
const args = process.argv.slice(2);
const allowed = new Set();
const opts = { fixtures: [], timeout: 15000 };
for (let i = 0; i < args.length; i++) {
  const a = args[i];
  if (a === "--allow") {
    if (i + 1 >= args.length || args[i + 1].startsWith("--")) {
      console.error("BLOCKED: --allow requires a code argument");
      process.exit(2);
    }
    allowed.add(args[++i]);
  } else if (a === "--fixtures") {
    // consumes up to two positional values (json, md); stops at next flag
    while (
      opts.fixtures.length < 2 &&
      i + 1 < args.length &&
      !args[i + 1].startsWith("--")
    ) {
      opts.fixtures.push(args[++i]);
    }
    if (opts.fixtures.length === 0) {
      console.error("BLOCKED: --fixtures requires <json> [md]");
      process.exit(2);
    }
  } else if (a === "--timeout") {
    const v = Number(args[++i]);
    if (!Number.isFinite(v) || v <= 0) {
      console.error("BLOCKED: --timeout requires a positive number of ms");
      process.exit(2);
    }
    opts.timeout = v;
  } else if (a === "--compositor" || a === "--child") {
    const key = a === "--compositor" ? "compositor" : "child";
    if (i + 1 >= args.length) {
      console.error(`BLOCKED: ${a} requires a path argument`);
      process.exit(2);
    }
    opts[key] = args[++i];
  } else {
    console.error(`BLOCKED: unknown argument ${a}`);
    process.exit(2);
  }
}

// ---------------------------------------------------------------------------
// (2) Rebuild gate: binary must exist AND be newer than its sources; if stale
// or missing, run a bounded release build and record how long it took.
// ---------------------------------------------------------------------------
function newestSourceMtime(roots, binPath) {
  let newest = 0;
  const visit = (p) => {
    let st;
    try {
      st = statSync(p);
    } catch {
      return;
    }
    if (st.isDirectory()) {
      for (const e of readdirSync(p)) visit(path.join(p, e));
    } else if (/\.(rs|toml)$/.test(p)) {
      newest = Math.max(newest, st.mtimeMs);
    }
  };
  for (const r of roots) visit(r);
  return newest;
}

function buildGate({ bin, label, cwd, cargoArgs }) {
  if (!existsSync(bin)) {
    console.error(
      `BLOCKED: ${label} binary not found at ${bin}.\n` +
        `Build step: (cd ${cwd} && cargo ${cargoArgs.join(" ")})`
    );
    process.exit(2);
  }
  const stale = newestSourceMtime(
    [path.join(cwd, "src"), path.join(cwd, "Cargo.toml")],
    bin
  );
  if (stale <= statSync(bin).mtimeMs) return { built: false };
  console.error(`[rebuild-gate] ${label} is stale — rebuilding (release)...`);
  const t0 = Date.now();
  try {
    execFileSync("cargo", cargoArgs, { cwd, stdio: "inherit" });
  } catch {
    console.error(`BLOCKED: rebuild of ${label} failed (exit != 0).`);
    process.exit(2);
  }
  const secs = ((Date.now() - t0) / 1000).toFixed(1);
  console.error(`[rebuild-gate] ${label} rebuilt in ${secs}s`);
  return { built: true, secs };
}

const COMPOSITOR =
  opts.compositor || path.join(lspMaxRoot, "target/release/lsp-max-compositor");
const CHILD =
  opts.child || path.join(repoRoot, "target/release/claude-code-config-lsp");

const gateCompositor = buildGate({
  bin: COMPOSITOR,
  label: "lsp-max-compositor",
  cwd: lspMaxRoot,
  cargoArgs: ["build", "--release", "-p", "lsp-max-compositor"],
});
const gateChild = buildGate({
  bin: CHILD,
  label: "claude-code-config-lsp",
  cwd: repoRoot,
  cargoArgs: ["build", "--release"],
});

// Contract fixtures: a real settings.json and a real markdown config doc.
const jsonFixture = opts.fixtures[0] || "tests/fixtures/claude-config/settings.json";
const mdFixture =
  opts.fixtures[1] || "tests/fixtures/claude-config/claude-global-CLAUDE.md";
const jsonPath = path.isAbsolute(jsonFixture) ? jsonFixture : path.join(repoRoot, jsonFixture);
const mdPath = path.isAbsolute(mdFixture) ? mdFixture : path.join(repoRoot, mdFixture);
if (!existsSync(jsonPath)) {
  console.error(`BLOCKED: JSON fixture not found: ${jsonPath}`);
  process.exit(2);
}
if (!existsSync(mdPath)) {
  console.error(`BLOCKED: MD fixture not found: ${mdPath}`);
  process.exit(2);
}

// The compositor resolves lsp-max.toml by walking cwd upward, and spawns the
// child command (bare "claude-code-config-lsp") via PATH — so we run it from
// the repo root with the child's build dir prepended to PATH.
const childDir = path.dirname(CHILD);
const child = spawn(COMPOSITOR, [], {
  cwd: repoRoot,
  stdio: ["pipe", "pipe", "pipe"],
  env: { ...process.env, PATH: `${childDir}:${process.env.PATH}` },
});

let buf = Buffer.alloc(0);
const diagnosticsByUri = {};
const wireLog = [];

child.stderr.on("data", (d) => process.stderr.write(`[compositor] ${d}`));
child.on("error", (e) => {
  console.error(`BLOCKED: failed to launch compositor: ${e.message}`);
  process.exit(2);
});

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
    const header = buf.subarray(0, headerEnd).toString("utf8");
    const m = /Content-Length: (\d+)/.exec(header);
    if (!m) return;
    const len = parseInt(m[1], 10);
    const bodyStart = headerEnd + 4;
    if (buf.length < bodyStart + len) return;
    const body = buf.subarray(bodyStart, bodyStart + len).toString("utf8");
    buf = buf.subarray(bodyStart + len);
    let msg;
    try {
      msg = JSON.parse(body);
    } catch {
      wireLog.push(`UNPARSEABLE FRAME: ${body}`);
      return;
    }
    wireLog.push(`<- ${msg.method || "(response)"}${msg.id ? ` id=${msg.id}` : ""}`);
    if (msg.method === "textDocument/publishDiagnostics") {
      diagnosticsByUri[msg.params.uri] = msg.params.diagnostics;
    }
  }
});

function open(uri, languageId, text) {
  send({
    jsonrpc: "2.0",
    method: "textDocument/didOpen",
    params: { textDocument: { uri, languageId, version: 1, text } },
  });
}

const jsonUri = pathToFileURL(jsonPath).toString();
const mdUri = pathToFileURL(mdPath).toString();
const deadline = Date.now() + opts.timeout;

send({
  jsonrpc: "2.0",
  id: 1,
  method: "initialize",
  params: { processId: process.pid, rootUri: pathToFileURL(repoRoot).toString(), capabilities: {} },
});
send({ jsonrpc: "2.0", method: "initialized", params: {} });

// Open the config JSON first, give the fan-out a bounded window to route it to
// the child and publish merged diagnostics, then open the markdown doc.
open(jsonUri, "json", readFileSync(jsonPath, "utf8"));

async function collectDiagnostics(uri, label) {
  return new Promise((resolve) => {
    const poll = () => {
      if (diagnosticsByUri[uri]) return resolve(diagnosticsByUri[uri]);
      if (Date.now() > deadline) return resolve(null); // typed timeout, not an exception
      setTimeout(poll, 25);
    };
    poll();
  });
}

const jsonDiags = await collectDiagnostics(jsonUri);
if (!jsonDiags) await reportNoDiagnostics(jsonUri);
open(mdUri, "markdown", readFileSync(mdPath, "utf8"));
const mdDiags = await collectDiagnostics(mdUri);
if (!mdDiags) await reportNoDiagnostics(mdUri);

// Assert child CCC-* codes arrived through the compositor. A timeout here is a
// FAIL (exit 1), not a BLOCKED: F4's notification-dispatch fix means child
// publishDiagnostics DO cross the front door; silence is a regression.
function reportNoDiagnostics(stuckUri) {
  return new Promise((resolve) => {
    console.error(`FAIL: no publishDiagnostics crossed the compositor front door within ${opts.timeout}ms.`);
    console.error(`  Stuck URI: ${stuckUri}`);
    console.error(
      "  F4 landed child-notification dispatch in lsp-max read_loop; diagnostics"
    );
    console.error(
      "  are expected through the front door. Silence = dispatch regression."
    );
    console.error("  Control: scripts/dogfood.mjs drives the child directly.");
    wireLog.forEach((l) => console.error(`  wire: ${l}`));
    send({ jsonrpc: "2.0", id: 2, method: "shutdown", params: null });
    send({ jsonrpc: "2.0", method: "exit", params: null });
    child.stdin.end();
    const finish = () => {
      child.kill();
      process.exit(1);
    };
    child.on("close", finish);
    setTimeout(finish, 3000);
  });
}

// Assert child CCC-* codes arrived through the compositor.
const allDiags = [
  ...jsonDiags.map((d) => ({ uri: jsonUri, ...d })),
  ...mdDiags.map((d) => ({ uri: mdUri, ...d })),
];
const seenCodes = new Set(allDiags.map((d) => d.code));
const cccCodes = [...seenCodes].filter((c) => typeof c === "string" && c.startsWith("CCC-"));
const unexpected = allDiags.filter((d) => !allowed.has(d.code));

console.log(`Compositor front door: ${COMPOSITOR}`);
console.log(`Child (via PATH):      ${CHILD}`);
console.log(`Rebuild gate: compositor ${gateCompositor.built ? `rebuilt (${gateCompositor.secs}s)` : "fresh"}`);
console.log(`Rebuild gate: child ${gateChild.built ? `rebuilt (${gateChild.secs}s)` : "fresh"}`);
console.log(`JSON fixture: ${path.relative(repoRoot, jsonPath)} -> ${jsonDiags.length} diagnostic(s)`);
console.log(`MD fixture:   ${path.relative(repoRoot, mdPath)} -> ${mdDiags.length} diagnostic(s)`);
console.log(`Diagnostic codes observed through the compositor: ${[...seenCodes].sort().join(", ") || "(none)"}`);
console.log(`CCC-* codes (child multiplexed at the wire level): ${cccCodes.join(", ") || "(none)"}`);
for (const entry of wireLog.slice(0, 40)) console.log(`  wire: ${entry}`);

let exit = 0;
if (cccCodes.length === 0) {
  console.error("FAIL: no CCC-* codes observed through the compositor front door.");
  exit = 1;
}
if (unexpected.length > 0) {
  console.error("FAIL: diagnostics not covered by --allow:");
  for (const d of unexpected) console.error(`  ${path.relative(repoRoot, fileUri(d.uri))} ${d.code}: ${d.message}`);
  exit = 1;
}

function fileUri(u) {
  return u.startsWith("file://") ? decodeURIComponent(u.replace("file://", "")) : u;
}

// shutdown, then exit notification, then close stdin so the compositor reaps.
send({ jsonrpc: "2.0", id: 2, method: "shutdown", params: null });
send({ jsonrpc: "2.0", method: "exit", params: null });
child.stdin.end();
child.on("close", () => process.exit(exit));
setTimeout(() => {
  child.kill();
  process.exit(exit);
}, 3000);
