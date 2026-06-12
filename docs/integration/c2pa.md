[//]: # (SPDX-License-Identifier: Apache-2.0)

# C2PA Integration

> **Stage**: Proposed — PR pending at [c2pa-org/specs-core#2009](https://github.com/c2pa-org/specs-core/pull/2009)

This document describes how Proof of Effort (CPoE) attestation evidence
integrates with the [Coalition for Content Provenance and Authenticity
(C2PA)](https://c2pa.org/) content credentials framework.

## Overview

C2PA manifests establish content provenance — who created something, what
tools were used, what edits were made. CPoE adds a layer C2PA doesn't
currently cover: whether the creation process involved a human.

## Mapping

CPoE evidence maps to C2PA as a custom assertion within a C2PA manifest:

| C2PA Concept        | CPoE Mapping                                      |
| ------------------- | ------------------------------------------------ |
| Manifest            | Container for CPoE assertion alongside other claims |
| Assertion           | `c2pa.process-evidence` (proposed) — references CPoE evidence via hashed URI |
| Action              | Content creation/editing session                 |
| Ingredient          | Source document referenced in Evidence Packet    |

### Assertion Structure

CPoE integrates with C2PA via a `c2pa.process-evidence` assertion (see
[c2pa-org/specs-core#2009](https://github.com/c2pa-org/specs-core/pull/2009))
that references externally-stored creation-process evidence using a hashed
URI. This approach:

- References the Evidence Packet or Written Authorship Report by content-addressed
  hash, rather than embedding it inline.
- Supports multiple evidence types: `attestation-record` (CPoE WAR),
  `commitment-chain`, `audit-log`, `version-history`.
- Provides tamper-evidence via the hash binding within the signed manifest.

### Trust Model

The C2PA trust chain extends naturally:

1. The C2PA manifest is signed by the content creator's certificate.
2. The `c2pa.process-evidence` assertion within the manifest contains a
   hashed URI referencing the CPoE Written Authorship Report.
3. The Attestation Result is independently verifiable and traces back to
   hardware-backed Evidence Packets from the Attester.

## Use Cases

- **Photo/video platforms** — Embedding human-process attestation alongside
  C2PA edit history to distinguish human-captured media from AI-generated
  content.
- **Publishing** — Proving that written content involved human compositional
  effort, attached as a C2PA assertion to the published document.
- **Code repositories** — Attaching CPoE evidence to commits via C2PA
  manifests on repository artifacts.

## Process Timeline Support

The `c2pa.process-evidence` assertion supports `processStart` and
`processEnd` timestamps (proposed in
[PR #2009](https://github.com/c2pa-org/specs-core/pull/2009)), allowing
Relying Parties to verify that the creation process spanned a plausible
duration. These map directly to the first and last checkpoint timestamps
in the CPoE Evidence Packet.

## Multi-Asset MIME Type Detection

The `C2paManifestBuilder` supports a `format()` setter for specifying the
`dc:format` field in the C2PA claim. This enables proper MIME type metadata
for multi-asset workflows:

- `image/jpeg`, `image/png` for photo assets
- `video/mp4`, `video/quicktime` for video assets
- `application/pdf` for document assets
- Omitting `format()` preserves backward compatibility (field is optional)

## AI Disclosure

CPoE's AI declaration maps to IPTC `digitalSourceType` values within C2PA
manifests:

| CPoE Declaration         | IPTC digitalSourceType                          |
| ------------------------ | ----------------------------------------------- |
| Human-authored           | `http://cv.iptc.org/newscodes/digitalsourcetype/humanWritten` |
| AI-assisted              | `http://cv.iptc.org/newscodes/digitalsourcetype/compositeWithTrainedAlgorithmicMedia` |
| AI-generated             | `http://cv.iptc.org/newscodes/digitalsourcetype/trainedAlgorithmicMedia` |

## CAWG Identity and TDM Assertions

CPoE manifests can include Creator Assertions Working Group (CAWG) identity
assertions, binding the creator's DID or X.509 certificate to the C2PA
manifest. Text and Data Mining (TDM) assertions can also be included to
express the creator's preferences regarding AI training use of their content.

## Status

The `c2pa.process-evidence` assertion is proposed but not yet part of the
C2PA specification. See [PR #2009](https://github.com/c2pa-org/specs-core/pull/2009)
for the current proposal and the [C2PA specification](https://c2pa.org/specifications/)
for the manifest format it extends.
