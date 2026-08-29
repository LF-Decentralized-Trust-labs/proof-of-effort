---
v: 3
docname: draft-pereira-licet-wearable-attester-latest
title: "LICET Wearable Attester: Topology and Trust Hierarchy for Physiological Intent Corroboration"
abbrev: LICET Wearable Attester
category: exp
ipr: trust200902
submissiontype: independent
area: Security
workgroup: Individual Submission
keyword:
  - attestation
  - RATS
  - wearable
  - biometrics
  - human intent
  - HRV
  - physiological signals

date: 2026-07
stand_alone: yes
pi:
  toc: yes
  tocdepth: "4"
  sortrefs: yes
  symrefs: yes

author:
  - fullname: Christian Rodrigues Pereira
    initials: C.
    surname: Pereira
    organization: eColabs
    email: christian@licet.dev

normative:
  RFC9334:
  RFC9711:
  RFC8610:
  RFC8949:

informative:
  CPoE-Protocol:
    title: "Cryptographic Proof of Effort (CPoE): Architecture and Evidence Format"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-02
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-protocol-00
  RATS-Composite:
    title: "Composite Attesters and Verifiers"
    author:
      - fullname: Michael Richardson
    seriesinfo:
      Internet-Draft: draft-richardson-rats-composite-attesters
  LICET-Spec:
    title: "LICET: Layered Intent Corroboration via Embedded Trust"
    author:
      - fullname: Christian Rodrigues Pereira
        initials: C.
        surname: Pereira
    date: 2026-04
    seriesinfo:
      Internet-Draft: draft-pereira-licet-human-intent-01
  Laborde2022:
    title: "Psychophysiological effects of slow-paced breathing at six cycles per minute with or without heart rate variability biofeedback"
    author:
      - fullname: Sylvain Laborde
    date: 2022
    seriesinfo:
      Psychophysiology: DOI 10.1111/psyp.13952
  Moak2007:
    title: "Supine low-frequency power of heart rate variability reflects baroreflex function, not cardiac sympathetic innervation"
    author:
      - fullname: Jeffrey P. Moak
    date: 2007
    seriesinfo:
      Heart Rhythm: PMC2204059

--- abstract

This document defines the Attester topology and trust hierarchy for LICET
(Layered Intent Corroboration via Embedded Trust) wearable devices operating as
composite Attesters under the RATS architecture {{RFC9334}}. A LICET wearable
measures heart rate variability, electrodermal activity, and derived features,
and produces cryptographically bound Evidence about a subject's physiological
state at the time of an authorization request. This document establishes L0-L3
as a graduated trust hierarchy in which evidential weight follows attestation
level, states the hard bounds on what physiological corroboration can claim
before any architecture is presented, and defines the appraisal logic by which a
Verifier carries those bounds into Attestation Results. Evidence encoding is
deferred to a subsequent revision and is expected to use the CPoE
evidence-packet schema with LICET-specific claim extensions rather than defining
a new schema.

--- middle

# Introduction

This document defines the Attester topology and trust hierarchy for LICET wearable
devices operating as composite Attesters under IETF RFC 9334 {{RFC9334}}.

LICET (Layered Intent Corroboration via Embedded Trust) {{LICET-Spec}} is a protocol
for physiological corroboration of human intent. A LICET wearable device measures
biometric signals — primarily heart rate variability (HRV), electrodermal activity
(EDA), and derived features — and produces cryptographically bound Evidence about the
subject's physiological state at the time of an authorization request.

This document:

- Establishes L0–L3 as a graduated trust hierarchy in which evidential weight follows
  attestation level ({{trust-hierarchy}})
- Defines the composite Attester topology under RFC 9334 §3.3 ({{attester-topology}})
- States hard bounds on claims before any architecture is presented ({{limitations}})
- Describes appraisal logic for Evidence packets ({{appraisal}})
- Defers encoding to a later revision once the topology is stable ({{encoding}})

The encoding will use the CPoE evidence-packet schema (CBOR tag 1129336645)
{{CPoE-Protocol}} with LICET-specific claim extensions. No new schema is defined here.

