[//]: # (SPDX-License-Identifier: Apache-2.0)

# IETF RATS Conformance

> **Status**: Normative — CPoP implements the RATS architecture (RFC 9334)

## Overview

CPoP is built directly on the IETF Remote ATtestation procedureS (RATS)
architecture defined in [RFC 9334](https://www.rfc-editor.org/rfc/rfc9334).
The protocol uses RATS roles, message flows, and data formats.

## RATS Role Mapping

| RATS Role                  | CPoP Implementation                                          |
| -------------------------- | ------------------------------------------------------------ |
| Attester                   | CPOP client (CLI, native app, browser extension)             |
| Evidence                   | Evidence Packet (CBOR tag `1129336656` / `CPOP`)             |
| Verifier                   | WritersProof API or local `cpop verify` command              |
| Attestation Result         | Writers Authenticity Report (CBOR tag `1129791826` / `CWAR`) |
| Relying Party              | Publisher, institution, or platform consuming the WAR        |
| Reference Value Provider   | Behavioral baselines (typing cadence, entropy thresholds)    |
| Endorser                   | Hardware attestation (TPM/Secure Enclave quote)              |

## EAT / EAR / AR4SI

CPoP uses Entity Attestation Token (EAT) and Entity Attestation Result (EAR)
formats with AR4SI trustworthiness vectors:

| Specification              | Usage in CPoP                                                |
| -------------------------- | ------------------------------------------------------------ |
| draft-ietf-rats-eat        | EAT profile URI: `urn:ietf:params:rats:eat:profile:pop:1.0` |
| draft-ietf-rats-ear        | EAR token structure for attestation results                  |
| draft-ietf-rats-ar4si      | 8-component trustworthiness vector                           |

### AR4SI Trust Vector Mapping

| Component (index) | RATS Meaning          | CPoP Mapping                               |
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

CPoP uses private-use CWT keys 70001–70009 for protocol-specific claims:

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

## References

- [RFC 9334 — Remote ATtestation procedureS (RATS) Architecture](https://www.rfc-editor.org/rfc/rfc9334)
- [draft-condrey-cpop-protocol](https://lf-decentralized-trust-labs.github.io/proof-of-process/draft-condrey-cpop-protocol.html)
- [draft-condrey-cpop-appraisal](https://lf-decentralized-trust-labs.github.io/proof-of-process/draft-condrey-cpop-appraisal.html)
