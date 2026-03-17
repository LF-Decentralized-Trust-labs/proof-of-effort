[//]: # (SPDX-License-Identifier: Apache-2.0)

<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/branding/production/png/pop-logo-white.png" width="420">
  <source media="(prefers-color-scheme: light)" srcset="assets/branding/production/png/pop-logo-black.png" width="420">
  <img alt="Proof-of-Process" src="assets/branding/production/png/pop-logo-black.png" width="420">
</picture>

### Cryptographic attestation of human cognitive involvement in digital content creation

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge)](LICENSE)
[![IETF Drafts](https://img.shields.io/badge/IETF-Internet--Drafts-blue?style=for-the-badge&logo=ietf)](https://lf-decentralized-trust-labs.github.io/proof-of-process/)
[![OpenSSF Scorecard](https://img.shields.io/ossf-scorecard/github.com/LF-Decentralized-Trust-labs/proof-of-process?style=for-the-badge&label=scorecard)](https://scorecard.dev/viewer/?uri=github.com/LF-Decentralized-Trust-labs/proof-of-process)

[![RATS WG](https://img.shields.io/badge/IETF-RATS_WG-informational?style=flat-square)](https://datatracker.ietf.org/wg/rats/about/)
[![RFC 9334](https://img.shields.io/badge/RFC-9334-informational?style=flat-square)](https://www.rfc-editor.org/rfc/rfc9334)
[![C2PA](https://img.shields.io/badge/C2PA-Content_Provenance-informational?style=flat-square)](https://c2pa.org/)
[![W3C VC](https://img.shields.io/badge/W3C-Verifiable_Credentials-informational?style=flat-square)](https://www.w3.org/TR/vc-data-model-2.0/)

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

IETF draft content is additionally subject to the [IETF Trust Legal Provisions](https://trustee.ietf.org/trust-legal-provisions.html). See [LICENSE.md](LICENSE.md) and [CONTRIBUTING.md](CONTRIBUTING.md).
