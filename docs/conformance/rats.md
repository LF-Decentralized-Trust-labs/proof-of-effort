[//]: # (SPDX-License-Identifier: Apache-2.0)

# IETF RATS Conformance

> **Status**: Normative — CPoE implements the RATS architecture (RFC 9334)

## Overview

CPoE is built directly on the IETF Remote ATtestation procedureS (RATS)
architecture defined in [RFC 9334](https://www.rfc-editor.org/rfc/rfc9334).
The protocol uses RATS roles, message flows, and data formats.

## RATS Role Mapping

| RATS Role                  | CPoE Implementation                                          |
| -------------------------- | ------------------------------------------------------------ |
| Attester                   | CPOP client (CLI, native app, browser extension)             |
| Evidence                   | Evidence Packet (CBOR tag `1129336656` / `CPOP`)             |
| Verifier                   | WritersProof API or local `cpoe verify` command              |
| Attestation Result         | Written Authorship Report (CBOR tag `1129791826` / `CWAR`) |
| Relying Party              | Publisher, institution, or platform consuming the WAR        |
| Reference Value Provider   | Behavioral baselines (typing cadence, entropy thresholds)    |
| Endorser                   | Hardware attestation (TPM/Secure Enclave quote)              |

## EAT / EAR / AR4SI

CPoE uses Entity Attestation Token (EAT) and Entity Attestation Result (EAR)
formats with AR4SI trustworthiness vectors:

| Specification              | Usage in CPoE                                                |
| -------------------------- | ------------------------------------------------------------ |
| draft-ietf-rats-eat        | EAT profile URI: `urn:ietf:params:rats:eat:profile:pop:1.0` |
| draft-ietf-rats-ear        | EAR token structure for attestation results                  |
| draft-ietf-rats-ar4si      | 8-component trustworthiness vector                           |

### AR4SI Trust Vector Mapping

| Component (index) | RATS Meaning          | CPoE Mapping                               |
| ----------------- | --------------------- | ------------------------------------------- |
| 0                 | Instance Identity     | Hardware attestation tier (TPM/SE)          |
| 1                 | Configuration         | Declaration signature validity              |
| 2                 | Executables           | Binary attestation presence                 |
| 3                 | File System           | Document hash chain integrity (H1/H2/H3)   |
| 4                 | Hardware              | TPM/Secure Enclave binding                  |
| 5                 | Runtime Opaque        | VDF proof strength + time plausibility      |
| 6                 | Storage Opaque        | Key hierarchy + session certificate         |
| 7                 | Sourced Data          | Behavioral entropy + jitter quality         |

### Private-Use CWT Keys

CPoE uses private-use CWT keys 70001–70009 for protocol-specific claims:

| Key   | Label           | Content                                |
| ----- | --------------- | -------------------------------------- |
| 70001 | Seal            | Three-hash seal (H1, H2, H3)          |
| 70002 | Evidence Ref    | SHA-256 of Evidence Packet             |
| 70003 | Entropy         | Entropy report                         |
| 70004 | Forgery Cost    | 8-component forgery cost estimate      |
| 70005 | Forensic        | Forensic analysis summary              |
| 70006 | Chain Length    | Checkpoint chain length                |
| 70007 | Chain Duration  | Total elapsed time (seconds)           |
| 70008 | Absence         | Absence claim (editing gaps)           |
| 70009 | Warnings        | Appraisal warnings array               |

## CWT-Encoded EAT Tokens

CPoE Attestation Results (Written Authorship Reports) are encoded as
CWT (CBOR Web Token) EAT tokens per draft-ietf-rats-eat. The token
carries standard EAT claims (`eat_profile`, `iat`, `eat_nonce`) alongside
CPoE-specific private-use claims (keys 70001-70009). The CWT is wrapped
in a COSE_Sign1 envelope for integrity and authenticity.

## CoRIM Reference Values

CPoE Verifiers can consume Concise Reference Integrity Manifests (CoRIM,
draft-ietf-rats-corim) to obtain reference values for appraisal. Reference
values include expected behavioral entropy thresholds, SWF difficulty
parameters, and approved hardware endorsement keys. The CoRIM environment
map identifies the CPoE attestation tier (T1-T4) and platform.

## SCITT Signed Statement Compatibility

CPoE Evidence Packets and Written Authorship Reports can be registered as
SCITT (Supply Chain Integrity, Transparency, and Trust) Signed Statements
per draft-ietf-scitt-architecture. Registration in a SCITT Transparency
Service provides an independent, append-only receipt that the attestation
existed at a given time, complementing CPoE's internal VDF-based temporal
proofs.

## References

- [draft-condrey-rats-hat -- Hardware Attestation of Time (HAT)](../draft-condrey-rats-hat.md) -- standalone HAT specification extracted from CPoE
- [RFC 9334 -- Remote ATtestation procedureS (RATS) Architecture](https://www.rfc-editor.org/rfc/rfc9334)
- [draft-ietf-rats-eat -- Entity Attestation Token](https://datatracker.ietf.org/doc/draft-ietf-rats-eat/)
- [draft-ietf-rats-corim -- Concise Reference Integrity Manifest](https://datatracker.ietf.org/doc/draft-ietf-rats-corim/)
- [draft-ietf-scitt-architecture -- SCITT Architecture](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/)
- [draft-condrey-cpoe-protocol](https://lf-decentralized-trust-labs.github.io/proof-of-effort/draft-condrey-cpoe-protocol.html)
- [draft-condrey-cpoe-appraisal](https://lf-decentralized-trust-labs.github.io/proof-of-effort/draft-condrey-cpoe-appraisal.html)
