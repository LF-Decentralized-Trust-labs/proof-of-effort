---
v: 3
docname: draft-condrey-cpop-protocol-latest
title: "Cryptographic Proof of Process (CPoP): Architecture and Evidence Format"
abbrev: CPoP Protocol
category: exp
ipr: trust200902
submissiontype: independent
area: Security
workgroup: Individual Submission
keyword:
  - attestation
  - RATS
  - provenance
  - authorship
  - SEAT

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
  RFC2104:
  RFC4648:
  RFC5705:
  RFC5869:
  RFC8017:
  RFC8610:
  RFC8949:
  RFC9052:
  RFC9106:
  RFC9266:
  RFC9334:
  RFC9711:

informative:
  CPoP-Appraisal:
    title: "Cryptographic Proof of Process (CPoP): Forensic Appraisal and Security Model"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-02
    seriesinfo:
      Internet-Draft: draft-condrey-cpop-appraisal-04
  RFC3161:
  RFC3552:
  RFC6838:
  RFC6962:
  RFC6973:
  Boneh2018:
    title: "Verifiable Delay Functions"
    target: "https://doi.org/10.1007/978-3-319-96884-1_25"
    author:
      - fullname: Dan Boneh
        initials: D.
        surname: Boneh
      - fullname: Joseph Bonneau
        initials: J.
        surname: Bonneau
      - fullname: Benedikt Bunz
        initials: B.
        surname: Bunz
      - fullname: Ben Fisch
        initials: B.
        surname: Fisch
    date: 2018
    seriesinfo:
      CRYPTO: "2018"
  Dolev-Yao:
    title: "On the Security of Public Key Protocols"
    target: "https://doi.org/10.1109/TIT.1983.1056650"
    author:
      - fullname: Danny Dolev
        initials: D.
        surname: Dolev
      - fullname: Andrew Yao
        initials: A.
        surname: Yao
    date: 1983
    seriesinfo:
      "IEEE Transactions on Information Theory": "29(2), 198-208"
  Salthouse1986:
    title: "Perceptual, Cognitive, and Motoric Aspects of Transcription Typing"
    target: "https://doi.org/10.1037/0033-2909.99.3.303"
    author:
      - fullname: Timothy A. Salthouse
        initials: T.A.
        surname: Salthouse
    date: 1986
    seriesinfo:
      "Psychological Bulletin": "99(3), 303-319"
  Monrose2000:
    title: "Keystroke Dynamics as a Biometric for Authentication"
    target: "https://doi.org/10.1016/S0167-739X(99)00059-X"
    author:
      - fullname: F. Monrose
        initials: F.
        surname: Monrose
      - fullname: A. Rubin
        initials: A.
        surname: Rubin
    date: 2000
    seriesinfo:
      "Future Generation Computer Systems": "16(4), 351-359"
  Monaco2018:
    title: "SoK: Keylogging Side Channels"
    target: "https://doi.org/10.1109/SP.2018.00026"
    author:
      - fullname: John V. Monaco
        initials: J.V.
        surname: Monaco
    date: 2018
    seriesinfo:
      "IEEE Symposium on Security and Privacy": "pp. 211-228"
  Dhakal2018:
    title: "Observations on Typing from 136 Million Keystrokes"
    target: "https://doi.org/10.1145/3173574.3174220"
    author:
      - fullname: Vivek Dhakal
        initials: V.
        surname: Dhakal
      - fullname: Anna Maria Feit
        initials: A.M.
        surname: Feit
      - fullname: Per Ola Kristensson
        initials: P.O.
        surname: Kristensson
      - fullname: Antti Oulasvirta
        initials: A.
        surname: Oulasvirta
    date: 2018
    seriesinfo:
      "ACM CHI": "2018"
  ScholaWrite:
    title: "ScholaWrite: A Dataset of End-to-End Scholarly Writing Process"
    target: "https://arxiv.org/abs/2502.02904"
    author:
      - fullname: Linghe Wang
        initials: L.
        surname: Wang
      - fullname: Minhwa Lee
        initials: M.
        surname: Lee
      - fullname: Ross Volkov
        initials: R.
        surname: Volkov
      - fullname: Luan Tuyen Chau
        initials: L.T.
        surname: Chau
      - fullname: Dongyeop Kang
        initials: D.
        surname: Kang
    date: 2025
    seriesinfo:
      arXiv: "2502.02904"
  ScholaWriteAugmented:
    title: "ScholaWrite-Augmented: Revision-Tracked Scholarly Writing Dataset with Annotated External Insertion Events"
    target: "https://github.com/writerslogic/scholawrite-augmented"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026
    seriesinfo:
      GitHub: "writerslogic/scholawrite-augmented"
  HaberStornetta1991:
    title: "How to Time-Stamp a Digital Document"
    target: "https://doi.org/10.1007/BF00196791"
    author:
      - fullname: Stuart Haber
        initials: S.
        surname: Haber
      - fullname: W. Scott Stornetta
        initials: W.S.
        surname: Stornetta
    date: 1991
    seriesinfo:
      "Journal of Cryptology": "3(2), 99-111"
  Condrey2026Attack:
    title: "On the Insecurity of Keystroke-Based AI Authorship Detection: Timing-Forgery Attacks Against Motor-Signal Verification"
    target: "https://arxiv.org/abs/2601.17280"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026
    seriesinfo:
      arXiv: "2601.17280"
  Biryukov2016:
    title: "Argon2: New Generation of Memory-Hard Functions for Password Hashing and Other Applications"
    target: "https://doi.org/10.1109/EuroSP.2016.31"
    author:
      - fullname: Alex Biryukov
        initials: A.
        surname: Biryukov
      - fullname: Daniel Dinu
        initials: D.
        surname: Dinu
      - fullname: Dmitry Khovratovich
        initials: D.
        surname: Khovratovich
    date: 2016
    seriesinfo:
      "IEEE EuroS&P": "pp. 292-302"
  RenDevadas2017:
    title: "Bandwidth Hard Functions for ASIC Resistance"
    target: "https://doi.org/10.1007/978-3-319-70500-2_16"
    author:
      - fullname: Ling Ren
        initials: L.
        surname: Ren
      - fullname: Srinivas Devadas
        initials: S.
        surname: Devadas
    date: 2017
    seriesinfo:
      "TCC 2017, LNCS": "10677, pp. 466-492"
  JESD79-4:
    title: "DDR4 SDRAM Standard"
    target: "https://www.jedec.org/standards-documents/docs/jesd79-4a"
    author:
      - org: JEDEC Solid State Technology Association
    date: 2012
    seriesinfo:
      JEDEC: "JESD79-4D"
  JESD79-5:
    title: "DDR5 SDRAM Standard"
    target: "https://www.jedec.org/standards-documents/docs/jesd79-5d"
    author:
      - org: JEDEC Solid State Technology Association
    date: 2020
    seriesinfo:
      JEDEC: "JESD79-5D"
  JESD238:
    title: "High Bandwidth Memory (HBM3) DRAM"
    target: "https://www.jedec.org/standards-documents/docs/jesd238b01"
    author:
      - org: JEDEC Solid State Technology Association
    date: 2022
    seriesinfo:
      JEDEC: "JESD238A"
  Takens1981:
    title: "Detecting Strange Attractors in Turbulence"
    target: "https://doi.org/10.1007/BFb0091924"
    author:
      - fullname: Floris Takens
        initials: F.
        surname: Takens
    date: 1981
    seriesinfo:
      "Lecture Notes in Mathematics": "898, 366-381"
  Orden2003:
    title: "Self-Organization of Cognitive Performance"
    target: "https://doi.org/10.1037/0096-3445.132.3.331"
    author:
      - fullname: Guy C. Van Orden
        initials: G.C.
        surname: Van Orden
      - fullname: John G. Holden
        initials: J.G.
        surname: Holden
      - fullname: Michael T. Turvey
        initials: M.T.
        surname: Turvey
    date: 2003
    seriesinfo:
      "Journal of Experimental Psychology: General": "132(3), 331-350"
  Sardar-RATS:
    title: "Security Considerations for Remote ATtestation procedureS (RATS)"
    target: "https://datatracker.ietf.org/doc/html/draft-sardar-rats-sec-cons-02"
    author:
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
    date: 2026-02
    seriesinfo:
      Internet-Draft: draft-sardar-rats-sec-cons-02
  RATS-Models:
    title: "Reference Interaction Models for Remote Attestation Procedures"
    target: "https://datatracker.ietf.org/doc/html/draft-ietf-rats-reference-interaction-models-15"
    author:
      - fullname: H. Birkholz
        initials: H.
        surname: Birkholz
      - fullname: M. Eckel
        initials: M.
        surname: Eckel
      - fullname: W. Pan
        initials: W.
        surname: Pan
      - fullname: E. Voit
        initials: E.
        surname: Voit
    date: 2025
    seriesinfo:
      Internet-Draft: draft-ietf-rats-reference-interaction-models-15
  SEAT-EXPAT:
    title: "Remote Attestation with Exported Authenticators"
    target: "https://datatracker.ietf.org/doc/html/draft-fossati-seat-expat-01"
    author:
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
      - fullname: Thomas Fossati
        initials: T.
        surname: Fossati
      - fullname: Tirumaleswar Reddy
        initials: T.
        surname: Reddy
      - fullname: Yaron Sheffer
        initials: Y.
        surname: Sheffer
      - fullname: Hannes Tschofenig
        initials: H.
        surname: Tschofenig
      - fullname: Ionut Mihalcea
        initials: I.
        surname: Mihalcea
    date: 2026-01
    seriesinfo:
      Internet-Draft: draft-fossati-seat-expat-01
  SEAT-Timing:
    title: "Pre-, Intra- and Post-handshake Attestation"
    target: "https://datatracker.ietf.org/doc/html/draft-usama-seat-intra-vs-post-03"
    author:
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
    date: 2026-01
    seriesinfo:
      Internet-Draft: draft-usama-seat-intra-vs-post-03
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
    date: 2023-07
    seriesinfo:
      Patterns: "4(7), 100779"
  C2PA:
    title: "C2PA Technical Specification"
    target: "https://c2pa.org/specifications/specifications/2.2/specs/C2PA_Specification.html"
    author:
      - org: Coalition for Content Provenance and Authenticity
    date: 2024
    seriesinfo:
      Version: "2.2"
  SEAT-UseCases:
    title: "Use Cases and Properties for Integrating Remote Attestation with Secure Channel Protocols"
    target: "https://datatracker.ietf.org/doc/html/draft-mihalcea-seat-use-cases-01"
    author:
      - fullname: Ionut Mihalcea
        initials: I.
        surname: Mihalcea
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
      - fullname: Thomas Fossati
        initials: T.
        surname: Fossati
      - fullname: Tirumaleswar Reddy
        initials: T.
        surname: Reddy
      - fullname: Yuning Jiang
        initials: Y.
        surname: Jiang
      - fullname: Meiling Chen
        initials: M.
        surname: Chen
    date: 2026-01
    seriesinfo:
      Internet-Draft: draft-mihalcea-seat-use-cases-01
  HHS-AI-RFI:
    title: "Request for Information: Accelerating the Adoption and Use of Artificial Intelligence as Part of Clinical Care"
    target: "https://www.regulations.gov/docket/HHS-ONC-2026-0001"
    author:
      - org: "U.S. Department of Health and Human Services, Office of the National Coordinator for Health Information Technology"
    date: 2025-12
    seriesinfo:
      "Federal Register": "2025-23641"
  NIST-AI-Agents-RFI:
    title: "Request for Information Regarding Security Considerations for Artificial Intelligence Agents"
    target: "https://www.regulations.gov/docket/NIST-2025-0035"
    author:
      - org: National Institute of Standards and Technology
    date: 2026-01
    seriesinfo:
      "Federal Register": "2026-00206"
  AG-Human-Authored:
    title: "Authors Guild Launches \"Human Authored\" Certification to Preserve Authenticity in Literature"
    target: "https://authorsguild.org/news/ag-launches-human-authored-certification-to-preserve-authenticity-in-literature/"
    author:
      - org: The Authors Guild
    date: 2025-01
  GPTZero:
    title: "GPTZero: AI Detection and Writing Process for Google Docs"
    target: "https://gptzero.me/chrome"
    author:
      - org: "GPTZero, Inc."
    date: 2025
  Amazon-KDP-AI:
    title: "Kindle Direct Publishing Content Guidelines"
    target: "https://kdp.amazon.com/en_US/help/topic/G200672390"
    author:
      - org: "Amazon.com, Inc."
    date: 2025

--- abstract

This document specifies the Cryptographic Proof of Process (CPoP) Evidence Framework, a specialized profile of Remote Attestation Procedures (RATS) designed to validate the provenance of effort in digital authorship. Unlike traditional provenance, which tracks file custody, CPoP attests to the continuous physical process of creation by entangling content hashes with temporal, behavioral, and physical constraints. Technical specifications for wire formats, sequential work functions, and hardware-anchored trust are provided.

--- to_be_removed_note_Discussion_Venues

Source for this draft and an issue tracker can be found at
<https://github.com/writerslogic/draft-condrey-cpop>.

--- middle

# Introduction {#introduction}

The rapid proliferation of generative artificial intelligence has created an authenticity crisis in digital discourse. While traditional provenance tracks the "custody of pixels," it fails to attest to the human-driven process of creation. This document specifies the Cryptographic Proof of Process (CPoP) protocol, which extends the RATS architecture {{RFC9334}} to validate the "provenance of effort."

Unlike traditional attestation which captures static system state, CPoP attests to a continuous physical process. Its Sequential Work Function (SWF) enforces temporal monotonicity, jitter seals bind behavioral entropy (human inter-keystroke timing) to the checkpoint chain, and entangled MACs bind physical state (thermodynamics) to the document's evolution.

By entangling content hashes with these physical constraints, this protocol enables an Attester to generate an Evidence Packet (.cpop) that imposes quantifiable cost on forgery of authorship claims, preserving privacy by design without disclosing document content. The formal threat model, including the adversarial Attester assumption that distinguishes CPoP from standard RATS deployments, is defined in {{threat-model}}.

This document is a companion to {{CPoP-Appraisal}}, which specifies the Verifier's forensic appraisal logic, the Attestation Result (WAR) wire format, and the quantitative security model. Implementers of Verifier components require both documents.

# Requirements Language {#requirements-language}

{::boilerplate bcp14-tagged}

# Problem Statement {#problem-statement}

Digital documents lack creation-process provenance. Existing
cryptographic mechanisms address complementary aspects -- COSE
signatures {{RFC9052}} prove key possession, trusted timestamps
{{RFC3161}} prove temporal existence, and media provenance
standards {{C2PA}} track post-creation custody -- but none
reveals how a document was produced or evolved during authorship.

Non-cryptographic approaches have emerged to address this gap,
but each carries fundamental limitations:

Surveillance-based methods:
: Screen recording, keystroke logging, and similar monitoring
  capture the authoring process but are inherently
  privacy-invasive, require continuous trust in a third-party
  archive, and produce evidence that cannot be independently
  verified without access to the surveillance
  infrastructure.

Detection-based methods:
: Stylometric classifiers and AI-generated content detectors
  operate on finished output rather than the creation process.
  These tools are probabilistic, degrade as generative models
  improve, are vulnerable to adversarial evasion, and exhibit
  systematic bias against non-native language speakers
  {{Liang2023}}. Their outputs do not constitute
  deterministic, independently reproducible evidence.

Proprietary commercial solutions:
: Several products have emerged to fill this gap but lack
  openness, verifiability, or both. GPTZero ships a Chrome
  extension that records writing replay in Google Docs and
  analyzes typing patterns using a proprietary algorithm
  {{GPTZero}}; the analysis is opaque, controlled
  by the detector rather than the author, and not independently
  reproducible. The Authors Guild "Human Authored" certification
  for books relies on author self-attestation with no
  cryptographic verification {{AG-Human-Authored}}.
  Amazon requires publishers to disclose AI-generated content on
  its Kindle Direct Publishing platform but provides no mechanism
  to verify compliance {{Amazon-KDP-AI}}.

The need for creation-process provenance extends across sectors
and jurisdictions. AI-assisted authoring is entering clinical
documentation workflows, prompting the U.S. Department of
Health and Human Services to seek input on how to maintain
accountability when AI tools contribute to medical records
{{HHS-AI-RFI}}. The autonomous content generation
capabilities of AI agents have led the National Institute of
Standards and Technology to examine security implications,
including the attribution of agent-produced output
{{NIST-AI-Agents-RFI}}. The common requirement across
these sectors is a standardized, open mechanism to verify how
content was produced rather than inferring it after the fact.
Such a mechanism must satisfy four properties:

1. *Privacy-preserving:* Evidence must not require
   disclosure of document content or invasive behavioral
   surveillance. Cryptographic hashes, not raw content, must form
   the evidentiary basis.
2. *Independently verifiable:* Proofs must be
   self-contained and deterministically checkable by any Verifier
   without access to external archives or proprietary
   systems.
3. *Tamper-evident:* Evidence must form a cryptographic
   chain resistant to retroactive modification, reordering, or
   selective omission.
4. *Process-documenting:* The mechanism must capture how
   a document evolved over time, not merely what it contains at a
   point in time.

# Use Cases {#use-cases}

