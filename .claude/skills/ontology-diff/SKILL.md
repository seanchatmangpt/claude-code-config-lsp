---
name: ontology-diff
description: Diff schema/claude-code-config.ttl before and after a ggen sync to show exactly which generated files changed and why. Use after editing the ontology and running ggen sync, before committing.
user-invocable: true
default-enabled: false
disable-model-invocation: false
effort: medium
argument-hint: "[--staged]"
---

# ontology-diff

Arguments: `$ARGUMENTS`

Run `git diff schema/claude-code-config.ttl` alongside `git diff -- src/`
(or `git diff --staged` if `$ARGUMENTS` contains `--staged`) and correlate
which generated Rust files changed as a direct consequence of which
ontology edit.
