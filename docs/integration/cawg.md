[//]: # (SPDX-License-Identifier: Apache-2.0)

# CAWG Integration

This document describes how Proof-of-Process (CPoP) attestation aligns with
the [Creator Assertions Working Group (CAWG)](https://cawg.io/) specifications.

## Overview

CAWG defines creator assertions — verifiable claims about who created
something and what they intended. CPoP adds a different kind of assertion:
evidence that a human cognitive process was involved, independent of who
the creator is.

## Mapping

| CAWG Concept            | CPoP Mapping                                    |
| ----------------------- | ---------------------------------------------- |
| Creator Assertion       | CPoP process attestation as an assertion type   |
| Identity Assertion      | Orthogonal — CPoP attests process, not identity |
| Metadata Assertion      | Evidence Packet metadata (session, timestamps) |

## Creator Process Assertion

CPoP introduces a new assertion category for CAWG: the **Creator Process
Assertion**. This assertion:

- Attests that the content creation involved measurable human cognitive effort.
- Is bound to a specific creation session via the Evidence Packet.
- Carries a confidence score from the CPoP Verifier's appraisal.
- Is independent of (and composable with) identity assertions.

## Relationship to Identity Assertions

CAWG identity assertions answer: "Who created this content?"
CPoP process assertions answer: "Was a human cognitive process involved?"

These are orthogonal and composable. A content item can carry:

- An identity assertion (CAWG) binding the content to a creator.
- A process assertion (CPoP) proving human cognitive involvement.
- Both, providing full provenance: who created it and that they did so
  through genuine human effort.

## Status

This integration pattern is a proposal, not part of the CAWG specification
yet. See [cawg.io](https://cawg.io/) for the current assertion format.
