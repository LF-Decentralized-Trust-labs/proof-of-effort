---
v: 3
docname: draft-condrey-cpoe-usecases-latest
title: "Cryptographic Proof of Effort (CPoE): Use Cases and Deployment Considerations"
abbrev: CPoE Use Cases
category: info
ipr: trust200902
submissiontype: independent
date: 2026-05
area: Security
workgroup: Individual Submission
keyword:
  - attestation
  - authorship
  - process evidence
  - content authenticity

stand_alone: yes
pi:
  toc: yes
  tocdepth: "4"
  sortrefs: yes
  symrefs: yes

author:
  - fullname: David Condrey
    initials: D.
    surname: Condrey
    organization: WritersLogic Inc
    abbrev: WritersLogic
    city: San Diego
    region: California
    country: United States
    email: david@writerslogic.com

normative:
  CPoE-Protocol:
    title: "Cryptographic Proof of Effort (CPoE): Architecture and Evidence Format"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-protocol-00
  CPoE-Appraisal:
    title: "Cryptographic Proof of Effort (CPoE): Forensic Appraisal and Security Model"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-appraisal-00

informative:
  Liang2023:
    title: "GPT Detectors Are Biased Against Non-Native English Writers"
    target: "https://doi.org/10.1016/j.patter.2023.100779"
    author:
      - fullname: Weixin Liang
        initials: W.
        surname: Liang
      - fullname: Mert Yuksekgonul
        initials: M.
        surname: Yuksekgonul
      - fullname: Yining Mao
        initials: Y.
        surname: Mao
      - fullname: Eric Wu
        initials: E.
        surname: Wu
      - fullname: James Zou
        initials: J.
        surname: Zou
    date: 2023
    seriesinfo:
      "Patterns": "4(7)"

--- abstract

This document provides use case analysis and deployment
considerations for the Cryptographic Proof of Effort (CPoE)
framework defined in {{CPoE-Protocol}}. It describes how
process-based authorship attestation addresses concrete needs
across creative industries, academic integrity, journalism,
legal and evidentiary standards, and content platforms. It
also discusses common AI-assisted authoring workflows and how
the Tool Receipt Protocol tracks human-to-machine effort
attribution. This document is informational and does not
define any protocol mechanisms; all normative requirements are
specified in {{CPoE-Protocol}} and {{CPoE-Appraisal}}.

--- middle

# Introduction {#introduction}

The Cryptographic Proof of Effort (CPoE) framework
{{CPoE-Protocol}} provides a standardized, privacy-preserving
mechanism for authors to produce cryptographic evidence of
their creative process. Rather than analyzing finished output
to infer how it was produced, CPoE captures the creation
process itself: behavioral signals, temporal binding, and
content evolution are recorded in tamper-evident Evidence
Packets under the author's sole control.

This document describes the motivating use cases for CPoE.
These use cases informed the design of the protocol and
illustrate how process attestation addresses needs that
neither content detection nor custody-based provenance can
satisfy. Each use case maps to the RATS architecture roles
(Attester, Verifier, Relying Party) defined in
{{CPoE-Protocol}}.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL",
"SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",
"NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document
are to be interpreted as described in BCP 14 {{!RFC2119}}
{{!RFC8174}} when, and only when, they appear in all
capitals, as shown here. This document is informational and
uses these terms only when referencing normative requirements
defined in {{CPoE-Protocol}} and {{CPoE-Appraisal}}.

# Use Cases {#use-cases}

The provenance gap identified in {{CPoE-Protocol}} has
concrete consequences across multiple domains. Where
detection-based methods have been deployed as substitutes for
process evidence, the results have been problematic: Liang et
al. found that GPT detectors misclassified over 61% of
essays by non-native English writers as AI-generated
{{Liang2023}}, and courts have begun ruling that sanctions
based solely on AI detection scores lack valid evidentiary
basis. The following use cases illustrate how process
attestation addresses needs that neither content detection
nor custody-based provenance can satisfy.

Importantly, CPoE does not assert that human-authored content
is inherently superior to AI-generated content. The value of
process attestation lies in transparency: enabling Relying
Parties to make informed trust decisions appropriate to their
context. In some contexts, AI-generated content is entirely
acceptable or even preferred; in others, stakeholders require
assurance about the nature and extent of human involvement.
CPoE provides the evidentiary basis for these decisions
without imposing a policy preference.

