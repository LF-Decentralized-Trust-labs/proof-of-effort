---
v: 3
docname: draft-condrey-cpoe-protocol-latest
title: "Cryptographic Proof of Effort (CPoE): Architecture and Evidence Format"
abbrev: CPoE Protocol
category: exp
ipr: trust200902
submissiontype: independent
date: 2026-05
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
  PoSME:
    title: "Proof of Sequential Memory Execution (PoSME)"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-posme-00
  RFC9266:
  RFC9334:
  RFC9711:

informative:
  CPoE-Anchoring:
    title: "Cryptographic Proof of Effort (CPoE): Temporal Anchoring Extensions"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-anchoring-00
  CPoE-UseCases:
    title: "Cryptographic Proof of Effort (CPoE): Use Cases and Deployment Considerations"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-usecases-00
  CPoE-Appraisal:
    title: "Cryptographic Proof of Effort (CPoE): Forensic Appraisal and Security Model"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-appraisal-00
  RATS-HAT:
    title: "Hardware Attestation of Time (HAT): TPM-Based Temporal Binding for Remote Attestation"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026
    seriesinfo:
      Internet-Draft: draft-condrey-hat-00
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

This document specifies the Cryptographic Proof of Effort (CPoE) Evidence Framework, a specialized profile of Remote Attestation Procedures (RATS) designed to attest to human cognitive involvement in digital content creation. Unlike traditional provenance, which tracks file custody, CPoE captures behavioral signals during the authoring process -- keystroke dynamics, cognitive load patterns, editing trajectories, and error correction topology -- and binds them into a cryptographic evidence chain that is computationally expensive to forge. The behavioral signals serve as the primary authentication mechanism; a time-locked Sequential Work Function (SWF) prevents retroactive fabrication of evidence by forcing real wall-clock time between checkpoints. Technical specifications for wire formats, behavioral binding, sequential work functions, and hardware-anchored trust are provided.

--- to_be_removed_note_Discussion_Venues

Source for this draft and an issue tracker can be found at
<https://github.com/LF-Decentralized-Trust-labs/proof-of-effort>.

--- middle

# Introduction {#introduction}

The rapid proliferation of generative artificial intelligence has created an authenticity crisis in digital discourse. While traditional provenance tracks the "custody of pixels," it fails to attest to the human-driven process of creation. This document specifies the Cryptographic Proof of Effort (CPoE) protocol, which extends the RATS architecture {{RFC9334}} to provide cryptographic attestation of human cognitive involvement in content creation.

CPoE does not measure computational work or quantify how "hard" someone worked. "Effort" in this context refers to the observable behavioral evidence of a human cognitive process: the characteristic patterns of inter-keystroke timing that exhibit biological motor noise (1/f spectral properties, Hurst exponent H in 0.55-0.85), cognitive load correlation between typing cadence and content complexity, error correction topology consistent with human self-monitoring, and editing trajectories that reflect compositional rather than transcriptive behavior. These behavioral signals are the primary authentication mechanism.

The protocol captures these signals during authoring and binds them into a hash-chained sequence of checkpoints. Each checkpoint entangles the document's content hash, behavioral entropy (jitter binding), and physical state markers with a time-locked Sequential Work Function (SWF) proof. The SWF uses memory-hard Argon2id computation to prevent retroactive fabrication: an adversary cannot generate valid Evidence without spending real wall-clock time proportional to the claimed authorship duration. The behavioral analysis and the time-lock operate as independent, complementary layers -- the SWF prevents backdating while the behavioral signals prevent behavioral spoofing.

By entangling content hashes with behavioral and physical constraints, this protocol enables an Attester to generate an Evidence Packet (.cpoe) that imposes quantifiable cost on forgery of authorship claims, preserving privacy by design without disclosing document content. The formal threat model, including the adversarial Attester assumption that distinguishes CPoE from standard RATS deployments, is defined in {{threat-model}}.

This document is a companion to {{CPoE-Appraisal}}, which specifies the Verifier's forensic appraisal logic, the Attestation Result (WAR) wire format, and the quantitative security model. Implementers of Verifier components require both documents.

# Requirements Language {#requirements-language}

{::boilerplate bcp14-tagged}

# Problem Statement {#problem-statement}

Digital documents lack creation-process provenance. Existing cryptographic mechanisms prove complementary properties -- COSE signatures {{RFC9052}} prove key possession, trusted timestamps {{RFC3161}} prove temporal existence, media provenance standards {{C2PA}} track post-creation custody -- but none reveals how a document was produced or evolved during authorship.

Non-cryptographic alternatives each carry fundamental limitations. Surveillance-based methods (screen recording, keystroke logging) are privacy-invasive, require trust in a third-party archive, and cannot be independently verified. Detection-based methods (stylometric classifiers, AI-content detectors) operate on finished output, are probabilistic, degrade as generative models improve, and exhibit systematic bias against non-native speakers {{Liang2023}}. Proprietary solutions (e.g., {{GPTZero}}, {{AG-Human-Authored}}, {{Amazon-KDP-AI}}) lack openness, independent verifiability, or both.

The need spans sectors: clinical documentation {{HHS-AI-RFI}}, AI agent attribution {{NIST-AI-Agents-RFI}}, and publishing workflows all require a standardized, open mechanism to verify how content was produced. Such a mechanism must satisfy four properties:

1. *Privacy-preserving:* Cryptographic hashes, not raw content, form the evidentiary basis.
2. *Independently verifiable:* The cryptographic chain must be deterministically checkable without access to external archives or proprietary systems. Behavioral analysis produces statistical assessments; independent Verifiers may reach different confidence levels from the same data.
3. *Tamper-evident:* Evidence must form a cryptographic chain resistant to retroactive modification, reordering, or selective omission.
4. *Process-documenting:* The mechanism must capture how a document evolved over time, not merely what it contains at a point in time.

# Use Cases {#use-cases}

The provenance gap identified in {{problem-statement}} has
concrete consequences across multiple domains: creative
industries (WGA/SAG-AFTRA contractual AI disclosure),
academic integrity (false AI detection accusations affecting
non-native speakers and disabled students), journalism
(editorial AI oversight policies lacking technical
enforcement), legal proceedings (evidentiary standards for
digital authorship under FRE Rules 901/902), regulatory
compliance (EU AI Act Article 50), and content platforms
(graduated trust based on process evidence rather than binary
classification). Detailed use case analysis is provided in
{{CPoE-UseCases}}.

# Scope and Applicability {#scope-and-applicability}

CPoE is designed as a voluntary, author-initiated mechanism.
The following normative constraints define its intended scope
of use and apply to all conforming implementations:

* Evidence collection MUST be initiated by the author.
  The Attester operates under the author's control; no
  external party may activate evidence collection without
  the author's explicit consent.
* Implementations MUST NOT be deployed as proctoring
  or surveillance mechanisms. CPoE does not capture screen
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
  {{CPoE-Appraisal}}).
* Implementations MUST support the assistive-mode
  feature flag (feature ID 60). When assistive-mode is
  declared in the profile-declaration, Verifiers MUST
  adjust behavioral thresholds to accommodate assistive
  input methods (switch access, eye-tracking, voice
  dictation with manual correction, screen readers with
  keyboard navigation). Users with motor disabilities,
  neurodivergent typing patterns, or non-standard input
  devices may produce behavioral signals outside the
  population norms cited in this specification (e.g.,
  Hurst exponent outside 0.55-0.85, atypical IKI
  distributions). The assistive-mode flag signals that
  the Verifier SHOULD rely primarily on temporal binding
  (SWF) and content evolution analysis rather than
  motor-signal forensics for these sessions.

CPoE Evidence Packets attest to a single author's behavioral
signals. Collaborative authoring (multiple authors editing
the same document simultaneously) is not addressed by this
specification. In collaborative environments, each author
SHOULD generate a separate Evidence Packet covering their
individual contributions. Mechanisms for partitioning
authorship attribution in collaborative sessions (e.g.,
per-author checkpoint streams, contribution-level Evidence)
are deferred to future work.

Whether any institution or platform requires CPoE evidence
as a condition of participation is a policy decision outside
the scope of this protocol. CPoE provides a technical
mechanism for producing verifiable evidence; it does not
mandate that evidence be produced or consumed. Institutions
deploying CPoE SHOULD ensure that requiring Evidence does
not create barriers for users who rely on assistive
technologies or non-standard input methods.

# System Model {#system-model}

This section defines the CPoE system model in terms of the RATS architecture {{RFC9334}} and identifies where CPoE diverges from standard remote attestation assumptions.

## RATS Entity Roles {#rats-entity-roles}

