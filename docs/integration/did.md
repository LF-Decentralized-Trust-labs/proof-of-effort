[//]: # (SPDX-License-Identifier: Apache-2.0)

# Decentralized Identity Integration

> **Stage**: Under development — credential schema and presentation format in progress

This document describes how Proof of Effort (CPoE) attestation results
integrate with decentralized identity and verifiable credential ecosystems,
including [DIF](https://identity.foundation/) specifications and
[W3C Verifiable Credentials](https://www.w3.org/TR/vc-data-model-2.0/).

## Overview

A CPoE Written Authorship Report can be wrapped as a W3C Verifiable Credential and
bound to a DID, producing a portable, privacy-preserving claim that a human
process was involved — verifiable by anyone, without a central authority.

## Mapping

| DID/VC Concept              | CPoE Mapping                                  |
| --------------------------- | -------------------------------------------- |
| DID Subject                 | Content creator (Attester)                   |
| Issuer                      | CPoE Verifier                                 |
| Verifiable Credential       | Written Authorship Report (WAR) as VC     |
| Credential Subject          | Process attestation claims                   |
| Verifiable Presentation     | Selective disclosure of CPoE evidence         |

## Verifiable Credential Structure

A CPoE Written Authorship Report expressed as a verifiable credential contains:

- **Issuer** — The DID of the CPoE Verifier that performed the appraisal.
- **Credential Subject** — The DID of the content creator, with claims
  about the attested process (confidence score, entropy metrics, session
  reference).
- **Evidence** — Reference to the Evidence Packet hash for auditability.
- **Proof** — Cryptographic proof from the Verifier.

## Privacy Considerations

- **Selective disclosure** — A Verifiable Presentation can reveal just the
  verdict (human/not-human) without exposing the underlying behavioral
  telemetry.
- **Unlinkability** — Attestation results for the same author don't need
  to be linkable across sessions unless the author opts in.
- **Data minimization** — Evidence Packets capture behavioral patterns, not
  biometric identifiers. The Written Authorship Report abstracts away the raw
  evidence entirely.

## Ecosystem Alignment

Potential alignment with existing LFDT efforts:

- **Identus** — CPoE credentials could be issued and managed within the
  Identus platform as a new credential type.
- **DIDComm** — Evidence Packets and attestation results could be exchanged
  via DIDComm messaging protocols.

## Status

This integration pattern is under development. The credential schema and
presentation format will be formalized as the specification matures.
See the [DIF](https://identity.foundation/) and
[W3C VC Data Model](https://www.w3.org/TR/vc-data-model-2.0/) for current
standards.
