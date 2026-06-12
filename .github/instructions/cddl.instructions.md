---
applyTo: "cddl/**/*.cddl"
---

Single schema: `cddl/cpoe.cddl` — covers both drafts.

## Tags

`cpoe-evidence = #6.1129336645(evidence-packet)` · `cpoe-war = #6.1129791826(attestation-result)`
Never change tag values — protocol constants.

## Conventions

- Types: `kebab-case` (`evidence-packet`, `session-context`)
- Keys: integers (`1 => uint, ; version`) — always commented
- Units: timestamps=ms, durations=ms, entropy=centibits
- Optional: `? 10 => tstr,` — Arrays: `[1* behavioral-sample]`

## Rules

- Never invent CBOR tags — only 1129336645, 1129791826, and 1129336658 exist
- Never change integer key assignments — wire-format stable
- Never remove fields — deprecate with comment if needed
- Validate: `make` checks CDDL syntax
