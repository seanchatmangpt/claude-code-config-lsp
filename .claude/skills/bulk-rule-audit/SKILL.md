---
name: bulk-rule-audit
description: Audits every hand-coded diagnostic rule in src/analyzers/*.rs for stale hardcoded valid-value lists or error message text that has drifted from the const arrays they're supposed to mirror. Use after changing any VALID_* const to catch messages that weren't updated alongside it.
user-invocable: true
default-enabled: false
effort: xhigh
allowed-tools:
  - Grep
  - Read
---

# bulk-rule-audit

Greps every `src/analyzers/*.rs` for hardcoded diagnostic message strings
that list valid values (e.g. `"valid: auto, block, interactive"`) and
cross-checks each one against the actual `VALID_*` const array it's
supposed to describe, flagging any mismatch. This exact class of bug (a
`VALID_PERMISSION_MODES` update that left a hardcoded message string
stale) was found once by manual dogfooding — this skill exists so it
doesn't have to be found that way again.
