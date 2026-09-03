# Name: Cryptographic Proof of Effort for Content Creation (CPOE)

## Description

Digital signatures establish control of a signing key. Trusted timestamps
establish that data existed no later than a point in time. Content provenance
systems record assertions about the origin and handling of an artifact. None of
these mechanisms provides an interoperable account of the process by which
digital content was created: how an artifact evolved, which contributions were
entered by a person, which were inserted or transformed by a tool, and which
claims about that process can be independently appraised.

This gap matters where a relying party needs evidence about a creation process
rather than an inference from finished content. Candidate use cases include
voluntary disclosure of AI-assisted contributions, editorial and publishing
workflows, academic submissions, and contractual attribution in creative work.
Today, implementations use vendor-specific event records, scoring methods, and
trust assumptions. Those artifacts are not portable between authoring tools,
verifiers, and relying parties, and their privacy and security properties are
difficult to compare.

The proposed BoF will determine whether the IETF should undertake work on a
vendor-neutral architecture and interoperable formats for content-process
evidence and appraisal results. It will examine:

* a common problem statement, terminology, and set of use cases;
* the threat model for evidence produced on a device controlled by the claimant;
* the minimum semantics needed for independently verifiable event chains,
  external contributions, temporal or witness anchors, and appraisal results;
* data minimization, consent, accessibility, unlinkability, retention, and
  protection of behavioral data;
* resource-use and device-autonomy requirements, including whether useful
  assurance can be provided without mandatory continuous computation or loss of
  user control;
* the relationship to RATS, EAT, EAR, CBOR, COSE, trusted timestamping, and
  content-provenance systems; and
* evidence of interest from implementers, prospective relying parties,
  researchers, and reviewers with relevant security, privacy, human-computer
  interaction, and accessibility expertise.

The BoF is not being asked to endorse the current CPoE proposal as a complete
solution. In particular, it will not assume that behavioral telemetry proves
identity or cognition, that a binary human-versus-AI judgment is possible, or
that a particular sequential-work construction, hardware trust anchor,
behavioral model, or scoring policy should be mandatory.

The BoF will be successful if it produces clear answers to the following
questions:

1. Is there a well-scoped interoperability problem that is appropriate for the
   IETF?
2. Are the security, privacy, accessibility, energy-use, and device-autonomy
   constraints understood well enough to define useful work?
3. Are multiple independent participants willing to author, review, implement,
   and deploy the resulting specifications?
4. Should the work proceed in an existing WG, in a new WG, through further
   non-WG discussion or research, or not in the IETF?

## Required Details

* Status: not WG forming
* Responsible AD: Christopher Inacio (requested; confirmation pending)
* BOF proponents: David Condrey <david@writerslogic.com>, WritersLogic Inc.
* BOF chairs: Scott Perry, Digital Governance Institute (proposed; confirmation
  pending); Kathleen Moriarty <Kathleen.Moriarty.ietf@gmail.com>, Center for
  Internet Security (proposed; confirmation pending)
* Number of people expected to attend: 75
* Length of session (1 or usually 2 hours): 2 hours
* Conflicts (whole Areas and/or WGs): RATS, COSE, SPICE, AIPREF, VCON,
  SECDISPATCH, DISPATCH, and SAAG
* Chair Conflicts: RATS (Kathleen Moriarty)
* Technology Overlap: RATS, EAT, EAR, CBOR, COSE, content provenance, creator
  assertions, trusted timestamping, transparency logs, and Verifiable
  Credentials
* Key Participant Conflict: RATS, COSE, SPICE, AIPREF, VCON, and SAAG

## Information for IAB/IESG

To allow evaluation of this proposal, the following summarizes existing work
and the anticipated standards gap:

* **Any protocols or practices that already exist in this space:**
  * The RATS architecture (RFC 9334) defines Attesters, Verifiers, Relying
    Parties, Evidence, and Attestation Results. EAT (RFC 9711) defines a token
    format for attestation claims, and the RATS WG is developing EAR for
    attestation results. The present RATS charter is centered on evidence about
    system components. It does not directly define a content-creation event
    model or determine which claims can safely be made when the claimant
    controls the Attesting Environment.
  * CBOR (RFC 8949), CDDL (RFC 8610), and COSE (RFC 9052) provide encoding,
    schema, and cryptographic building blocks, but do not define process-evidence
    semantics.
  * Trusted timestamping (RFC 3161), transparency logs, and epoch markers can
    establish existence or freshness relative to an external service. They do
    not describe how content evolved between anchors.
  * C2PA Content Credentials and related creator-identity work provide artifact
    provenance and signed assertions. A process-evidence format could be carried
    by or referenced from those systems, but is not currently standardized by
    them. The Content Authenticity Initiative maintains open-source Rust,
    JavaScript, Python, C/C++, Node.js, and mobile SDKs that could provide the
    manifest, signing, embedding, and validation layer for such an integration.
  * SLSA and in-toto define practices and attestation formats for software-build
    provenance. Their producer/consumer model, signed predicates, and explicit
    assurance levels are useful prior art, but they do not describe human
    content-creation events or appraisal of those events.
  * The Berkeley Protocol on Digital Open Source Investigations defines minimum
    professional practices for identifying, collecting, preserving, verifying,
    and analyzing digital open-source information. It is not an Internet wire
    protocol or a creation-evidence format, but it is relevant relying-party
    practice for preservation, chain of custody, verification, and ethical use
    of digital evidence.
  * Authoring platforms maintain revision histories, audit logs, and tool
    interaction records. These are generally application-specific and do not
    share portable evidence or appraisal semantics.
  * Commercial and experimental systems perform post-hoc AI-text detection or
    collect behavioral composition data. These systems do not share an
    interoperable evidence format, security model, or privacy profile.
* **Which modifications to existing protocols or practices are required:** The
  BoF should determine whether process attestation can be expressed as a RATS
  profile or requires an explicit augmentation to the RATS architecture.
  Possible work includes process-evidence claims or media types, mappings to EAT
  and EAR, and conventions for carrying or referencing evidence in existing
  content-provenance systems. CBOR, CDDL, and COSE are expected to be reused
  unless concrete interoperability gaps are identified.
* **Which entirely new protocols or practices are required:** Candidate work,
  subject to BoF consensus, includes a solution-independent information model
  for content-creation events and contribution provenance; a portable,
  cryptographically verifiable evidence-chain format; a contribution-receipt
  format for tool-generated or transformed content; optional external freshness,
  timestamp, or witness profiles; and an appraisal-result format that states the
  trust boundary, validation performed, policy used, and limitations. The work
  should not standardize a universal human-authorship score, a normative
  behavioral classifier, institutional enforcement policy, or a requirement
  that users surrender control of their devices.
