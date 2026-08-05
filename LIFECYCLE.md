[//]: # (SPDX-License-Identifier: CC-BY-4.0)

# Project Lifecycle

This document tracks the Proof of Effort (CPoE) project's progress through
the [LF Decentralized Trust project lifecycle][lifecycle]. It is a living
checklist, updated manually as milestones are reached.

**Current stage: Labs**

| Stage | Status |
| ----- | ------ |
| Labs | **Current** |
| Incubation | Not yet proposed |
| Graduated | -- |

[lifecycle]: https://lf-decentralized-trust.github.io/governance/governing-documents/project-lifecycle/

---

## Labs → Incubation

Entry to Incubation requires a formal proposal to the TAC. The criteria below
are drawn from the [Incubation Entry Considerations][entry].

[entry]: https://lf-decentralized-trust.github.io/governance/guidelines/project-incubation-entry-considerations/

### Governance and Community

<!-- lifecycle:maintainer-count -->
- [ ] **Multiple maintainers** — at least 2 from different organizations
  - Current: 1 maintainer, 1 organization
<!-- /lifecycle:maintainer-count -->

<!-- lifecycle:contributor-count -->
- [ ] **Active contributors** — multiple non-maintainer contributors with merged PRs
  - Current: 1 external contributor with a merged PR (#97), alongside 1 maintainer
<!-- /lifecycle:contributor-count -->

- [ ] **TAC sponsors** — at least 2 sponsors from different organizations willing to champion the proposal

- [x] **Code of Conduct** — LF Decentralized Trust CoC adopted ([CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md))

- [x] **Governance documented** — decision-making process defined ([GOVERNANCE.md](GOVERNANCE.md))

- [x] **Contributing guide** — contribution workflow, DCO, and IP terms ([CONTRIBUTING.md](CONTRIBUTING.md))

- [ ] **Public community channel** — Discord, mailing list, or regular community calls

### Legal and Licensing

- [x] **Apache-2.0 license** on all code ([LICENSE](LICENSE))

- [x] **IETF TLP compliance** for specification content ([LICENSE.md](LICENSE.md))

- [x] **DCO enforcement** — Signed-off-by required on all commits

- [ ] **Trademark** — project name cleared or transferred to LFDT

### Technical Maturity

<!-- lifecycle:release-count -->
- [ ] **At least one release** — published artifacts exist
  - Current: none. Two git tags (`cpop-jitter-v0.2.1`, `cpop-protocol-v0.1.1`) point at a
    single commit; no crate of either name is published, and there are no GitHub Releases
<!-- /lifecycle:release-count -->

- [x] **CI/CD pipeline** — automated build, test, and release ([6 workflows](.github/workflows/))

- [x] **Dependency management** — Dependabot configured for 3 ecosystems

<!-- lifecycle:scorecard -->
- [x] **OpenSSF Scorecard** — automated scoring in CI
  - Current: scorecard.yml runs weekly; badge referenced in README
<!-- /lifecycle:scorecard -->

- [ ] **OpenSSF Best Practices Badge** — badge application started

- [x] **Security policy** — vulnerability disclosure process ([SECURITY.md](SECURITY.md))

- [ ] **Security team** — at least 3 members recommended by LFDT policy
  - Current: 1 member

### Distinct Value

- [x] **Unique contribution** — no existing LFDT project provides cryptographic attestation of human cognitive involvement in content creation

- [x] **Standards alignment** — implements IETF RATS architecture (RFC 9334); two Internet-Drafts submitted

---

## Incubation → Graduated

Graduation is evaluated on process maturity, not product maturity. The criteria
below are drawn from the [Incubation Exit Criteria][exit]. These are scored by
the TAC during annual reviews.

[exit]: https://lf-decentralized-trust.github.io/governance/governing-documents/project-incubation-exit/

### Legal (must score 10)

- [x] Apache-2.0 license
- [ ] OpenSSF License score = 10
- [ ] Trademark cleared with LFDT

### Diversity

<!-- lifecycle:org-count -->
- [ ] **MAINTAINERS.md lists 3+ organizations**
  - Current: 1 organization
<!-- /lifecycle:org-count -->

- [ ] No single entity is vital to the project's success

### Releases

<!-- lifecycle:yearly-releases -->
- [ ] **Minimum 2 releases per year**
  - Current: no published releases
<!-- /lifecycle:yearly-releases -->

- [ ] Packaging score SHOULD = 10 (OpenSSF)

### Testing

- [x] CI tests exist (CDDL validation, link checking)
- [ ] Rust test suite runs in CI — the crates under `impl/` are not built or tested by any workflow
- [ ] Comprehensive unit and integration test suites with documented coverage
- [ ] CI test score SHOULD = 10 (OpenSSF)

### Security

- [ ] Dangerous Workflow score MUST = 10
- [ ] Token Permissions score MUST = 10
- [ ] Branch Protection score MUST = 9
- [ ] Dependency Update score MUST = 10
- [ ] Signed Releases score MUST = 10
- [ ] Security audit or review completed

### Structure

- [x] Common repo structure (README, CONTRIBUTING, CODE_OF_CONDUCT, LICENSE, SECURITY, MAINTAINERS, GOVERNANCE, CHANGELOG)
- [x] GitHub Community Standards checklist

### Maintenance

- [ ] Code Review score MUST = 10 (all PRs require review)
- [ ] Active Discord/mailing list presence
- [ ] Regular community calls or async coordination

### Production Adoption

- [ ] **ADOPTERS.md** with real-world usage documented

### Documentation

- [x] Documentation website (GitHub Pages with rendered IETF drafts)
- [x] Architecture and integration guides ([docs/](docs/))

---

## Graduated → Dormant

A project enters Dormant status if:

- No releases for 12+ months
- No maintainer activity for 6+ months
- TAC determines the project no longer meets Graduated criteria

Anyone can propose this transition. The community has an opportunity to contest
before the TAC makes a final decision.

## Dormant → Archived

Automatic after 6 months in Dormant status. Reactivation requires a new
proposal to the TAC, following the same process as Incubation entry.

---

## Status History

| Date | Event |
| ---- | ----- |
| 2026-02 | First IETF Internet-Drafts submitted (draft-condrey-rats-pop-protocol-00, -appraisal-00) |
| 2026-03 | Repository created as LF Decentralized Trust lab |
| 2026-03 | GOVERNANCE.md added; lifecycle tracking started |
