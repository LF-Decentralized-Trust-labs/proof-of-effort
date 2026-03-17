[//]: # (SPDX-License-Identifier: CC-BY-4.0)

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- IETF Internet-Draft: `draft-condrey-cpop-protocol` — CPoP architecture and evidence format.
- IETF Internet-Draft: `draft-condrey-cpop-appraisal` — Forensic appraisal and security model.
- CDDL schema for CPoP evidence packets and attestation results (`cddl/cpop.cddl`).
- Build tooling based on [martinthomson/i-d-template](https://github.com/martinthomson/i-d-template).
- GitHub Actions workflows for editor's copy publishing, IETF datatracker submission, and generated file updates.
- LF Decentralized Trust governance files: CODE_OF_CONDUCT.md, SECURITY.md, MAINTAINERS.md, CONTRIBUTING.md.
- Architecture overview and integration documentation for C2PA, CAWG, and DIF.
- Issue and pull request templates.
- Reference implementation crates: `cpop-jitter` (timing entropy) and `cpop-protocol` (wire format, CBOR/COSE codec).
- Rust CI workflow for build, test, clippy, and format checks.

[Unreleased]: https://github.com/LF-Decentralized-Trust-labs/proof-of-process/commits/main