CPoE maps to RATS entity roles {{RFC9334}} as follows. Verifier and Relying Party follow standard RATS definitions; the remaining roles have CPoE-specific characteristics:

Attester:
: The authoring application and its host platform. Unlike traditional RATS deployments, the Attester is operated by the entity whose claims are being verified (the author).

Attesting Environment (AE):
: The authoring application, OS entropy interfaces, and hardware Secure Elements (TPM/SE) when available.

Endorser:
: Hardware manufacturers that issue TPM endorsement certificates and platform attestation credentials for T3/T4 tiers.

Reference Value Provider:
: The CPoE specification itself (behavioral patterns, SWF parameters) and calibration services that provide updated forensic baselines.

Target Environment:
: The document editor session and its content state. The Target Environment is measured by the Attesting Environment at each checkpoint via content hashing and behavioral telemetry capture.

## Compatibility with RATS Architecture {#rats-compatibility}

CPoE extends RATS from device state attestation to process attestation, verifying that a physical process (human authorship) occurred as claimed. The fundamental problem structure is identical: an Attester generates Evidence, conveys it to a Verifier, and the Verifier produces Attestation Results for Relying Parties.

CPoE implements a critical trust inversion: in traditional remote attestation, the adversary is external (malware, network attackers). In CPoE, the Attester is operated by the author, and the primary adversary is the Attester operator themselves. This inversion shapes the security model:

* Evidence must be unforgeable by the entity generating it
* Temporal claims must be bound to physical constraints the Attester cannot circumvent
* Behavioral entropy must be computationally expensive to simulate
* Hardware attestation provides value only when the hardware root of trust is genuinely inaccessible to the Attester operator

The RATS architecture accommodates this through its layered trust model and configurable Appraisal Policies ({{RFC9334}}). The companion appraisal document ({{CPoE-Appraisal}}) defines domain-specific verification procedures. The rationale for Experimental status is provided in {{experimental-status-rationale}}.

## Adversarial Attester as RATS Profile {#adversarial-attester-profile}

CPoE constitutes a RATS profile in which the standard Attester-operator trust assumption is explicitly relaxed. RFC 9334 Section 3 defines Attester roles functionally (the entity producing Evidence) without requiring the operator to be trusted. CPoE leverages this by treating the Attester operator as the primary adversary and relying on physics-based constraints, memory-hard functions, and hardware trust anchors (at T3/T4) rather than Attester honesty.

This profile is analogous to anti-cheat and DRM attestation patterns where the platform operator is adversarial, and differs from traditional RATS deployments (firmware integrity, supply chain verification) where external compromise is the primary threat. Implementations and Verifier policies MUST account for this trust model when interpreting CPoE Evidence.

## Evidence Format and EAT Relationship {#evidence-eat-relationship}

CPoE Evidence Packets use a domain-specific CBOR structure (tag 1129336645) rather than CWT/JWT-wrapped EAT tokens. This design reflects the fundamental difference between CPoE Evidence and traditional EAT claims:

* EAT ({{RFC9711}}) models entity state as a set of claims at a point in time. CPoE Evidence models a continuous physical process as an ordered sequence of checkpoints with SWF proofs, behavioral entropy, and physical state bindings.
* The checkpoint chain structure (sequential process-proofs, hash-linked deltas, jitter bindings) has no natural representation in the EAT claims model.
* Wrapping each checkpoint as a separate EAT would lose the cryptographic chain integrity that is central to CPoE's security properties.

The EAT profile URI "urn:ietf:params:rats:eat:profile:cpoe:1.0" applies to the Attestation Result (WAR) format defined in {{CPoE-Appraisal}}, which carries EAT-compatible claims (verdict, attestation tier, forensic assessments). The Evidence Packet uses its own profile URI "urn:ietf:params:cpoe:profile:1.0" to identify the CPoE Evidence format. Generic EAT tooling cannot parse CPoE Evidence without CPoE-specific support, but can consume WAR Attestation Results via the EAR compatibility mapping defined in {{CPoE-Appraisal}}.

## Reference Value Trust Model {#reference-value-trust}

The Reference Value Provider role in CPoE differs from traditional RATS deployments where RVPs supply known-good platform measurements from a trusted supply chain. In CPoE, Reference Values take two forms:

Specification-defined values:
: SWF parameters, forensic thresholds, and profile requirements defined in this document and {{CPoE-Appraisal}}. These are static and trusted by virtue of the specification itself.

Author-generated behavioral baselines:
: Per-author behavioral profiles (baseline-digest) accumulated across sessions within the Attesting Environment. These are self-asserted by the Attester and MUST NOT be treated as trusted Reference Values. The baseline-digest is signed by the Evidence Packet's signing key and bound to the author identity via identity-fingerprint, but at T1/T2 tiers the signing key is software-controlled and the baseline could be synthetically constructed (see {{attack-taxonomy}}).

Verifiers MUST treat behavioral baselines as corroborative evidence whose weight scales with the confidence-tier (population-reference through mature), not as independent attestation of authenticity. The forensic mechanisms (SNR, CLC, error topology) operate independently of baseline data and provide the primary discrimination between biological and synthetic authoring.

# Protocol Overview {#protocol-overview}

This section provides an end-to-end overview of the CPoE protocol, mapping the message flow to the RATS passport model and illustrating the lifecycle of an Evidence Packet from creation through appraisal.

## Passport Model Message Flow {#passport-model}

CPoE follows the RATS passport model ({{RFC9334}}, Section 8.1; {{RATS-Models}}):