## The Claim: Corroboration, Not Proof

**The claim throughout this document is corroboration, not proof.**

Physiological signals narrow the inference space; they do not establish intent with
certainty. The limitations in {{limitations}} bound this claim explicitly before any
architecture is presented. A Relying Party that treats LICET Evidence as proof of
intent, rather than as corroborating evidence, is misapplying this specification.

## Conventions and Definitions

{::boilerplate bcp14-tagged}

The following terms are used as defined in RFC 9334 {{RFC9334}}: Attester, Verifier,
Relying Party, Endorser, Evidence, Attestation Result, Claims, Reference Values.

Additional terms:

L0–L3:
: The LICET trust hierarchy levels defined in {{trust-hierarchy}}. Evidential weight
  follows attestation level.

Physiological calm fingerprint:
: The multivariate baseline of a subject's biometric signals (RMSSD, HF power, EDA,
  HR) under genuine resting conditions, established through a personalized baseline
  enrollment process.

Mahalanobis distance:
: The distance of a current biometric measurement from the subject's calm fingerprint,
  expressed in units of standard deviations accounting for feature covariance.


# Limitations {#limitations}

These limitations appear in this section, not in an appendix. A reviewer who reaches
the architecture without understanding the hard bounds will misread the system's
claims.

## Paced Breathing: Hard Bound on Mahalanobis Claims {#paced-breathing}

**This is the limitation that genuinely breaks the Mahalanobis design.**

A coerced subject trained in resonance-frequency breathing at approximately 0.1 Hz
(six cycles per minute) produces a physiologically authentic calm vector — elevated
RMSSD and HF power, low EDA, low HR — that is internally consistent and matches the
calm fingerprint. This is not a forgery of the baseline; it is the baseline,
volitionally manufactured {{Laborde2022}}.

The Mahalanobis design measures distance from the calm fingerprint. Paced breathing
moves the subject to that fingerprint. The distance collapses to zero without
producing any anomalous signal. The system cannot distinguish:

1. Genuine calm under no coercion
2. Volitional calm via paced breathing under coercion

**Consequence:** No claim derived from Mahalanobis distance alone survives this
scenario. The Mahalanobis layer provides corroborating evidence under adversarial
assumptions that exclude trained volitional vagal control. This assumption MUST be
stated wherever the layer's output is interpreted.

A respiratory periodicity index based on RSA coefficient of variation (RSA CV) can
detect trained paced breathing by its characteristic regularity signature. Paced
breathing at resonance frequency produces highly regular inter-breath intervals
(RSA CV in the order of 0.03), whereas genuine resting HRV exhibits substantially
higher inter-breath variability (RSA CV in the order of 0.40); these are illustrative
values derived from the slow-paced breathing literature {{Laborde2022}} and the
authors' implementation measurements, not normative thresholds. This narrows the
attack surface but does not close it. RSA CV is a discriminant, not a proof. The
`respiratory_periodicity_warning` flag in the Evidence packet is a detector output,
not a falsification of the Mahalanobis claim. Both MUST be surfaced; interpretation
is left to the Relying Party.

## T2 Self-Report Gap {#t2-gap}

The T2 layer in LICET relies on behavioral signals produced by the subject (typing
dynamics, gaze entropy, touchscreen interaction). These signals are produced by the
subject asserting intent and cannot be independently verified by the wearable
Attester.

**Consequence:** T2 Evidence does not carry evidential weight independent of a
concurrently valid T3 or T4 physiological layer. A Relying Party policy that grants
authorization on T2 alone accepts self-report as its primary evidence. This MUST be
explicit in Relying Party policy.


# The Claim Stated Precisely {#claim}

A LICET ZKP (zero-knowledge proof) in an authorization bundle proves:

- That a measurement was taken at a claimed time
- That the measurement's Mahalanobis distance from the subject's baseline falls
  within a claimed range
- That the cryptographic chain from sensor to proof is intact

The ZKP does NOT prove:

