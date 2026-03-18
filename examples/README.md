[//]: # (SPDX-License-Identifier: Apache-2.0)

# CPoP Example Data

This directory contains canonical examples of CPoP data structures in
CBOR diagnostic notation ([RFC 8949 Section 8][diag]).  They are intended
as implementation references and are **not** real evidence.

| File | Structure | CBOR Tag | Description |
| ---- | --------- | -------- | ----------- |
| `evidence-packet.cddl-diag` | `evidence-packet` | 1129336656 (`CPOP`) | T2 (attested-software) Evidence Packet with 3 checkpoints, jitter proof, behavioral metrics, and physical state |
| `writers-authenticity-report.cddl-diag` | `attestation-result` | 1129791826 (`CWAR`) | Writers Authenticity Report with an "authentic" verdict, entropy report, forgery cost estimate, and forensic summary |

All hash digests, UUIDs, signatures, and timestamps are synthetic
placeholder values.  Integer map keys follow the schema in
[`cddl/cpop.cddl`](../cddl/cpop.cddl).

[diag]: https://www.rfc-editor.org/rfc/rfc8949#section-8