## Creative Industries and Labor Rights {#uc-creative-industries}

Professional guilds and labor organizations in the creative
industries have established contractual frameworks governing
AI use in content production. The Writers Guild of America's
2023 Minimum Basic Agreement provides that AI-generated
material cannot be credited as "literary material" and
requires studios to disclose when materials provided to
writers incorporate AI-generated content. Similar provisions
exist in SAG-AFTRA agreements for digital replicas and
synthetic performers, and the Authors Guild has published
model contract clauses addressing AI use.

These contractual obligations currently lack a standardized
technical enforcement mechanism. A screenwriter's authoring
application acts as the Attester, producing an Evidence
Packet (.cpoe) that documents the writing process. When the
writer uses AI tools for research or drafting, the Tool
Receipt Protocol defined in {{CPoE-Appraisal}} records each
AI contribution with a signed receipt binding the tool's
output to the evidence chain. The studio or guild acts as
the Relying Party, consuming the Attestation Result (.cwar)
to verify compliance with contractual AI disclosure
requirements. The Attestation Result includes the
human-to-machine effort ratio, providing quantified
attribution rather than a binary human-or-AI determination.

## Academic Integrity {#uc-academic-integrity}

Educational institutions face a dilemma: post-hoc AI
detectors produce probabilistic scores that cannot reliably
distinguish AI-generated text from the writing of non-native
speakers, students with certain disabilities, or those with
concise writing styles {{Liang2023}}. Sanctions based on
detector output alone have been overturned by courts as
lacking valid evidentiary basis, with affected families
incurring substantial legal costs. Institutions that rely on
these tools expose themselves to litigation under due process
and civil rights frameworks.

CPoE inverts this model. Rather than accusing students based
on statistical analysis of their output, it enables students
to voluntarily submit affirmative evidence of their
authorship process. The student's writing environment acts as
the Attester, producing an Evidence Packet over the course of
the assignment. The institution's learning management system
acts as the Relying Party, evaluating whether the submitted
evidence is consistent with the claimed authorship timeline
and effort. Participation is opt-in: CPoE serves as a
provenance tool that rewards transparency, not a surveillance
mechanism that presumes guilt.

This use case requires particular attention to privacy.
Behavioral telemetry constitutes biometric data; the privacy
protections defined in {{CPoE-Protocol}} (jitter
quantization, data minimization, and Verifier data retention
limits) are essential safeguards. Institutions deploying CPoE
should establish clear data governance policies and ensure
students retain control over their Evidence Packets.

## Journalism and News Provenance {#uc-journalism}

Major news organizations including the Associated Press and
BBC have adopted editorial policies requiring human oversight
of AI-assisted content production, and several have
co-founded initiatives to combat AI-generated disinformation.
These policies require reporters to disclose AI tool use but
lack a machine-verifiable mechanism for enforcement.

A reporter's authoring application acts as the Attester,
producing Evidence Packets during article composition. The
news organization's editorial system acts as the Verifier,
appraising the evidence before publication. The resulting
Attestation Result can be embedded in the article's metadata
for downstream consumers (fact-checkers, aggregators, and
readers) who act as secondary Relying Parties. When reporters
use AI tools for research or drafting, Tool Receipts document
each contribution, preserving an auditable record of the
human editorial process that complements C2PA media
provenance for any associated images or video.

## Legal and Evidentiary Standards {#uc-legal-evidentiary}

CPoE Evidence Packets are designed to align with existing
legal standards for digital evidence authentication. The U.S.
Federal Rules of Evidence already provide a framework: Rule
901(b)(9) admits evidence about "a process or system, showing
it produces an accurate result," and Rules 902(13) and
902(14) (amended 2017) establish that hash-authenticated
electronic records and certified process outputs are
self-authenticating. CPoE's hash-chained evidence structure,
cryptographic signatures, and deterministic verification
procedures map naturally to these evidentiary requirements.

Regulatory frameworks are also converging on mandatory AI
content disclosure. The EU AI Act (Article 50) requires
disclosure of AI-generated content published on matters of
public interest, with enforcement beginning August 2026.
These obligations create demand for a standardized technical
mechanism that can produce verifiable, machine-readable
attestation of how content was produced.

