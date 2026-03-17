---
applyTo: "cddl/**/*.cddl"
---

Single schema: `cddl/cpop.cddl` — covers both drafts.

## Tags

`pop-evidence = #6.1129336656(evidence-packet)` · `pop-war = #6.1129791826(attestation-result)`
Never change tag values — protocol constants.

## Conventions

- Types: `kebab-case` (`evidence-packet`, `session-context`)
- Keys: integers (`1 => uint, ; version`) — always commented
- Units: timestamps=ms, durations=ms, entropy=centibits
- Optional: `? 10 => tstr,` — Arrays: `[1* behavioral-sample]`

## Rules

- Never invent CBOR tags — only 1129336656 and 1129791826 exist
- Never change integer key assignments — wire-format stable
- Never remove fields — deprecate with comment if needed
- Validate: `make` checks CDDL syntax
