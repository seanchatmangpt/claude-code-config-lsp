---
name: release-checklist
description: Runs the full pre-release checklist — build, test, clippy, version consistency, CHANGELOG entry. Auto-invoked by the release-manager agent; not meant to be typed directly.
user-invocable: false
default-enabled: true
disable-model-invocation: true
effort: high
context: fork
agent: release-manager
---

# release-checklist

Runs `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and checks
that `Cargo.toml`'s version matches the latest `CHANGELOG.md` entry. Invoked
by `release-manager`, never directly by a user.