In this use case, a filer's authoring environment acts as the
Attester, producing Evidence Packets bound to a hardware
Secure Element (T3 or T4 tier as defined in {{CPoE-Protocol}})
for non-repudiation. The regulatory body or court acts as the
Relying Party, consuming Attestation Results that include
forgery cost estimates to quantify the economic assurance of
the evidence.

## Content Platforms and Marketplaces {#uc-content-marketplace}

Content platforms may distinguish between human-authored and
AI-generated submissions for editorial placement, pricing, or
labeling purposes. Authors act as Attesters, submitting
Evidence Packets alongside their content. The platform's
Verifier produces Attestation Results that inform trust
decisions.

Absence Proofs as defined in {{CPoE-Appraisal}} enable the
platform to identify submissions that lack process evidence,
indicating unknown provenance rather than necessarily AI
generation. The forgery cost bounds in the Attestation Result
provide a quantitative basis for graduated trust: platforms
can distinguish between content with strong process evidence,
content with partial evidence, and content with no provenance
at all, rather than relying on a binary authentic-or-not
classification.

# AI-Assisted Workflows {#ai-assisted-workflows}

A common AI-assisted workflow involves the author typing
prompts to an AI tool, receiving generated text, then editing
and revising the output. In this scenario, the author's
keystrokes for prompts and edits produce genuine behavioral
signals, but the majority of the final content may be
machine-generated.

CPoE addresses this through the Tool Receipt Protocol
defined in {{CPoE-Appraisal}}: each AI tool contribution is
recorded with a signed receipt binding the tool's output hash
and character count to the checkpoint chain. The Attestation
Result reports effort attribution (human-to-machine character
ratio) derived from tool receipts, enabling Relying Parties
to distinguish "author composed 5,000 words" from "author
typed 200 words of prompts and edited 300 words of a
5,000-word AI-generated draft."

When tool receipts are absent, the behavioral signals attest
only to the human interaction observed by the Attester; they
do not prove that undisclosed AI tools were not used. Relying
Parties SHOULD consider the absence of tool receipts when
evaluating Attestation Results, as specified in
{{CPoE-Appraisal}}.

# Security Considerations {#security-considerations}

This document describes use cases and deployment
considerations. It does not define protocol mechanisms or
security-sensitive operations. The full security analysis for
CPoE, including the threat model, forgery cost bounds, and
adversarial analysis, is provided in {{CPoE-Protocol}} and
{{CPoE-Appraisal}}.

Deployers should note that the security properties available
to each use case depend on the Attestation Tier selected.
Tier 1 (CORE) provides software-only evidence with lower
forgery cost bounds. Tiers 3 and 4 incorporate hardware
Secure Elements that raise the forgery cost substantially and
provide non-repudiation properties relevant to the legal and
evidentiary use case ({{uc-legal-evidentiary}}).

# IANA Considerations {#iana-considerations}

This document has no IANA actions.

# Privacy Considerations {#privacy-considerations}

The full privacy analysis for CPoE is provided in
{{CPoE-Protocol}}. This section highlights privacy
considerations specific to the use cases described in this
document.

Behavioral telemetry collected by CPoE Attesters constitutes
biometric data under multiple regulatory frameworks (e.g.,
GDPR, CCPA/CPRA, BIPA). Deployments in all use cases MUST
comply with the privacy protections specified in
{{CPoE-Protocol}}, including jitter quantization of timing
data, data minimization principles, and Verifier data
retention limits.

The academic integrity use case ({{uc-academic-integrity}})
requires particular care. Students may face implicit
coercion to participate even when the system is nominally
opt-in. Institutions deploying CPoE SHOULD establish clear
data governance policies, obtain informed consent, and ensure
that non-participation does not result in adverse inference
or penalty.

The journalism use case ({{uc-journalism}}) involves
potential identification of individual reporters through
behavioral patterns. Organizations SHOULD configure Attesters
to use the minimum evidence profile sufficient for their
editorial requirements.

--- back

# Acknowledgments {#acknowledgments}
{:numbered="false"}

The use cases described in this document were informed by
discussions with stakeholders across the creative industries,
academic institutions, journalism organizations, and legal
practitioners. The authors thank the contributors to the CPoE
open specification process for their feedback on real-world
deployment requirements.
