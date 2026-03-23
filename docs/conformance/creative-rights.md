[//]: # (SPDX-License-Identifier: Apache-2.0)

# Creative Rights Conformance (WGA MBA / SAG-AFTRA)

> **Status**: Mapped — authorship attestation satisfies disclosure requirements

## WGA Minimum Basic Agreement (2023)

The [WGA 2023 MBA](https://www.wgacontract2023.org/the-campaign/summary-of-the-2023-wga-mba)
establishes that:

1. AI is not a writer — GAI-produced material cannot be "literary material"
2. AI output is not source material for undermining writer credit
3. Companies must disclose if materials provided contain AI-generated content
4. Writers may choose to use AI (with company consent) but cannot be required to

### CPoP Coverage

| MBA Requirement                       | CPoP Mechanism                                         |
| ------------------------------------- | ------------------------------------------------------ |
| Prove human authorship                | Behavioral forensic analysis (15+ checks, V1–V5 verdict) |
| Disclose AI tool usage                | Declaration `ai_tools[]` with name, version, extent    |
| Distinguish human vs AI content       | IPTC `digitalSourceType` + `AiExtent` mapping          |
| Non-repudiable disclosure             | Ed25519-signed declaration bound to evidence chain     |
| Verifiable by third parties           | Public verification via `cpop verify` or WritersProof API |

## SAG-AFTRA AI Provisions

The [SAG-AFTRA contracts](https://www.sagaftra.org/contracts-industry-resources/member-resources/artificial-intelligence)
require informed consent for digital replicas and clear disclosure of
AI-generated performer content.

### CPoP Coverage

| SAG-AFTRA Requirement                 | CPoP Mechanism                                         |
| ------------------------------------- | ------------------------------------------------------ |
| Distinguish human vs AI performance   | Behavioral attestation (keystroke dynamics, entropy)   |
| Consent chain                         | Declaration signed by author with DID identity         |
| Content provenance                    | Checkpoint chain with VDF time proofs                  |

## Compliance Check

The reference implementation provides a `CreativeRightsCompliance` structure
that derives compliance status from a CPoP declaration and EAR token:

- `human_authored` — Whether EAR verdict affirms human authorship
- `gai_source_disclosed` — Whether AI tools are disclosed per MBA Section 72
- `wga_mba_compliant` — Composite compliance flag

## References

- [WGA 2023 MBA Summary](https://www.wgacontract2023.org/the-campaign/summary-of-the-2023-wga-mba)
- [SAG-AFTRA AI Resources](https://www.sagaftra.org/contracts-industry-resources/member-resources/artificial-intelligence)