The provenance gap identified in
{{problem-statement}} has concrete consequences
across multiple domains. Where detection-based methods have been
deployed as substitutes for process evidence, the results have
been problematic: Liang et al. found that GPT detectors
misclassified over 61% of essays by non-native English writers
as AI-generated {{Liang2023}}, and courts have
begun
ruling that sanctions based solely on AI detection scores lack
valid evidentiary basis. The following use cases illustrate how
process attestation addresses needs that neither content
detection nor custody-based provenance can satisfy.

Importantly, CPoP does not assert that human-authored content is
inherently superior to AI-generated content. The value of
process attestation lies in transparency: enabling Relying
Parties to make informed trust decisions appropriate to their
context. In some contexts, AI-generated content is entirely
acceptable or even preferred; in others, stakeholders require
assurance about the nature and extent of human involvement.
CPoP provides the evidentiary basis for these decisions without
imposing a policy preference.

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
Packet (.cpop) that documents the writing process. When the
writer uses AI tools for research or drafting, the Tool
Receipt Protocol defined in {{CPoP-Appraisal}}
records each AI contribution with a signed receipt binding
the tool's output to the evidence chain. The studio or guild
acts as the Relying Party, consuming the Attestation Result
(.cwar) to verify compliance with contractual AI disclosure
requirements. The Attestation Result includes the
human-to-machine effort ratio, providing quantified
attribution rather than a binary human-or-AI determination.

## Academic Integrity {#uc-academic-integrity}

Educational institutions face a dilemma: post-hoc AI
detectors produce probabilistic scores that cannot
reliably distinguish AI-generated text from the writing of
non-native speakers, students with certain disabilities,
or those with concise writing styles
{{Liang2023}}. Sanctions based on detector
output alone have been overturned by courts as lacking valid
evidentiary basis, with affected families incurring
substantial legal costs. Institutions that rely on these
tools expose themselves to litigation under due process and
civil rights frameworks.

CPoP inverts this model. Rather than accusing students based
on statistical analysis of their output, it enables students
to voluntarily submit affirmative evidence of their authorship
process. The student's writing environment acts as the
Attester, producing an Evidence Packet over the course of
the assignment. The institution's learning management system
acts as the Relying Party, evaluating whether the submitted
evidence is consistent with the claimed authorship timeline
and effort. Participation is opt-in: CPoP serves as a
provenance tool that rewards transparency, not a surveillance
mechanism that presumes guilt.

This use case requires particular attention to privacy.
Behavioral telemetry constitutes biometric data; the privacy
protections defined in
{{privacy-considerations}} (jitter
quantization, data minimization, and Verifier data
retention limits) are essential safeguards. Institutions
deploying CPoP should establish clear data governance policies
and ensure students retain control over their Evidence
Packets.

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

CPoP Evidence Packets are designed to align with existing
legal standards for digital evidence authentication. The
U.S. Federal Rules of Evidence already provide a framework:
Rule 901(b)(9) admits evidence about "a process or system,
showing it produces an accurate result," and Rules 902(13)
and 902(14) (amended 2017) establish that hash-authenticated
electronic records and certified process outputs are
self-authenticating. CPoP's hash-chained evidence structure,
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
Secure Element (T3 or T4 tier) for non-repudiation. The
regulatory body or court acts as the Relying Party, consuming
Attestation Results that include forgery cost estimates to
quantify the economic assurance of the evidence.

## Content Platforms and Marketplaces {#uc-content-marketplace}

Content platforms may distinguish between human-authored and
AI-generated submissions for editorial placement, pricing,
or labeling purposes. Authors act as Attesters, submitting
Evidence Packets alongside their content. The platform's
Verifier produces Attestation Results that inform trust
decisions.

Absence Proofs as defined in {{CPoP-Appraisal}}
enable the platform to identify submissions that lack
process evidence, indicating unknown provenance rather
than necessarily AI generation. The forgery cost bounds
in the Attestation Result provide a quantitative basis for
graduated trust: platforms can distinguish between content
with strong process evidence, content with partial evidence,
and content with no provenance at all, rather than relying
on a binary authentic-or-not classification.

# Scope and Applicability {#scope-and-applicability}

CPoP is designed as a voluntary, author-initiated mechanism.
The following normative constraints define its intended scope
of use and apply to all conforming implementations:

* Evidence collection MUST be initiated by the author.
  The Attester operates under the author's control; no
  external party may activate evidence collection without
  the author's explicit consent.
* Implementations MUST NOT be deployed as proctoring
  or surveillance mechanisms. CPoP does not capture screen
  content, camera input, biometric identification, or
  application activity outside the authoring environment.
* The author MUST retain sole control over whether to
  share Evidence Packets with a Verifier or Relying Party.
  Evidence Packets are generated and stored locally until
  the author chooses to release them.
* Verifiers MUST NOT automatically reject Evidence
  based solely on atypical timing patterns. Atypical
  input patterns may reflect assistive technologies,
  non-Latin input methods, or individual variation
  rather than forgery (see
  {{CPoP-Appraisal}}).

Whether any institution or platform requires CPoP evidence
as a condition of participation is a policy decision outside
the scope of this protocol. CPoP provides a technical
mechanism for producing verifiable evidence; it does not
mandate that evidence be produced or consumed.

# System Model {#system-model}

This section defines the CPoP system model in terms of the RATS architecture {{RFC9334}} and identifies where CPoP diverges from standard remote attestation assumptions.

## RATS Entity Roles {#rats-entity-roles}

CPoP maps to RATS entity roles {{RFC9334}} as follows. Verifier and Relying Party follow standard RATS definitions; the remaining roles have CPoP-specific characteristics:

Attester:
: The authoring application and its host platform. Unlike traditional RATS deployments, the Attester is operated by the entity whose claims are being verified (the author).

Attesting Environment (AE):
: The authoring application, OS entropy interfaces, and hardware Secure Elements (TPM/SE) when available.

Endorser:
: Hardware manufacturers that issue TPM endorsement certificates and platform attestation credentials for T3/T4 tiers.

Reference Value Provider:
: The CPoP specification itself (behavioral patterns, SWF parameters) and calibration services that provide updated forensic baselines.

Target Environment:
: The document editor session and its content state. The Target Environment is measured by the Attesting Environment at each checkpoint via content hashing and behavioral telemetry capture.

## Compatibility with RATS Architecture {#rats-compatibility}

CPoP extends RATS from device state attestation to process attestation, verifying that a physical process (human authorship) occurred as claimed. The fundamental problem structure is identical: an Attester generates Evidence, conveys it to a Verifier, and the Verifier produces Attestation Results for Relying Parties.

CPoP implements a critical trust inversion: in traditional remote attestation, the adversary is external (malware, network attackers). In CPoP, the Attester is operated by the author, and the primary adversary is the Attester operator themselves. This inversion shapes the security model:

* Evidence must be unforgeable by the entity generating it
* Temporal claims must be bound to physical constraints the Attester cannot circumvent
* Behavioral entropy must be computationally expensive to simulate
* Hardware attestation provides value only when the hardware root of trust is genuinely inaccessible to the Attester operator

The RATS architecture accommodates this through its layered trust model and configurable Appraisal Policies ({{RFC9334}}). The companion appraisal document ({{CPoP-Appraisal}}) defines domain-specific verification procedures. The rationale for Experimental status is provided in {{experimental-status-rationale}}.

## Adversarial Attester as RATS Profile {#adversarial-attester-profile}

CPoP constitutes a RATS profile in which the standard Attester-operator trust assumption is explicitly relaxed. RFC 9334 Section 3 defines Attester roles functionally (the entity producing Evidence) without requiring the operator to be trusted. CPoP leverages this by treating the Attester operator as the primary adversary and relying on physics-based constraints, memory-hard functions, and hardware trust anchors (at T3/T4) rather than Attester honesty.

This profile is analogous to anti-cheat and DRM attestation patterns where the platform operator is adversarial, and differs from traditional RATS deployments (firmware integrity, supply chain verification) where external compromise is the primary threat. Implementations and Verifier policies MUST account for this trust model when interpreting CPoP Evidence.

## Evidence Format and EAT Relationship {#evidence-eat-relationship}

CPoP Evidence Packets use a domain-specific CBOR structure (tag 1129336656) rather than CWT/JWT-wrapped EAT tokens. This design reflects the fundamental difference between CPoP Evidence and traditional EAT claims:

* EAT ({{RFC9711}}) models entity state as a set of claims at a point in time. CPoP Evidence models a continuous physical process as an ordered sequence of checkpoints with SWF proofs, behavioral entropy, and physical state bindings.
* The checkpoint chain structure (sequential process-proofs, hash-linked deltas, jitter bindings) has no natural representation in the EAT claims model.
* Wrapping each checkpoint as a separate EAT would lose the cryptographic chain integrity that is central to CPoP's security properties.

The EAT profile URI "urn:ietf:params:rats:eat:profile:pop:1.0" applies to the Attestation Result (WAR) format defined in {{CPoP-Appraisal}}, which carries EAT-compatible claims (verdict, attestation tier, forensic assessments). The Evidence Packet uses its own profile URI "urn:ietf:params:ccpop:profile:1.0" to identify the CPoP Evidence format. Generic EAT tooling cannot parse CPoP Evidence without CPoP-specific support, but can consume WAR Attestation Results via the EAR compatibility mapping defined in {{CPoP-Appraisal}}.

## Reference Value Trust Model {#reference-value-trust}

The Reference Value Provider role in CPoP differs from traditional RATS deployments where RVPs supply known-good platform measurements from a trusted supply chain. In CPoP, Reference Values take two forms:

Specification-defined values:
: SWF parameters, forensic thresholds, and profile requirements defined in this document and {{CPoP-Appraisal}}. These are static and trusted by virtue of the specification itself.

Author-generated behavioral baselines:
: Per-author behavioral profiles (baseline-digest) accumulated across sessions within the Attesting Environment. These are self-asserted by the Attester and MUST NOT be treated as trusted Reference Values. The baseline-digest is signed by the Evidence Packet's signing key and bound to the author identity via identity-fingerprint, but at T1/T2 tiers the signing key is software-controlled and the baseline could be synthetically constructed (see {{baseline-forgery}}).

Verifiers MUST treat behavioral baselines as corroborative evidence whose weight scales with the confidence-tier (population-reference through mature), not as independent attestation of authenticity. The forensic mechanisms (SNR, CLC, error topology) operate independently of baseline data and provide the primary discrimination between biological and synthetic authoring.

# Protocol Overview {#protocol-overview}

This section provides an end-to-end overview of the CPoP protocol, mapping the message flow to the RATS passport model and illustrating the lifecycle of an Evidence Packet from creation through appraisal.

## Passport Model Message Flow {#passport-model}

CPoP follows the RATS passport model ({{RFC9334}}, Section 8.1; {{RATS-Models}}):

~~~ ascii-art
+----------+  .cpop   +-----------+  .cwar   +-----------+
| Attester +--------->+  Verifier +--------->+  Relying  |
| (Author/ |Evidence  |           |Attestation|   Party   |
|    AE)   |          |           | Results  |(Publisher)|
+----------+          +-----+-----+          +-----------+
                            ^
              Endorsements  |  Reference Values
             (TPM/SE certs) | (SWF params, baselines)
                            |
               +------------+------------+
               |  Endorser / Ref. Value  |
               |       Provider          |
               +-------------------------+
