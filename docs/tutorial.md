# Tutorial: Your First claude-code-config-lsp Server

This tutorial walks you from zero to a working LSP server that validates Claude
Code configuration files. By the end you will have a running server, an editor
connected to it, and diagnostics firing on a bad `SKILL.md`.

**Prerequisites**

- Rust toolchain (stable, ≥ 1.83)
- `ggen` CLI installed
- `lsp-max` sibling checkout at `../lsp-max`
- An editor with LSP support (VS Code, Neovim, Helix)

---

## Step 1 — Bootstrap the consumer

```sh
mkdir my-config-lsp
cd my-config-lsp
git init
```

Create `ggen.toml`:

```toml
[project]
name    = "my-config-lsp"
version = "26.6.25"

[ontology]
source  = "schema/domain.ttl"
imports = [
    "../lsp-max/ontology/lsp318.ttl",
    "../lsp-max/ontology/max-protocol.ttl",
    "../lsp-max/ontology/law-axes.ttl",
]

[[packs]]
name     = "lsp-max"
registry = "local"
path     = "../lsp-max"

[[generation.rules]]
name        = "cargo"
query       = { inline = "SELECT ('my-config-lsp' AS ?name) ('26.6.25' AS ?version) WHERE {}" }
template    = { pack = "lsp-max", output = "templates", file = "cargo.toml.tera" }
output_file = "Cargo.toml"
mode        = "Overwrite"

[[generation.rules]]
name        = "backend"
query       = { inline = "SELECT ('MyConfig' AS ?server_name) WHERE {}" }
template    = { pack = "lsp-max", output = "templates", file = "backend.rs.tera" }
output_file = "src/backend.rs"
mode        = "Overwrite"

[[generation.rules]]
name        = "lib"
query       = { inline = "SELECT ('backend' AS ?module_name) ('MyConfig' AS ?server_name) WHERE {}" }
template    = { pack = "lsp-max", output = "templates", file = "lib.rs.tera" }
output_file = "src/lib.rs"
mode        = "Overwrite"

[[generation.rules]]
name        = "main"
query       = { inline = "SELECT ('MyConfig' AS ?server_name) ('my_config_lsp' AS ?snake_name) WHERE {}" }
template    = { pack = "lsp-max", output = "templates", file = "main.rs.tera" }
output_file = "src/main.rs"
mode        = "Overwrite"
```

Create a minimal ontology stub at `schema/domain.ttl`:

```turtle
@prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix mc:   <https://my-config-lsp.rs/ontology/> .
```

## Step 2 — Generate the source tree

```sh
ggen sync
```

Expected output: `"status": "success"` with `src/backend.rs`, `src/lib.rs`,
`src/main.rs`, and `Cargo.toml` written.

## Step 3 — Build and run

```sh
cargo build
./target/debug/my-config-lsp
```

The server starts on stdio waiting for LSP initialize messages.

## Step 4 — Connect an editor

**VS Code** — add to `.vscode/settings.json`:

```json
{
  "languageServerExample.serverPath": "${workspaceFolder}/target/debug/my-config-lsp"
}
```

Or use the generic LSP client extension and point it at the binary.

**Neovim (nvim-lspconfig)**:

```lua
vim.lsp.start({
  name = "my-config-lsp",
  cmd  = { "/path/to/target/debug/my-config-lsp" },
  filetypes = { "markdown", "json", "toml" },
  root_dir = vim.fn.getcwd(),
})
```

## Step 5 — Trigger your first diagnostic

Open a `SKILL.md` file with a malformed frontmatter name:

```markdown
---
name: MY BAD SKILL NAME
description: does something
---
```

The `name` field violates the Claude Code Skill naming rule (uppercase characters
are forbidden; max 64 chars; cannot contain `anthropic` or `claude`). Once the
SKILL.md analyzer is wired, the server should emit a `CCC-SKILL-001` diagnostic
on line 2.

## Step 6 — Extend with a real analyzer

Add a `[[generation.rules]]` block for the `claude_md` analyzer in your
`ggen.toml`, pointing at `analyzer_stub.rs.tera`, then add the Skill surface to
your domain ontology. Run `ggen sync` — the analyzer stub appears at
`src/analyzers/claude_md.rs`. Fill in the validation logic there.

---

You now have a working, ggen-generated LSP server consuming the `lsp-max` pack.
The next step is the how-to guides for adding specific validation surfaces.
