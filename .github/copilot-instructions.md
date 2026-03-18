IETF Internet-Draft spec for Proof-of-Process (CPoP) attestation — LF Decentralized Trust lab.
CPoP: cryptographic attestation of human cognitive involvement in digital content creation. Built on IETF RATS (RFC 9334).

## Stack

Drafts: kramdown-rfc · Schemas: CDDL (RFC 8610) · Build: `make` via martinthomson/i-d-template · CI: GitHub Actions

## Layout

```
draft-condrey-cpop-protocol.md   # Protocol spec
draft-condrey-cpop-appraisal.md  # Appraisal spec
cddl/cpop.cddl              # CBOR schema (tags: CPOP=1129336656, CWAR=1129791826)
docs/architecture.md                 # RATS role mapping
docs/integration/{c2pa,cawg,did}.md  # Ecosystem integration
```

## RATS mapping

Attester (emits CPOP) · Verifier (emits CWAR) · Relying Party (consumes CWAR)

## Rules

- Normative: MUST/SHOULD/MAY per BCP 14 — IETF English per RFC 7322
- CDDL types: `kebab-case` — map keys: integers — timestamps: ms — entropy: centibits
- Never invent RFC numbers, CDDL types, or CBOR tags — verify against source files
- Build with `make` before committing