* **Open source projects implementing this work:**
  * The
    [LF Decentralized Trust Proof of Effort Lab](https://github.com/LF-Decentralized-Trust-labs/proof-of-effort)
    contains editor's copies of the CPoE specifications, CDDL schemas,
    diagnostic examples, integration documents, and a minimal implementation
    of the sequential-memory component. It is not yet a complete,
    independently interoperable implementation.
  * [WritersProof](https://writerslogic.com/about/) is a browser and desktop
    implementation that captures writing-process events and can export `.cpoe`
    evidence packages; a standalone web verifier is available at
    <https://verify.writersproof.com/>. WritersLogic states that the underlying
    protocol and reference implementation are Apache-2.0 licensed. WritersProof
    is controlled by the same organization as the BoF proponent, and a direct
    public source-code URL for the application should be supplied before the
    BoF request is submitted.
  * Independently developed open-source projects demonstrate overlapping
    requirements without claiming CPoE compatibility.
    [KeyWitness](https://github.com/magicseth/keywitness), distributed as the
    typed.by iOS keyboard, captures per-keystroke timing, touch geometry, and
    force and seals the typed text as an Ed25519-signed W3C Verifiable
    Credential using `eddsa-jcs-2022` and `did:key`; its README declares MIT,
    though no license file is present. It has no trusted timestamp or
    transparency log, which is one of the layers CPoE specifies. The
    [Tracked Writing File Format (TWFF)](https://github.com/Functional-Intelligence-Research-Lab/twff)
    includes an Apache-2.0 container specification, schemas, validators, and the
    Glass Box reference editor for deterministic writing-process logs.
    [Proof SDK](https://github.com/EveryInc/proof-sdk) is an MIT-licensed
    collaborative editor and provenance model that tracks human and agent
    contributions. [humanshipd](https://github.com/Flagrare/humanshipd) is an
    early MIT-licensed Rust proof of concept for local capture, signed process
    records, RFC 3161 anchoring, and offline verification.
    [Certified Human-Made](https://github.com/dabhunt/krita-certified-human-made)
    is a GPL-3.0 Krita plug-in that records strokes, layers, timing, and tool use
    and generates verifiable process certificates.
    [OpenFab](https://github.com/open-fab-ai/openfab) is an Apache-2.0 software
    provenance system that records AI/human attribution and signed in-toto/SLSA
    attestations.
  * Reusable open-source infrastructure includes the
    [CAI C2PA SDKs](https://opensource.contentauthenticity.org/docs/introduction/),
    the [SLSA framework and tooling](https://slsa.dev/), and Cisco's
    [Model Provenance Kit](https://github.com/cisco-ai-defense/model-provenance-kit).
    These implement complementary artifact, build, or model-lineage mechanisms,
    not CPoE process-evidence semantics.
  * No independent implementation of the CPoE wire formats has yet been
    confirmed. [Genotone](https://genotone.com/) describes an independently
    developed internal prototype for C2PA-based audio provenance and open
    verification, but no publicly licensed source implementation was identified
    as of 2026-09-01; it is therefore related implementation evidence, not an
    open-source CPoE implementation.

## Agenda

* 5 minutes: Administrivia, goals, and decision questions — proposed chairs and
  responsible AD (5/120)
* 15 minutes: Problem statement and concrete interoperability use cases — David
  Condrey; draft-condrey-cpoe-usecases (20/120)
* 15 minutes: Prior IETF discussion and proposed scope and non-goals — David
  Condrey and proposed chairs; draft-condrey-cpoe-protocol (35/120)
* 20 minutes: Adversarial analysis and technical feasibility — invited
  independent security reviewer; draft-condrey-cpoe-protocol and
  draft-condrey-cpoe-appraisal (55/120)
* 15 minutes: Privacy, accessibility, energy use, and device autonomy — invited
  privacy, accessibility, and human-computer interaction reviewers (70/120)
* 15 minutes: Existing standards, gaps, and possible work items — invited RATS,
  CAWG/C2PA, and W3C Verifiable Credentials participants (85/120)
* 20 minutes: Implementation, deployment, and research commitments — short
  statements from confirmed independent participants, followed by open
  discussion (105/120)
* 15 minutes: BoF questions, consensus assessment, and next steps — proposed
  chairs and responsible AD (120/120)

The chairs should publish the exact consensus questions before the session. The
agenda intentionally reserves substantial time for critical review and the BoF
decision rather than solution presentations.

## Links to the mailing list, draft charter if any (for WG-forming BoF), relevant Internet-Drafts, etc.

* Mailing List: Proposed `cpoe@ietf.org`; creation request will be made before
  submission. Prior discussion is archived on
  [SECDISPATCH](https://mailarchive.ietf.org/arch/browse/secdispatch/) and
  [RATS](https://mailarchive.ietf.org/arch/browse/rats/).
* Draft charter: N/A (not WG forming)
* Relevant Internet-Drafts:
  * Use Cases:
    * [Cryptographic Proof of Effort: Use Cases and Deployment Considerations](https://lf-decentralized-trust-labs.github.io/proof-of-effort/draft-condrey-cpoe-usecases.html)
  * Proposed Solutions:
    * [Architecture and Evidence Format — current editor's copy](https://lf-decentralized-trust-labs.github.io/proof-of-effort/draft-condrey-cpoe-protocol.html)
    * [Forensic Appraisal and Security Model — current editor's copy](https://lf-decentralized-trust-labs.github.io/proof-of-effort/draft-condrey-cpoe-appraisal.html)
    * [Temporal Anchoring Extensions — current editor's copy](https://lf-decentralized-trust-labs.github.io/proof-of-effort/draft-condrey-cpoe-anchoring.html)
  * Datatracker versions under the earlier Proof of Process name:
    * [Architecture and Evidence Format](https://datatracker.ietf.org/doc/draft-condrey-rats-pop-protocol/)
    * [Forensic Appraisal and Security Model](https://datatracker.ietf.org/doc/draft-condrey-rats-pop-appraisal/)
* Open-source repository and issue tracker:
  * <https://github.com/LF-Decentralized-Trust-labs/proof-of-effort>
  * <https://github.com/LF-Decentralized-Trust-labs/proof-of-effort/issues>
* Prior IETF consideration:
  * [IETF 125 joint DISPATCH/SECDISPATCH minutes](https://datatracker.ietf.org/meeting/125/materials/minutes-125-dispatch-202603160100-01)
  * [IETF 125 RATS minutes](https://datatracker.ietf.org/meeting/125/materials/minutes-125-rats-202603180100-00)
* IPR:
  * [Related IPR disclosure 7168](https://datatracker.ietf.org/ipr/7168/)
