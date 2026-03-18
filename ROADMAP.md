[//]: # (SPDX-License-Identifier: Apache-2.0)

# Roadmap

## Current Status

CPoP (Cryptographic Proof of Process) is an **LF Decentralized Trust lab** with two **IETF individual Internet-Drafts** (not yet adopted by any working group):

- `draft-condrey-cpop-protocol-06` -- Architecture and Evidence Format (category: experimental)
- `draft-condrey-cpop-appraisal-04` -- Forensic Appraisal and Security Model (category: experimental)

Both are independent submissions to the IETF under the Security area, aligned with the RATS (Remote Attestation Procedures) architecture defined in RFC 9334.

The repository contains a formal CDDL schema, architecture documentation, integration proposals for three ecosystems, and two published Rust crates on crates.io:

- [`cpop-jitter`](https://crates.io/crates/cpop-jitter) v0.2.1 -- Timing jitter entropy primitive (`no_std` compatible)
- [`cpop-protocol`](https://crates.io/crates/cpop-protocol) v0.1.1 -- Wire format, CBOR/COSE codec, evidence builder/verifier

## Near-Term Milestones

### Reference Implementations

- [x] `cpop-jitter` crate: timing jitter entropy collection, SWF proof generation
- [x] `cpop-protocol` crate: CBOR wire format, COSE signing, evidence builder/verifier
- [ ] End-to-end attestation flow: Attester produces Evidence Packet, Verifier produces Writers Authenticity Report
- [ ] Wasm build target for `cpop-protocol` (feature-flagged, dependencies in place)
- [ ] Example Attester integration (editor plugin or CLI tool capturing keystroke dynamics)
- [ ] Example Verifier service consuming Evidence Packets and producing WARs

### Specification

- [x] CDDL schema validation tooling integrated into CI
- [ ] Test vectors for Evidence Packet and Writers Authenticity Report encoding
- [ ] IANA registration requests drafted (CBOR tags, media types)

### Interoperability

- [ ] Cross-implementation interop test suite against the CDDL schema
- [ ] Second independent implementation (any language) to validate the specification

## Medium-Term Goals

### IETF Working Group Adoption

- [ ] Present CPoP at IETF SECDISPATCH for routing guidance
- [ ] Present at IETF RATS WG to evaluate fit as a RATS profile
- [ ] Gather review from IETF Security Area Directorate
- [ ] Seek working group adoption (RATS or a new WG, depending on SECDISPATCH outcome)

### Community Growth

- [ ] Second active maintainer (see [MAINTAINERS.md](MAINTAINERS.md) for the process)
- [ ] External contributors with merged pull requests
- [ ] Engagement with academic researchers in keystroke dynamics and behavioral biometrics
- [ ] Published interop results from at least two independent implementations

### LFDT Project Graduation

LFDT labs graduate to full projects by demonstrating community, code maturity, and governance readiness. Requirements include:

- [ ] Diverse maintainer base (multiple organizations)
- [ ] Active contributor community
- [ ] Stable release of reference implementation crates (1.0)
- [ ] Documented governance beyond single-maintainer model
- [ ] Quarterly reporting to LFDT Technical Advisory Council
- [ ] Security audit or review of the reference implementation

## Long-Term Vision

### Standardization

- [ ] CPoP protocol draft advanced to RFC (experimental or standards-track, depending on WG consensus)
- [ ] CPoP appraisal draft advanced to RFC
- [ ] IANA registrations finalized for CBOR tags (`1129336656` / CPOP, `1129791826` / CWAR) and media types
- [ ] EAR (EAT Attestation Result) compatibility confirmed with RATS tooling for Writers Authenticity Reports

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
| Working C2PA manifest with CPoP assertion | Not started |

### CAWG (Creator Assertions Working Group)

| Item | Status |
| ---- | ------ |
| Integration design | Documented ([guide](docs/integration/cawg.md)) |
| Creator Process Assertion type definition | Proposed, not yet submitted to CAWG |
| Composability with CAWG identity assertions | Designed |
| Working CAWG assertion with CPoP evidence | Not started |

### DID / Verifiable Credentials

| Item | Status |
| ---- | ------ |
| Integration design | Documented ([guide](docs/integration/did.md)) |
| WAR as W3C Verifiable Credential schema | Under development |
| Selective disclosure via Verifiable Presentations | Designed |
| Identus platform integration | Not started |
| DIDComm transport for evidence exchange | Not started |
