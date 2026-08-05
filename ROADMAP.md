[//]: # (SPDX-License-Identifier: Apache-2.0)

# Roadmap

## Current Status

CPoE (Cryptographic Proof of Effort) is an **LF Decentralized Trust lab** with two **IETF individual Internet-Drafts** (not yet adopted by any working group):

- `draft-condrey-rats-pop-protocol-06` -- Architecture and Evidence Format (category: experimental)
- `draft-condrey-rats-pop-appraisal-04` -- Forensic Appraisal and Security Model (category: experimental)

The drafts are published on the IETF datatracker under the earlier Proof of Process (PoP)
series name, while the `docname` in this repository's sources is `draft-condrey-cpoe-*`.
The two do not currently match. Because the publish workflow submits on any `draft-*` tag,
resolving this is a prerequisite to the next submission: publishing under the `cpoe` name
starts a new `-00` series rather than continuing the existing revision history.

Both are independent submissions to the IETF under the Security area, aligned with the RATS (Remote Attestation Procedures) architecture defined in RFC 9334.

Three further draft sources in this repository -- `draft-condrey-cpoe-anchoring`,
`draft-condrey-cpoe-usecases`, and `draft-condrey-hat` -- have not been submitted and do
not yet appear on the datatracker.

The repository contains the formal CDDL schema, architecture documentation, and integration proposals for three ecosystems. A reference implementation is maintained separately.

## Near-Term Milestones

### Specification

- [x] CDDL schema validation tooling integrated into CI
- [ ] Test vectors for Evidence Packet and Written Authorship Report encoding
- [ ] IANA registration requests drafted (CBOR tags, media types)

### Interoperability

- [ ] Cross-implementation interop test suite against the CDDL schema
- [ ] Second independent implementation (any language) to validate the specification

## Medium-Term Goals

### IETF Working Group Adoption

- [ ] Present CPoE at IETF SECDISPATCH for routing guidance
- [ ] Present at IETF RATS WG to evaluate fit as a RATS profile
- [ ] Gather review from IETF Security Area Directorate
- [ ] Seek working group adoption (RATS or a new WG, depending on SECDISPATCH outcome)

### Community Growth

- [ ] Second active maintainer (see [MAINTAINERS.md](MAINTAINERS.md) for the process)
- [ ] External contributors with merged pull requests
- [ ] Engagement with academic researchers in keystroke dynamics and behavioral biometrics
- [ ] Published interop results from at least two independent implementations

### LFDT Project Graduation

See [LIFECYCLE.md](LIFECYCLE.md) for the full checklist of incubation and graduation criteria with current status.

## Long-Term Vision

### Standardization

- [ ] CPoE protocol draft advanced to RFC (experimental or standards-track, depending on WG consensus)
- [ ] CPoE appraisal draft advanced to RFC
- [ ] IANA registrations finalized for CBOR tags (`1129336645` / CPoE, `1129791826` / CWAR) and media types
- [ ] EAR (EAT Attestation Result) compatibility confirmed with RATS tooling for Written Authorship Reports

### Ecosystem Adoption

- [ ] Attester SDKs for major platforms (desktop, mobile, web via Wasm)
- [ ] Verifier-as-a-service reference deployment
- [ ] Integration with content provenance and identity ecosystems (see below)
- [ ] Adoption by at least one content platform or publishing tool

## Integration Pipeline

### C2PA (Coalition for Content Provenance and Authenticity)

| Item | Status |
| ---- | ------ |
| Integration design | Documented ([guide](docs/integration/c2pa.md)) |
| `c2pa.process-evidence` assertion proposal | PR submitted ([c2pa-org/specs-core#2009](https://github.com/c2pa-org/specs-core/pull/2009)) |
| Hashed URI reference to Evidence Packet / WAR | Designed, not yet implemented |
| Working C2PA manifest with CPoE assertion | Not started |

### CAWG (Creator Assertions Working Group)

| Item | Status |
| ---- | ------ |
| Integration design | Documented ([guide](docs/integration/cawg.md)) |
| Creator Process Assertion type definition | Proposed, not yet submitted to CAWG |
| Composability with CAWG identity assertions | Designed |
| Working CAWG assertion with CPoE evidence | Not started |

### DID / Verifiable Credentials

| Item | Status |
| ---- | ------ |
| Integration design | Documented ([guide](docs/integration/did.md)) |
| WAR as W3C Verifiable Credential schema | Under development |
| Selective disclosure via Verifiable Presentations | Designed |
| Identus platform integration | Not started |
| DIDComm transport for evidence exchange | Not started |
