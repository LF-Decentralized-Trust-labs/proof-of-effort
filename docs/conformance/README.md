[//]: # (SPDX-License-Identifier: Apache-2.0)

# Standards Conformance

This directory documents how the Proof-of-Process (CPoP) protocol aligns
with external standards, specifications, and regulatory frameworks.

CPoP is designed to operate *beneath* existing provenance and identity
standards, adding a behavioral attestation layer. These conformance documents
describe the concrete mapping between CPoP structures and each external
standard.

## Documents

| Standard | Document | Status |
| -------- | -------- | ------ |
| IETF RATS | [rats.md](rats.md) | **Normative** — CPoP implements RFC 9334 |
| CBOR / COSE | [cbor-cose.md](cbor-cose.md) | **Normative** — Wire format per RFC 8949 + RFC 9052 |
| C2PA | [../integration/c2pa.md](../integration/c2pa.md) | Proposed — soft binding registered |
| W3C DID Core | [did.md](did.md) | Implemented — `did:key` + `did:web` |
| W3C VC Data Model 2.0 | [vc.md](vc.md) | Implemented — credential projection |
| NIST AI RMF / AI 100-4 | [nist.md](nist.md) | Mapped — subcategory alignment |
| ISO/IEC 42001 | [iso42001.md](iso42001.md) | Mapped — Annex A controls |
| IPTC Digital Source Type | [iptc.md](iptc.md) | Implemented — vocabulary mapping |
| EU AI Act Article 50 | [eu-ai-act.md](eu-ai-act.md) | Aligned — machine-readable disclosure |
| WGA MBA / SAG-AFTRA | [creative-rights.md](creative-rights.md) | Mapped — authorship attestation |