- That the physiological state reflects genuine uncoerced calm
- That the subject was not performing volitional vagal enhancement ({{paced-breathing}})
- That the T2 behavioral layer reflects actual behavioral state ({{t2-gap}})

**ZKP ≠ proof-of-measurement-of-intent.** The proof validates the measurement chain.
The inference from measurement to intent is a probabilistic claim bounded by
{{limitations}}. Every Attestation Result that includes a ZKP MUST include a
`zkp-scope` claim ({{zkp-scope}}) that states this explicitly.


# Attester Topology Under RFC 9334 {#attester-topology}

## Composite Device Model (RFC 9334 §3.3)

A LICET wearable operates as a Composite Attester as defined in RFC 9334 §3.3
{{RFC9334}}. The composite device consists of sub-attesters that each produce Claims.
The Composite Attester aggregates them into a single Evidence message. The trust
level of the composite is bounded by the weakest sub-attester in the chain.

| Sub-attester      | Environment                              | Produces                                      |
|-------------------|------------------------------------------|-----------------------------------------------|
| Sensor layer      | TEE or hardware secure element           | Raw biometric measurements                    |
| Processing layer  | Application environment                  | Derived features (RMSSD, HF power, Mahalanobis distance) |
| Crypto layer      | Secure enclave / ZKP prover              | Authorization bundle and proof                |
| Identity layer    | Attester credentials                     | Attestation cert chain (L2 and above)         |

## Roles (RFC 9334 §4.1)

| Role             | Entity                                   | Notes                                               |
|------------------|------------------------------------------|-----------------------------------------------------|
| Attester         | LICET wearable device                    | Produces Evidence about physiological state         |
| Verifier         | LICET Verifier service                   | Appraises Evidence against Endorsements             |
| Relying Party    | Authorization endpoint                   | Consumes Attestation Result; applies policy         |
| Endorser         | Device manufacturer / eColabs            | Provides device cert and baseline endorsements      |

## Attestation Models

The LICET wearable supports both models defined in RFC 9334:

Passport Model (RFC 9334 §5.1):
: The Verifier appraises Evidence and produces an Attestation Result token that the
  Relying Party consumes without re-verifying raw Evidence. Suitable for low-latency
  authorization flows.

Background-Check Model (RFC 9334 §5.2):
: The Relying Party forwards Evidence to the Verifier at authorization time. Suitable
  for high-assurance flows where the Relying Party maintains its own appraisal policy.


# Trust Hierarchy: L0–L3 {#trust-hierarchy}

Evidential weight follows attestation level. Higher levels require stronger
attestation of the measurement environment and the credential chain.

## L0: Uncertified Sensor

A consumer wearable with no hardware attestation, no secure element, and no cert
chain. The device reports measurements; there is no attestation of measurement
integrity.

Evidential weight:
: Dimensionality. L0 contributes to the feature vector roughly what an inertial
  signal contributes — it adds a dimension that would cost an adversary effort to
  match, but it does not resist a software-layer attack on the measurement chain.

Supported claim:
: "A device reported a measurement consistent with the claimed physiological state at
  the claimed time." The inference from measurement to intent depends entirely on the
  Relying Party's trust in the device model.

## L1: Software Attestation

The measurement software runs in an attested execution environment (e.g., Android
StrongBox, iOS Secure Enclave application). The device produces a Platform
Attestation Result that covers the measurement software.

Evidential weight:
: The measurement software is covered by a hardware root of trust, but the sensors
  themselves are not attested. A hardware attack on the sensor output is not detected.

Supported claim:
: "Attested software on an attested platform reported a measurement consistent with
  the claimed physiological state." Software-layer forgery is detected; hardware-layer
  forgery is not.

## L2: Certified Device with Attestation Chain {#l2}

A device with a manufacturer cert chain that a Verifier can actually verify. The
Attester credential includes an x.509 cert chain rooted at an Endorser trusted by
the Verifier. Evidence messages are signed with the Attester's private key; the
chain is verifiable at appraisal time.

