[//]: # (SPDX-License-Identifier: Apache-2.0)

<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/branding/production/png/pop-logo-white.png" width="420">
  <source media="(prefers-color-scheme: light)" srcset="assets/branding/production/png/pop-logo-black.png" width="420">
  <img alt="Cryptographic Proof of Process" src="assets/branding/production/png/pop-logo-black.png" width="420">
</picture>

### Cryptographic attestation of human cognitive involvement in digital content creation

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge)](LICENSE)
[![IETF Drafts](https://img.shields.io/badge/IETF-Internet--Drafts-blue?style=for-the-badge&logo=ietf)](https://lf-decentralized-trust-labs.github.io/proof-of-process/)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/LF-Decentralized-Trust-labs/proof-of-process/badge?style=for-the-badge)](https://scorecard.dev/viewer/?uri=github.com/LF-Decentralized-Trust-labs/proof-of-process)

[![cpop-protocol](https://img.shields.io/crates/d/cpop-protocol?style=flat-square&label=cpop-protocol)](https://crates.io/crates/cpop-protocol)
[![cpop-jitter](https://img.shields.io/crates/d/cpop-jitter?style=flat-square&label=cpop-jitter)](https://crates.io/crates/cpop-jitter)
[![C2PA](https://img.shields.io/badge/C2PA-integration_proposed-orange?style=flat-square)](docs/integration/c2pa.md)
[![RATS WG](https://img.shields.io/badge/IETF-RATS_WG-informational?style=flat-square)](https://datatracker.ietf.org/wg/rats/about/)
[![RFC 9334](https://img.shields.io/badge/RFC-9334-informational?style=flat-square)](https://www.rfc-editor.org/rfc/rfc9334)

An [LF Decentralized Trust](https://www.lfdecentralizedtrust.org/) Lab

[Read the Spec][protocol-html] · [Architecture](docs/architecture.md) · [CDDL Schema](cddl/cpop.cddl) · [Contributing](CONTRIBUTING.md)

</div>

---

> [!IMPORTANT]
> **Status**: These are individual Internet-Drafts, not yet adopted by any IETF working group. The protocol is under active development. Feedback is welcome via [GitHub Issues](https://github.com/LF-Decentralized-Trust-labs/proof-of-process/issues) or the [RATS mailing list](https://mailarchive.ietf.org/arch/browse/rats/).

## What This Repository Contains

This repository is the home of the **Proof-of-Process (CPoP) protocol specification** — two IETF Internet-Drafts, a formal CDDL data schema, and supporting documentation for a cryptographic attestation protocol for evaluating human cognitive involvement in digital content creation.

**Concretely, this repo contains:**

- **Two IETF Internet-Drafts** in [kramdown-rfc](https://github.com/cabo/kramdown-rfc) format, built and published automatically via GitHub Actions:
  - [`draft-condrey-cpop-protocol`](draft-condrey-cpop-protocol.md) — The core protocol: architecture, evidence format, and wire encoding
  - [`draft-condrey-cpop-appraisal`](draft-condrey-cpop-appraisal.md) — The appraisal methodology: forensic evaluation, security model, and trust calibration
- **A CDDL schema** ([`cddl/cpop.cddl`](cddl/cpop.cddl)) defining the CBOR-encoded wire format for Evidence Packets and Writers Authenticity Reports
- **Architecture and integration documentation** mapping CPoP to [C2PA](docs/integration/c2pa.md), [CAWG](docs/integration/cawg.md), and [DID/VC](docs/integration/did.md) ecosystems
- **Reference implementation crates** in Rust:
  - [`cpop-jitter`](crates/cpop-jitter/) — Timing jitter entropy primitive (`no_std` compatible)
  - [`cpop-protocol`](crates/cpop-protocol/) — Wire format, CBOR/COSE codec, evidence builder/verifier
- **A complete build pipeline** ([`Makefile`](Makefile) + [GitHub Actions](.github/workflows/)) that compiles drafts to HTML/TXT and publishes editor's copies to GitHub Pages

## Why This Exists

We can verify *who* performed a digital action. We cannot verify *whether a human was actually involved*. That distinction matters now — generative AI can produce text, code, and media that is indistinguishable from human output. No existing protocol provides cryptographic attestation of human cognitive involvement in content creation.

CPoP defines a way to collect behavioral evidence (keystroke dynamics, pause patterns, editing trajectories) during content creation and turn it into a cryptographically verifiable attestation result. No biometric databases. No surveillance. Physics-constrained evidence that a human process occurred.

## How the Protocol Works

CPoP is built on the [IETF RATS architecture (RFC 9334)](https://www.rfc-editor.org/rfc/rfc9334), using the standard Attester → Verifier → Relying Party topology:

```
                         Reference Values
                          (baselines)
                              │
                              ▼
  ┌────────────┐  Evidence  ┌────────────┐  Attestation  ┌────────────┐
  │            │  Packet    │            │  Result        │            │
  │  Attester  │──────────→ │  Verifier  │ ─────────────→│  Relying   │
  │            │  (CPOP)    │            │  (CWAR)        │  Party     │
  └────────────┘            └────────────┘                └────────────┘
       │                          │
       │ Collects:                │ Evaluates:
       │ · Keystroke dynamics     │ · Behavioral entropy
       │ · Pause patterns         │ · Baseline divergence
       │ · Editing trajectories   │ · Forgery cost analysis
       │ · HW attestation         │ · Confidence scoring
```

1. **Attester** — Captures behavioral evidence during content creation and packages it into CBOR-encoded Evidence Packets (tag `1129336656` / `CPOP`).
2. **Verifier** — Evaluates evidence against human-process baselines and produces Writers Authenticity Reports (tag `1129791826` / `CWAR`).
3. **Relying Party** — Consumes attestation results to make trust decisions about content provenance.

See [`docs/architecture.md`](docs/architecture.md) for the full RATS role mapping with Endorser and Reference Value Provider flows.

## Specification

| Draft | Title | Editor's Copy | Status |
| ----- | ----- | :-----------: | :----: |
| `draft-condrey-cpop-protocol` | Architecture and Evidence Format | [HTML][protocol-html] | Active |
| `draft-condrey-cpop-appraisal` | Forensic Appraisal and Security Model | [HTML][appraisal-html] | Active |

[protocol-html]: https://lf-decentralized-trust-labs.github.io/proof-of-process/draft-condrey-cpop-protocol.html
[appraisal-html]: https://lf-decentralized-trust-labs.github.io/proof-of-process/draft-condrey-cpop-appraisal.html

Editor's copies are rebuilt on every push to `main` and published via GitHub Pages.

## CDDL Schema

The wire format is formally defined in [`cddl/cpop.cddl`](cddl/cpop.cddl) using [CDDL (RFC 8610)](https://www.rfc-editor.org/rfc/rfc8610):

| Structure | CBOR Tag | Mnemonic | Description |
| --------- | -------- | -------- | ----------- |
| `evidence-packet` | `1129336656` | `CPOP` | Behavioral telemetry, document refs, session context, crypto bindings |
| `attestation-result` | `1129791826` | `CWAR` | Writers Authenticity Report: appraisal verdict, entropy scores, confidence, forensic metadata |

All structures use CBOR integer-keyed maps. Timestamps are in milliseconds; entropy estimates are in centibits (1/100th of a bit).

## Ecosystem

CPoP sits beneath existing provenance and identity frameworks, adding an "evidence of effort" layer:

| Ecosystem | Role | Integration Guide |
| --------- | ---- | :---------------: |
| [IETF RATS WG](https://datatracker.ietf.org/wg/rats/about/) | CPoP implements the RATS architecture (RFC 9334) | [Mailing list](https://mailarchive.ietf.org/arch/browse/rats/) |
| [C2PA](https://c2pa.org/) | CPoP evidence as assertions within content credential manifests | [Guide](docs/integration/c2pa.md) |
| [CAWG](https://cawg.io/) | Creator process assertions alongside CAWG identity assertions | [Guide](docs/integration/cawg.md) |
| [W3C VC](https://www.w3.org/TR/vc-data-model-2.0/) / [DIF](https://identity.foundation/) | Attestation results as verifiable credentials bound to DIDs | [Guide](docs/integration/did.md) |
| [IETF SECDISPATCH](https://datatracker.ietf.org/wg/secdispatch/about/) | Venue for routing new security protocol work within the IETF | [Mailing list](https://mailarchive.ietf.org/arch/browse/secdispatch/) |

### Integration Status

| Integration | Stage | Status | External Dependencies |
| ----------- | ----- | ------ | --------------------- |
| [C2PA](docs/integration/c2pa.md) | Proposed | Soft binding registered | [c2pa-org/softbinding-algorithm-list#45](https://github.com/c2pa-org/softbinding-algorithm-list/pull/45) |
| [CAWG](docs/integration/cawg.md) | Proposal | Not yet submitted | — |
| [DID/VC](docs/integration/did.md) | Under development | Schema in progress | — |

## Standards Conformance

CPoP is designed to interoperate with existing provenance, identity, and governance frameworks. Detailed conformance documentation is in [`docs/conformance/`](docs/conformance/).

| Standard | Status | Documentation |
| -------- | ------ | :-----------: |
| IETF RATS (RFC 9334, EAT, EAR, AR4SI) | **Normative** | [rats.md](docs/conformance/rats.md) |
| CBOR / COSE (RFC 8949, RFC 9052) | **Normative** | [cbor-cose.md](docs/conformance/cbor-cose.md) |
| C2PA Content Credentials | Proposed | [c2pa.md](docs/integration/c2pa.md) |
| W3C DID Core 1.0 | Implemented | [did.md](docs/conformance/did.md) |
| W3C VC Data Model 2.0 | Implemented | [vc.md](docs/conformance/vc.md) |
| IPTC Digital Source Type | Implemented | [iptc.md](docs/conformance/iptc.md) |
| NIST AI RMF 1.0 / AI 100-4 | Mapped | [nist.md](docs/conformance/nist.md) |
| ISO/IEC 42001 (AIMS) | Mapped | [iso42001.md](docs/conformance/iso42001.md) |
| EU AI Act Article 50 | Aligned | [eu-ai-act.md](docs/conformance/eu-ai-act.md) |
| WGA MBA / SAG-AFTRA | Mapped | [creative-rights.md](docs/conformance/creative-rights.md) |

## Building Locally

<details>
<summary><strong>Prerequisites</strong></summary>

- Python 3 with `pip`
- Ruby with `gem`
- GNU Make

</details>

```sh
pip install xml2rfc   # virtualenv recommended
gem install kramdown-rfc
make
```

On first run, `make` clones the [i-d-template](https://github.com/martinthomson/i-d-template) toolchain into `lib/`.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for DCO sign-off requirements, IETF intellectual property terms, and contribution workflow.

This project follows the [LF Decentralized Trust Code of Conduct](CODE_OF_CONDUCT.md).

## Project Governance

| Document | Purpose |
| -------- | ------- |
| [MAINTAINERS.md](MAINTAINERS.md) | Active maintainers and governance process |
| [SECURITY.md](SECURITY.md) | Vulnerability disclosure policy |
| [CHANGELOG.md](CHANGELOG.md) | Notable changes |

## License

This project is licensed under the [Apache License, Version 2.0](LICENSE).

IETF draft content is additionally subject to the [IETF Trust Legal Provisions](https://trustee.ietf.org/trust-legal-provisions.html). See [LICENSE.md](LICENSE.md) for details on how the Apache-2.0 license and IETF TLP interact, and [CONTRIBUTING.md](CONTRIBUTING.md) for contributor obligations.
