[//]: # (SPDX-License-Identifier: Apache-2.0)

# EU AI Act Article 50 Conformance

> **Status**: Aligned — machine-readable AI content disclosure

## Requirement

[EU AI Act Article 50](https://artificialintelligenceact.eu/article/50/)
(effective August 2026) requires that AI-generated text content be
"marked in a machine-readable format and detectable as artificially
generated or manipulated."

## CPoP Compliance

CPoP provides the inverse proof — machine-readable attestation that content
was **human-authored** — which satisfies Article 50 by distinguishing
AI-generated from human-generated content.

### Machine-Readable Disclosure

| Mechanism                        | Format                                                |
| -------------------------------- | ----------------------------------------------------- |
| Evidence Packet                  | CBOR-encoded with IPTC `digitalSourceType`            |
| HTML meta tag                    | `<meta name="ai-disclosure" content="none">`          |
| C2PA assertion                   | `com.writerslogic.pop-attestation.v1`                 |
| ZWC watermark                    | Invisible steganographic content binding              |
| W3C Verifiable Credential        | `ProcessAttestationCredential` with DID binding       |

### Declaration Structure

CPoP declarations include structured AI disclosure:

- `ai_tools[]` — List of AI tools used, with name, version, purpose, and extent
- `AiExtent` — Categorical AI involvement level (None, Minimal, Moderate, Substantial)
- `input_modalities[]` — Typed/dictated/pasted with percentages

This data is cryptographically signed by the author, providing non-repudiable
disclosure of AI involvement (or lack thereof).

## References

- [EU AI Act — Article 50](https://artificialintelligenceact.eu/article/50/)
- [W3C AI Content Disclosure Community Group](https://www.w3.org/community/ai-content-disclosure/)
