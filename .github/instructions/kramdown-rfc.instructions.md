---
applyTo: "draft-*.md"
---

kramdown-rfc → xml2rfc v3. Front matter: `v: 3`, `docname: draft-*-latest`.

## Syntax

- Headings: `##` (not XML) — References: `{{RFC9334}}`, `{{!RFC8610}}` (normative `!`)
- Figures: fenced block + `{: #fig-name title="Title"}` — types: `cddl`, `cbor-diag`, `ascii-art`
- Section refs: `{{Section 4.1 of RFC9334}}` — Figure refs: `{{fig-name}}`
- Def lists: `term:\n: definition`

## BCP 14

MUST, MUST NOT, SHALL, SHOULD, MAY — CAPITALIZED in normative use only.
DO: `The Attester MUST include a timestamp.`
DON'T: `must include` / `needs to` / `has to`

## Build

`make` (all) · `make draft-condrey-cpop-protocol.html` (one) · `make clean`

## Accuracy

- Never invent reference tags — verify RFC/I-D exists
- Never add BCP 14 keywords to informative text
- Never change `docname` or `ipr` without understanding implications