~~~ ascii-art
+----------+  .cpoe   +-----------+  .cwar   +-----------+
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
{: #fig-passport-model title="CPoE Passport Model Message Flow"}

The CPoE-specific message flow is:

1. The Attester (authoring application running in the Attesting Environment) collects behavioral telemetry during content creation and generates an Evidence Packet (.cpoe) containing SWF proofs, jitter bindings, and physical state markers.
2. The Evidence Packet is conveyed to a Verifier, which appraises chain integrity, temporal ordering, behavioral entropy, and content binding per the procedures defined in {{CPoE-Appraisal}}.
3. The Verifier produces a Cryptographic Written Authorship Report (.cwar) containing EAT claims, forensic assessment scores, and forgery cost estimates.
4. The Relying Party (publisher, reader, or automated platform) consumes the WAR to make trust decisions about the claimed authorship provenance.

Endorsers (hardware manufacturers) supply TPM endorsement certificates and Secure Element attestations that Verifiers use to validate hardware-bound claims in T3/T4 Evidence. Reference Value Providers supply the expected behavioral patterns, SWF difficulty parameters, and profile specifications that Verifiers use as appraisal baselines.

Note: In some deployments, the Relying Party (e.g., a content management system or publishing platform) may receive an Evidence Packet alongside content and forward it to a Verifier on behalf of the author. This is operationally similar to the RATS background-check model ({{RFC9334}}, Section 8.2), where the Relying Party initiates verification. However, the Evidence Packet is still generated by the Attester and travels with the content as a self-contained credential, making this architecturally consistent with the passport model. Deployments using this pattern SHOULD ensure that the Relying Party does not modify the Evidence Packet before forwarding it to the Verifier.

## Evidence Lifecycle {#evidence-lifecycle}

The following sequence illustrates the end-to-end lifecycle of a CPoE attestation session:

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
   |                | emit .cpoe file     |                  |
   |                |                     |                  |
   |                | .cpoe (Evidence)    |                  |
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

Attesters SHOULD use checkpoint intervals between 10 and 120 seconds (default 30 seconds). Verifiers MUST accept Evidence with any positive checkpoint interval but MAY flag intervals outside this range in warnings. When behavioral entropy capture is active (ENHANCED or MAXIMUM content tier), each checkpoint SHOULD contain jitter-binding data from at least 20 inter-keystroke intervals. Checkpoints with fewer than 20 intervals provide insufficient data for behavioral analysis; the Attester SHOULD extend the checkpoint interval rather than emit a low-entropy checkpoint. Verifiers MUST accept checkpoints with any number of intervals (including zero for paste-only intervals) but SHOULD flag checkpoints with fewer than 20 intervals as providing no behavioral assurance. Each checkpoint interval produces one link in the hash chain. The SWF computation runs continuously during the interval, binding the author's behavioral entropy and the platform's physical state to the elapsed wall-clock time. At session end or rollover boundary, the Attester seals the chain into a .cpoe Evidence Packet. For long-running authoring projects, the Session Continuation mechanism ({{session-continuation}}) allows a series of Evidence Packets to be cryptographically linked.

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

The Attester MAY include a baseline-verification structure (evidence-packet key 19) that enables Verifiers to assess behavioral consistency across writing sessions by the same author. A baseline-verification contains a session-behavioral-summary (IKI histogram, coefficient of variation, Hurst exponent, cognitive pause frequency, duration, keystroke count) and optionally a baseline-digest summarizing the author's historical profile with a COSE_Sign1 signature. When baseline-verification is present, the identity-fingerprint within the baseline MUST match the Evidence Packet's identity-fingerprint. Verifiers MUST reject Evidence where these values differ.

The identity-fingerprint MUST be computed as SHA-256(public-key) where public-key is the raw bytes of the author's signing key. The baseline-digest aggregates per-session statistics using Welford's online algorithm (streaming-stats). The session-merkle-root field contains a Merkle Mountain Range root over incorporated session hashes; Attesters without MMR tracking MUST set it to 32 zero bytes, and Verifiers MUST NOT elevate confidence-tier based on a zero value.

The confidence-tier indicates baseline maturity: population-reference (1, 0-4 sessions), emerging (2, 5-9), established (3, 10-19), mature (4, 20+). During initial enrollment (no prior sessions), the Attester MUST omit baseline-digest and digest-signature. When baseline-digest is present, digest-signature MUST also be present; Verifiers MUST verify the COSE_Sign1 signature and that identity-fingerprint equals SHA-256 of the signing key.

# Threat Model {#threat-model}

This section defines the adversary model following the methodology of {{RFC3552}} and incorporating insights from RATS security analysis {{Sardar-RATS}}. The threat model assumes a Dolev-Yao style adversary {{Dolev-Yao}} with domain-specific constraints.

The following table maps CPoE threat categories to the RATS threat analysis in {{Sardar-RATS}} and {{RFC9334}} Section 12:

| RATS Threat Category | CPoE Coverage | CPoE Mechanism |
|---|---|---|
| Compromised Attester | Primary threat ({{adversarial-attester}}) | SWF time-binding, behavioral entropy, hardware isolation (T3/T4) |
| Spoofed Attestation Results | WAR signature verification | COSE_Sign1 over WAR, Verifier identity binding |
| Evidence replay | {{attack-taxonomy}} | Physical freshness anchors, entangled VDF mode |
| Man-in-the-middle | {{attack-taxonomy}} | TLS channel binding (EKM), SEAT platform attestation |
| Compromised Verifier | Out of scope (trusted Verifier assumption) | Relying Party MAY cross-verify with multiple Verifiers |
| Supply chain attack on AE | {{attack-taxonomy}} | T3/T4 hardware attestation; T1/T2 accept this risk |

## Adversarial Attester Model {#adversarial-attester}

The PRIMARY threat in CPoE is an adversarial Attester -- an author who controls the Attesting Environment and seeks to generate Evidence for content they did not authentically author. This inverts the standard RATS trust assumption where the Attester is trusted to report honestly.

The adversarial Attester has the following capabilities:

* *Full software control:* Can modify, instrument, or replace any software component including the authoring application and operating system
* *Timing manipulation:* Can adjust system clocks, virtualize execution environments, and attempt to compress or expand apparent time
* *Entropy injection:* Can inject synthetic behavioral data (keystroke timing, jitter sequences) from pre-recorded or generated sources
* *Content pre-generation:* Can generate document content using AI tools or other assistance before initiating the attestation session
* *Parallel execution:* Can run multiple attestation sessions simultaneously or use distributed resources

The adversary is constrained by:

* *Physics:* Cannot violate thermodynamic laws or accelerate hardware beyond physical limits
* *Memory bandwidth:* SWF computations are bounded by available memory bandwidth
* *Hardware isolation:* In T3/T4 tiers, cannot extract keys from Secure Elements without physical tampering
* *Economic rationality:* Will not expend resources exceeding the value of successful forgery

## Security Goals {#security-goals}

CPoE provides the following authentication properties, defined in terms of adversary advantage:

Temporal Authenticity:
: Given Evidence claiming authorship duration D, an adversary cannot produce valid Evidence in time significantly less than D. Formally: Adv_temporal = Pr\[Verify(E) = accept AND Time(Generate(E)) < D - epsilon\] is negligible for meaningful epsilon.

Behavioral Authenticity:
: Given Evidence containing behavioral entropy B, an adversary cannot efficiently generate synthetic entropy that is indistinguishable from biological origin. The cost of generating synthetic behavioral data satisfying all forensic constraints must exceed a defined threshold.

Content Binding:
: Evidence E is cryptographically bound to document D such that E cannot be repurposed to attest a different document D'. This property is unconditional given collision resistance of H.

Non-repudiation (T3/T4):
: In hardware-bound tiers, Evidence is signed with keys that the Attester cannot extract or duplicate, providing non-repudiation of the attestation act.

## Attack Taxonomy {#attack-taxonomy}

The following attacks are in scope for CPoE defenses. Cross-references to individual attacks (e.g., {{attack-taxonomy}}) refer to rows in this table and the corresponding defense analysis in {{security-considerations}}.

{: #retype-attack}
{: #replay-attack}
{: #behavioral-transplant}
{: #relay-attack}
{: #swf-acceleration}
{: #ae-spoofing}
{: #baseline-forgery}
{: #late-initialization}
{: #diversion-attack}

| Attack | Description | Primary Defense | Tier |
|---|---|---|---|
| Retype | Adversary generates content externally, then retypes it while collecting "authentic" telemetry | Cognitive load correlation, error topology analysis, semantic-temporal SWF binding ({{sec-retype-defense}}) | All |
| Replay | Reusing previously valid Evidence for new claims | Physical Freshness anchors; entangled mode prevents parallel pre-computation | T2+ |
| Behavioral Transplant | Extracting behavioral data from legitimate Evidence and inserting into forged Evidence | Jitter-tag and entangled-binding HMACs keyed from SWF output ({{jitter-tag-computation}}, {{entangled-binding}}); SWF seed incorporates jitter intervals | All |
| Relay | Forwarding challenges or Evidence between a legitimate author and adversary session | Hardware-bound signing (T3/T4), out-of-band presence challenges | T3/T4 |
| SWF Acceleration | Specialized hardware computes SWF proofs faster than consumer hardware | Argon2id memory-hardness; HAT in T3/T4 tiers | All |
| AE Spoofing | Virtualized or modified Attesting Environment presented as genuine | Hardware attestation (T3/T4); T1/T2 Evidence weighted accordingly | T3/T4 |
| Baseline Forgery | Synthetic baseline manufactured via enrollment with forged behavioral patterns | Baseline is non-verdict; cross-metric consistency required; hardware key binding (T3/T4). Verifiers MUST NOT treat baseline consistency as independent evidence of authenticity. | All |
| Late Initialization | Content generated externally, then CPoE capture initiated on completed document | Retype defenses, witness-anchored nonce binding ({{CPoE-Anchoring}}), beacon binding. T1 without external anchoring provides no start-time assurance. | T2+ |
| Diversion | Evidence redirected to unintended Verifier or Relying Party | TLS EKM channel binding ({{RFC9266}}) via "EXPORTER-CPoE-channel-binding" label in evidence-packet key 11. SEAT {{SEAT-EXPAT}} MAY combine platform and process attestation at T3/T4. | All |

## Out-of-Scope Threats {#out-of-scope-threats}

The following threats are explicitly out of scope:

* *Nation-state HSM compromise:* Adversaries capable of extracting keys from certified HSMs via invasive physical attacks
* *Physics-level laboratory spoofing:* Adversaries capable of simulating thermal trajectories and entropy sources at sub-microsecond precision
* *Quantum computation:* Attacks requiring large-scale quantum computers (hash function collision finding, Argon2id inversion)

# Core Principles and Claims {#core-principles}

Building on the threat model defined above, CPoE operates on five primary constraints, ordered by their role in the security model:

* Behavioral Authentication (Primary): Human motor-signal entropy -- inter-keystroke timing, cognitive load correlation, error correction patterns, and editing trajectories -- provides the primary evidence of human cognitive involvement. These behavioral signals exhibit characteristic statistical properties (1/f spectral noise, long-range dependence, content-correlated pauses) that are computationally expensive to simulate across all dimensions simultaneously.
* Time-Locked Binding (Complementary): Memory-hard Sequential Work Functions (SWF) establish a temporal floor on evidence generation. The SWF does not prove authorship; it prevents an adversary from fabricating behavioral evidence retroactively by forcing real wall-clock time between checkpoints. This is a time-lock, not a proof of work.
* Physical Freshness: Replay and simulation attacks are defeated by anchoring sessions to non-reproducible physical markers (kernel entropy deltas, platform-specific observables). Every session incorporates non-deterministic physical freshness sampled within the AE.
* Out-of-Band Presence: Utilizing secondary physical devices (e.g., smartphone QR scans) to bridge the digital-physical gap and ensure a human is in the loop.
* Asymmetric Verification: The sequential work function allows proofs to be verified probabilistically via Merkle-sampled audit proofs, ensuring scalability and DoS resistance.

# Protocol Rationale and Terminology {#rationale-and-terminology}

The Cryptographic Proof of Effort (CPoE) framework follows the RATS architecture while introducing domain-specific extensions for physical process attestation.

CPoE Evidence Packet (.cpoe):
: An Attester artifact containing SWF Merkle trees, jitter seals, and physical liveness markers (CBOR tag 1129336645, encoding ASCII "CPOE").

WAR Result (.cwar):
: A Verifier Attestation Result containing signed EAT claims and forensic assessments (CBOR tag 1129791826). The WAR format is specified in {{CPoE-Appraisal}}.

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
: The Attester generates a final checkpoint, signs the transcript root with the Attester's signing key (hardware-bound for T3/T4; software-managed for T1/T2), and prepares the transport container (.cpoe). On completion, the AE transitions to IDLE. If finalization fails, the Attester MUST retry or emit a partial Evidence Packet with a "finalization-incomplete" limitation.

# Evidence Content Tiers {#evidence-tiers}

CPoE Evidence Packets are classified by the depth of behavioral and forensic data collected:

CORE (Tier Value 1):
: Checkpoint chain with SWF proofs, hash-based content binding, and physical freshness anchors. Proves temporal ordering and content integrity.

ENHANCED (Tier Value 2):
: All CORE components plus behavioral entropy capture (Jitter Seals), intra-checkpoint correlation, and edit-graph-hash commitment ({{edit-graph-hash}}). Adds evidence of interactive authoring behavior.

MAXIMUM (Tier Value 3):
: All ENHANCED components plus entangled MACs binding physical state, error topology analysis, edit graph histograms ({{edit-graph-histograms}}), and forgery cost bounds. Provides the strongest available evidence.

CPoE Evidence is classified along two orthogonal axes. Evidence Content
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

| CPoE Tier | NIST AAL | EAT Security Level (RFC 9711) |
|---|---|---|
| T1: Software-Only | AAL1 | unrestricted (1) |
| T2: Attested Software | AAL2 | restricted (2) |
| T3: Hardware-Bound | AAL3 | hardware (4) |
| T4: Hardware-Hardened | LoA4 | hardware (4) |

T3 and T4 both map to EAT security level "hardware" (4) because the EAT
specification does not distinguish PUF-level binding from standard
TPM key binding. EAT security level "secure-restricted" (3) is not used
because CPoE tiers define hardware binding in terms of key isolation and
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

The CPoE specification defines three implementation profiles that establish Mandatory-to-Implement (MTI) requirements for interoperability.

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

The profile-uri field in an Evidence Packet MUST be set to "urn:ietf:params:cpoe:profile:1.0" for Evidence conforming to this specification. This URI identifies the CPoE Evidence format and is distinct from the EAT profile URI "urn:ietf:params:rats:eat:profile:cpoe:1.0", which identifies CPoE Attestation Results (see {{evidence-eat-relationship}}).

In the document-ref structure, byte-length is the length in bytes of the UTF-8 encoded document, and char-count is the number of Unicode scalar values (code points).

If the Evidence Packet omits the attestation-tier field, the Verifier
MUST derive the tier from evidence content: T1 if no hardware
attestation is present, T2 if platform attestation hooks are
detected, T3 if TPM key binding is verified, T4 if anti-tamper
evidence and PUF binding are confirmed. The attestation-tier field is
informational; Verifiers MUST NOT rely on it as authoritative
when contradicted by the evidence content.

# Evidence Format and CDDL {#wire-format}

Evidence Packets are CBOR-encoded {{RFC8949}} and identified by semantic tag 1129336645. The CDDL notation {{RFC8610}} is used to define the wire format.

~~~ cddl
; CBOR tag wrappers
cpoe-evidence = #6.1129336645(evidence-packet)
cpoe-war = #6.1129791826(attestation-result)

; Stub: full attestation-result definition in [CPoE-Appraisal]
attestation-result = {* int => any}

; Primary structures
evidence-packet = {
    1 => uint,                    ; version (must be 1)
    2 => tstr,                    ; profile-uri
    3 => uuid,                    ; packet-id
    4 => cpoe-timestamp,           ; created
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
    ? 20 => witness-service,      ; witness service declaration (T2+)
    * int => any,                  ; extension fields
}

checkpoint = {
    1 => uint,                    ; sequence (monotonic)
    2 => uuid,                    ; checkpoint-id
    3 => cpoe-timestamp,           ; timestamp (local)
    4 => hash-value,              ; content-hash
    5 => uint,                    ; char-count
    6 => edit-delta,              ; delta
    7 => hash-value,              ; prev-hash
    8 => hash-value,              ; checkpoint-hash
    9 => process-proof,           ; SWF proof
    ? 10 => jitter-binding,         ; behavioral-entropy (ENHANCED+)
    ? 11 => physical-state,         ; physical-state binding (ENHANCED+)
    ? 12 => hash-digest,             ; entangled-binding (HMAC output; ENHANCED+)
    ? 13 => [+ receipt],             ; receipts (self or tool)
    ? 14 => [+ active-probe],        ; active liveness probes
    ? 15 => hat-proof,               ; HAT temporal proof (T3/T4)
    ? 16 => beacon-anchor,           ; public randomness anchor (optional)
    ? 17 => bstr .size 32,           ; verifier-nonce (interactive mode)
    ? 18 => witness-anchor,          ; witness-anchored nonce (T2+)
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

Verifiers MUST reject checkpoints where the claimed-duration value is
zero or exceeds twice the elapsed wall-clock time between the current
checkpoint timestamp and the previous checkpoint timestamp. When
hardware attestation (HAT) is available, Verifiers SHOULD cross-check
claimed-duration against the HAT-attested interval.

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
    ? 4 => [+ inertial-sample],   ; accelerometer/vibration (ENHANCED+)
}

inertial-sample = [
    cpoe-timestamp,                ; sample time (milliseconds)
    int,                          ; x-axis (micro-g)
    int,                          ; y-axis (micro-g)
    int,                          ; z-axis (micro-g)
]

physical-liveness = {
    1 => [+ thermal-sample],      ; thermal trajectory
    2 => bstr .size 32,           ; entropy-anchor
}

thermal-sample = [
    cpoe-timestamp,                ; sample time
    int,                          ; temperature delta (millidegrees)
]

presence-challenge = {
    1 => bstr .size (16..256),    ; challenge-nonce (128+ bits)
    2 => bstr .cbor COSE_Sign1,    ; device-signature
    3 => cpoe-timestamp,           ; response-time
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
    4 => cpoe-timestamp,           ; transfer-time
}

tool-receipt = {
    1 => tstr,                    ; tool-id (provider URI)
    2 => hash-value,              ; output-commit
    ? 3 => hash-value,            ; input-ref (prompt hash)
    4 => cpoe-timestamp,           ; issued-at
    5 => bstr .cbor COSE_Sign1,   ; tool-signature
    ? 6 => uint,                  ; output-char-count
}

active-probe = {
    1 => probe-type,              ; challenge category
    2 => cpoe-timestamp,           ; stimulus-time
    3 => cpoe-timestamp,           ; response-time
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
    11 => cpoe-timestamp,          ; computed-at
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
cpoe-timestamp = uint              ; epoch milliseconds (no tag 1; see protocol §6.7)
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

The attestation-result type used in the cpoe-war tag wrapper is
defined as a stub above for CDDL completeness; the full
definition appears in {{CPoE-Appraisal}}.

To ensure cross-architecture determinism, all temporal and entropy measurements MUST be encoded as unsigned integers (`uint`). Timestamps and durations are expressed in milliseconds. Entropy estimates are expressed in centibits (1/100th of a bit).

cpoe-timestamp is a bare unsigned integer representing milliseconds since
the Unix epoch (1970-01-01T00:00:00Z). CBOR tag 1 is not used because
RFC 8949 Section 3.4.2 defines it as epoch seconds; CPoE requires
millisecond precision for IKI measurements and jitter-binding
timestamps. cpoe-timestamp values MUST be positive (greater than zero).
A cpoe-timestamp value of zero is invalid. Verifiers MUST reject
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
originates from another CPoE-enabled authoring environment. The
tool-id identifies the source environment, output-commit binds
the pasted content, and evidence-ref references the source
Evidence Packet. Self-receipt verification is specified in
{{CPoE-Appraisal}}.

A `tool-receipt` records content contributed by an external tool
(e.g., a large language model) that is not itself a CPoE Attester.
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
    "CPoE-EditGraph-v1" ||
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

Edit-delta keys 9-11 carry 8-bin histograms summarizing the editing trajectory. These fields MUST be present in MAXIMUM content tier and MUST be absent in CORE. They MAY be present in ENHANCED. The edit graph is represented as a set of histograms over operation types (cursor trajectory, revision depth) and inter-event timing (pause duration). Each histogram is an array of exactly 8 unsigned integers representing event counts per bin. The histogram bin boundaries and computation procedure are specified in {{CPoE-Appraisal}}.

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
{{CPoE-Appraisal}}.

Per-checkpoint physical-state (checkpoint key 11) captures instantaneous
thermal and entropy measurements. Packet-level physical-liveness
(evidence-packet key 18) provides a session-wide thermal trajectory
for replay detection. physical-liveness SHOULD be included in ENHANCED
and MAXIMUM profiles. When both are present, Verifiers MUST verify
that per-checkpoint thermal values are consistent with the session-wide
trajectory.

When available, the Attester MAY include inertial-sample data
(physical-state key 4) containing tri-axis accelerometer readings
sampled during the checkpoint interval. Each sample records the
timestamp (milliseconds) and acceleration in micro-g (1e-6 * 9.81
m/s^2) on three orthogonal axes. Inertial data captures mechanical
shockwaves from physical keystrokes, enabling cross-modal
verification: the Verifier can assess whether the digital IKI
signal is consistent with physical keystroke impulses by computing
cross-spectral coherence (see {{CPoE-Appraisal}}). In T1/T2 tiers,
inertial data is software-reported and MUST NOT be treated as
unforgeable; its value is limited to increasing the dimensionality
of data an adversary must fabricate consistently. In T3/T4 tiers
where a hardware-isolated sensor provides the readings, inertial
coherence provides meaningful anti-injection protection. Inertial
samples MUST be normalized to relative deltas from a session
baseline to prevent device fingerprinting.

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
{{CPoE-Appraisal}}.

## Checkpoint Hash Computation {#checkpoint-hash-computation}

The checkpoint-hash field MUST be computed as follows:

~~~ pseudocode
checkpoint-hash = H(
    "CPoE-Checkpoint-v1" ||
    prev-hash ||
    content-hash ||
    CBOR-encode(edit-delta) ||
    CBOR-encode(jitter-binding) ||
    CBOR-encode(physical-state) ||
    process-proof.merkle-root
)
~~~

Where H is the Evidence Packet's selected hash function, \|\| denotes concatenation, and CBOR-encode produces deterministic CBOR per Section 4.2.1 of {{RFC8949}}. The UTF-8 Domain Separation Tag (DST) prefix "CPoE-Checkpoint-v1" MUST be prepended as the first input to prevent cross-context hash collisions. When edit-graph-hash (edit-delta key 5) is present, it is already included within the CBOR-encoded edit-delta and requires no separate term in the checkpoint-hash computation.

For the first checkpoint in the initial Evidence Packet of a series (or a standalone packet), prev-hash MUST be set to H(CBOR-encode(document-ref)). This anchors the chain to the document identity. For the first checkpoint in a continuation packet (previous-packet-ref present), prev-hash MUST be set to the final checkpoint-hash of the preceding Evidence Packet (see {{session-continuation}}).

When jitter-binding and physical-state fields are absent (CORE profile), the checkpoint-hash MUST be computed without those terms: checkpoint-hash = H("CPoE-Checkpoint-v1" \|\| prev-hash \|\| content-hash \|\| CBOR-encode(edit-delta) \|\| process-proof.merkle-root).

## Checkpoint Computation Order {#checkpoint-computation-order}

The fields within a checkpoint MUST be computed in the following order:

1. When edit-graph-hash is present: compute the edit-graph-hash from the accumulated cursor-positions, revision-depths, and pause-durations arrays per {{edit-graph-hash}}. When histograms are present (MAXIMUM tier): populate the 8-bin arrays per {{edit-graph-histograms}}. Assemble the edit-delta structure including these fields.
2. Compute the SWF: run the proof-algorithm with the derived seed (which incorporates the edit-graph-hash when present). For modes 20/21, evaluate Argon2id iteratively for the configured number of steps. For mode 10, evaluate Argon2id once then iterate SHA-256 with memory-hard waypoints at every W-th step. Construct the Merkle tree over all intermediate states using tagged hashing ({{merkle-tree-construction}}) to obtain the merkle-root (process-proof key 4).
3. Derive the shared PRK and per-field keys via the two-stage HKDF hierarchy ({{key-derivation-hierarchy}}).
4. Compute the jitter-tag using the tag-key and jitter-binding.intervals as HMAC input. Assemble the jitter-binding structure (intervals, entropy-estimate, jitter-tag).
5. Compute the entangled-binding using the binding-key and prev-hash, content-hash, jitter-binding, and physical-state as HMAC input.
6. Compute the checkpoint-hash over the DST "CPoE-Checkpoint-v1", prev-hash, content-hash, edit-delta, jitter-binding, physical-state, and merkle-root.

This ordering ensures that each subsequent computation can reference the outputs of prior steps. Implementations MUST follow this order to produce interoperable checkpoints.

## Evidence Protection {#evidence-protection}

For T3 and T4 Attestation Tiers, Evidence Packets MUST be wrapped
in a COSE_Sign1 envelope {{RFC9052}}. For T1 and T2 tiers, COSE_Sign1
wrapping is RECOMMENDED. The COSE_Sign1 payload is the complete
CBOR-encoded evidence-packet including tag 1129336645; ES256 or
EdDSA is RECOMMENDED for the algorithm identifier.

For T3/T4 tiers, the signing key MUST be bound to a hardware
Secure Element (TPM or platform SE). For T1/T2 tiers, a
software-managed key is acceptable.

When COSE_Sign1 wrapping is not used (e.g., offline file-based
conveyance), the Evidence Packet's integrity relies solely on
the internal hash chain. Relying Parties MUST evaluate the
trust implications of unwrapped Evidence.

For online conveyance, COSE_Sign1-wrapped Evidence Packets can be encapsulated within a Conceptual Message Wrapper (CMW) for transport via the SEAT cmw_attestation TLS extension {{SEAT-EXPAT}}. This enables CPoE Evidence to be delivered alongside platform attestation evidence in a single post-handshake authentication exchange, which is the preferred attestation timing model {{SEAT-Timing}}. The SEAT use cases {{SEAT-UseCases}} identify runtime attestation and operation-triggered re-attestation as key requirements, both of which CPoE's continuous checkpoint model satisfies.

## ASCII-Armored Encoding {#ascii-armor}

Evidence Packets and Attestation Results MAY be encoded in an ASCII-armored representation for text-based transport or archival. The format uses `-----BEGIN CPoE EVIDENCE-----` / `-----END CPoE EVIDENCE-----` boundaries for Evidence Packets (.cpoe) and `-----BEGIN CPoE WAR-----` / `-----END CPoE WAR-----` for Attestation Results (.cwar).

The payload is Base64-encoded ({{RFC4648}}, Section 4) with lines not exceeding 76 characters. The payload MUST be the complete CBOR-encoded structure including the CBOR semantic tag (and COSE_Sign1 envelope when present). No key-value headers are permitted between the boundary line and the Base64 block. Implementations MUST accept both armored and raw CBOR encodings.

# Sequential Work Function {#swf-construction}

CPoE uses a Sequential Work Function (SWF) as defined in
{{PoSME}} to bind Evidence to sustained computational effort.
The SWF primitive provides three modes: `swf-argon2id` (20),
`swf-argon2id-entangled` (21), and `swf-sha256` (10). The
generic SWF construction, Merkle-sampled verification protocol,
Fiat-Shamir sample derivation, Merkle tree construction,
security bound, and seed grinding resistance proofs are
specified normatively in {{PoSME}}. This section specifies only
the CPoE-specific SWF integration: seed derivation from
checkpoint data, mandatory parameters, entangled binding, jitter
tags, and temporal anchoring mechanisms.

## SWF Construction Summary {#swf-algorithm}

The SWF construction, verification protocol, Fiat-Shamir sample
derivation, and Merkle tree construction are defined normatively
in {{PoSME}}. CPoE implementations MUST conform to the PoSME
specification for all generic SWF operations. The CPoE-specific
domain separation tag prefix is "CPoE-" (e.g., "CPoE-salt-v1",
"CPoE-Fiat-Shamir-v1"). PoSME defines the algorithm; CPoE
defines the application-specific seed derivation and binding
computations below.

## Verification {#swf-verification}

Verification procedures for all three modes (10, 20, 21) are
specified in {{PoSME}}. CPoE Verifiers MUST additionally verify
the CPoE-specific seed derivation ({{swf-seed-derivation}}) and
entangled binding ({{entangled-binding-computation}}) after
completing the generic SWF verification.

## Fiat-Shamir and Merkle Construction {#fiat-shamir-sampling}

Fiat-Shamir sample derivation and Merkle tree construction
follow {{PoSME}} with the "CPoE-Fiat-Shamir-v1" domain
separation tag. See {{PoSME}} for the normative specification.

## SWF Seed Derivation {#swf-seed-derivation}

To prevent cross-mode seed collisions, the seed derivation
MUST include the proof-algorithm identifier as the first
term after the version prefix, providing domain separation
between modes. The identifier is encoded as a single byte
via I2OSP(proof-algorithm, 1).

For `swf-argon2id` (Mode 20), the SWF seed for each
checkpoint MUST be derived as:

~~~ pseudocode
seed = H(
    "CPoE-SWF-Seed-v1" ||
    I2OSP(20, 1) ||
    prev-hash ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state) ||
    edit-graph-hash
)
~~~

When edit-graph-hash is absent, the seed MUST be computed
without the final term. Mode 10 (`swf-sha256`) uses the
same derivation with I2OSP(10, 1). Mode 21
(`swf-argon2id-entangled`) additionally includes
prev-swf-output (the final Argon2id state from the
immediately preceding checkpoint) after prev-hash, creating
a strict cryptographic dependency chain: each checkpoint's
SWF cannot begin until the previous checkpoint's SWF has
completed.

When a verifier-nonce is present ({{CPoE-Anchoring}}),
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
    "CPoE-SWF-Seed-v1" ||
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
seed = H("CPoE-SWF-Seed-v1" \|\| prev-hash \|\| local-nonce). For the first
checkpoint, the nonce provides non-determinism when
initial-jitter-sample is unavailable. Implementations MUST
NOT use a fully deterministic seed derivation.

## Merkle Tree Construction {#merkle-tree-construction}

The SWF Merkle tree construction (domain-separated tagged
hashing per {{RFC6962}} with 0x00/0x01/0x02 prefix tags) is
specified in {{PoSME}}. The Merkle root is stored in
process-proof.merkle-root (key 4).

## Mandatory SWF Parameters {#mandatory-swf-params}

Conforming Attesters SHOULD use the following minimum SWF parameters
for each Evidence Content Tier. The underlying SWF algorithm is
defined in {{PoSME}}; these values are CPoE-specific calibration
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

Attesters claiming ENHANCED or MAXIMUM content tier with `swf-argon2id` MUST use `swf-argon2id-entangled` (21) instead of `swf-argon2id` (20). This ensures that each checkpoint's SWF computation depends on the previous checkpoint's final state, creating a strict inter-checkpoint sequential dependency that eliminates parallel pre-computation. Verifiers MUST reject ENHANCED or MAXIMUM Evidence that uses proof-algorithm 20 instead of 21, assigning verdict invalid (4).

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
    salt = "CPoE-key-derivation-v1",
    IKM  = process-proof.merkle-root || process-proof.input
)
binding-key = HKDF-Expand(PRK, "CPoE-entangled-binding-v1", hash_len)
tag-key     = HKDF-Expand(PRK, "CPoE-jitter-tag-v1", hash_len)
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

## Temporal Anchoring Mechanisms {#temporal-anchoring}

CPoE defines three temporal anchoring mechanisms that bind
checkpoints to external time sources, forming a hierarchy of
increasing assurance:

| Mechanism | Key | Online? | Freshness | Specified In |
|-----------|-----|---------|-----------|--------------|
| Beacon-anchor | 16 | No (passive) | Minutes | {{CPoE-Anchoring}} |
| Witness-anchor | 18 | Yes (1 RTT) | Seconds | {{CPoE-Anchoring}} |
| Verifier-nonce | 17 | Yes (2 RTT) | Immediate | {{CPoE-Anchoring}} |

An Evidence Packet MAY include any combination. When multiple
anchoring mechanisms are present, the Verifier SHOULD use the
strongest available: verifier-nonce > witness-anchor >
beacon-anchor. Attesters claiming T2 or higher assurance SHOULD
include a witness-anchor in every checkpoint. The Evidence Packet
MUST NOT claim a tier higher than T1 if any checkpoint lacks a
valid witness-anchor.

The full specification of beacon-anchored binding,
witness-anchored nonce binding (including the witness service
protocol, nonce expiry, duplicate checks, transparency log, and
CDDL definitions), and verifier-nonce binding is provided in
{{CPoE-Anchoring}}.

## Security Bound {#swf-security}

The SWF security bound and seed grinding resistance proof are
specified in {{PoSME}}. In summary: an adversary who skips
fraction f of steps is detected with probability 1-(1-f)^k where
k is the number of sampled proofs. Seed grinding is strictly
anti-profitable for all skip fractions and sample counts (see
{{PoSME}} for the formal proof). These bounds apply independently
to each CPoE checkpoint.

## Hardware-Anchored Time (HAT) {#hat}

Hardware-Anchored Time (HAT) binds SWF computations to TPM-attested clock readings at T3/T4 tiers. HAT is specified in {{RATS-HAT}}. Attesters claiming T3/T4 MUST include HAT proofs in checkpoint key 15.

# Experimental Status Rationale {#experimental-status-rationale}

This document and its companion {{CPoE-Appraisal}} are published with Experimental status for the following reasons:

1. *Novel RATS trust model:* The adversarial Attester model inverts standard RATS trust assumptions and requires deployment validation.
2. *Physical process attestation:* Extending RATS from device state to continuous physical process attestation is a new application area requiring cross-platform validation.
3. *Calibration parameters:* SWF step counts, checkpoint intervals, and forensic thresholds are initial values subject to revision as memory bandwidth evolves (HBM4, PIM architectures). Verifiers MUST reject Evidence with version greater than 1 unless they implement the corresponding protocol version.
4. *Ecosystem dependencies:* Tool Receipt key discovery, EAT profile, and SEAT integration reference specifications still in development.
5. *Forensic thresholds:* MUST-level forensic thresholds in {{CPoE-Appraisal}} may be relaxed to SHOULD based on false positive analysis across diverse author populations.

Implementers are encouraged to report deployment experience, particularly regarding SWF performance, false positive rates, and adversarial evasion attempts.

# IANA Considerations {#iana-considerations}

This document requests the following IANA registrations:

## CBOR Tags {#iana-cbor-tags}

This document requests registration of two CBOR tags in the
"CBOR Tags" registry per RFC 8949, Section 9.2:

Tag 1129336645:

Tag:
: 1129336645

Data Item:
: map

Semantics:
: CPoE Evidence Packet (see {{wire-format}} of this document)

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
: CPoE Attestation Result (see {{CPoE-Appraisal}})

Point of Contact:
: David Condrey (david@writerslogic.com)

Description of Semantics:
: \[this document\], {{CPoE-Appraisal}}

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
: CPoE Attestation Result Profile

Profile URI:
: urn:ietf:params:rats:eat:profile:cpoe:1.0

Description:
: Profile for Cryptographic Proof of Effort (CPoE) Attestation
  Results (WAR format) as defined in {{CPoE-Appraisal}}. The
  CPoE Evidence Packet format uses a domain-specific CBOR
  structure (tag 1129336645) that is not an EAT token; this
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

| Field | application/cpoe+cbor | application/cwar+cbor |
|---|---|---|
| Subtype name | cpoe+cbor | cwar+cbor |
| Required parameters | none | none |
| Optional parameters | none | none |
| Encoding | binary (CBOR, {{RFC8949}}) | binary (CBOR, {{RFC8949}}) |
| Security considerations | {{security-considerations}}. Contains cryptographic hashes and timing data, no document content. Receivers MUST validate CBOR structure. May contain privacy-sensitive behavioral data. | {{CPoE-Appraisal}}. Contains forensic assessment outcomes and forgery cost estimates. Receivers MUST validate CBOR structure and COSE signatures. |
| Interoperability | {{wire-format}}. CBOR maps tagged 1129336645. | {{CPoE-Appraisal}}. CBOR maps tagged 1129791826. |
| Published specification | \[this document\] | \[this document\], {{CPoE-Appraisal}} |
| Applications | Authorship attestation, content provenance, digital forensics, publishing workflows | Authorship verification, content trust platforms, relying party applications |
| Fragment identifiers | N/A | N/A |
| File extension(s) | .cpoe | .cwar |
| Magic number(s) | N/A (CBOR tag 1129336645) | N/A (CBOR tag 1129791826) |
| Intended usage | COMMON | COMMON |
| Restrictions | none | none |
| Contact | David Condrey (david@writerslogic.com) | David Condrey (david@writerslogic.com) |
| Change controller | David Condrey (david@writerslogic.com) | David Condrey (david@writerslogic.com) |

## TLS Exporter Label {#iana-tls-exporter}

This document requests registration of the following TLS exporter
label in the "TLS Exporter Labels" registry defined in {{RFC5705}}:

Value:
: EXPORTER-CPoE-channel-binding

DTLS-OK:
: Y

Recommended:
: N

Reference:
: \[this document\]

## Future Registry Considerations {#iana-future-registries}

This specification and its companion {{CPoE-Appraisal}} define
several enumerated code points in their CDDL schemas.
Code points defined in this document: proof-algorithm,
attestation-tier, content-tier, hash-algorithm, hash-salt-mode,
binding-type, probe-type, and confidence-tier. Code points
defined in {{CPoE-Appraisal}}: verdict, absence-type, and
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

This section provides security analysis following {{RFC3552}} guidelines. The threat model is defined in {{threat-model}} with the adversarial Attester as the primary threat actor. Detailed forensic security analysis is provided in {{CPoE-Appraisal}}.

The CPoE adversary model assumes an attacker with: (a) polynomial-time computation, (b) access to the same SWF parameters as honest Attesters, (c) ability to generate arbitrary behavioral telemetry, but (d) inability to violate the hardness assumptions of the selected hash function or memory-hard function. The attacker's goal is to produce Evidence that passes Verifier appraisal without genuine human cognitive involvement in content creation.

## Security Layer Model {#sec-layer-model}

CPoE security guarantees operate at three distinct layers with different assurance properties. Verifiers MUST NOT treat lower layers as providing the formal guarantees of higher layers.

Layer 1, Behavioral Authentication (Primary):
: Behavioral signals -- inter-keystroke timing entropy (SNR, spectral slope), cognitive load correlation (CLC), error correction topology, editing trajectory analysis, and session consistency -- provide the primary evidence of human cognitive involvement. These mechanisms detect fabricated behavioral data by exploiting the computational difficulty of simultaneously satisfying multiple independent statistical constraints that characterize biological motor control. Behavioral authentication is the core value proposition of CPoE. Thresholds are grounded in the keystroke dynamics literature {{Monrose2000}}{{Monaco2018}}{{Salthouse1986}}{{Dhakal2018}} and calibrated against real-world composition data {{ScholaWrite}}{{ScholaWriteAugmented}}. Single-feature timing approaches are demonstrably insecure {{Condrey2026Attack}}; the multi-dimensional cross-domain entanglement architecture is a direct response to those attack classes.

Layer 2, Temporal Binding (Cryptographic):
: The SWF forces minimum real wall-clock cost derivable from Argon2id memory-hardness {{RFC9106}} and the sequential dependency chain. This is a time-lock that prevents retroactive fabrication of evidence, not a proof of work. With entangled mode at 50% duty cycle, forging N hours of Evidence requires approximately N/2 hours of computation at minimum. This layer complements Layer 1 by ensuring that an adversary cannot generate behavioral evidence after-the-fact.

Layer 3, Hardware Attestation (Physical):
: T3/T4 non-repudiation via hardware roots of trust provides physical freshness markers non-reproducible by the Attester operator.

The forgery cost estimates in the WAR incorporate all three layers. Layer 1 mechanisms inform the forensic-summary and forensic-flag fields. Layer 2 provides the C_swf temporal cost bound. Layer 3 provides the C_hardware cost bound when hardware attestation is present.

## Primary Threat: Adversarial Attester {#sec-primary-threat}

Unlike traditional remote attestation where external adversaries threaten system integrity, CPoE's primary threat is the Attester operator themselves. The author controls the Attesting Environment and has incentive to claim authenticity for AI-generated or assisted content.

This threat model inversion has fundamental implications:

* Software-only attestation (T1) provides minimal assurance since the Attester controls all software
* Cryptographic proofs must be bound to physical constraints the Attester cannot circumvent
* Behavioral entropy must be economically expensive to forge, not merely cryptographically secure
* Trust in Evidence scales with the Attestation Tier and the cost of bypassing its guarantees

At T1/T2, the security guarantee is: (a) the SWF imposes a quantifiable wall-clock floor on evidence generation (Layer 2), and (b) an adversary must simultaneously fabricate plausible values across multiple independent behavioral dimensions (IKI distribution, coefficient of variation, Hurst exponent, cognitive load correlation, error correction topology, editing trajectory) that jointly satisfy Verifier forensic analysis. The multi-dimensional fabrication burden is empirically grounded in the keystroke dynamics literature but has not been formally bounded against adaptive adversaries. Single-feature timing forgery is demonstrably feasible {{Condrey2026Attack}}; the protocol's defense relies on the joint consistency requirement across independent feature classes, which raises the practical cost of forgery but does not provide a cryptographic guarantee. Relying Parties SHOULD calibrate trust in T1/T2 Evidence accordingly.

## Retype Attack Defenses {#sec-retype-defense}

The retype attack (see {{attack-taxonomy}}) is the canonical forgery vector. Defenses are layered:

Cognitive Load Correlation (CLC):
: Verifiers analyze correlation between content complexity and typing cadence as specified in {{CPoE-Appraisal}}.

Error Topology Analysis:
: Authentic authoring produces characteristic error patterns: corrections localized near recent insertions, deletion-to-insertion ratios consistent with human cognitive models {{Salthouse1986}}{{ScholaWrite}}{{ScholaWriteAugmented}}, and long-range temporal dependence in revision patterns consistent with 1/f noise in cognitive performance {{Orden2003}}. Retyping produces either unnaturally low error rates or randomly distributed artificial errors.

Temporal Cost:
: Even successful retype attacks require real-time effort. A 5,000-word document with 10-second checkpoint intervals requires 8+ hours of continuous typing effort to forge. The attack does not scale economically for high-volume forgery.

Relying Parties need to be aware that retype attacks remain viable for short documents or high-value targets willing to invest real time. CPoE provides graduated assurance proportional to document length and checkpoint density.

## Relay and Replay Attack Defenses {#sec-relay-replay}

As defined in {{attack-taxonomy}} and {{attack-taxonomy}}, these attacks are defeated through Physical Freshness anchors binding Evidence to non-reproducible physical state:

* Thermal trajectories captured during SWF computation cannot be replayed
* Kernel entropy pool deltas are bound to specific execution moments
* Out-of-band presence challenges (QR scans) verify real-time physical proximity

Verifiers MUST reject Evidence where physical freshness markers are stale, inconsistent with timestamps, or exhibit patterns suggesting simulation.

## SWF Acceleration Defenses {#sec-swf-speedup}

As analyzed in {{attack-taxonomy}}, specialized hardware attacks are mitigated by:

* *Memory-hardness:* For modes 20/21, every SWF step is a full Argon2id evaluation bounded by memory bandwidth (approximately 50 GB/s for DDR5 {{JESD79-5}}), not ALU throughput. ASICs provide minimal advantage per step, and this resistance compounds across all steps in the chain. For mode 10, memory-hard waypoints ({{swf-algorithm}}) bound the ASIC advantage at waypoint steps to the Argon2id limit.
* *Hardware-Anchored Time (T3/T4):* The HAT temporal sandwich protocol ({{hat}}) brackets each SWF computation with TPM-attested clock readings, preventing time compression even with faster computation. The Verifier checks that the TPM clock delta meets or exceeds the expected SWF duration.
* *Merkle sampling:* Skipping SWF steps is detected probabilistically. With k=100 samples, skipping 5% of steps has >99.4% detection probability. Seed grinding does not improve the adversary's position (see {{PoSME}}).

## Trust Gradation by Tier {#sec-tier-trust}

Relying Parties SHOULD interpret Evidence according to its Attestation Tier. Detailed per-tier verification constraints are defined in {{CPoE-Appraisal}}.

| Tier | Forgery Resistance | Suitability |
|------|-------------------|-------------|
| T1 | Temporal ordering only; behavioral claims forgeable | Low-stakes or supplementary |
| T2 | Moderate; platform attestation hooks | Casual forgery deterrence |
| T3 | Strong; requires physical SE access | Most applications |
| T4 | Invasive hardware attack required | High-stakes legal/financial |

## Forgery Cost Bounds {#sec-economic-bounds}

Implementations SHOULD report quantified forgery cost estimates in Attestation Results. For CORE profile with `swf-argon2id` (90 steps, m=65536 KiB), each checkpoint requires ~9 seconds of sustained memory-hard computation on consumer hardware (DDR4, ~25 GB/s), a ~30% duty cycle within the 30-second checkpoint interval. Entangled mode creates strict inter-checkpoint sequential dependencies: forging N hours of ENHANCED authorship requires approximately N/2 hours of real computation on consumer hardware.

Applying a conservative ASIC advantage factor of 10x, a committed adversary reduces this to approximately N/20 hours. For a 5-hour document, the adversary cost is approximately 15 minutes of ASIC computation plus capital cost of hardware. The forgery cost scales linearly with session duration; parallel machines can produce independent forgeries but cannot accelerate a single session's sequential chain. Relying Parties SHOULD evaluate forgery cost against the adversary's economic context, not the honest Attester's computation time.

Detailed ASIC advantage analysis (TMTO bounds, memory bandwidth scaling, silicon optimization factors) is provided in {{CPoE-Appraisal}}. Verifiers SHOULD use an ASIC advantage factor of 10x when computing c-swf in forgery-cost-estimate.

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
* When beacon-anchoring ({{CPoE-Anchoring}}) is used, the binding keys incorporate externally-verifiable randomness, providing temporal anchoring even without hardware attestation

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

## Selective Checkpoint Omission {#sec-selective-omission}

The hash chain prevents modification of included checkpoints but
does not prevent omission of trailing checkpoints. An adversary
can truncate the chain at any point and claim the authoring
session ended there. Without external anchoring, the Verifier
cannot determine whether additional checkpoints were generated
and discarded. Witness-anchored nonce binding
({{CPoE-Anchoring}}) with transparency logging provides
detection of omission: the witness log records the checkpoint
count per session, enabling Verifiers to detect gaps. Deployments
where selective omission is a concern SHOULD require witness
anchoring with transparency logging. Beacon anchoring
({{CPoE-Anchoring}}) provides weaker omission detection: the
Verifier can verify that the final checkpoint's beacon round is
consistent with the claimed session end time, but cannot detect
omitted intermediate checkpoints.

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
* Enforce a maximum Evidence Packet size of 16 MiB after CBOR decoding
* Enforce a maximum CBOR nesting depth of 32 levels during parsing
* Reject Evidence Packets containing more than 10,000 checkpoints

T3/T4 implementations MUST additionally:

* Store signing keys exclusively in hardware Secure Elements
* Bind SWF seeds to TPM monotonic counters
* Verify platform integrity before Evidence generation

## Internationalization and Input Method Considerations {#sec-i18n}

The behavioral model described in this specification is calibrated
primarily against alphabetic keyboard input (Latin script, QWERTY
layout). Input Method Editors (IMEs) for CJK languages (Chinese
Pinyin/Wubi, Japanese Kana/Romaji, Korean Hangul) produce
fundamentally different keystroke patterns: phonetic key sequences
followed by candidate selection, with composition events that may
not generate standard key-down/key-up pairs. Similarly,
right-to-left scripts (Arabic, Hebrew) produce bidirectional cursor
movement patterns that differ from left-to-right editing
trajectories.

Implementations MUST capture IME composition events as behavioral
input alongside raw keystrokes. Verifiers MUST NOT apply
alphabetic-typing forensic thresholds (Hurst exponent ranges, IKI
distribution expectations) to Evidence generated with IME-based
input without appropriate recalibration. The jitter-binding
structure captures inter-event intervals regardless of input
method; the forensic interpretation of those intervals is
input-method-dependent and is specified in {{CPoE-Appraisal}}.

Future revisions of this specification may define input-method
profile identifiers to enable Verifiers to select appropriate
forensic baselines automatically.

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

CPoE Evidence Packets do not contain document content. Content binding uses cryptographic hashes (H, as selected by hash-algorithm) which are computationally irreversible. The author-salted mode (hash-salt-mode=1) provides additional protection by preventing rainbow-table correlation across documents.

However, hash-based content binding provides confidentiality only when the document space is effectively unbounded. In constrained domains where the set of candidate documents is small (e.g., standardized legal forms, academic paper templates, published preprints), an observer with access to the candidate set can compute hashes and match against content-hash values in checkpoints, revealing document identity and editing trajectory without accessing the document itself. Authors working in constrained document spaces MUST use author-salted mode to prevent this correlation. Even with salting, the checkpoint count and temporal pattern of an Evidence Packet may reveal document length and composition timeline; authors should consider this metadata exposure when deciding whether to share Evidence.

## Behavioral Fingerprinting {#priv-fingerprinting}

Jitter sequences in ENHANCED and MAXIMUM profiles constitute behavioral biometrics. Keystroke dynamics are classified as biometric data under multiple legal frameworks including GDPR Article 4(14), CCPA/CPRA, and Illinois BIPA. Deployments that collect, store, or transmit jitter-binding data MUST comply with applicable biometric data regulations, which may require explicit informed consent, purpose limitation, data retention limits, and right-to-deletion mechanisms. To protect author privacy, Verifiers are expected to:

* Discard jitter data after the verification session completes
* Avoid correlating jitter across multiple Evidence Packets to prevent author deanonymization
* Use jitter data solely for authenticity verification

Attesters MUST apply a minimum quantization step of 5 milliseconds to all inter-keystroke interval values before encoding in jitter-binding. Finer resolution MUST NOT be used without explicit operator configuration and documented consent.

## Physical State Leakage {#priv-physical-leakage}

Thermal trajectories and kernel entropy deltas in the physical-state field may reveal information about the Attester's hardware configuration. Implementations SHOULD normalize thermal data to relative deltas rather than absolute values to prevent device fingerprinting.

Inertial samples, when present, carry additional fingerprinting risk: accelerometer frequency response and noise floor characteristics can identify specific device models. Implementations MUST normalize inertial data to relative deltas from a per-session baseline and SHOULD apply the same quantization and privacy budget constraints as jitter data.

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

The following test vectors validate SWF implementations using the
CPoE domain separation tags. All vectors use SHA-256 (hash-algorithm
value 1).

## swf-sha256 (Mode 10) Test Vector {#test-vector-mode10}
{:numbered="false"}

~~~ test-vectors
Seed: "cpoe-genesis-v1"
Seed (hex): 63706f652d67656e657369732d7631
Salt: H(0x00 || "CPoE-salt-v1" || seed)

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

Salt (hex): 20d10e03c713b5be87fbecccab13465e
             513eac5835e52ac0ac9c8d50dab9fba8

Intermediate States:
  state_0 (Argon2id):
    f4a9461757a2ab266e7572ffbfc662b9
    c3afd5d6b2233d163f0d28add6ed529f
  state_1000 (waypoint, Argon2id):
    2c926557fd907959bcd7a970a42b837c
    3738cf6f104bf862741c38cbe5fd3924
  state_5000 (waypoint, Argon2id):
    35e8e8fb91f7fbe1a4078f42074dc1ea
    a5b3892749170b0892787bbef5f4e6f0
  state_9999 (SHA-256):
    de7e5e1928f5bc4db0f36eb407b67772
    2b4000337ef6c197e91a211220ea58c5
  state_10000 (waypoint, Argon2id, final):
    a207cf20421f2a231503d811352f1b45
    fa75f7819b627f71ae0e7e626f64a51a
~~~

## swf-argon2id (Mode 20) Test Vector {#test-vector-mode20}
{:numbered="false"}

~~~ test-vectors
Seed: "cpoe-genesis-v1"
Seed (hex): 63706f652d67656e657369732d7631

Argon2id Parameters (per step):
  Time Cost (t): 1
  Memory Cost (m): 65536 KiB
  Parallelism (p): 1
  Output Length: 32 bytes

Steps: 3

Intermediate States:
  state_0 (Argon2id, seed as password,
           salt=H(0x00 || "CPoE-salt-v1" || seed)):
    f4a9461757a2ab266e7572ffbfc662b9
    c3afd5d6b2233d163f0d28add6ed529f
  state_1 (Argon2id, state_0 as password,
           salt=H(0x01 || "CPoE-salt-v1" || I2OSP(1, 4))):
    c16d4c36d8bec173d03b302740dccb5e
    c221d90d5cfbab4ac852851270a7839f
  state_2 (Argon2id, state_1 as password,
           salt=H(0x01 || "CPoE-salt-v1" || I2OSP(2, 4))):
    6a5e0491d3d27a1880a2896732739cc6
    c279262bb56bd74d20125320bde7ab70
  state_3 (Argon2id, state_2 as password,
           salt=H(0x01 || "CPoE-salt-v1" || I2OSP(3, 4))):
    458670264b4dd3be8598749ad33567d2
    4a4e50eddc2f6b2751ae1f17713a31b1
~~~

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The author thanks the participants of the RATS working group for
their ongoing work on remote attestation architecture and security
considerations that informed this specification.
