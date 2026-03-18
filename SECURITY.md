[//]: # (SPDX-License-Identifier: CC-BY-4.0)

# Proof-of-Process, an LF Decentralized Trust Project Security Policy

## About this document

This document defines how security vulnerability reporting is handled in Proof-of-Process, an LF Decentralized Trust Project.
The approach aligns with the [LF Decentralized Trust Security Policy]. Please
review that document to understand the basis of the security reporting for Proof-of-Process.

This vulnerability policy borrows heavily from the
recommendations of the OpenSSF Vulnerability Disclosure working group. For
up-to-date information on the latest recommendations related to vulnerability
disclosures, please visit the [GitHub of that working
group](https://github.com/ossf/wg-vulnerability-disclosures).

If you are already familiar with the security policies of Proof-of-Process, and
ready to report a vulnerability, please jump to [Report Intakes](#report-intakes).

[LF Decentralized Trust Security Policy]: https://lf-decentralized-trust.github.io/governance/governing-documents/security

## What Is a Vulnerability Disclosure Policy?

This document explains how to report security vulnerabilities to the
Proof-of-Process project and what to expect from the security team in
response.

## Security Team

The current Proof-of-Process security team is:

| Name            | Email ID               | Discord ID | Area/Specialty        |
| --------------- | ---------------------- | ---------- | --------------------- |
| David Condrey   | david@writerslogic.com | condreydavid | Protocol, CDDL schema |

> **Note:** The LFDT security policy recommends at least three security team
> members. The team is currently below the recommended size and is actively
> seeking additional members. Until the team reaches full capacity, security
> team responsibilities are carried by the project maintainer with support
> from the [LF Decentralized Trust Community Architects].

### Joining the Security Team

The project welcomes contributors who are interested in joining the security
team. Candidates should have familiarity with the CPoP protocol and its CDDL
schema, and a willingness to participate in vulnerability triage and
coordinated disclosure. Prior experience with security response processes is
valued but not required.

To volunteer, contact the project maintainer (David Condrey,
david@writerslogic.com) or open an issue on this repository expressing your
interest. All additions to the security team are made via approved Pull
Requests to this file.

The security team for Proof-of-Process carries out the following duties and responsibilities.
Members are added and removed from the team via approved Pull Requests to this
repository. For additional background into the role of the security team, see
the [People Infrastructure] section of the LF Decentralized Trust Security Policy.

[People Infrastructure]: https://lf-decentralized-trust.github.io/governance/governing-documents/security.html#people-infrastructure

**Responsibilities:**

1. Acknowledge the receipt of vulnerability reports to the reporter within 2
   business days.

2. Assess the issue. Engage with the reporter to ask any outstanding questions
about the report and how to reproduce it. If the report was received by email
and may be a security vulnerability, open a GitHub Security Advisory on the
repository to manage the report. If the report is not considered a
vulnerability, then the reporter should be informed and this process can be
halted. If the report is a regular bug (but not a security vulnerability), the
reporter should be informed (if necessary) of the regular process for reporting
issues.

3. Some issues may require more time and resources to correct. If a particular
report is complex, discuss an embargo period with the reporter during which
time the report will not be publicly disclosed. The embargo period should be
negotiated with the reporter and must not be longer than 90 days.

4. If necessary, create a private patch development infrastructure for the issue
   by emailing the [LF Decentralized Trust Community Architects].

[LF Decentralized Trust Community Architects]: mailto:community-architects@lfdecentralizedtrust.org

5. Request a CVE for the issue (see the [CNA/CVE Reporting](#cnacve-reporting)
   section).

6. Decide a date for the public release of the vulnerability report, the date
   the embargo period ends.

7. If applicable, notify members of the embargo list of the vulnerability,
upcoming patch and release, as described above.

8. Publish a new (software) release in which the vulnerability is addressed.

9. Publicly disclose the issue within 48 hours after the release via a
GitHub security advisory (see the [(GitHub) Security
Advisories](#github-security-advisories) section for details).

## Discussion Forums

Discussions about each reported vulnerability should be carried out in the
private GitHub security advisory about the vulnerability. If necessary, a private
channel specific to the issue may be created on the LF Decentralized Trust Discord server
with invited participants added to the discussion.

## Report Intakes

Proof-of-Process has the following ways to submit security
vulnerabilities. While the security team members will do their best to
respond to bugs disclosed in all possible ways, it is encouraged for bug
finders to report through the following approved channels:

- Email the [LF Decentralized Trust Foundation security
list](mailto:security@lists.lfdecentralizedtrust.org): To report a security issue, please
send an email with the name of the project/repository, a description of the issue, the
steps you took to create the issue, affected versions, and if known,
mitigations. If in triaging the email, the security team determines the issue may be
a security vulnerability, a [GitHub security vulnerability report] will be
opened.
- Open a [GitHub security vulnerability report]: Open a draft security advisory
on the "Security" tab of this GitHub repository. See [GitHub Security
Advisories](#github-security-advisories) to learn more about the security
infrastructure in GitHub.

[GitHub security vulnerability report]: https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability

## CNA/CVE Reporting

When applicable, Proof-of-Process uses GitHub's **CVE Numbering Authority
(CNA)** capability to request **Common Vulnerabilities and Exposures (CVE)**
identifiers for security vulnerabilities in project tooling and
infrastructure.

## Embargo List

Proof-of-Process maintains a private embargo list. If you wish to
be added to the embargo list, please email the [LF Decentralized Trust Foundation security
mailing list](mailto:security@lists.lfdecentralizedtrust.org), including the project name
(Proof-of-Process) and reason for being added to the embargo list. Requests
will be assessed by the Proof-of-Process security team in conjunction with the
appropriate LF Decentralized Trust Staff, and a decision will be made to accommodate or not
the request.

For more information about the embargo list, please see the [Embargo List
section of the LF Decentralized Trust Security
Policy](https://lf-decentralized-trust.github.io/governance/governing-documents/security.html#embargo-list).

## (GitHub) Security Advisories

Proof-of-Process uses GitHub Security Advisories to manage the public
disclosure of security vulnerabilities.

## Private Patch Deployment Infrastructure

In creating patches and new releases that address security vulnerabilities,
Proof-of-Process uses the private development features of GitHub for security
vulnerabilities. GitHub has [extensive
documentation](https://docs.github.com/en/code-security/security-advisories/repository-security-advisories)
about these features.
