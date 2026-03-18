[//]: # (SPDX-License-Identifier: CC-BY-4.0)

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## 2026-03-17

### Added

- IETF Internet-Draft: `draft-condrey-cpop-protocol` — CPoP architecture and evidence format.
- IETF Internet-Draft: `draft-condrey-cpop-appraisal` — Forensic appraisal and security model.
- CDDL schema for CPoP evidence packets and attestation results (`cddl/cpop.cddl`).
- Architecture overview (`docs/architecture.md`) and integration docs for C2PA, CAWG, and DIF.
- Reference implementation crates: `cpop-jitter` (timing entropy) and `cpop-protocol` (wire format, CBOR/COSE codec).
- LF Decentralized Trust governance files: CODE_OF_CONDUCT.md, SECURITY.md, MAINTAINERS.md, CONTRIBUTING.md.
- Build tooling based on [martinthomson/i-d-template](https://github.com/martinthomson/i-d-template) with Makefile.
- GitHub Actions workflows for editor's copy publishing, IETF datatracker submission, and generated file updates.
- Rust CI workflow for build, test, clippy, and format checks.
- Issue and pull request templates.
- Dependabot configuration for GitHub Actions.
- Project branding assets (icon and logo in SVG/PNG).
- `.editorconfig` and `.gitignore`.
- NOTICE and LICENSE.md files.

### Changed

- Bumped `actions/cache` from 4 to 5, `actions/upload-artifact` from 4 to 7, `actions/checkout` from 5 to 6 via Dependabot.

## 2026-03-15

### Added

- Initial repository creation with Apache-2.0 license.

[Unreleased]: https://github.com/LF-Decentralized-Trust-labs/proof-of-process/commits/main
