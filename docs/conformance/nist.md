[//]: # (SPDX-License-Identifier: Apache-2.0)

# NIST AI RMF / AI 100-4 Conformance

> **Status**: Mapped — subcategory alignment documented

## NIST AI RMF 1.0 (AI 100-1)

The [NIST AI Risk Management Framework](https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf)
defines four functions (GOVERN, MAP, MEASURE, MANAGE) with 72 subcategories.
CPoP addresses the following:

| Subcategory | Description                                      | CPoP Coverage                                                   |
| ----------- | ------------------------------------------------ | --------------------------------------------------------------- |
| GV-1.1      | Legal/regulatory requirements documented          | Declaration with AI disclosure fields per EU AI Act Article 50  |
| GV-1.2      | Trustworthiness characteristics integrated        | AR4SI trustworthiness vector (8 components)                     |
| MS-2.6      | AI system performance assessed                    | Forensic assessment score with 5 verdict levels (V1–V5)        |
| MS-2.11     | Fairness assessed and documented                  | Biological plausibility ranges, not demographic profiling       |
| MG-4.1      | Post-deployment monitoring                        | Continuous sentinel monitoring with checkpoint chain             |

## NIST AI 100-4 (Reducing Risks from Synthetic Content)

[AI 100-4](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.100-4.pdf)
addresses four provenance techniques. CPoP implements three:

| Technique               | CPoP Implementation                              |
| ----------------------- | ------------------------------------------------ |
| Provenance metadata     | Evidence Packet with document refs, timestamps, author identity, edit history |
| Digital watermarking     | ZWC steganographic watermark (`com.writerslogic.zwc-watermark.1`) |
| Content authentication  | Ed25519 signatures binding metadata to content via checkpoint chain |
| Content detection       | Forensic analysis (15+ behavioral checks, SNR, Lyapunov, labyrinth) |

## References

- [NIST AI 100-1 — AI Risk Management Framework](https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf)
- [NIST AI 100-4 — Reducing Risks Posed by Synthetic Content](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.100-4.pdf)
