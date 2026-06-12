[//]: # (SPDX-License-Identifier: CC-BY-4.0)

# Governance

This document defines the governance model for the Proof of Effort (CPoE)
project, an [LF Decentralized Trust](https://www.lfdecentralizedtrust.org/) lab.

## Principles

1. **Openness.** Participation is open to anyone willing to abide by the
   [Code of Conduct](CODE_OF_CONDUCT.md) and the contribution requirements in
   [CONTRIBUTING.md](CONTRIBUTING.md).
2. **Transparency.** Decisions are made in public. GitHub Issues, Pull Requests,
   and mailing list threads are the authoritative record.
3. **Merit.** Influence is earned through sustained, quality contributions.
4. **Consensus.** Decisions are reached by consensus among Maintainers. Voting
   is a last resort.

## Roles

### Contributors

Anyone who opens an issue, submits a pull request, reviews code, or
participates in project discussions. Contributors must sign off on the
[Developer Certificate of Origin (DCO)](https://developercertificate.org/) for
all commits.

### Maintainers

Maintainers have merge authority, release responsibility, and stewardship of the
project's technical direction. They are listed in
[MAINTAINERS.md](MAINTAINERS.md), which also defines the process for adding and
removing Maintainers.

All Maintainers share equal authority. There is no hierarchy among Maintainers.

### Security Team

The Security Team handles vulnerability reports per
[SECURITY.md](SECURITY.md). Membership is drawn from Maintainers and trusted
Contributors.

## Decision-Making

### Consensus

The project operates by lazy consensus: a proposal (issue, PR, or mailing list
post) is considered accepted if no Maintainer objects within a reasonable review
period (typically 7 calendar days for non-trivial changes, 72 hours for routine
changes).

Silence is assent. If a Maintainer has concerns, they are expected to raise them
during the review period.

### Objections

Any Maintainer may object to a proposal. Objections must include a technical
rationale. The proposer and objecting Maintainer should attempt to resolve the
disagreement through discussion on the relevant issue or PR.

### Voting

If consensus cannot be reached after good-faith discussion, any Maintainer may
call for a vote. Votes are conducted as follows:

- Each active Maintainer has one vote.
- Voting is public, via comments on the relevant issue or PR.
- The voting period is 7 calendar days from the call for vote.
- A simple majority of active Maintainers is required to pass.
- In the event of a tie, the proposal does not pass.

### Scope of Authority

| Decision Type | Required Approval |
| ------------- | ----------------- |
| Routine PR (bug fix, docs, CI) | One Maintainer approval |
| Specification change (draft text, CDDL schema) | Two Maintainer approvals, or lazy consensus after 7 days |
| New integration or ecosystem binding | Two Maintainer approvals, or lazy consensus after 7 days |
| Release (crate publish, draft submission) | One Maintainer approval, with 72-hour notice to all Maintainers |
| Architectural change | Consensus among all active Maintainers |
| Add or remove Maintainer | Per the process in [MAINTAINERS.md](MAINTAINERS.md) |
| Change to this governance document | Consensus among all active Maintainers |

## IETF Considerations

The IETF Internet-Drafts in this repository are subject to the
[IETF Trust Legal Provisions](https://trustee.ietf.org/trust-legal-provisions.html)
and the intellectual property policies in
[BCP 78](https://www.rfc-editor.org/info/bcp78) and
[BCP 79](https://www.rfc-editor.org/info/bcp79).

Governance of the specification text is shared between this project's Maintainers
and the IETF process. If the drafts are adopted by an IETF working group, the
working group's consensus process takes precedence for specification content,
while this governance document continues to apply to the repository,
implementations, and project operations.

## Conflict Resolution

1. **Technical disagreements** are resolved through the decision-making process
   above.
2. **Conduct violations** are handled per the
   [Code of Conduct](CODE_OF_CONDUCT.md), with escalation to the
   LF Decentralized Trust conduct contacts as defined in that document.
3. **Governance disputes** that cannot be resolved among Maintainers may be
   escalated to the LF Decentralized Trust Technical Advisory Council (TAC)
   for mediation.

## LF Decentralized Trust Oversight

This project operates under the umbrella of LF Decentralized Trust. The
LFDT Technical Advisory Council (TAC) provides oversight and guidance.
Maintainers are expected to:

- Respond to TAC requests regarding repository contents and management.
- Contribute to quarterly reports as required for lab and project status.
- Follow LFDT policies on security, licensing, and community standards.

The TAC does not make day-to-day technical decisions but may intervene in
governance disputes or policy compliance matters.

## Amendments

Changes to this document require consensus among all active Maintainers, with a
minimum review period of 14 calendar days. Amendments take effect upon merge to
the `main` branch.
