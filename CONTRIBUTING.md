[//]: # (SPDX-License-Identifier: CC-BY-4.0)

# Contributing to Proof-of-Process

This document covers how to contribute to the CPoP protocol specification.

## IETF Intellectual Property Notice

This repository relates to activities in the Internet Engineering Task Force
([IETF](https://www.ietf.org/)). All material in this repository is considered
Contributions to the IETF Standards Process, as defined in the intellectual
property policies of IETF currently designated as
[BCP 78](https://www.rfc-editor.org/info/bcp78),
[BCP 79](https://www.rfc-editor.org/info/bcp79) and the
[IETF Trust Legal Provisions (TLP) Relating to IETF Documents](https://trustee.ietf.org/trust-legal-provisions.html).

Any edit, commit, pull request, issue, comment or other change made to this
repository constitutes Contributions to the IETF Standards Process
(https://www.ietf.org/).

You agree to comply with all applicable IETF policies and procedures, including,
BCP 78, 79, the TLP, and the TLP rules regarding code components (e.g. being
subject to a Simplified BSD License) in Contributions.

## Developer Certificate of Origin

All contributions to this project must be signed off under the
[Developer Certificate of Origin (DCO)](https://developercertificate.org/).
The DCO is a lightweight way to certify that you wrote or otherwise have the
right to submit your contribution. Every commit must include a `Signed-off-by`
trailer matching the commit author:

```
Signed-off-by: Your Name <your.email@example.com>
```

Use `git commit -s` to add this automatically. The DCO sign-off is enforced on
all pull requests.

## How to Contribute

### Reporting Issues

- Use the [GitHub issue tracker](https://github.com/LF-Decentralized-Trust-labs/proof-of-process/issues)
  to report bugs or suggest improvements.
- Search existing issues before opening a new one.

### Submitting Changes

1. Fork the repository and create a feature branch from `main`.
2. Make your changes. For spec edits, the drafts use
   [kramdown-rfc](https://github.com/cabo/kramdown-rfc) syntax.
3. Build locally with `make` to verify your changes compile. This requires
   `xml2rfc`, `kramdown-rfc`, and the
   [i-d-template](https://github.com/martinthomson/i-d-template) toolchain.
4. Commit with DCO sign-off (`git commit -s`).
5. Open a pull request against `main`.

### What to Contribute

Contributions are welcome across the following areas:

- **Specification text** — Clarifications, corrections, or new sections for the
  protocol or appraisal drafts.
- **CDDL schema** — Improvements or extensions to the evidence and attestation
  result schema in `cddl/`.
- **Integration documentation** — Guides for C2PA, CAWG, DIF, or other
  ecosystem integration patterns in `docs/integration/`.
- **Build and CI improvements** — Enhancements to the build pipeline or
  automated validation.

### Style Guidelines

- Draft text follows the conventions of
  [kramdown-rfc](https://github.com/cabo/kramdown-rfc) and
  [RFC 7322 (RFC Style Guide)](https://www.rfc-editor.org/rfc/rfc7322).
- CDDL schemas follow [RFC 8610](https://www.rfc-editor.org/rfc/rfc8610)
  notation conventions.
- Use descriptive commit messages.

## Code of Conduct

This project follows the
[LF Decentralized Trust Code of Conduct](CODE_OF_CONDUCT.md).

## Questions

Open an [issue](https://github.com/LF-Decentralized-Trust-labs/proof-of-process/issues)
or post to the [IETF RATS mailing list](https://mailarchive.ietf.org/arch/browse/rats/).