~~~
{: #fig-passport-model title="CPoP Passport Model Message Flow"}

The CPoP-specific message flow is:

1. The Attester (authoring application running in the Attesting Environment) collects behavioral telemetry during content creation and generates an Evidence Packet (.cpop) containing SWF proofs, jitter bindings, and physical state markers.
2. The Evidence Packet is conveyed to a Verifier, which appraises chain integrity, temporal ordering, behavioral entropy, and content binding per the procedures defined in {{CPoP-Appraisal}}.
3. The Verifier produces a Cryptographic Writers Authenticity Report (.cwar) containing EAT claims, forensic assessment scores, and forgery cost estimates.
4. The Relying Party (publisher, reader, or automated platform) consumes the WAR to make trust decisions about the claimed authorship provenance.

Endorsers (hardware manufacturers) supply TPM endorsement certificates and Secure Element attestations that Verifiers use to validate hardware-bound claims in T3/T4 Evidence. Reference Value Providers supply the expected behavioral patterns, SWF difficulty parameters, and profile specifications that Verifiers use as appraisal baselines.

Note: In some deployments, the Relying Party (e.g., a content management system or publishing platform) may receive an Evidence Packet alongside content and forward it to a Verifier on behalf of the author. This is operationally similar to the RATS background-check model ({{RFC9334}}, Section 8.2), where the Relying Party initiates verification. However, the Evidence Packet is still generated by the Attester and travels with the content as a self-contained credential, making this architecturally consistent with the passport model. Deployments using this pattern SHOULD ensure that the Relying Party does not modify the Evidence Packet before forwarding it to the Verifier.

## Evidence Lifecycle {#evidence-lifecycle}

The following sequence illustrates the end-to-end lifecycle of a CPoP attestation session:

~~~ ascii-art
 Author        Attester (AE)          Verifier         Relying Party
   |                |                     |                  |
   | begin session  |                     |                  |
   +--------------->|                     |                  |
   |                |--+ sample physical  |                  |
   |                |  | freshness anchor |                  |
   |                |<-+ (entropy, thermal)|                 |
   |                |                     |                  |
   |                |--+ collect initial  |                  |
   |                |  | jitter sample    |                  |
   |                |<-+ (32+ bytes)      |                  |
   |                |                     |                  |
   | keystrokes,    |                     |                  |
   | edits, pauses  |                     |                  |
   +--------------->| capture jitter,     |                  |
   |                | semantic events     |                  |
   |                |                     |                  |
   |                |--+ CHECKPOINT:      |                  |
   |                |  | compute SWF,     |                  |
   |                |  | bind jitter +    |                  |
   |                |  | physical state,  |                  |
   |                |  | extend hash chain|                  |
   |                |<-+                  |                  |
   |                |                     |                  |
   |   ... (repeat per checkpoint interval) ...             |
   |                |                     |                  |
   | end session    |                     |                  |
   +--------------->| SEAL: sign chain,   |                  |
   |                | emit .cpop file     |                  |
   |                |                     |                  |
   |                | .cpop (Evidence)    |                  |
   |                +-------------------->|                  |
   |                |                     | appraise:        |
   |                |                     | chain, SWF,      |
   |                |                     | entropy, MACs    |
   |                |                     |                  |
   |                |                     | .cwar (Result)   |
   |                |                     +----------------->|
   |                |                     |                  | trust
   |                |                     |                  | decision
   |                |                     |                  |
~~~
{: #fig-evidence-lifecycle title="Evidence Packet Lifecycle"}

Attesters SHOULD use checkpoint intervals between 10 and 120 seconds (default 30 seconds). Verifiers MUST accept Evidence with any positive checkpoint interval but MAY flag intervals outside this range in warnings. Each checkpoint interval produces one link in the hash chain. The SWF computation runs continuously during the interval, binding the author's behavioral entropy and the platform's physical state to the elapsed wall-clock time. At session end or rollover boundary, the Attester seals the chain into a .cpop Evidence Packet. For long-running authoring projects, the Session Continuation mechanism ({{session-continuation}}) allows a series of Evidence Packets to be cryptographically linked.

## Session Continuation {#session-continuation}

Long-running authoring projects (e.g., books, dissertations,
multi-day reports) may produce checkpoint chains that exceed
practical memory and file-size limits. To support these
workflows, the Attester MAY partition the authoring process
into a series of Evidence Packets linked by the
previous-packet-ref field (evidence-packet key 14).

Attesters MUST begin a new Evidence Packet when the checkpoint
count exceeds 1000 or the serialized packet size exceeds 10 MiB,
whichever occurs first. Attesters SHOULD roll over to a new
Evidence Packet when the checkpoint count within the current
packet approaches these limits. A maximum of 10,000 checkpoints
per Evidence Packet is RECOMMENDED as an upper bound for
implementations that do not enforce the 1000-checkpoint trigger.

The following rules govern session continuation:

1. The first Evidence Packet in a series MUST omit the
   previous-packet-ref field (key 14) and MUST set
   packet-sequence (key 15) to 1, or omit both fields
   entirely. Omission of both fields indicates a standalone
   (non-continuation) Evidence Packet.
2. Each subsequent Evidence Packet MUST include
   previous-packet-ref containing the hash (using the series'
   hash-algorithm) of the complete CBOR-encoded preceding
   Evidence Packet. packet-sequence MUST be set to one greater
   than the preceding packet's packet-sequence value.
3. The document-ref (evidence-packet key 5) MUST be
   identical across all packets in a series. Verifiers MUST
   reject a continuation packet whose document-ref differs
   from the initial packet's.
4. The first checkpoint in a continuation packet MUST set
   its prev-hash to the final checkpoint-hash of the preceding
   Evidence Packet (not to H(CBOR-encode(document-ref)),
   which is reserved for the series' first checkpoint). This
   cryptographically links the two packets into one logical
   chain.
5. Checkpoint sequence numbers (checkpoint key 1) MUST be
   globally monotonic across the series. The first checkpoint
   in a continuation packet MUST have a sequence number
   strictly greater than the last checkpoint in the preceding
   packet.
6. The Attester MUST seal the current Evidence Packet
   before beginning a new one. Concurrent open packets for
   the same document-ref MUST NOT exist.

This mechanism preserves the full cryptographic chain
integrity of a single Evidence Packet while bounding per-file
resource consumption. A Verifier appraising a series
reconstructs the logical chain by following
previous-packet-ref links from the final packet back to the
initial packet.

## Behavioral Baseline Verification {#behavioral-baseline}

The Attester MAY include a baseline-verification structure
(evidence-packet key 19) that enables Verifiers to assess
behavioral consistency across writing sessions by the same
author. This mechanism strengthens authorship attribution
by comparing a session's behavioral summary against an
accumulated baseline digest.

A baseline-verification structure contains a
session-behavioral-summary capturing the current session's
typing metrics (IKI histogram, coefficient of variation,
Hurst exponent, cognitive pause frequency, duration, and
keystroke count). Optionally, it includes a baseline-digest
summarizing the author's historical behavioral profile and
a COSE_Sign1 signature over the CBOR-encoded digest. When
baseline-verification is present, the identity-fingerprint
within the baseline MUST match the Evidence Packet's
identity-fingerprint. Verifiers MUST reject Evidence where
these values differ.

The baseline-digest aggregates per-session statistics using
Welford's online algorithm (streaming-stats) for numerically
stable incremental computation of mean and variance. The
identity-fingerprint field MUST equal SHA-256 of the author's
Ed25519 public key, binding the digest to the signing
identity. The identity-fingerprint MUST be computed as
SHA-256(canonical-identity) where canonical-identity is the
UTF-8 encoded author identifier used for signing. For
COSE_Sign1-wrapped Evidence, this is the Subject field of the
signing certificate or the COSE Key ID (kid). The exact
identifier form MUST be consistent across all Evidence Packets
from the same author. The session-merkle-root field contains the root of
a Merkle Mountain Range over hashes of all sessions
incorporated into the digest, enabling efficient append-only
proof of session inclusion. Attesters that have not yet
implemented session-level MMR tracking MUST set
session-merkle-root to 32 zero bytes. Verifiers MUST treat
a zero session-merkle-root as providing no session inclusion
proof and MUST NOT elevate the confidence-tier based on it.

The confidence-tier field indicates the maturity of the
baseline. Attesters and Verifiers use the following
thresholds:

population-reference (1):
: 0-4 sessions observed. Comparison is limited to
  population-level human-vs-machine discrimination.

emerging (2):
: 5-9 sessions. Meaningful per-author consistency
  emerges.

established (3):
: 10-19 sessions. Author identity is distinguishable
  from other authors.

mature (4):
: 20 or more sessions. Full authorship attribution
  with high confidence.

During initial enrollment (fewer than one prior session),
the Attester MUST omit the baseline-digest and
digest-signature fields, including only the
session-behavioral-summary. The Verifier treats enrollment
packets as contributing to baseline accumulation without
performing baseline comparison.

When a baseline-digest is present, the digest-signature
field MUST also be present. Verifiers MUST verify the
COSE_Sign1 signature over the CBOR-encoded baseline-digest
and MUST verify that the identity-fingerprint equals
SHA-256 of the Evidence Packet's signing key.

# Threat Model {#threat-model}

This section defines the adversary model following the methodology of {{RFC3552}} and incorporating insights from RATS security analysis {{Sardar-RATS}}. The threat model assumes a Dolev-Yao style adversary {{Dolev-Yao}} with domain-specific constraints.

The following table maps CPoP threat categories to the RATS threat analysis in {{Sardar-RATS}} and {{RFC9334}} Section 12:

| RATS Threat Category | CPoP Coverage | CPoP Mechanism |
|---|---|---|
| Compromised Attester | Primary threat ({{adversarial-attester}}) | SWF time-binding, behavioral entropy, hardware isolation (T3/T4) |
| Spoofed Attestation Results | WAR signature verification | COSE_Sign1 over WAR, Verifier identity binding |
| Evidence replay | {{replay-attack}} | Physical freshness anchors, entangled VDF mode |
| Man-in-the-middle | {{diversion-attack}} | TLS channel binding (EKM), SEAT platform attestation |
| Compromised Verifier | Out of scope (trusted Verifier assumption) | Relying Party MAY cross-verify with multiple Verifiers |
| Supply chain attack on AE | {{ae-spoofing}} | T3/T4 hardware attestation; T1/T2 accept this risk |

## Adversarial Attester Model {#adversarial-attester}

The PRIMARY threat in CPoP is an adversarial Attester -- an author who controls the Attesting Environment and seeks to generate Evidence for content they did not authentically author. This inverts the standard RATS trust assumption where the Attester is trusted to report honestly.

The adversarial Attester has the following capabilities:

* *Full software control:* Can modify, instrument, or replace any software component including the authoring application and operating system
* *Timing manipulation:* Can adjust system clocks, virtualize execution environments, and attempt to compress or expand apparent time
* *Entropy injection:* Can inject synthetic behavioral data (keystroke timing, jitter sequences) from pre-recorded or generated sources
* *Content pre-generation:* Can generate document content using AI tools or other assistance before initiating the attestation session
* *Parallel execution:* Can run multiple attestation sessions simultaneously or use distributed resources

The adversary is constrained by:

* *Physics:* Cannot violate thermodynamic laws or accelerate hardware beyond physical limits
* *Memory bandwidth:* MHSF computations are bounded by available memory bandwidth
* *Hardware isolation:* In T3/T4 tiers, cannot extract keys from Secure Elements without physical tampering
* *Economic rationality:* Will not expend resources exceeding the value of successful forgery

## Security Goals {#security-goals}

CPoP provides the following authentication properties, defined in terms of adversary advantage:

Temporal Authenticity:
: Given Evidence claiming authorship duration D, an adversary cannot produce valid Evidence in time significantly less than D. Formally: Adv_temporal = Pr\[Verify(E) = accept AND Time(Generate(E)) < D - epsilon\] is negligible for meaningful epsilon.

Behavioral Authenticity:
: Given Evidence containing behavioral entropy B, an adversary cannot efficiently generate synthetic entropy that is indistinguishable from biological origin. The cost of generating synthetic behavioral data satisfying all forensic constraints must exceed a defined threshold.

Content Binding:
: Evidence E is cryptographically bound to document D such that E cannot be repurposed to attest a different document D'. This property is unconditional given collision resistance of H.

Non-repudiation (T3/T4):
: In hardware-bound tiers, Evidence is signed with keys that the Attester cannot extract or duplicate, providing non-repudiation of the attestation act.

## Attack Taxonomy {#attack-taxonomy}

The following attacks are in scope for CPoP defenses:

### Retype Attack {#retype-attack}

The canonical forgery attack against CPoP: an adversary generates content using AI or other assistance, then retypes the pre-existing content while collecting "authentic" behavioral telemetry. This attack exploits the gap between typing existing text and composing original text.

CPoP defends against retype attacks through:

* *Cognitive Load Correlation:* Authentic composition exhibits increased inter-keystroke intervals during high-complexity passages. Retyping known text shows uniform timing regardless of content complexity. Evidence with semantic-timing correlation r < 0.2 is flagged for additional scrutiny (see {{sec-retype-defense}}).
* *Error Topology:* Authentic authoring exhibits characteristic error patterns (hesitations, deletions near recent insertions, self-corrections). Retyping from reference exhibits either unnaturally low error rates or artificially injected errors lacking positional correlation.
* *Semantic-Temporal Binding:* The SWF proof binds the document's semantic evolution to wall-clock time. Retyping requires real-time effort proportional to document length, even if content was pre-generated.

Retype attacks remain economically viable for short documents. The forgery cost scales with document length and checkpoint frequency, providing graduated assurance rather than binary security.

### Replay Attack {#replay-attack}

Attempting to reuse previously valid Evidence for new claims. Defeated by Physical Freshness anchors that bind Evidence to non-reproducible physical state (thermal trajectories, kernel entropy samples). Entangled VDF Mode (`swf-argon2id-entangled`) further closes the parallel generation attack surface by creating a strict cryptographic dependency on the completion of the previous interval's memory-hard function, preventing an adversary from pre-computing SWF proofs in parallel.

### Relay Attack {#relay-attack}

Forwarding challenges or Evidence between a legitimate author and an adversary's session. In CPoP, this manifests as claiming credit for another author's work. Defeated by hardware-bound signing (T3/T4) and out-of-band presence challenges that verify physical proximity.

### SWF Acceleration Attack {#swf-acceleration}

Using specialized hardware to compute SWF proofs faster than consumer hardware. Mitigated by Argon2id's memory-hardness (computation bounded by memory bandwidth, not ALU throughput) and Hardware-Anchored Time in T3/T4 tiers.

### AE Spoofing {#ae-spoofing}

Presenting a virtualized or modified Attesting Environment as genuine. In T1/T2 tiers, this is possible and Evidence SHOULD be weighted accordingly. T3/T4 tiers require hardware attestation that is difficult to spoof without physical access to the Secure Element.

### Baseline Forgery {#baseline-forgery}

An adversarial Attester manufactures a synthetic baseline by
performing enrollment sessions using the same synthetic
behavioral patterns intended for subsequent forgery. This
attack is feasible because the baseline-digest is accumulated
and signed entirely within the Attesting Environment. The
adversary can establish a "consistent" but entirely synthetic
baseline, then present forgery sessions that match it.

This attack is mitigated by:

* *Baseline is non-verdict:* Baseline comparison modulates
  Verifier confidence but does not constitute an independent
  forensic flag (see {{CPoP-Appraisal}}). The forensic
  mechanisms (SNR, CLC, perplexity, cadence analysis) operate
  independently of baseline data.
* *Cross-metric consistency:* An adversary must simultaneously
  maintain consistent IKI histograms, coefficient of
  variation, Hurst exponent, and cognitive pause frequency
  across all forged sessions. Synthesizing coherent values
  for all four metrics is substantially harder than spoofing
  any single metric.
* *Hardware binding (T3/T4):* When the signing key is
  hardware-bound, the adversary cannot create a separate
  baseline under a different key and swap it in.

Verifiers MUST NOT treat baseline consistency as independent
evidence of authenticity. The confidence-tier field reflects
statistical robustness of the baseline, not trustworthiness
of the Attesting Environment.

### Diversion Attack {#diversion-attack}

An adversary redirects Evidence intended for one Verifier to a different Verifier or Relying Party context. CPoP Evidence Packets do not inherently bind to a specific Verifier identity. To mitigate this, implementations SHOULD use the TLS Exported Keying Material (EKM) mechanism defined in {{RFC9266}} to bind Evidence to the transport session.

When the Attester conveys Evidence over TLS, it SHOULD populate the optional channel-binding field (key 11) in the evidence-packet structure as follows:

1. The Attester calls the TLS exporter function with label "EXPORTER-CPoP-channel-binding" and an empty context value, requesting 32 bytes of output.
2. The Attester sets binding-type to 1 (tls-exporter) and binding-value to the 32-byte EKM output.
3. The Verifier, upon receiving the Evidence Packet, calls the same TLS exporter function on its side of the connection and compares the result to the binding-value in the channel-binding field.
4. If the values do not match, the Verifier MUST reject the Evidence Packet as potentially diverted.

The EKM label "EXPORTER-CPoP-channel-binding" is specific to this protocol. The empty context ensures the binding depends solely on the TLS session keys, which are unique per connection.

For offline verification (where no TLS session exists between Attester and Verifier), the channel-binding field is absent and Relying Parties SHOULD evaluate Evidence provenance through out-of-band channels.

When CPoP Evidence is conveyed over an attested TLS channel, implementations MAY leverage the SEAT exported authenticator mechanism {{SEAT-EXPAT}} to combine platform attestation (proving the Attesting Environment's integrity) with CPoP process attestation (proving the authorship process). The TLS channel binding described above is compatible with the SEAT evidence binding approach, which derives binding values from TLS exporters. At T3/T4 tiers, SEAT platform attestation provides the hardware trust anchor that corroborates CPoP's Attesting Environment claims.

## Out-of-Scope Threats {#out-of-scope-threats}

The following threats are explicitly out of scope:

* *Nation-state HSM compromise:* Adversaries capable of extracting keys from certified HSMs via invasive physical attacks
* *Physics-level laboratory spoofing:* Adversaries capable of simulating thermal trajectories and entropy sources at sub-microsecond precision
* *Quantum computation:* Attacks requiring large-scale quantum computers (hash function collision finding, Argon2id inversion)

# Core Principles and Claims {#core-principles}

Building on the threat model defined above, CPoP operates on five primary constraints:

* Physics-based Cost: Memory-Hard Sequential Functions (MHSF) establish an economic lower bound on forgery, ensuring consumer hardware remains competitive with specialized ASICs.
* Physical Freshness: Replay and simulation attacks are defeated by anchoring sessions to irreversible physical markers (Thermal Trajectories and Kernel Entropy pools). Every session incorporates Non-deterministic Physical Freshness sampled within the AE at the start of the sequential work function execution.
* Biological Binding: Captured human motor-signal randomness (jitter) serves as the non-deterministic seed for the SWF proof.
* Out-of-Band Presence: Utilizing secondary physical devices (e.g., smartphone QR scans) to bridge the digital-physical gap and ensure a human is in the loop.
* Asymmetric Verification: The sequential work function allows proofs to be verified probabilistically via Merkle-sampled audit proofs, ensuring scalability and DoS resistance.

# Protocol Rationale and Terminology {#rationale-and-terminology}

The Cryptographic Proof of Process (CPoP) framework follows the RATS architecture while introducing domain-specific extensions for physical process attestation.

CPoP Evidence Packet (.cpop):
: An Attester artifact containing SWF Merkle trees, jitter seals, and physical liveness markers (CBOR tag 1129336656, encoding ASCII "CPOP").

WAR Result (.cwar):
: A Verifier Attestation Result containing signed EAT claims and forensic assessments (CBOR tag 1129791826). The WAR format is specified in {{CPoP-Appraisal}}.

SWF:
: Sequential Work Function. For Argon2id modes (20/21), a chain of iterated Argon2id evaluations each individually memory-hard. For SHA-256 mode (10), a single Argon2id evaluation followed by iterated SHA-256 hashing. See {{swf-construction}}.

# Attester State Machine {#attester-state-machine}

The Attesting Environment (AE) MUST implement the following formal state machine governing the Evidence Packet lifecycle:

~~~ ascii-art
          session start          checkpoint trigger
  IDLE ─────────────────> CAPTURING ─────────────────> CHECKPOINTING
   ^                          ^                             |
   |    session end           |    checkpoint complete      |
   +──── FINALIZING <─────────+<────────────────────────────+
~~~

IDLE:
: The AE is inactive. No Evidence collection is in progress. Transition to CAPTURING occurs on session start (author initiates evidence collection).

CAPTURING:
: The AE captures semantic events (keystrokes, edits, pauses) and physical telemetry into a hash-linked buffer. Events are appended and the block hash is updated. Transition to CHECKPOINTING occurs on checkpoint trigger (interval elapsed or rollover threshold reached). Transition to FINALIZING occurs on session end.

CHECKPOINTING:
: The current event block is frozen and the AE computes the SWF over the entangled seed (previous hash, current jitter, physical markers). No new events are accepted into the current block during SWF computation. On successful completion, the checkpoint is appended to the chain and the AE transitions back to CAPTURING. If checkpoint computation fails, the Attester MUST discard the failed checkpoint and return to CAPTURING from the last successful checkpoint state. The Attester SHOULD record the failure in the limitations field (evidence-packet key 8).

FINALIZING:
: The Attester generates a final checkpoint, signs the transcript root with the Attester's signing key (hardware-bound for T3/T4; software-managed for T1/T2), and prepares the transport container (.cpop). On completion, the AE transitions to IDLE. If finalization fails, the Attester MUST retry or emit a partial Evidence Packet with a "finalization-incomplete" limitation.

# Evidence Content Tiers {#evidence-tiers}

CPoP Evidence Packets are classified by the depth of behavioral and forensic data collected:

CORE (Tier Value 1):
: Checkpoint chain with SWF proofs, hash-based content binding, and physical freshness anchors. Proves temporal ordering and content integrity.

ENHANCED (Tier Value 2):
: All CORE components plus behavioral entropy capture (Jitter Seals), intra-checkpoint correlation, and edit-graph-hash commitment ({{edit-graph-hash}}). Adds evidence of interactive authoring behavior.

MAXIMUM (Tier Value 3):
: All ENHANCED components plus entangled MACs binding physical state, error topology analysis, edit graph histograms ({{edit-graph-histograms}}), and forgery cost bounds. Provides the strongest available evidence.

CPoP Evidence is classified along two orthogonal axes. Evidence Content
Tier (CORE/ENHANCED/MAXIMUM) determines the depth of behavioral and
forensic data collected. Attestation Assurance Level (T1-T4) determines
the strength of hardware trust anchoring. These axes are independent:
a T3 CORE packet provides hardware-bound signing with minimal behavioral
data, while a T1 MAXIMUM packet provides rich behavioral data with
software-only signing.

# Attestation Assurance Levels {#attestation-assurance-levels}

The attestation tier system maps to established assurance frameworks
including NIST SP 800-63B Authenticator Assurance Levels (AAL),
ISO/IEC 29115 Levels of Assurance (LoA), and Entity Attestation Token
(EAT) security levels as defined in {{RFC9711}}.

| CPoP Tier | NIST AAL | EAT Security Level (RFC 9711) |
|---|---|---|
| T1: Software-Only | AAL1 | unrestricted (1) |
| T2: Attested Software | AAL2 | restricted (2) |
| T3: Hardware-Bound | AAL3 | hardware (4) |
| T4: Hardware-Hardened | LoA4 | hardware (4) |

T3 and T4 both map to EAT security level "hardware" (4) because the EAT
specification does not distinguish PUF-level binding from standard
TPM key binding. EAT security level "secure-restricted" (3) is not used
because CPoP tiers define hardware binding in terms of key isolation and
anti-tamper properties, not the firmware/software security boundary that
"secure-restricted" describes. Relying Parties requiring finer-grained
assurance distinctions SHOULD use the attestation-tier value (T1-T4)
from the WAR Attestation Result rather than relying solely on EAT
security-level.

## Tier T1: Software-Only {#tier-t1-software}

Binding Strength:
: none or hmac_local

NIST AAL Mapping:
: AAL1

Security Properties:

:   * SWF timing provides temporal ordering
    * Hash chains provide tamper evidence
    * Jitter entropy provides behavioral binding
    * No hardware root of trust; keys stored in software

## Tier T2: Attested Software {#tier-t2-attested}

T2 extends T1 with optional hardware attestation hooks. The AE attempts to use platform security features (Keychain, DeviceCheck) but degrades gracefully. Maps to AAL2.

## Tier T3: Hardware-Bound {#tier-t3-hardware-bound}

Requires TPM 2.0 or platform Secure Enclave key binding. Evidence generation MUST fail if hardware is unavailable. Maps to AAL3.

## Tier T4: Hardware-Hardened {#tier-t4-hardware-hardened}

Discrete TPM + PUF binding + Enclave execution. Anti-tamper evidence required. Exceeds AAL3 requirements; maps to ISO/IEC 29115 LoA4.

# Profile Architecture {#profile-architecture}

The CPoP specification defines three implementation profiles that establish Mandatory-to-Implement (MTI) requirements for interoperability.

| Feature ID | Feature Name | CORE | ENHANCED | MAXIMUM |
|---|---|---|---|---|
| 1 | swf-argon2id | M | M | M |
| 2 | content-binding | M | M | M |
| 4 | checkpoint-chain | M | M | M |
| 50 | behavioral-entropy | O | M | M |
| 51 | edit-graph-hash | O | M | M |
| 52 | edit-graph-histograms | O | O | M |
| 60 | assistive-mode | O | O | O |
| 105 | hardware-attestation | O | O | M |

Feature IDs 1-9 are reserved for core protocol features. IDs 50-99 are reserved for behavioral features. IDs 100-199 are reserved for hardware features. Future revisions may define additional features within these ranges.

## Conformance Requirements {#conformance}

A conforming Attester MUST implement at least the CORE profile. A conforming Verifier MUST be capable of validating all three profiles. Verifiers MUST reject Evidence containing unrecognized integer keys in the range 0-99 (reserved for this specification). Verifiers MUST ignore unrecognized keys with values 100 or greater. Verifiers receiving an Evidence Packet with version greater than 1 MUST reject the packet unless they implement the corresponding protocol version.

The profile-uri field in an Evidence Packet MUST be set to "urn:ietf:params:ccpop:profile:1.0" for Evidence conforming to this specification. This URI identifies the CPoP Evidence format and is distinct from the EAT profile URI "urn:ietf:params:rats:eat:profile:pop:1.0", which identifies CPoP Attestation Results (see {{evidence-eat-relationship}}).

In the document-ref structure, byte-length is the length in bytes of the UTF-8 encoded document, and char-count is the number of Unicode scalar values (code points).

If the Evidence Packet omits the attestation-tier field, the Verifier
MUST assess the tier from the evidence content: T1 if no hardware
attestation is present, T2 if platform attestation hooks are
detected, T3 if TPM key binding is verified, T4 if anti-tamper
evidence and PUF binding are confirmed. When attestation-tier is
absent, the Verifier MUST derive the tier from evidence content:
T1 if no COSE_Sign1 wrapper is present, T2 if software-signed,
T3/T4 based on HAT proof presence. The attestation-tier field is
informational; Verifiers MUST NOT rely on it as authoritative
when contradicted by the evidence content.

# Evidence Format and CDDL {#wire-format}

Evidence Packets are CBOR-encoded {{RFC8949}} and identified by semantic tag 1129336656. The CDDL notation {{RFC8610}} is used to define the wire format.

~~~ cddl
; CBOR tag wrappers
pop-evidence = #6.1129336656(evidence-packet)
pop-war = #6.1129791826(attestation-result)

; Stub: full attestation-result definition in [CPoP-Appraisal]
attestation-result = {* int => any}

; Primary structures
evidence-packet = {
    1 => uint,                    ; version (must be 1)
    2 => tstr,                    ; profile-uri
    3 => uuid,                    ; packet-id
    4 => pop-timestamp,           ; created
    5 => document-ref,            ; document
    6 => [3* checkpoint],         ; checkpoints (min 3)
    ? 7 => attestation-tier,      ; T1-T4
    ? 8 => [* tstr],              ; limitations
    ? 9 => profile-declaration,   ; profile
    ? 10 => [+ presence-challenge], ; QR/OOB proofs
    ? 11 => channel-binding,      ; TLS EKM binding
    ; key 12 reserved for future use
    ? 13 => content-tier,         ; Evidence Content Tier
    ? 14 => hash-value,            ; previous-packet-ref
    ? 15 => uint,                  ; packet-sequence (1-based)
    ; keys 16-17 reserved for future use
    ? 18 => physical-liveness,    ; physical-liveness markers
    ? 19 => baseline-verification, ; behavioral baseline verification
    * int => any,                  ; extension fields
}

checkpoint = {
    1 => uint,                    ; sequence (monotonic)
    2 => uuid,                    ; checkpoint-id
    3 => pop-timestamp,           ; timestamp (local)
    4 => hash-value,              ; content-hash
    5 => uint,                    ; char-count
    6 => edit-delta,              ; delta
    7 => hash-value,              ; prev-hash
    8 => hash-value,              ; checkpoint-hash
    9 => process-proof,           ; SWF proof
    ? 10 => jitter-binding,         ; behavioral-entropy (ENHANCED+)
    ? 11 => physical-state,         ; physical-state binding (ENHANCED+)
    ? 12 => hash-digest,             ; entangled-binding (ENHANCED+)
    ? 13 => [+ receipt],             ; receipts (self or tool)
    ? 14 => [+ active-probe],        ; active liveness probes
    ? 15 => hat-proof,               ; HAT temporal proof (T3/T4)
    ? 16 => beacon-anchor,           ; public randomness anchor (optional)
    ? 17 => bstr .size 32,           ; verifier-nonce (interactive mode)
    * int => any,                    ; extension fields
}

document-ref = {
    1 => hash-value,              ; content-hash
    ? 2 => tstr,                  ; filename
    3 => uint,                    ; byte-length
    4 => uint,                    ; char-count
    ? 5 => hash-salt-mode,        ; salting mode
    ? 6 => hash-digest,             ; salt-commitment
}

process-proof = {
    1 => proof-algorithm,         ; algorithm id
    2 => proof-params,            ; SWF params
    3 => hash-digest,             ; input (seed)
    4 => hash-digest,             ; merkle-root
    5 => [+ merkle-proof],        ; sampled proofs
    6 => uint,                     ; claimed-duration (milliseconds)
}

; Subsidiary type definitions
attestation-tier = &(
    software-only: 1,             ; T1: AAL1
    attested-software: 2,         ; T2: AAL2
    hardware-bound: 3,            ; T3: AAL3
    hardware-hardened: 4,         ; T4: LoA4
)

content-tier = &(
    core: 1,
    enhanced: 2,
    maximum: 3,
)

proof-algorithm = &(
    swf-sha256: 10,             ; SHA-256 iterative (no memory-hard)
    swf-argon2id: 20,
    swf-argon2id-entangled: 21, ; Entangled VDF Mode
)

hash-salt-mode = &(
    unsalted: 0,
    author-salted: 1,
)

proof-params = {
    1 => uint,                    ; time-cost (t)
    2 => uint,                    ; memory-cost (m, KiB)
    3 => uint,                    ; parallelism (p)
    4 => uint,                    ; steps
    ? 5 => uint,                  ; waypoint-interval (Mode 10 only)
    ? 6 => uint,                  ; waypoint-memory (KiB, Mode 10 only)
}

jitter-binding = {
    1 => [+ uint],                ; intervals (milliseconds)
    2 => uint,                    ; entropy-estimate (centibits)
    3 => hash-digest,             ; jitter-tag (keyed consistency tag)
}

merkle-proof = {
    1 => uint,                    ; leaf-index
    2 => [+ hash-digest],        ; sibling-path
    3 => hash-digest,             ; leaf-value
}

edit-delta = {
    1 => uint,                    ; chars-added
    2 => uint,                    ; chars-deleted
    3 => uint,                    ; op-count
    ? 4 => [* edit-position],     ; positions
    ? 5 => hash-digest,           ; edit-graph-hash (ENHANCED+)
    ; keys 6-8 reserved for future use
    ? 9 => [8*8 uint],             ; cursor-trajectory-histogram (MAXIMUM)
    ? 10 => [8*8 uint],            ; revision-depth-histogram (MAXIMUM)
    ? 11 => [8*8 uint],            ; pause-duration-histogram (MAXIMUM)
}

edit-position = [
    uint,                         ; offset
    int,                          ; change (+/-), must be non-zero
]

physical-state = {
    1 => [+ int],                 ; thermal (relative, millidegrees)
    2 => int,                     ; entropy-delta (signed)
    ? 3 => bstr .size 32,         ; kernel-commitment
}

physical-liveness = {
    1 => [+ thermal-sample],      ; thermal trajectory
    2 => bstr .size 32,           ; entropy-anchor
}

thermal-sample = [
    pop-timestamp,                ; sample time
    int,                          ; temperature delta (millidegrees)
]

presence-challenge = {
    1 => bstr .size (16..256),    ; challenge-nonce (128+ bits)
    2 => bstr,                    ; device-signature (must be COSE_Sign1)
    3 => pop-timestamp,           ; response-time
}

profile-declaration = {
    1 => tstr,                    ; profile-id
    2 => [+ uint],                ; feature-flags
}

binding-type = &(
    tls-exporter: 1,
)

channel-binding = {
    1 => binding-type,            ; binding-type
    2 => bstr .size 32,           ; binding-value (EKM output)
}

receipt = self-receipt / tool-receipt

self-receipt = {
    1 => tstr,                    ; tool-id (source environment)
    2 => hash-value / compact-ref, ; output-commit
    3 => hash-value / compact-ref, ; evidence-ref (source packet)
    4 => pop-timestamp,           ; transfer-time
}

tool-receipt = {
    1 => tstr,                    ; tool-id (provider URI)
    2 => hash-value,              ; output-commit
    ? 3 => hash-value,            ; input-ref (prompt hash)
    4 => pop-timestamp,           ; issued-at
    5 => bstr .cbor COSE_Sign1,   ; tool-signature
    ? 6 => uint,                  ; output-char-count
}

active-probe = {
    1 => probe-type,              ; challenge category
    2 => pop-timestamp,           ; stimulus-time
    3 => pop-timestamp,           ; response-time
    4 => bstr,                    ; stimulus-data (challenge payload)
    5 => bstr,                    ; response-data (captured response)
    ? 6 => uint,                  ; response-latency (milliseconds)
}

probe-type = &(
    galton-board: 1,              ; Galton invariant challenge
    reflex-gate: 2,               ; motor reflex timing gate
    spatial-target: 3,            ; spatial accuracy challenge
)

; Behavioral Baseline Verification
baseline-verification = {
    ? 1 => baseline-digest,       ; omitted during enrollment
    2 => session-behavioral-summary, ; current session metrics
    ? 3 => bstr .cbor COSE_Sign1,  ; digest-signature
}

baseline-digest = {
    1 => uint,                    ; version (must be 1)
    2 => uint,                    ; session-count
    3 => uint,                    ; total-keystrokes
    4 => streaming-stats,         ; iki-stats
    5 => streaming-stats,         ; cv-stats
    6 => streaming-stats,         ; hurst-stats
    7 => [9* float32],            ; aggregate-iki-histogram
    8 => streaming-stats,         ; pause-stats
    9 => bstr .size 32,           ; session-merkle-root (MMR)
    10 => confidence-tier,        ; baseline maturity
    11 => pop-timestamp,          ; computed-at
    12 => bstr .size 32,          ; identity-fingerprint
}

session-behavioral-summary = {
    1 => [9* float32],            ; iki-histogram
    2 => float32,                 ; iki-cv
    3 => float32,                 ; hurst
    4 => float32,                 ; pause-frequency
    5 => uint,                    ; duration-secs
    6 => uint,                    ; keystroke-count
}

streaming-stats = {
    1 => uint,                    ; count
    2 => float32,                 ; mean
    3 => float32,                 ; m2 (Welford's sum of squared diffs)
    4 => float32,                 ; min
    5 => float32,                 ; max
}

confidence-tier = &(
    population-reference: 1,      ; 0-4 sessions
    emerging: 2,                  ; 5-9 sessions
    established: 3,               ; 10-19 sessions
    mature: 4,                    ; 20+ sessions
)

; Base types
uuid = bstr .size 16
pop-timestamp = uint              ; epoch milliseconds (no tag 1; see protocol §6.7)
hash-digest = bstr .size 32 /        ; SHA-256
              bstr .size 48 /        ; SHA-384
              bstr .size 64          ; SHA-512
hash-value = {
    1 => hash-algorithm,
    2 => hash-digest,              ; length must match algorithm output
}
compact-ref = {
    1 => hash-algorithm,          ; algorithm used for full hash
    2 => bstr .size (8..32),      ; truncated-digest (8-32 bytes)
    3 => uint,                    ; prefix-length (bytes in digest)
}
hash-algorithm = &(
    sha256: 1,
    sha384: 2,
    sha512: 3,
)
~~~

The attestation-result type used in the pop-war tag wrapper is
defined as a stub above for CDDL completeness; the full
definition appears in {{CPoP-Appraisal}}.

To ensure cross-architecture determinism, all temporal and entropy measurements MUST be encoded as unsigned integers (`uint`). Timestamps and durations are expressed in milliseconds. Entropy estimates are expressed in centibits (1/100th of a bit).

pop-timestamp is a bare unsigned integer representing milliseconds since
the Unix epoch (1970-01-01T00:00:00Z). CBOR tag 1 is not used because
RFC 8949 Section 3.4.2 defines it as epoch seconds; CPoP requires
millisecond precision for IKI measurements and jitter-binding
timestamps. pop-timestamp values MUST be positive (greater than zero).
A pop-timestamp value of zero is invalid. Verifiers MUST reject
Evidence Packets containing any zero-valued timestamp.

When hash-salt-mode is author-salted (1), the author generates a
random salt of at least 16 bytes. The salt-commitment field MUST
contain H(salt). To verify content binding, the author discloses
the salt to the Verifier, which checks that H(disclosed_salt)
matches the salt-commitment.

SHA-256 (value 1) is mandatory-to-implement. Conforming Attesters and Verifiers MUST support SHA-256. Support for SHA-384 and SHA-512 is OPTIONAL.

The hash digest length MUST match the algorithm output length: 32 bytes for SHA-256, 48 bytes for SHA-384, and 64 bytes for SHA-512.

All hash-value fields within a single Evidence Packet MUST use the
same hash algorithm. Verifiers MUST reject Evidence Packets
containing mixed hash algorithms.

Let H denote the hash function indicated by the hash-algorithm
field in the Evidence Packet. All internal hash computations in
this specification -- including the SWF iteration loop, Merkle
tree construction, seed derivation, checkpoint-hash computation,
Fiat-Shamir sampling, HMAC operations, and HKDF instantiation
-- MUST use H. The Argon2id output length (`len`
parameter) MUST equal the output length of H (32, 48, or 64
bytes). When this document writes H(...) in formulas, the hash
function used MUST be the one identified by hash-algorithm.

Encoders MUST use CBOR unsigned integer (major type 0) or
negative integer (major type 1) encoding for Evidence Packet
fields typed as `uint` or `int` in the CDDL. Floating-point
encodings (CBOR major type 7) MUST NOT be used for these
fields. Fields explicitly typed as `float32` in the CDDL
MUST use IEEE 754 single-precision encoding (CBOR additional
info 26). Half-precision (additional info 25) MUST NOT be
used, as it provides insufficient precision for streaming
variance computations. These float32 fields appear in both
Evidence Packet structures (streaming-stats,
session-behavioral-summary) and Attestation Result structures
(entropy-report, forgery-cost-estimate).

The `compact-ref` type provides a space-efficient
alternative to full `hash-value` references in
self-receipt structures. A compact-ref contains a truncated
hash prefix (minimum 8 bytes, providing 64 bits of collision
resistance) along with the hash algorithm identifier and the
prefix-length in bytes. Compact references MUST NOT be used
for security-critical bindings (checkpoint-hash, content-hash,
prev-hash); they are permitted only in self-receipt fields
where the referenced Evidence Packet can be independently
retrieved and verified against the full hash. Verifiers
encountering a compact-ref MUST resolve it to a full hash-value
before performing integrity checks.

Checkpoint key 13 accepts two receipt types, distinguished by the
presence of key 5 (tool-signature):

A `self-receipt` records cross-tool composition where content
originates from another CPoP-enabled authoring environment. The
tool-id identifies the source environment, output-commit binds
the pasted content, and evidence-ref references the source
Evidence Packet. Self-receipt verification is specified in
{{CPoP-Appraisal}}.

A `tool-receipt` records content contributed by an external tool
(e.g., a large language model) that is not itself a CPoP Attester.
The tool-id MUST be a URI under the tool provider's control. The
output-commit field contains a hash-value of the tool's generated
content, using the Evidence Packet's hash algorithm. The optional
input-ref field contains a hash-value of the prompt provided to
the tool; inclusion is an author privacy decision. The
tool-signature field MUST contain a COSE_Sign1 structure
{{RFC9052}} whose payload is the CBOR encoding of the map
`{1: tool-id, 2: output-commit, 4: issued-at}`. The signing key
is the tool provider's, not the Attester's. The Verifier validates
tool-signatures against trusted tool provider public keys.
The optional output-char-count field records the character count
of the tool's generated output, enabling effort attribution
without requiring the Verifier to access the generated content.

Compact references (compact-ref) MUST NOT be used in tool-receipt
fields. All hash-value fields in a tool-receipt MUST use the
Evidence Packet's hash algorithm per the consistency requirement
above.

The mechanism by which tool providers generate and sign receipts,
including key discovery and trust establishment, will be specified
in a companion document. Until a companion specification defines
tool provider key discovery, Verifiers MUST treat tool-receipts
with unresolvable tool-id URIs as unverified. Unverified
tool-receipts MUST NOT contribute to an authentic verdict but MAY
be included in the Evidence Packet for informational purposes.

Extension keys in evidence-packet and checkpoint structures MUST
use integer values 100 or greater. Keys 0-99 are reserved for use
by this specification and future revisions.

The op-count field in edit-delta counts the number of discrete
editing operations (insertions, deletions, and replacements)
during the checkpoint interval. A single paste operation counts
as one operation regardless of character count.

In edit-position entries, the change value MUST be non-zero.
Positive values indicate insertion of characters at the offset;
negative values indicate deletion. A zero change value is
semantically meaningless and MUST NOT appear.

## Edit Graph Hash {#edit-graph-hash}

The edit-graph-hash (edit-delta key 5) provides a cryptographic
commitment to the full editing trajectory within a checkpoint
interval. It MUST be present in ENHANCED and MAXIMUM content
tiers and MAY be present in CORE.

The Attestation Environment MUST compute the edit-graph-hash
as follows:

~~~ pseudocode
edit-graph-input = [
    cursor-positions,    ; [* uint] char offsets, 100ms sample
    revision-depths,     ; [* uint] edit depth at each sample
    pause-durations      ; [* uint] inter-keystroke gaps in ms
]

edit-graph-hash = H(
    "CPoP-EditGraph-v1" ||
    CBOR-encode(edit-graph-input)
)
~~~

Where H is the Evidence Packet's selected hash function and
CBOR-encode produces deterministic CBOR per Section 4.2.1 of
{{RFC8949}}. The cursor-positions array MUST contain cursor
position samples at minimum 100ms intervals. The
revision-depths array MUST record the number of prior edits
at each cursor position at the same sample rate. The
pause-durations array MUST record all inter-keystroke intervals
exceeding 500ms. Arrays MUST be truncated to the 10,000 most
recent samples measured from the current checkpoint boundary.

When the edit-graph-hash is present, it is entangled into the
SWF seed derivation ({{swf-seed-derivation}}).

## Edit Graph Histograms {#edit-graph-histograms}

Edit-delta keys 9-11 carry 8-bin histograms summarizing the
editing trajectory. These fields MUST be present in MAXIMUM
content tier and MUST be absent in CORE. They MAY be present
in ENHANCED.

Each histogram is an array of exactly 8 unsigned integers
representing event counts per bin. The bin boundaries are:

Cursor Trajectory Histogram (key 9):
: Cumulative cursor travel distance in characters between
  consecutive samples. Bins: \[0, 50\], \[51, 200\],
  \[201, 500\], \[501, 1000\], \[1001, 2500\],
  \[2501, 5000\], \[5001, 10000\], \[10001, +inf\).

Revision Depth Histogram (key 10):
: Maximum revision depth per edited region. Bins correspond
  to depths 0 through 6 individually, with bin 7 covering
  depth 7 and above.

Pause Duration Histogram (key 11):
: Inter-keystroke pause durations in milliseconds. Pauses
  below 500ms are excluded. Bins: \[500, 1000\],
  \[1001, 2000\], \[2001, 5000\], \[5001, 10000\],
  \[10001, 30000\], \[30001, 60000\], \[60001, 300000\],
  \[300001, +inf\).

The device-signature in a presence-challenge MUST be a COSE_Sign1
structure {{RFC9052}} covering the challenge-nonce.
The signing key MUST be hardware-bound on the secondary device.
The Verifier obtains the corresponding public key through prior
device registration (the registration mechanism is out of scope
for this document). Platform-specific attestation formats
(e.g., Apple App Attest, Android Key Attestation) MAY be
carried as the COSE_Sign1 payload to provide hardware-tier
evidence alongside the presence proof.

Active-probe structures (checkpoint key 14) record challenge-response
interactions issued by the Attester or Verifier during the authoring
session. Each probe records the probe-type, the stimulus timestamp
and payload, and the captured biological response. Galton-board
probes (type 1) present a visual randomness challenge whose response
distribution is compared against biological invariants. Reflex-gate
probes (type 2) measure involuntary motor reflex latency. Spatial-target
probes (type 3) measure pointing accuracy to on-screen targets.
The response-latency field, when present, MUST equal
(response-time minus stimulus-time) in milliseconds. Verifiers
appraise active-probe responses as specified in
{{CPoP-Appraisal}}.

Per-checkpoint physical-state (checkpoint key 11) captures instantaneous
thermal and entropy measurements. Packet-level physical-liveness
(evidence-packet key 18) provides a session-wide thermal trajectory
for replay detection. physical-liveness SHOULD be included in ENHANCED
and MAXIMUM profiles. When both are present, Verifiers MUST verify
that per-checkpoint thermal values are consistent with the session-wide
trajectory.

The baseline-verification structure (evidence-packet key 19)
carries per-session behavioral metrics and, when available, a
signed digest of the author's accumulated behavioral baseline.
The session-behavioral-summary uses a fixed 9-bin IKI histogram
with bin edges at 0, 50, 100, 150, 200, 300, 500, 1000, and
2000 milliseconds. The streaming-stats type within
baseline-digest records count, mean, M2 (Welford's sum of
squared differences from the mean), min, and max for each
tracked metric. This representation supports numerically
stable incremental updates without retaining raw sample data.
Appraisal of baseline-verification data is specified in
{{CPoP-Appraisal}}.

## Checkpoint Hash Computation {#checkpoint-hash-computation}

The checkpoint-hash field MUST be computed as follows:

~~~ pseudocode
checkpoint-hash = H(
    "CPoP-Checkpoint-v1" ||
    prev-hash ||
    content-hash ||
    CBOR-encode(edit-delta) ||
    CBOR-encode(jitter-binding) ||
    CBOR-encode(physical-state) ||
    process-proof.merkle-root
)
~~~

Where H is the Evidence Packet's selected hash function, \|\| denotes concatenation, and CBOR-encode produces deterministic CBOR per Section 4.2.1 of {{RFC8949}}. The UTF-8 Domain Separation Tag (DST) prefix "CPoP-Checkpoint-v1" MUST be prepended as the first input to prevent cross-context hash collisions. When edit-graph-hash (edit-delta key 5) is present, it is already included within the CBOR-encoded edit-delta and requires no separate term in the checkpoint-hash computation.

For the first checkpoint in the initial Evidence Packet of a series (or a standalone packet), prev-hash MUST be set to H(CBOR-encode(document-ref)). This anchors the chain to the document identity. For the first checkpoint in a continuation packet (previous-packet-ref present), prev-hash MUST be set to the final checkpoint-hash of the preceding Evidence Packet (see {{session-continuation}}).

When jitter-binding and physical-state fields are absent (CORE profile), the checkpoint-hash MUST be computed without those terms: checkpoint-hash = H("CPoP-Checkpoint-v1" \|\| prev-hash \|\| content-hash \|\| CBOR-encode(edit-delta) \|\| process-proof.merkle-root).

## Checkpoint Computation Order {#checkpoint-computation-order}

The fields within a checkpoint MUST be computed in the following order:

1. When edit-graph-hash is present: compute the edit-graph-hash from the accumulated cursor-positions, revision-depths, and pause-durations arrays per {{edit-graph-hash}}. When histograms are present (MAXIMUM tier): populate the 8-bin arrays per {{edit-graph-histograms}}. Assemble the edit-delta structure including these fields.
2. Compute the SWF: run the proof-algorithm with the derived seed (which incorporates the edit-graph-hash when present). For modes 20/21, evaluate Argon2id iteratively for the configured number of steps. For mode 10, evaluate Argon2id once then iterate SHA-256 with memory-hard waypoints at every W-th step. Construct the Merkle tree over all intermediate states using tagged hashing ({{merkle-tree-construction}}) to obtain the merkle-root (process-proof key 4).
3. Derive the shared PRK and per-field keys via the two-stage HKDF hierarchy ({{key-derivation-hierarchy}}).
4. Compute the jitter-tag using the tag-key and jitter-binding.intervals as HMAC input. Assemble the jitter-binding structure (intervals, entropy-estimate, jitter-tag).
5. Compute the entangled-binding using the binding-key and prev-hash, content-hash, jitter-binding, and physical-state as HMAC input.
6. Compute the checkpoint-hash over the DST "CPoP-Checkpoint-v1", prev-hash, content-hash, edit-delta, jitter-binding, physical-state, and merkle-root.

This ordering ensures that each subsequent computation can reference the outputs of prior steps. Implementations MUST follow this order to produce interoperable checkpoints.

## Evidence Protection {#evidence-protection}

For T3 and T4 Attestation Tiers, Evidence Packets MUST be wrapped
in a COSE_Sign1 envelope {{RFC9052}}. For T1 and T2 tiers, COSE_Sign1
wrapping is RECOMMENDED. The COSE_Sign1 payload is the complete
CBOR-encoded evidence-packet including tag 1129336656; ES256 or
EdDSA is RECOMMENDED for the algorithm identifier.

For T3/T4 tiers, the signing key MUST be bound to a hardware
Secure Element (TPM or platform SE). For T1/T2 tiers, a
software-managed key is acceptable.

When COSE_Sign1 wrapping is not used (e.g., offline file-based
conveyance), the Evidence Packet's integrity relies solely on
the internal hash chain. Relying Parties MUST evaluate the
trust implications of unwrapped Evidence.

For online conveyance, COSE_Sign1-wrapped Evidence Packets can be encapsulated within a Conceptual Message Wrapper (CMW) for transport via the SEAT cmw_attestation TLS extension {{SEAT-EXPAT}}. This enables CPoP Evidence to be delivered alongside platform attestation evidence in a single post-handshake authentication exchange, which is the preferred attestation timing model {{SEAT-Timing}}. The SEAT use cases {{SEAT-UseCases}} identify runtime attestation and operation-triggered re-attestation as key requirements, both of which CPoP's continuous checkpoint model satisfies.

## ASCII-Armored Encoding {#ascii-armor}

For transport over text-based protocols, embedding in source
code repositories, or human-readable archival, Evidence
Packets and Attestation Results MAY be encoded in an
ASCII-armored representation. The armored format consists of
a header line, Base64-encoded payload ({{RFC4648}},
Section 4), and a footer line.

Evidence Packets (.cpop) use the following armor boundaries:

~~~ artwork
-----BEGIN POP EVIDENCE-----
<Base64-encoded CBOR payload>
-----END POP EVIDENCE-----
~~~

Attestation Results (.cwar) use the following armor
boundaries:

~~~ artwork
-----BEGIN POP WAR-----
<Base64-encoded CBOR payload>
-----END POP WAR-----
~~~

The following rules apply to the armored encoding:

1. The payload MUST be the complete CBOR-encoded structure
   (including the CBOR semantic tag). When COSE_Sign1 wrapping
   is present, the entire COSE_Sign1 envelope is the
   payload.
2. Base64 line length MUST NOT exceed 76 characters per
   {{RFC4648}}. Decoders MUST ignore line
   breaks and whitespace within the Base64 block.
3. No additional headers or metadata are defined between
   the header line and the Base64 block. Implementations
   MUST NOT insert key-value headers (unlike PEM, which
   permits Proc-Type and DEK-Info headers).
4. Implementations MUST accept both armored and raw CBOR
   encodings. The media type for armored Evidence is
   application/cpop+cbor with a Content-Transfer-Encoding
   of base64. The media type for armored Attestation Results
   is application/cwar+cbor with the same transfer
   encoding.

# Sequential Work Function {#swf-construction}

CPoP uses a Sequential Work Function (SWF) to bind Evidence to sustained computational effort. For `swf-argon2id` (20) and `swf-argon2id-entangled` (21), the SWF consists of iterated Argon2id {{RFC9106}} evaluations, each individually memory-hard, providing ASIC-resistant sequential work. For `swf-sha256` (10), the SWF uses a single Argon2id evaluation followed by iterated SHA-256 hashing, intended for constrained environments where Argon2id is impractical. This construction is NOT a Verifiable Delay Function in the formal sense {{Boneh2018}}; it does not provide efficient public verification of the delay claim from the output alone.

Instead, verification relies on Merkle-sampled audit proofs: the Attester commits to a Merkle tree over intermediate states, and the Verifier checks a random subset of state transitions. This provides probabilistic verification in O(k * log n) time where k is the sample count and n is the step count.

## Construction {#swf-algorithm}

The SWF is computed as follows.

For `swf-argon2id` (20) and `swf-argon2id-entangled` (21):

~~~ pseudocode
hash_len = output_length(H)          ; 32, 48, or 64 bytes
state_0  = Argon2id(seed, salt=H(0x00 || "CPoP-salt-v1" || seed),
                    t=t, m=m, p=1, len=hash_len)
for i in 1..steps:
    state_i = Argon2id(state_{i-1},
                       salt=H(0x01 || "CPoP-salt-v1" || I2OSP(i, 4)),
                       t=t, m=m, p=1, len=hash_len)
merkle_root = MerkleTree(state_0, state_1, ...,
                         state_steps).root
~~~

Each step is a full Argon2id evaluation bounded by memory bandwidth, ensuring ASIC resistance at every link in the chain. The salt for state\_0 MUST be derived from the seed: salt = H(0x00 \|\| "CPoP-salt-v1" \|\| seed). For subsequent steps i >= 1, the salt MUST be H(0x01 \|\| "CPoP-salt-v1" \|\| I2OSP(i, 4)), where I2OSP encodes i as a 4-byte big-endian integer per {{RFC8017}}. The 0x00 and 0x01 type-tag prefixes provide unambiguous domain separation between the seed-derived initial salt and step-indexed salts, preventing collisions even when the seed value equals I2OSP(i, 4) for some i. The Argon2id output length (`len`) MUST equal the output length of H to ensure SWF state sizes are consistent with the selected hash algorithm.

For `swf-sha256` (10):

~~~ pseudocode
hash_len = 32                        ; SHA-256 fixed
state_0  = Argon2id(seed, salt=H(0x00 || "CPoP-salt-v1" || seed),
                    t=t, m=m, p=1, len=hash_len)
for i in 1..steps:
    if i mod W == 0:
        state_i = Argon2id(state_{i-1},
                           salt=H(0x01 || "CPoP-salt-v1" || I2OSP(i, 4)),
                           t=1, m=m_waypoint, p=1, len=hash_len)
    else:
        state_i = H(state_{i-1})
merkle_root = MerkleTree(state_0, state_1, ...,
                         state_steps).root
~~~

The `swf-sha256` mode uses a single Argon2id initialization followed by iterated SHA-256 hashing with periodic **memory-hard waypoints**. At every W-th step (where W is the waypoint-interval parameter), the Attester MUST compute a full Argon2id evaluation instead of SHA-256, using waypoint-memory (m\_waypoint) as the memory cost. These waypoints bound the ASIC advantage to the Argon2id memory-bandwidth limit at waypoint steps, preventing the ~10,000x speedup that custom SHA-256 ASICs achieve on the intervening steps. Verifiers SHOULD reflect the reduced forgery cost relative to Argon2id-only modes in the forgery-cost-estimate by setting c-swf based on the effective number of Argon2id evaluations (steps / W). When waypoint-interval (proof-params key 5) is absent, the Verifier MUST treat the mode as having no waypoints and apply the lower c-swf bound accordingly.

The merkle-root field (process-proof key 4) MUST contain the Merkle tree root computed over all intermediate states. The final state (state\_steps) is verified as the leaf at index "steps" in the Merkle tree.

## Verification Protocol {#swf-verification}

### Modes 20/21 Verification (Merkle-Sampled) {#verification-modes-20-21}

For `swf-argon2id` (20) and `swf-argon2id-entangled` (21), the
Verifier MUST:

1. Recompute Argon2id from the declared seed to obtain state\_0.
2. For each Fiat-Shamir sampled proof in the Merkle tree, verify
   the sibling path against the committed root using tagged hashing
   (see {{merkle-tree-construction}}) and recompute the state
   transition: Argon2id(state\_i,
   salt=H(0x01 \|\| "CPoP-salt-v1" \|\| I2OSP(i+1, 4)),
   t=t, m=m, p=1, len=hash\_len). Verify the result equals
   state\_{i+1}.
3. Verify the final state (state\_steps) by checking its Merkle
   proof against the committed root (process-proof key 4,
   merkle-root). If the final-leaf index is not included in the
   Fiat-Shamir sample set, the Verifier MUST additionally derive
   or request a proof for it.

A minimum of 20 sampled proofs is REQUIRED for CORE profile.
ENHANCED profile requires 50 proofs. MAXIMUM profile requires 100
proofs.

NOTE: Each sampled proof requires one Argon2id evaluation to
verify. Verification cost is O(k) Argon2id evaluations, each
requiring m KiB of memory. For CORE (k=20, m=65536 KiB),
verification requires approximately 2 seconds on reference
hardware. Verifiers MUST verify samples sequentially or limit
concurrent evaluations to avoid excessive memory consumption.

### Mode 10 Verification (Deterministic Full-Chain) {#verification-mode-10}

For `swf-sha256` (10), the Verifier SHOULD perform deterministic
full-chain verification rather than Merkle-sampled verification.
The Verifier MUST:

1. Recompute state\_0 = Argon2id(seed,
   salt=H(0x00 \|\| "CPoP-salt-v1" \|\| seed),
   t=t, m=m, p=1, len=32).
2. Recompute the full chain sequentially: for i in 1..steps,
   if i mod W == 0, compute state\_i =
   Argon2id(state\_{i-1},
   salt=H(0x01 \|\| "CPoP-salt-v1" \|\| I2OSP(i, 4)),
   t=1, m=m\_waypoint, p=1, len=32);
   otherwise, compute state\_i = H(state\_{i-1}).
3. Construct the Merkle tree from all recomputed states using
   tagged hashing ({{merkle-tree-construction}}).
4. Verify the computed Merkle root equals the committed root
   (process-proof key 4).

Full-chain verification requires steps/W Argon2id evaluations
plus steps SHA-256 evaluations. For CORE (W=1000, steps=10000),
this costs approximately 10 Argon2id evaluations (~1 second) plus
10,000 SHA-256 evaluations (~1 millisecond). This is comparable
to or less than Mode 20 sampled verification cost.

Full-chain verification provides deterministic guarantees: all
waypoints AND all SHA-256 transitions are verified with zero
false negatives. Unlike Merkle-sampled verification, it is not
subject to the waypoint sampling gap (where uniform random
sampling under-represents the rare but expensive waypoint
steps).

When full-chain verification is impractical (e.g., constrained
Verifier environments), the Verifier MAY fall back to
Merkle-sampled verification per {{fiat-shamir-sampling}}. In
this fallback mode, the Verifier MUST additionally verify all
waypoint transitions by requesting Merkle proofs for index pairs
(iW-1, iW) for i in 1..steps/W. This ensures memory-hard
waypoints are always verified regardless of the Fiat-Shamir
sample draw. The Attester MUST include these mandatory waypoint
proofs in addition to the k Fiat-Shamir sampled proofs when the
Verifier indicates sampled verification mode.

## Fiat-Shamir Sample Derivation {#fiat-shamir-sampling}

Merkle proof sample positions MUST be derived deterministically
via Fiat-Shamir transform:

~~~ pseudocode
sample_seed = H(
    "CPoP-Fiat-Shamir-v1" ||
    I2OSP(proof-algorithm, 2) ||
    CBOR-encode(proof-params) ||
    process-proof.input ||
    merkle_root
)
for j in 0..k-1:
    okm_j   = HKDF-Expand(sample_seed, I2OSP(j, 4), 4)
    index_j = OS2IP(okm_j) mod (steps + 1)
~~~

The sample seed MUST incorporate a versioned domain separation
tag ("CPoP-Fiat-Shamir-v1") followed by the full proof context:
the algorithm identifier, all parameters, the SWF input seed, and
the Merkle root. The DST prefix provides version agility
consistent with all other hash constructions in the protocol and
prevents cross-protocol confusion. The algorithm and parameter
binding prevents cross-algorithm confusion attacks where an
adversary exploits a seed that produces the same Merkle root under
a cheaper algorithm. CBOR-encode produces deterministic CBOR per
Section 4.2.1 of {{RFC8949}}.

Where k is the number of required samples (20 for CORE, 50 for
ENHANCED, 100 for MAXIMUM). HKDF-Expand is instantiated with H as
the underlying hash function per {{RFC5869}}. I2OSP
and OS2IP are the Integer-to-Octet-String and Octet-String-to-Integer
primitives as defined in {{RFC8017}}. The Attester
MUST include Merkle proofs for exactly these indices. The Verifier
recomputes the sample positions from the committed root and seed,
then verifies only those proofs.

If the derivation produces duplicate indices (index_j equal to a
previously derived index), the Attester MUST continue generating
additional indices by incrementing j beyond k-1 until k distinct
indices are obtained. The Verifier MUST verify that all k sample
indices are distinct.

Sample indices are in the range \[0, steps\] inclusive.
Padded Merkle tree leaves (indices greater than steps)
are never sampled by this derivation.

## SWF Seed Derivation {#swf-seed-derivation}

For `swf-sha256` (10), the SWF seed for each checkpoint
MUST be derived as:

~~~ pseudocode
seed = H(
    "CPoP-SWF-Seed-v1" ||
    prev-hash ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state) ||
    edit-graph-hash
)
~~~

The `swf-sha256` mode uses iterated SHA-256 as described
in {{swf-algorithm}}. When edit-graph-hash is absent, the
seed MUST be computed without the final term.

For `swf-argon2id` (20), the SWF seed for each
checkpoint MUST be derived as:

~~~ pseudocode
seed = H(
    "CPoP-SWF-Seed-v1" ||
    prev-hash ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state) ||
    edit-graph-hash
)
~~~

When edit-graph-hash is absent, the seed MUST be computed
without the final term.

For `swf-argon2id-entangled` (21), the Attester MUST
entangle the previous checkpoint's SWF output. The seed is
derived as:

~~~ pseudocode
seed = H(
    "CPoP-SWF-Seed-v1" ||
    prev-hash ||
    prev-swf-output ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state) ||
    edit-graph-hash
)
~~~

When edit-graph-hash is absent, the seed MUST be computed
without the final term.

Where `prev-swf-output` is the final Argon2id state
from the immediately preceding checkpoint's sequential work
function computation. This creates a strict cryptographic
dependency chain: each checkpoint's SWF cannot begin until the
previous checkpoint's SWF has completed.

When a verifier-nonce is present ({{verifier-nonce-binding}}),
the nonce MUST be inserted into the seed derivation after
prev-swf-output (for Mode 21) or after prev-hash (for Modes
10/20), before any behavioral data. When both verifier-nonce and
beacon-anchor are present in the seed, the concatenation order
MUST be: prev-hash \|\| verifier-nonce \|\| beacon-anchor \|\|
jitter-sample. When edit-graph-hash is present, it MUST be the
final term in the seed derivation, after all behavioral data
and any beacon-anchor value.

For the first checkpoint (sequence = 1), all modes use:

~~~ pseudocode
seed = H(
    "CPoP-SWF-Seed-v1" ||
    CBOR-encode(document-ref) ||
    initial-jitter-sample
)
~~~

The first checkpoint seed does not include edit-graph-hash
because no editing trajectory has accumulated at session start.

Where initial-jitter-sample is a minimum 32-byte sample of
behavioral entropy collected before the first checkpoint.
When jitter-binding and physical-state are absent (CORE profile
without behavioral data), the seed MUST incorporate at least
the prev-hash and a locally-generated 32-byte random nonce:
seed = H("CPoP-SWF-Seed-v1" \|\| prev-hash \|\| local-nonce). For the first
checkpoint, the nonce provides non-determinism when
initial-jitter-sample is unavailable. Implementations MUST
NOT use a fully deterministic seed derivation.

NOTE: The test vectors in {{test-vectors}} use a
simplified fixed seed for implementation validation. Production
implementations MUST use the derivation specified above.

## Merkle Tree Construction {#merkle-tree-construction}

The SWF Merkle tree is constructed over all intermediate states
using domain-separated hashing following the tagged hash
construction from {{RFC6962}}:

* Leaf nodes: H(0x00 \|\| state\_i) for i in 0..steps, where
  leaf-index = i and leaf-value = state\_i. The total number
  of leaves is (steps + 1). The 0x00 prefix tag identifies
  leaf-level hashes.
* Internal nodes: H(0x01 \|\| left\_child \|\| right\_child).
  The 0x01 prefix tag identifies internal node hashes. This
  domain separation prevents second-preimage attacks on the
  tree structure where an adversary constructs an internal
  node that masquerades as a leaf or vice versa.
* Padding: If the number of leaves is not a power of 2, the
  tree is padded with sentinel values: pad\_value =
  H(0x02 \|\| I2OSP(steps + 1, 4)), repeated until the count
  reaches the next power of 2. The 0x02 prefix tag and
  embedded tree size make padding nodes cryptographically
  distinguishable from both leaf and internal nodes and bind
  the padding to the specific chain length.
* The Merkle root is stored in process-proof.merkle-root (key 4).

The final state (state\_steps) is the leaf at
index "steps" and is verified by checking its Merkle proof
against the committed root.

## Mandatory SWF Parameters {#mandatory-swf-params}

Conforming Attesters SHOULD use the following minimum SWF parameters
for each Evidence Content Tier. These values are initial calibration
parameters (see {{experimental-status-rationale}}) and may be revised
based on deployment experience.

For `swf-argon2id` (20) and `swf-argon2id-entangled` (21):

| Parameter | CORE | ENHANCED | MAXIMUM |
|---|---|---|---|
| time-cost (t) | 1 | 1 | 1 |
| memory-cost (m, KiB) | 65536 | 65536 | 65536 |
| parallelism (p) | 1 | 1 | 1 |
| steps | 90 | 150 | 210 |
| Merkle samples (k) | 20 | 50 | 100 |

For `swf-sha256` (10):

| Parameter | CORE | ENHANCED | MAXIMUM |
|---|---|---|---|
| time-cost (t) | 1 | 1 | 1 |
| memory-cost (m, KiB) | 65536 | 65536 | 131072 |
| parallelism (p) | 1 | 1 | 1 |
| steps | 10000 | 50000 | 100000 |
| waypoint-interval (W) | 1000 | 1000 | 1000 |
| waypoint-memory (KiB) | 32768 | 65536 | 65536 |
| Effective Argon2id evals | 10 | 50 | 100 |
| Merkle samples (k) | 20 | 50 | 100 |

The waypoint-interval and waypoint-memory parameters MUST be declared
in proof-params (keys 5 and 6) for Mode 10 Evidence. Verifiers MUST
reject Mode 10 Evidence that omits these parameters.

Verifiers MUST reject Evidence where declared proof-params are
below the mandatory minimums for the claimed content tier.

Expected wall-clock times for `swf-argon2id` modes on reference
hardware (DDR4, approximately 25 GB/s memory bandwidth {{JESD79-4}}): each
Argon2id step with t=1, m=65536 KiB requires approximately
100ms {{Biryukov2016}}. Target duty cycles at 30-second default checkpoint
intervals: CORE approximately 30% (90 steps, ~9s), ENHANCED
approximately 50% (150 steps, ~15s), MAXIMUM approximately 70%
(210 steps, ~21s). For ENHANCED and MAXIMUM, entangled mode
(swf-argon2id-entangled) is MANDATORY per {{entangled-mode-requirement}}.

For `swf-sha256` mode, the initial Argon2id evaluation requires
approximately 50-100ms; subsequent SHA-256 steps add approximately
0.1ms per 1000 steps.

## Entangled Mode Requirement {#entangled-mode-requirement}

Attesters claiming ENHANCED or MAXIMUM content tier with `swf-argon2id` MUST use `swf-argon2id-entangled` (21) instead of `swf-argon2id` (20). This ensures that each checkpoint's SWF computation depends on the previous checkpoint's final state, creating a strict inter-checkpoint sequential dependency that eliminates parallel pre-computation. Verifiers MUST reject ENHANCED or MAXIMUM Evidence that uses proof-algorithm 20 instead of 21.

## Entangled Binding Computation {#entangled-binding-computation}

When present, the entangled-binding field (checkpoint key 12) MUST be
computed as HMAC-H {{RFC2104}} with keys derived via the
two-stage HKDF hierarchy defined below.

### Key Derivation Hierarchy {#key-derivation-hierarchy}

All keyed consistency tags within a checkpoint MUST derive keys from
a single pseudorandom key (PRK) using the HKDF Extract-then-Expand
paradigm per {{RFC5869}}:

~~~ pseudocode
PRK = HKDF-Extract(
    salt = "CPoP-key-derivation-v1",
    IKM  = process-proof.merkle-root || process-proof.input
)
binding-key = HKDF-Expand(PRK, "CPoP-entangled-binding-v1", hash_len)
tag-key     = HKDF-Expand(PRK, "CPoP-jitter-tag-v1", hash_len)
~~~

The HKDF-Extract step concentrates entropy from the Merkle root
(a hash output) and the SWF input seed into a properly structured
PRK. Including process-proof.input alongside the Merkle root ensures
that the PRK depends on both the computation output and its input,
providing defense-in-depth against multi-target Merkle root collision
attacks. HKDF-Expand then derives independent keys for each
consistency tag using distinct info strings.

### Entangled Binding {#entangled-binding}

The entangled-binding value is computed as:

~~~ pseudocode
binding-input = prev-hash || content-hash ||
                CBOR-encode(jitter-binding) ||
                CBOR-encode(physical-state)
entangled-binding = HMAC-H(binding-key, binding-input)
~~~

Where \|\| denotes concatenation and CBOR-encode produces deterministic
CBOR per Section 4.2.1 of {{RFC8949}}.

NOTE: In the adversarial Attester model, the Attester generates
the SWF output and therefore knows the binding key. The
entangled-binding provides internal consistency but does not
prevent forgery by a malicious Attester (see
{{sec-binding-limitations}}).

## Jitter Tag Computation {#jitter-tag-computation}

When present, the jitter-tag field (jitter-binding key 3) MUST be
computed as HMAC-H using the tag-key derived from the shared PRK
(see {{key-derivation-hierarchy}}):

~~~ pseudocode
tag-input = CBOR-encode(jitter-binding.intervals)
jitter-tag = HMAC-H(tag-key, tag-input)
~~~

The jitter-tag binds the timing measurements to the checkpoint's
SWF computation, preventing transplantation of jitter data from a
different session. It is subject to the same adversarial Attester
limitation as the entangled-binding ({{sec-binding-limitations}}).

NOTE: In the adversarial Attester model, the Attester generates both
the SWF output (from which the binding keys are derived) and the
input data. The entangled-binding and jitter-tag therefore provide
data integrity binding but do not prevent an adversarial Attester
from computing tags over fabricated data. Their security value is
limited to ensuring internal consistency within an honestly-generated
checkpoint. See {{security-considerations}}.

## Beacon-Anchored Binding (Optional) {#beacon-binding}

Attesters MAY anchor checkpoints to a public randomness beacon to
provide verifiable temporal ordering without hardware attestation.
When beacon-anchoring is used, the Attester MUST:

1. Commit to the checkpoint data (including the SWF Merkle root)
   before the beacon value is available.
2. Wait for the beacon to publish the value for the round
   corresponding to the checkpoint timestamp plus a protocol-defined
   delay (RECOMMENDED: 60 seconds).
3. Incorporate the beacon value into the key derivation:

~~~ pseudocode
PRK = HKDF-Extract(
    salt = "CPoP-key-derivation-v1",
    IKM  = process-proof.merkle-root ||
           process-proof.input ||
           beacon-anchor.beacon-value
)
~~~

4. Compute the entangled-binding and jitter-tag using keys derived
   from the beacon-augmented PRK.
5. Include the beacon-anchor field (checkpoint key 16) in the
   checkpoint.

The beacon-anchor structure is:

~~~ cddl
beacon-anchor = {
    1 => tstr,          ; beacon-source (URI of beacon service)
    2 => uint,          ; beacon-round (round number)
    3 => bstr .size 32, ; beacon-value (randomness output)
}
~~~

The Verifier MUST independently fetch the beacon value for the
declared round from the beacon-source and verify it matches
beacon-anchor.beacon-value. Verifiers MUST apply a timeout of no
more than 30 seconds when fetching beacon values. If the beacon
service is unavailable, the Verifier MUST treat the Evidence as
non-beacon-anchored and proceed with remaining verification steps.
The Verifier MUST verify that the beacon round is consistent with
the checkpoint timestamp plus the protocol-defined delay.

This mechanism transforms T1 temporal ordering from self-reported
timestamps to externally verifiable time commitments {{HaberStornetta1991}}. The beacon
value is not predictable at checkpoint creation time, preventing
retroactive fabrication of Evidence with backdated timestamps.

## Verifier-Nonce Binding (Optional) {#verifier-nonce-binding}

For interactive deployments where the Verifier is online during
the authoring session, the Attester MAY incorporate a
Verifier-supplied nonce into each checkpoint's SWF seed. This
provides the strongest available anti-precomputation guarantee:
the SWF cannot begin until the Verifier releases the nonce, and
the Verifier chooses the nonce after the preceding checkpoint is
submitted.

### Protocol {#verifier-nonce-protocol}

1. The Attester submits checkpoint N-1 to the Verifier.
2. The Verifier responds with a 32-byte random nonce:
   verifier-nonce = CSPRNG(32).
3. The Attester incorporates the nonce into checkpoint N's SWF
   seed derivation (see {{swf-seed-derivation}}):

~~~ pseudocode
seed = H(
    "CPoP-SWF-Seed-v1" ||
    prev-hash ||
    prev-swf-output ||          ; Mode 21 only
    verifier-nonce ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state)
)
~~~

4. The Attester includes the verifier-nonce in checkpoint key 17.
5. The Verifier checks that the nonce in the submitted checkpoint
   matches the nonce it issued.

### Security Properties {#verifier-nonce-security}

The verifier-nonce is strictly stronger than beacon anchoring
because:

* The nonce is chosen by the Verifier specifically for this
  Attester and checkpoint, with full 256-bit entropy.
* The Attester cannot predict the nonce (it is revealed only
  after the preceding checkpoint is submitted).
* The Attester cannot precompute the SWF because the seed
  depends on the nonce.
* No dependency on external beacon services or network
  availability.

The verifier-nonce creates a strict request-response dependency
chain: the Verifier controls the pace of checkpoint production
by controlling nonce release timing. This is appropriate for
real-time supervised authoring but not for offline or
asynchronous workflows.

When both verifier-nonce and beacon-anchoring are used, the
seed MUST incorporate both values. The two mechanisms are
complementary: the nonce provides per-Attester freshness, while
the beacon provides publicly verifiable temporal ordering.

## Security Bound {#swf-security}

An adversary who skips fraction f of steps will be detected with probability 1-(1-f)^k where k is the number of sampled proofs. With k=20 and f=0.1, detection probability exceeds 0.878. With k=100 and f=0.05, detection probability exceeds 0.994. This detection probability applies independently to each checkpoint. For a session of N checkpoints, the probability that at least one forged checkpoint is detected is 1 - (1 - p\_detect)^N.

This bound holds under the random oracle model for the selected
hash function H. The Attester commits to the Merkle root before
sample positions are derived via the Fiat-Shamir transform. Finding
a root that biases all k samples away from skipped steps
requires inverting H, which is computationally infeasible under
standard assumptions.

### Seed Grinding Resistance {#seed-grinding}

A grinding adversary may try multiple seeds, selecting one where
the Fiat-Shamir samples avoid skipped steps. This strategy is
strictly anti-profitable for all skip fractions and sample counts:

Theorem: For any fraction f in (0,1) of skipped steps and sample
count k >= 2, the expected total work of a grinding adversary
strictly exceeds honest computation.

Proof sketch: Let n be the step count. Per grinding trial, the
adversary computes a new seed (negligible cost), executes
(1-f)\*n steps honestly, fills f\*n positions with invalid data,
builds the Merkle tree, and derives sample positions. The
probability of all k samples missing skipped positions is
(1-f)^k per trial. The expected number of trials is (1-f)^{-k}.
The expected total work is:

~~~ pseudocode
W_grind = (1-f)*n * (1-f)^{-k} = n * (1-f)^{1-k}
~~~

Since k >= 2, the exponent (1-k) <= -1, so (1-f)^{1-k} =
1 / (1-f)^{k-1} > 1 for all f in (0,1). Therefore W\_grind > n =
W\_honest.

The grinding overhead is multiplicative and grows rapidly: for
k=20 and f=0.10, W\_grind is approximately 8.2n (8.2x honest work);
for k=100 and f=0.05, W\_grind is approximately 131n. Grinding is
counterproductive because each trial requires computing the full
honest chain to obtain the Merkle root from which sample positions
are derived, but the savings from skipping are sublinear in the
trial count. This result holds regardless of the adversary's
computational resources.

## Hardware-Anchored Time (HAT) {#hat}

In T3/T4 tiers, the AE MUST construct a HAT proof bracketing each
SWF computation to bind the temporal claim to the TPM's internal
clock. This prevents "SWF Speed-up" attacks by ensuring that the
temporal proof is bound to hardware-attested time that the
adversarial Attester cannot compress or manipulate.

### HAT Proof Construction {#hat-construction}

The Attester MUST obtain TPM-attested time readings before and after
each SWF computation:

~~~ pseudocode
T_before = TPM2_GetTime(aikHandle)
seed = H("CPoP-SWF-Seed-v1" || ... || T_before.attestation)
... execute SWF ...
T_after = TPM2_GetTime(aikHandle)
~~~

The HAT proof structure is:

~~~ cddl
hat-proof = {
    1 => bstr,     ; time-before (TPMS_TIME_ATTEST_INFO)
    2 => bstr,     ; time-after (TPMS_TIME_ATTEST_INFO)
    3 => bstr,     ; sig-before (AIK signature)
    4 => bstr,     ; sig-after (AIK signature)
}
~~~

The Attester MUST include the hat-proof in checkpoint key 15. The
SWF seed derivation ({{swf-seed-derivation}}) MUST incorporate the
T\_before.attestation value, binding the SWF computation to the
pre-SWF TPM clock reading.

### HAT Verification {#hat-verification}

The Verifier MUST perform the following checks on HAT proofs:

1. Verify both signatures (sig-before, sig-after) against the AIK
   public key from the TPM Endorsement hierarchy.
2. Verify that resetCount is identical in both time-before and
   time-after attestations, confirming no platform reboot occurred
   between measurements.
3. Verify T\_before.clockInfo.safe is true, confirming the TPM clock
   was not disturbed.
4. Verify the time delta: T\_after.clock - T\_before.clock MUST be
   greater than or equal to the expected SWF duration for the
   declared steps and parameters. This bounds the minimum
   computation time to what the TPM's hardware clock observed.
5. Verify that process-proof.input (seed) incorporates
   T\_before.attestation as specified in {{swf-seed-derivation}}.
6. For checkpoints after the first, verify T\_before.clock is
   strictly greater than the preceding checkpoint's T\_after.clock,
   ensuring temporal chain continuity at the hardware level.

The temporal sandwich bounds both acceleration (the SWF cannot
complete in less time than the TPM clock interval) and replay
(the TPM clock is monotonic and the resetCount check detects
reboots). Combined with Mode 21 cryptographic entanglement, this
creates a double-layered temporal chain: hardware-attested time
intervals AND cryptographic sequential dependency.

# Experimental Status Rationale {#experimental-status-rationale}

This document and its companion {{CPoP-Appraisal}} are published
with Experimental status for the following reasons:

1. *Novel RATS trust model:* CPoP inverts the standard RATS
   trust assumption by treating the Attester as the primary
   adversary. This adversarial Attester model has no precedent
   in IETF attestation standards. Implementation and deployment
   experience is needed to validate the practical effectiveness
   of this trust inversion.

2. *Physical process attestation:* Extending RATS from device
   state attestation to continuous physical process attestation
   is a new application area. The efficacy of SWF-based temporal
   binding and behavioral entropy capture as anti-forgery
   mechanisms requires validation across diverse hardware
   platforms, authoring workflows, and adversarial scenarios.

3. *Calibration parameters:* The SWF step counts, checkpoint
   intervals, Merkle sample counts, and forensic thresholds
   defined in this specification are initial values. Field
   deployment may reveal that adjustments are needed to balance
   security, performance, and usability across the range of
   target environments.

4. *Ecosystem dependencies:* The Tool Receipt protocol depends
   on a tool provider key discovery mechanism that is deferred
   to a companion document. The EAT profile and SEAT integration
   points reference specifications that are themselves in
   development.

5. *Forensic thresholds:* The MUST-level forensic flagging
   thresholds in {{CPoP-Appraisal}} (spectral flatness, CLC
   correlation, CoV bounds, etc.) ensure interoperable Verifier
   behavior for initial deployments but may be relaxed to SHOULD
   in a future revision based on false positive analysis across
   diverse author populations.

Implementers are encouraged to report deployment experience,
particularly regarding SWF performance on diverse hardware,
false positive rates in forensic assessment, and adversarial
evasion attempts.

# IANA Considerations {#iana-considerations}

This document requests the following IANA registrations:

## CBOR Tags {#iana-cbor-tags}

This document requests registration of two CBOR tags in the
"CBOR Tags" registry per RFC 8949, Section 9.2:

Tag 1129336656:

Tag:
: 1129336656

Data Item:
: map

Semantics:
: CPoP Evidence Packet (see {{wire-format}} of this document)

Point of Contact:
: David Condrey (david@writerslogic.com)

Description of Semantics:
: \[this document\]

Tag 1129791826:

Tag:
: 1129791826

Data Item:
: map

Semantics:
: CPoP Attestation Result (see {{CPoP-Appraisal}})

Point of Contact:
: David Condrey (david@writerslogic.com)

Description of Semantics:
: \[this document\], {{CPoP-Appraisal}}

## Note on SMI Private Enterprise Number {#iana-smi-pen}

This specification does not require or register an SMI Private
Enterprise Number. WritersLogic Inc (PEN 65074) is the
author's organizational affiliation and is noted here for
transparency; no IANA action is requested for this PEN.

## EAT Profile {#iana-eat-profile}

This document requests registration of the following EAT
profile in the "EAT Profiles" registry (or its successor
registry established by {{RFC9711}} or the EAR specification):

Profile Name:
: CPoP Attestation Result Profile

Profile URI:
: urn:ietf:params:rats:eat:profile:pop:1.0

Description:
: Profile for Cryptographic Proof of Process (CPoP) Attestation
  Results (WAR format) as defined in {{CPoP-Appraisal}}. The
  CPoP Evidence Packet format uses a domain-specific CBOR
  structure (tag 1129336656) that is not an EAT token; this
  profile URI identifies the Verifier output format. See
  {{evidence-eat-relationship}} for the architectural
  rationale.

Reference:
: \[this document\]

Change Controller:
: David Condrey (david@writerslogic.com)

## Media Types {#iana-media-types}

This document requests provisional registration of the following
media types in the Standards tree per {{RFC6838}}:

application/cpop+cbor:

Type name:
: application

Subtype name:
: cpop+cbor

Required parameters:
: none

Optional parameters:
: none

Encoding considerations:
: binary (CBOR, {{RFC8949}})

Security considerations:
: See {{security-considerations}} of this document.
  CPoP Evidence Packets contain cryptographic hashes and timing
  data but no document content. Receivers MUST validate CBOR
  structure before processing. Evidence Packets may contain
  privacy-sensitive behavioral data (keystroke timing).

Interoperability considerations:
: See {{wire-format}} of this document. Evidence Packets are
  CBOR-encoded maps tagged with CBOR tag 1129336656.

Published specification:
: \[this document\]

Applications that use this media type:
: Authorship attestation systems, content provenance platforms,
  digital forensics tools, and publishing workflow systems that
  generate or consume CPoP Evidence Packets.

Fragment identifier considerations:
: N/A; this media type does not define fragment identifiers.

Additional information:

: Deprecated alias names for this type:
  : N/A

  Magic number(s):
  : N/A (identified by CBOR tag 1129336656)

  File extension(s):
  : .cpop

  Macintosh file type code(s):
  : N/A

Person & email address to contact for further information:
: David Condrey (david@writerslogic.com)

Intended usage:
: COMMON

Restrictions on usage:
: none

Author:
: David Condrey

Change controller:
: David Condrey (david@writerslogic.com)

application/cwar+cbor:

Type name:
: application

Subtype name:
: cwar+cbor

Required parameters:
: none

Optional parameters:
: none

Encoding considerations:
: binary (CBOR, {{RFC8949}})

Security considerations:
: See {{CPoP-Appraisal}}. Attestation Results contain forensic
  assessment outcomes and forgery cost estimates. Receivers
  MUST validate CBOR structure and COSE signatures before
  relying on verdict fields.

Interoperability considerations:
: See {{CPoP-Appraisal}}. Attestation Results are CBOR-encoded
  maps tagged with CBOR tag 1129791826.

Published specification:
: \[this document\], {{CPoP-Appraisal}}

Applications that use this media type:
: Authorship verification systems, content trust platforms,
  and relying party applications that consume CPoP Attestation
  Results (Cryptographic Writers Authenticity Reports).

Fragment identifier considerations:
: N/A; this media type does not define fragment identifiers.

Additional information:

: Deprecated alias names for this type:
  : N/A

  Magic number(s):
  : N/A (identified by CBOR tag 1129791826)

  File extension(s):
  : .cwar

  Macintosh file type code(s):
  : N/A

Person & email address to contact for further information:
: David Condrey (david@writerslogic.com)

Intended usage:
: COMMON

Restrictions on usage:
: none

Author:
: David Condrey

Change controller:
: David Condrey (david@writerslogic.com)

## TLS Exporter Label {#iana-tls-exporter}

This document requests registration of the following TLS exporter
label in the "TLS Exporter Labels" registry defined in {{RFC5705}}:

Value:
: EXPORTER-CPoP-channel-binding

DTLS-OK:
: Y

Recommended:
: N

Reference:
: \[this document\]

## Future Registry Considerations {#iana-future-registries}

This specification and its companion {{CPoP-Appraisal}} define
several enumerated code points in their CDDL schemas.
Code points defined in this document: proof-algorithm,
attestation-tier, content-tier, hash-algorithm, hash-salt-mode,
binding-type, probe-type, and confidence-tier. Code points
defined in {{CPoP-Appraisal}}: verdict, absence-type, and
cost-unit. During the Experimental phase, these values
are defined inline in the CDDL and do not require separate
IANA registries. Extension keys in the evidence-packet and
checkpoint structures use integer values 100 or greater
(keys 0-99 are reserved for this specification).

If this specification advances beyond Experimental status,
IANA registries with "Specification Required" or "Expert
Review" policies will be established for these code points
to enable interoperable extension.

# Security Considerations {#security-considerations}

This section provides security analysis following {{RFC3552}} guidelines. The threat model is defined in {{threat-model}} with the adversarial Attester as the primary threat actor. Detailed forensic security analysis is provided in {{CPoP-Appraisal}}.

The CPoP adversary model assumes an attacker with: (a) polynomial-time computation, (b) access to the same SWF parameters as honest Attesters, (c) ability to generate arbitrary behavioral telemetry, but (d) inability to violate the hardness assumptions of the selected hash function or memory-hard function. The attacker's goal is to produce Evidence that passes Verifier appraisal without genuine human cognitive involvement in content creation.

## Security Layer Model {#sec-layer-model}

CPoP security guarantees operate at three distinct layers with different assurance properties. Verifiers MUST NOT treat lower layers as providing the formal guarantees of higher layers.

Layer 1, Temporal Binding (Cryptographic):
: The SWF forces minimum real wall-clock cost derivable from Argon2id memory-hardness {{RFC9106}} and the sequential dependency chain. With entangled mode at 50% duty cycle, forging N hours of Evidence requires approximately N/2 hours of computation at minimum. This is a formal, quantifiable guarantee independent of behavioral signal quality.

Layer 2, Behavioral Analysis (Statistical):
: SNR, CLC, error topology, cadence analysis, and session consistency mechanisms detect fabricated behavioral data and raise the practical cost of forgery. These mechanisms provide defense-in-depth but carry no formal cryptographic guarantee. Thresholds are grounded in the keystroke dynamics literature {{Monrose2000}}{{Monaco2018}}{{Salthouse1986}}{{Dhakal2018}} and calibrated against real-world composition data {{ScholaWrite}}{{ScholaWriteAugmented}}. Single-feature timing approaches are demonstrably insecure {{Condrey2026Attack}}; the multi-dimensional cross-domain entanglement architecture is a direct response to those attack classes.

Layer 3, Hardware Attestation (Physical):
: T3/T4 non-repudiation via hardware roots of trust provides physical freshness markers non-reproducible by the Attester operator.

The forgery cost estimates in the WAR derive from Layer 1 and Layer 3 only. Layer 2 mechanisms inform the forensic-summary and forensic-flag fields but do not contribute to the C_swf or C_hardware cost bounds.

## Primary Threat: Adversarial Attester {#sec-primary-threat}

Unlike traditional remote attestation where external adversaries threaten system integrity, CPoP's primary threat is the Attester operator themselves. The author controls the Attesting Environment and has incentive to claim authenticity for AI-generated or assisted content.

This threat model inversion has fundamental implications:

* Software-only attestation (T1) provides minimal assurance since the Attester controls all software
* Cryptographic proofs must be bound to physical constraints the Attester cannot circumvent
* Behavioral entropy must be economically expensive to forge, not merely cryptographically secure
* Trust in Evidence scales with the Attestation Tier and the cost of bypassing its guarantees

## Retype Attack Defenses {#sec-retype-defense}

The retype attack (see {{retype-attack}}) is the canonical forgery vector. Defenses are layered:

Cognitive Load Correlation (CLC):
: Verifiers analyze correlation between content complexity and typing cadence as specified in {{CPoP-Appraisal}}.

Error Topology Analysis:
: Authentic authoring produces characteristic error patterns: corrections localized near recent insertions, deletion-to-insertion ratios consistent with human cognitive models {{Salthouse1986}}{{ScholaWrite}}{{ScholaWriteAugmented}}, and fractal self-similarity in revision patterns {{Takens1981}}{{Orden2003}}. Retyping produces either unnaturally low error rates or randomly distributed artificial errors.

Temporal Cost:
: Even successful retype attacks require real-time effort. A 5,000-word document with 10-second checkpoint intervals requires 8+ hours of continuous typing effort to forge. The attack does not scale economically for high-volume forgery.

Relying Parties need to be aware that retype attacks remain viable for short documents or high-value targets willing to invest real time. CPoP provides graduated assurance proportional to document length and checkpoint density.

## Relay and Replay Attack Defenses {#sec-relay-replay}

As defined in {{replay-attack}} and {{relay-attack}}, these attacks are defeated through Physical Freshness anchors binding Evidence to non-reproducible physical state:

* Thermal trajectories captured during SWF computation cannot be replayed
* Kernel entropy pool deltas are bound to specific execution moments
* Out-of-band presence challenges (QR scans) verify real-time physical proximity

Verifiers MUST reject Evidence where physical freshness markers are stale, inconsistent with timestamps, or exhibit patterns suggesting simulation.

## SWF Acceleration Defenses {#sec-swf-speedup}

As analyzed in {{swf-acceleration}}, specialized hardware attacks are mitigated by:

* *Memory-hardness:* For modes 20/21, every SWF step is a full Argon2id evaluation bounded by memory bandwidth (approximately 50 GB/s for DDR5 {{JESD79-5}}), not ALU throughput. ASICs provide minimal advantage per step, and this resistance compounds across all steps in the chain. For mode 10, memory-hard waypoints ({{swf-algorithm}}) bound the ASIC advantage at waypoint steps to the Argon2id limit.
* *Hardware-Anchored Time (T3/T4):* The HAT temporal sandwich protocol ({{hat}}) brackets each SWF computation with TPM-attested clock readings, preventing time compression even with faster computation. The Verifier checks that the TPM clock delta meets or exceeds the expected SWF duration.
* *Merkle sampling:* Skipping SWF steps is detected probabilistically. With k=100 samples, skipping 5% of steps has >99.4% detection probability. Seed grinding does not improve the adversary's position ({{seed-grinding}}).

## Trust Gradation by Tier {#sec-tier-trust}

Relying Parties SHOULD interpret Evidence according to its Attestation Tier. Detailed per-tier verification constraints are defined in {{CPoP-Appraisal}}.

| Tier | Forgery Resistance | Suitability |
|------|-------------------|-------------|
| T1 | Temporal ordering only; behavioral claims forgeable | Low-stakes or supplementary |
| T2 | Moderate; platform attestation hooks | Casual forgery deterrence |
| T3 | Strong; requires physical SE access | Most applications |
| T4 | Invasive hardware attack required | High-stakes legal/financial |

## Forgery Cost Bounds {#sec-economic-bounds}

Implementations SHOULD report quantified forgery cost estimates in Attestation Results. For CORE profile with `swf-argon2id` (90 steps, m=65536 KiB):

* Sequential computation time: 90 Argon2id evaluations at approximately 100ms each requires ~9 seconds of sustained memory-hard computation per checkpoint on consumer hardware (DDR4, ~25 GB/s memory bandwidth).
* Memory requirement: 64 MiB per step (steps are sequential, not concurrent)
* Duty cycle: approximately 30% of the 30-second checkpoint interval
* Energy cost per checkpoint: approximately $0.0001 USD at consumer electricity rates

For ENHANCED profile with `swf-argon2id-entangled` (150 steps), duty cycle rises to approximately 50%. For MAXIMUM (210 steps), approximately 70%. Entangled mode creates strict inter-checkpoint sequential dependencies: forging N hours of ENHANCED authorship requires approximately N/2 hours of real computation. The forgery cost scales linearly with session duration and superlinearly with checkpoint count due to session consistency requirements across the behavioral dimensions.

### ASIC Advantage Analysis {#asic-advantage}

The ASIC advantage for Argon2id-based SWF modes is bounded by three
independent factors:

Time-Memory Tradeoff (TMTO):
: Single-pass Argon2id (t=1) permits at most ~2x reduction in
  time-area product via ranking-based tradeoff attacks
  ({{RenDevadas2017}}). Multi-pass reduces this to ~1.33x
  ({{RFC9106}}, Section 7). CPoP uses t=1 because the TMTO
  advantage is offset by the multiplicative effect of iterated
  evaluations: an adversary gaining 2x per step gains 2x overall
  (not 2^steps), while t>1 would reduce Attester throughput and
  therefore the achievable step count per checkpoint interval.

Memory Bandwidth:
: Each Argon2id step is bounded by memory bandwidth, not ALU
  throughput. Consumer DDR4 provides ~25 GB/s {{JESD79-4}}; DDR5 provides
  ~50 GB/s {{JESD79-5}}. HBM3 (available in datacenter ASICs) provides
  ~800 GB/s per stack {{JESD238}} but at substantially higher cost per
  device. The effective economic advantage is approximately
  3-4x when amortized over device cost.

Silicon Optimization:
: Custom Argon2id ASICs can eliminate instruction decode overhead
  and optimize the Blake2b core, providing an estimated 1.5-2x
  advantage over general-purpose CPUs for the same memory
  bandwidth {{Biryukov2016}}{{RenDevadas2017}}.

Combined Advantage:
: The multiplicative combination of these factors yields an upper
  bound of approximately 8-16x for a fully optimized ASIC versus
  consumer DDR4 hardware. However, the economic cost of such
  hardware ensures that the forgery cost in USD remains
  substantial. Verifiers SHOULD use a conservative ASIC advantage
  factor of 10x when computing c-swf in forgery-cost-estimate.

For `swf-sha256` (Mode 10) SHA-256 iterations between waypoints, the
ASIC advantage exceeds 10,000x. The memory-hard waypoints
({{swf-algorithm}}) ensure that the effective ASIC advantage over a
full Mode 10 chain is bounded by the Argon2id advantage at waypoint
steps, since waypoints dominate the total computation time.

## Denial of Service {#sec-dos}

SWF verification is asymmetric: Merkle-sampled proofs verify in O(k) versus O(n) generation, where k is the sample count and n is the step count. For `swf-argon2id` modes, each sampled proof requires one Argon2id evaluation (~100ms), so verification cost is approximately k * 100ms (2s for CORE, 5s for ENHANCED, 10s for MAXIMUM). This is substantially less than generation cost (9-21s per checkpoint). Implementations SHOULD implement rate limiting on Evidence submission.

## Consistency Binding Limitations {#sec-binding-limitations}

The entangled-binding and jitter-tag fields are HMAC values keyed
from the SWF output via a two-stage HKDF hierarchy
({{key-derivation-hierarchy}}). In the adversarial Attester model,
the Attester generates the SWF output and therefore knows the
binding keys. An adversarial Attester can compute valid bindings
over fabricated data (synthetic jitter, manufactured physical
state). These fields provide internal consistency checking but do
not prevent forgery by the Attester. Their value is limited to:

* Binding data fields to the SWF computation within an honestly-generated checkpoint, ensuring the data was committed at checkpoint-generation time (when the SWF output was computed) rather than substituted afterward
* Providing internal consistency verification (note: the binding keys are derivable from the public merkle-root field; these bindings do not provide third-party tamper detection)
* In T3/T4 tiers, the packet-level hardware-bound signature (see {{attester-state-machine}}) provides the actual integrity guarantee
* When beacon-anchoring ({{beacon-binding}}) is used, the binding keys incorporate externally-verifiable randomness, providing temporal anchoring even without hardware attestation

## Edit Graph Commitment Limitations {#sec-edit-graph-limitations}

The edit-graph-hash ({{edit-graph-hash}}) entangles editing
trajectory data into the SWF seed, binding the claimed editing
pattern to the SWF time proof. This binding ensures that an
adversary cannot substitute a different editing pattern after
SWF computation without recomputing the entire chain.

However, the edit graph commitment does not prove that the
committed data reflects genuine human authoring. In T1/T2
tiers, the adversarial Attester controls the Attestation
Environment and can fabricate plausible cursor positions,
revision depths, and pause durations before computing the SWF.
The edit-graph-hash adds one additional dimension of data that
must be fabricated consistently, but does not create a
cryptographic barrier to fabrication.

The MAXIMUM-tier histograms ({{edit-graph-histograms}}) enable
statistical analysis of editing patterns by Verifiers. These
histograms may reveal anomalies such as unnaturally uniform
distributions, missing burstiness in pause patterns, or
zero revision depth across all regions. Such statistical tests
are probabilistic and can be defeated by a sufficiently
sophisticated adversary. Verifiers SHOULD treat histogram
analysis as one signal among many, not as definitive
authentication.

## Physical Freshness by Tier {#sec-physical-freshness-tiers}

In T1 (Software-Only) and T2 (Attested Software) tiers, the Attester
controls all software including the operating system. Physical state
readings (thermal trajectories, kernel entropy deltas) are obtained
from OS interfaces that the adversarial Attester can intercept or
fabricate. Verifiers MUST NOT treat physical-state or physical-liveness
fields as evidence of physical freshness in T1/T2 Evidence Packets.
Their value in these tiers is limited to increasing the dimensionality
of data that an adversary must fabricate consistently.

Physical freshness provides meaningful anti-replay protection only in
T3/T4 tiers where hardware attestation binds physical state readings
to a trusted execution environment.

## Implementation Security Requirements {#sec-implementation-requirements}

Conforming implementations MUST:

* Use constant-time comparison for all cryptographic operations
* Zero sensitive memory (keys, jitter data) after use
* Validate all input lengths and formats before processing
* Reject Evidence with inconsistent internal state (e.g., checkpoint-hash verification failure)

T3/T4 implementations MUST additionally:

* Store signing keys exclusively in hardware Secure Elements
* Bind SWF seeds to TPM monotonic counters
* Verify platform integrity before Evidence generation

## Signing Key Lifecycle and Rotation {#sec-key-rotation}

For T1/T2 tiers where signing keys are software-managed, the
Attester SHOULD implement a key rotation policy to limit the
impact of key compromise. The following normative guidelines
apply:

1. *Rotation frequency:* Software-managed signing keys
   SHOULD be rotated at least every 90 days or after generating
   10,000 Evidence Packets, whichever comes first. Attesters
   MAY rotate more frequently.
2. *Key binding:* Each Evidence Packet's COSE_Sign1
   envelope identifies the signing key. When a key is rotated,
   the Attester MUST begin using the new key for all subsequent
   Evidence Packets. Evidence Packets MUST NOT be re-signed
   with a new key after initial generation.
3. *Key continuity:* To allow Verifiers to validate
   older Evidence Packets, the Attester SHOULD publish a key
   history document (format out of scope) linking retired
   public keys to their validity periods. Verifiers MUST
   accept Evidence signed with a retired key if the Evidence
   creation timestamp falls within the key's validity period.
   Verifiers SHOULD maintain signing key history for at least
   365 days.
4. *Compromise response:* If a signing key is
   suspected compromised, the Attester MUST immediately cease
   using it and SHOULD publish a revocation. Evidence Packets
   signed with a compromised key after the suspected
   compromise date SHOULD be treated as inconclusive by
   Verifiers.

For T3/T4 tiers, key management is governed by the hardware
Secure Element's lifecycle. Key rotation in hardware tiers
typically requires device re-provisioning, which is outside
the scope of this specification.

# Privacy Considerations {#privacy-considerations}

This section addresses privacy in accordance with {{RFC6973}}.

## Data Minimization {#priv-minimization}

CPoP Evidence Packets do not contain document content. Content binding uses cryptographic hashes (H, as selected by hash-algorithm) which are computationally irreversible. The author-salted mode (hash-salt-mode=1) provides additional protection by preventing rainbow-table correlation across documents.

## Behavioral Fingerprinting {#priv-fingerprinting}

Jitter sequences in ENHANCED and MAXIMUM profiles constitute behavioral biometrics. To protect author privacy, Verifiers are expected to:

* Discard jitter data after the verification session completes
* Avoid correlating jitter across multiple Evidence Packets to prevent author deanonymization
* Use jitter data solely for authenticity verification

Attesters MUST apply a minimum quantization step of 5 milliseconds to all inter-keystroke interval values before encoding in jitter-binding. Finer resolution MUST NOT be used without explicit operator configuration and documented consent.

## Physical State Leakage {#priv-physical-leakage}

Thermal trajectories and kernel entropy deltas in the physical-state field may reveal information about the Attester's hardware configuration. Implementations SHOULD normalize thermal data to relative deltas rather than absolute values to prevent device fingerprinting.

## Unlinkability {#priv-unlinkability}

Authors who wish to remain pseudonymous SHOULD use per-document signing keys and the author-salted content binding mode to prevent cross-document linkage.

## Privacy Budget {#priv-budget}

Each Evidence Packet released by an Attester discloses
behavioral telemetry that incrementally reduces the author's
anonymity set. Attesters SHOULD maintain an accounting of
cumulative privacy exposure and enforce budgetary limits
on high-fidelity data disclosure:

Jitter resolution:
: Attesters SHOULD default to the coarsest quantization
  (5ms) that satisfies the target content tier's entropy
  threshold. Finer resolution (1ms) SHOULD only be emitted
  when explicitly required by the Verifier's appraisal
  policy and the author has consented to the increased
  disclosure.

Physical-state detail:
: CORE and ENHANCED profiles SHOULD omit per-checkpoint
  physical-state (key 11) unless required by the deployment's
  appraisal policy. When included, thermal values MUST be
  normalized to relative deltas.

Session-level budget:
: Attesters SHOULD track the cumulative number of Evidence
  Packets generated for a given author identity (or signing
  key). When the cumulative count exceeds 1,000 packets with
  the same behavioral source, the Attester SHOULD warn the
  author that cross-document stylometric correlation risk is
  elevated and SHOULD recommend key rotation and increased
  jitter quantization.

Selective disclosure:
: When the Relying Party's trust requirements are satisfied
  by a lower content tier, the Attester SHOULD generate
  Evidence at the lowest tier sufficient for the use case,
  minimizing the behavioral data emitted.

--- back

# SWF Test Vectors {#test-vectors}
{:numbered="false"}

The following test vectors validate SWF implementations.

These test vectors use the type-tagged salt derivation (0x00/0x01
prefixes) and memory-hard waypoints for Mode 10 as specified in
{{swf-algorithm}}. All vectors use SHA-256 (hash-algorithm value 1).

## swf-sha256 (Mode 10) Test Vector {#test-vector-mode10}
{:numbered="false"}

NOTE: This vector uses the `swf-sha256` construction with
memory-hard waypoints (W=1000, waypoint-memory=32768 KiB):
Argon2id initialization, then iterated SHA-256 with Argon2id
waypoints at every 1000th step. The initial salt uses the
type-tagged derivation H(0x00 \|\| "CPoP-salt-v1" \|\| seed).
Waypoint salts use H(0x01 \|\| "CPoP-salt-v1" \|\| I2OSP(i, 4)).

~~~ test-vectors
Seed: "cpop-genesis-v1"
Seed (hex): 7769746e657373642d67656e657369732d7631
Salt: H(0x00 || "CPoP-salt-v1" || seed)  [H = SHA-256]

Argon2id Parameters (initialization):
  Time Cost (t): 1
  Memory Cost (m): 65536 KiB
  Parallelism (p): 1
  Output Length: 32 bytes

Argon2id Parameters (waypoints):
  Time Cost (t): 1
  Memory Cost (m): 32768 KiB
  Parallelism (p): 1
  Output Length: 32 bytes

Steps: 10,000
Waypoint Interval (W): 1000

Salt (hex): 966efc16acdedf88bd3b841d9576d6b9
             5b3a58dfba2d9b2087b6f02da126d296

Intermediate States:
  state_0 (Argon2id):
    55518d63068b5f245d9dccf5919cbcdc
    1fa1b3256e89a5c1eb7a7b37609b323f
  state_1000 (waypoint, Argon2id):
    f880ebfd403904f134c8ddaaa85e21dd
    4803293a8e5eb95eafe7ec88944f28c6
  state_5000 (waypoint, Argon2id):
    f9884b1c4bd487cda521ee3476079ae1
    8be449a086ec06ffbd4f8b09c75ad9f9
  state_9999 (SHA-256):
    b0ccd34431edab8f4fe568bee0fa4bdd
    ac971a3d7057bf23d33097d87eb81968
  state_10000 (waypoint, Argon2id, final):
    19cbc991d4f154f47f912aa232a0c36b
    c9f205c6cc1609984a142c9bd1f745a7
~~~

## swf-argon2id (Mode 20) Test Vector {#test-vector-mode20}
{:numbered="false"}

NOTE: This vector uses the `swf-argon2id` construction: iterated
Argon2id evaluations with type-tagged salts. Each step feeds the
previous state as the password input. The salt for state\_0 is
H(0x00 \|\| "CPoP-salt-v1" \|\| seed). The salt for step i >= 1 is
H(0x01 \|\| "CPoP-salt-v1" \|\| I2OSP(i, 4)). Implementers should
verify state\_0 matches the `swf-sha256` vector above (identical
Argon2id initialization).

~~~ test-vectors
Seed: "cpop-genesis-v1"
Seed (hex): 7769746e657373642d67656e657369732d7631

Argon2id Parameters (per step):
  Time Cost (t): 1
  Memory Cost (m): 65536 KiB
  Parallelism (p): 1
  Output Length: 32 bytes

Steps: 3

Intermediate States:
  state_0 (Argon2id, seed as password,
           salt=H(0x00 || "CPoP-salt-v1" || seed)):
    55518d63068b5f245d9dccf5919cbcdc
    1fa1b3256e89a5c1eb7a7b37609b323f
  state_1 (Argon2id, state_0 as password,
           salt=H(0x01 || "CPoP-salt-v1" || I2OSP(1, 4))):
    6a6df1cfbce07c09036526e19f7b6e73
    ef2ce911d1ea77a66bb23bde5b033a79
  state_2 (Argon2id, state_1 as password,
           salt=H(0x01 || "CPoP-salt-v1" || I2OSP(2, 4))):
    bfa124c53651b2aedc79f48ec562342f
    91efc8bc61cd8f833a5e63efbb41af44
  state_3 (Argon2id, state_2 as password,
           salt=H(0x01 || "CPoP-salt-v1" || I2OSP(3, 4))):
    bdd55e641b507d2d2d49cb67cb34c78d
    92952ce025ef1b22a906f4721bcceb7c
~~~

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The author thanks the participants of the RATS working group for
their ongoing work on remote attestation architecture and security
considerations that informed this specification.