Evidential weight:
: This is where LICET Evidence starts carrying its own weight. A Verifier can confirm:
  device identity (manufacturer-issued cert, not self-signed), platform integrity
  (Platform Attestation Result over firmware), and key provenance (Attester key
  generated in and bound to a secure element). An adversary that cannot compromise
  the secure element cannot forge the Evidence message.

Supported claim:
: "A certified device with a verifiable attestation chain reported a measurement
  consistent with the claimed physiological state."

## L3: Hardware-Attested Sensor

Sensor-level hardware attestation. The sensor output itself is signed or bound to
the secure element at the hardware layer — the analog-to-digital conversion falls
within the hardware trust boundary.

Evidential weight:
: The full chain — from analog biometric signal to ZKP — is covered by hardware
  attestation. This is the regime where LICET makes its strongest claim.

Supported claim:
: "A hardware-attested sensor on a certified device produced a measurement consistent
  with the claimed physiological state, with the measurement chain attested to the
  silicon boundary."

Note: No commercially available consumer wearable currently meets L3 as defined here.
L3 is the target architecture for eColabs' hardware program. Current deployments
operate at L0 (simulation mode) or L1 (platform attestation via mobile OS).

## Relationship to CPoE T1–T4 Tiers {#tier-mapping}

The CPoE evidence-packet schema {{CPoE-Protocol}} defines a T1–T4 signal-type
hierarchy. LICET L0–L3 is an orthogonal attestation-quality hierarchy: L0–L3
describes the trustworthiness of the measurement chain, while T1–T4 describes
the type of signal being measured. Both hierarchies appear in the same Evidence
packet and MUST NOT be conflated.

The approximate correspondence is:

| LICET level | Signal type (CPoE tier) | Basis                                                        |
|-------------|-------------------------|--------------------------------------------------------------|
| L0          | T1 / T2                 | Uncertified sensor; dimensionality comparable to behavioral (T1) or inertial (T2) signal |
| L1          | T2 / T3                 | Platform-attested software; physiological signal with software-layer guarantee |
| L2          | T3                      | Certified device; physiological signal with verifiable hardware-backed chain  |
| L3          | T3 / T4                 | Hardware-attested sensor; physiological signal attested to silicon boundary   |

An Evidence packet carrying an L0 Attester credential paired with a T3 (ECG)
signal MUST be appraised at L0 evidential weight regardless of signal type. The
lower of (attestation level, signal type tier) governs the Attestation Result.

## Trust Hierarchy Summary

| Level | Sensor attestation | Cert chain          | ZKP scope                        | Evidential weight         |
|-------|--------------------|---------------------|----------------------------------|---------------------------|
| L0    | None               | None                | None / software hash             | Dimensionality only       |
| L1    | Software           | Self-signed or none | Platform-attested software       | Software-layer resistance |
| L2    | Software           | Verifiable (x.509)  | Attested device and software     | Independent weight        |
| L3    | Hardware           | Verifiable (x.509)  | Silicon-boundary attestation     | Strongest available claim |


# Appraisal Logic {#appraisal}

## Appraisal by Trust Level

The Verifier appraises Evidence packets according to the trust level of the Attester
that produced them. Appraisal policy MUST be explicit about the minimum acceptable
level. A high-assurance Relying Party (e.g., medical consent, high-value transaction)
SHOULD require L2 ({{l2}}) minimum.

Evidence from an Attester below the policy minimum MUST NOT be used to produce an
Attestation Result that grants the requested authorization level.

## Limitation Flags in Attestation Results

The following limitation flags MUST be surfaced in the Attestation Result when present:

| Flag                             | Source                    | Meaning                                                                              |
|----------------------------------|---------------------------|--------------------------------------------------------------------------------------|
| `respiratory_periodicity_warning`| Respiratory periodicity   | Detected paced breathing; Mahalanobis distance may not reflect genuine calm         |
| `t2_self_report_only`            | Attestation level check   | T2 without corroborating T3/T4; weight is self-report only                          |
| `baseline_immature`              | Baseline maturity check   | Baseline below minimum session count; Mahalanobis reference is provisional          |
| `sensor_uncertified`             | Attester level check      | L0 or L1 device; measurement chain is not hardware-attested                         |

