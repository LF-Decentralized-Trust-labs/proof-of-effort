[//]: # (SPDX-License-Identifier: Apache-2.0)

# C2PA Integration

> **Stage**: Proposed — PR pending at [c2pa-org/specs-core#2009](https://github.com/c2pa-org/specs-core/pull/2009)

This document describes how Proof-of-Process (CPoP) attestation evidence
integrates with the [Coalition for Content Provenance and Authenticity
(C2PA)](https://c2pa.org/) content credentials framework.

## Overview

C2PA manifests establish content provenance — who created something, what
tools were used, what edits were made. CPoP adds a layer C2PA doesn't
currently cover: whether the creation process involved a human.

## Mapping

CPoP evidence maps to C2PA as a custom assertion within a C2PA manifest:

| C2PA Concept        | CPoP Mapping                                      |
| ------------------- | ------------------------------------------------ |
| Manifest            | Container for CPoP assertion alongside other claims |
| Assertion           | `c2pa.process-evidence` (proposed) — references CPoP evidence via hashed URI |
| Action              | Content creation/editing session                 |
| Ingredient          | Source document referenced in Evidence Packet    |

### Assertion Structure

CPoP integrates with C2PA via a `c2pa.process-evidence` assertion (see
[c2pa-org/specs-core#2009](https://github.com/c2pa-org/specs-core/pull/2009))
that references externally-stored creation-process evidence using a hashed
URI. This approach:

- References the Evidence Packet or Writers Authenticity Report by content-addressed
  hash, rather than embedding it inline.
- Supports multiple evidence types: `attestation-record` (CPoP WAR),
  `commitment-chain`, `audit-log`, `version-history`.
- Provides tamper-evidence via the hash binding within the signed manifest.

### Trust Model

The C2PA trust chain extends naturally:

1. The C2PA manifest is signed by the content creator's certificate.
2. The `c2pa.process-evidence` assertion within the manifest contains a
   hashed URI referencing the CPoP Writers Authenticity Report.
3. The Attestation Result is independently verifiable and traces back to
   hardware-backed Evidence Packets from the Attester.

## Use Cases

- **Photo/video platforms** — Embedding human-process attestation alongside
  C2PA edit history to distinguish human-captured media from AI-generated
  content.
- **Publishing** — Proving that written content involved human compositional
  effort, attached as a C2PA assertion to the published document.
- **Code repositories** — Attaching CPoP evidence to commits via C2PA
  manifests on repository artifacts.

## Status

The `c2pa.process-evidence` assertion is proposed but not yet part of the
C2PA specification. See [PR #2009](https://github.com/c2pa-org/specs-core/pull/2009)
for the current proposal and the [C2PA specification](https://c2pa.org/specifications/)
for the manifest format it extends.