## ZKP Scope Claim {#zkp-scope}

Every Evidence message that includes a ZKP MUST include a `zkp-scope` claim that
states explicitly what the proof covers. A Relying Party that receives a ZKP without
a `zkp-scope` claim MUST treat the proof as covering measurement chain integrity only
and MUST NOT infer absence of coercion.

~~~ cddl
zkp-scope = {
  proves: [+ zkp-proven-property],
  does-not-prove: [+ zkp-excluded-property]
}

zkp-proven-property = text  ; e.g., "measurement_chain_integrity",
                             ;       "mahalanobis_range", "timestamp"

zkp-excluded-property = text ; e.g., "intent", "absence_of_coercion",
                              ;        "absence_of_volitional_vagal_enhancement"
~~~


# Evidence Encoding (Deferred) {#encoding}

Evidence encoding is deferred to a subsequent revision, pending review of the
topology and appraisal logic in {{attester-topology}} through {{appraisal}}.

The encoding will use the CPoE evidence-packet schema (CBOR tag 1129336645)
{{CPoE-Protocol}} with LICET-specific claim keys registered as extensions. No new
base schema is defined here.

**Rationale for deferral:** Fixing encoding before the L0–L3 chain is stable risks
recutting CDDL each time the trust model shifts. Topology and appraisal logic are
established first; encoding follows.


# Resolved Design Decisions {#resolved-design-decisions}

The following items were raised as open questions in the initial draft and resolved
during review with David Condrey (Writerslogic Inc.):

1. **L2 cert chain endorser model:** Multi-endorser model adopted. The device
   manufacturer covers device identity and hardware calibration trust; eColabs covers
   the LICET-specific baseline endorsement. These two endorsement scopes MUST NOT be
   collapsed into a single Endorser.

2. **Composite Attester boundary:** The sensor layer and processing layer remain
   separate sub-attesters. Merging them would collapse the per-layer distinction that
   the L1-vs-L3 evidential weight difference depends on.

3. **Limitation flags:** The four flags defined in {{appraisal}} are the complete set
   for this revision. The `baseline_immature` flag covers the enrollment gap.
   No additional flags are required at this time.

4. **ZKP scope claim format:** The `zkp-scope` claim MUST map to registered CPoE claim
   keys rather than free-form text. Extension key registration is deferred to the
   encoding revision ({{encoding}}); the CPoE specification will define the extension
   keys on the CPoE side.

5. **L3 definition:** "Analog-to-digital conversion within the hardware trust boundary"
   is confirmed as the correct L3 criterion. This matches sensor-to-TEE binding as
   understood in RFC 9334 and is kept as written.


# Security Considerations

The security properties of LICET Evidence are bounded by the trust level of the
Attester ({{trust-hierarchy}}) and the hard bounds stated in {{limitations}}.

Implementers MUST NOT represent LICET Evidence as proof of intent. The claim is
corroboration ({{claim}}). Relying Party policy determines what corroboration level
is sufficient for a given authorization decision.

The `respiratory_periodicity_warning` flag ({{appraisal}}) MUST be checked before
relying on Mahalanobis distance as the primary evidence for an authorization
decision. A warning flag does not invalidate the Evidence; it bounds the claim.

The `zkp-scope` claim ({{zkp-scope}}) MUST be present in every Evidence message
that includes a ZKP. Its absence MUST be treated as equivalent to a scope limited
to measurement chain integrity only.


# IANA Considerations

This document has no IANA actions at this time. Claim key registration for
LICET-specific extensions to the CPoE evidence-packet schema will be addressed in
a subsequent revision once the encoding section ({{encoding}}) is finalized.


--- back

# Acknowledgments
{:numbered="false"}

The author thanks David Condrey (Writerslogic Inc.) for identifying
paced breathing as the primary limitation of the Mahalanobis design, for the
correction of RFC 9334 composite Attester citations, and for guidance on the
topology-first document structure that informed this draft.
