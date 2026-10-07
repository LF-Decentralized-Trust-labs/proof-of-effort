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

date: 2026-09
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
--- abstract

This document defines the Attester topology and trust hierarchy for LICET
(Layered Intent Corroboration via Embedded Trust) wearable devices operating as
composite Attesters under the RATS architecture. A LICET wearable
measures heart rate variability, electrodermal activity, and derived features,
and produces cryptographically bound Evidence about a subject's physiological
state at the time of an authorization request. This document establishes L0-L3
as a graduated trust hierarchy in which evidential weight follows attestation
level, states the hard bounds on what physiological corroboration can claim
before any architecture is presented, and defines the appraisal logic by which a
Verifier carries those bounds into Attestation Results. Evidence is encoded
as a CPoE evidence packet with LICET-specific extension keys; no new schema is
defined.

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
- Defines the Evidence encoding as extension keys on the CPoE evidence packet ({{encoding}})

The encoding uses the CPoE evidence-packet schema (CBOR tag 1129336645)
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
`respiratory-periodicity-warning` flag in the Evidence packet is a detector output,
not a falsification of the Mahalanobis claim. Both MUST be surfaced; interpretation
is left to the Relying Party.

## Behavioral Corroboration Layer Self-Report Gap {#behavioral-gap}

The LICET behavioral corroboration layer (distinct from the CPoE attestation tier
T2) relies on signals produced by the subject asserting intent: typing dynamics,
gaze entropy, and touchscreen interaction patterns. These signals cannot be
independently verified by the wearable Attester; they are self-reported by
definition.

**Consequence:** Behavioral corroboration Evidence does not carry evidential weight
independent of a concurrently valid physiological layer (L2 or L3). A Relying
Party policy that grants authorization on behavioral signals alone accepts
self-report as its primary evidence. This MUST be explicit in Relying Party policy.


# The Claim Stated Precisely {#claim}

A LICET ZKP (zero-knowledge proof) in an authorization bundle proves:

- That a measurement was taken at a claimed time
- That the measurement's Mahalanobis distance from the subject's baseline falls
  within a claimed range
- That the cryptographic chain from sensor to proof is intact

The ZKP does NOT prove:

- That the physiological state reflects genuine uncoerced calm
- That the subject was not performing volitional vagal enhancement ({{paced-breathing}})
- That the behavioral corroboration layer reflects actual behavioral state ({{behavioral-gap}})

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
| Endorser (device)    | Device manufacturer                  | Provides device identity cert and hardware calibration trust |
| Endorser (baseline)  | eColabs                              | Provides LICET-specific baseline endorsement; scope MUST NOT be collapsed with device Endorser |

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

## Relationship to CPoE Attestation Tiers {#tier-mapping}

LICET L0–L3 is this document's label for the attestation assurance axis. The CPoE
evidence-packet schema {{CPoE-Protocol}} expresses the same axis through the
`attestation-tier` field (evidence-packet key 7), where T1–T4 corresponds to
increasing hardware trust anchoring strength. L0–L3 and T1–T4 are not orthogonal
hierarchies; they name the same axis.

The value mapping to `attestation-tier` (key 7) is:

| LICET level | CPoE `attestation-tier` | Attestation strength                      |
|-------------|-------------------------|-------------------------------------------|
| L0          | T1                      | Software-only; no hardware trust anchor   |
| L1          | T2                      | Attested software; platform-bound key     |
| L2          | T3                      | Hardware-bound; verifiable cert chain     |
| L3          | T4                      | Silicon-boundary; sensor-to-TEE binding   |

An Evidence packet MUST encode the Attester trust level as `attestation-tier`
(CPoE evidence-packet key 7). This field is informational: a Verifier MUST derive
the effective tier independently from the actual attestation evidence present in
the packet and MUST NOT grant a higher evidential weight than the evidence
supports, even if the encoded `attestation-tier` claims a higher level. Evidential
weight follows the verified tier, not the claimed value.

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
| `respiratory-periodicity-warning`| Respiratory periodicity   | Detected paced breathing; Mahalanobis distance may not reflect genuine calm         |
| `behavioral-self-report-only`    | Behavioral layer check    | Behavioral corroboration layer active without concurrent physiological layer (L2/L3); weight is self-report only |
| `baseline-immature`              | Baseline maturity check   | Baseline below minimum session count; Mahalanobis reference is provisional          |
| `sensor-uncertified`             | Attester level check      | L0 or L1 device; measurement chain is not hardware-attested                         |

The encoding of these flags is defined in {{flag-encoding}}.

## ZKP Scope Claim {#zkp-scope}

Every Evidence message that includes a ZKP MUST include a `zkp-scope` claim that
states explicitly what the proof covers. A Relying Party that receives a ZKP without
a `zkp-scope` claim MUST treat the proof as covering measurement chain integrity only
and MUST NOT infer absence of coercion.

The encoding of the claim is defined in {{zkp-scope-encoding}}.


# Evidence Encoding {#encoding}

LICET Evidence is carried as a CPoE Evidence Packet {{CPoE-Protocol}} (CBOR tag
1129336645) with LICET-specific extension keys. This document defines no base
schema, no new CBOR tag, and no new media type.

CPoE reserves evidence-packet and checkpoint keys 0-99 for its own use and
requires Verifiers to ignore unrecognized keys with values 100 or greater. Every
LICET extension defined here therefore uses a key at or above 100, and a CPoE
Verifier with no LICET support appraises the packet as ordinary CPoE Evidence
rather than rejecting it.

## Attestation Level Is Not Separately Encoded {#level-encoding}

The L0-L3 hierarchy of {{trust-hierarchy}} and the CPoE attestation tier T1-T4
are the same axis. Both grade the strength of the hardware trust anchoring
beneath a measurement; neither describes the kind of signal measured. CPoE's
orthogonal axis is the Evidence Content Tier (CORE, ENHANCED, MAXIMUM), which
governs collection depth.

A LICET Attester therefore MUST NOT encode its attestation level in an extension
key. It MUST set the CPoE `attestation-tier` field (evidence-packet key 7) to the
corresponding value:

| LICET level | CPoE attestation-tier | Value |
|-------------|-----------------------|-------|
| L0          | `software-only`       | 1     |
| L1          | `attested-software`   | 2     |
| L2          | `hardware-bound`      | 3     |
| L3          | `hardware-hardened`   | 4     |

The appraisal rule of {{tier-mapping}} follows from this identity rather than
from a comparison between two hierarchies: there is one attestation axis, and the
Verifier appraises against it. Where {{trust-hierarchy}} and CPoE describe the
same level in different words, the CPoE definition governs the encoding and the
LICET text governs the evidential interpretation.

An Attester that cannot substantiate a tier MUST omit key 7 rather than assert a
lower one; CPoE requires the Verifier to derive the tier from evidence content in
that case.

## LICET Physiological Evidence (evidence-packet key 100) {#licet-evidence}

~~~ cddl
licet-evidence = {
    1 => uint,                    ; licet-version (MUST be 1)
    2 => hash-value,              ; calm-fingerprint-ref
    ? 3 => uint,                  ; mahalanobis-distance (milli-sigma); Disclosure mode only
    4 => confidence-tier,         ; baseline-maturity
    ? 5 => uint,                  ; rsa-cv (milli-units)
    ? 6 => [1*8 licet-signal],    ; per-modality summaries
    ? 7 => zkp-scope,             ; REQUIRED when key 9 is present
    ? 8 => hash-value,            ; sensor-attestation-ref (REQUIRED at tier 4)
    ? 9 => zkp-proof,             ; Zero-knowledge mode only
}

licet-signal = {
    1 => signal-modality,
    2 => streaming-stats,
    ? 3 => uint,                  ; sample-count
}

signal-modality = &(
    hrv-rmssd:   1,
    hrv-hf:      2,
    hr:          3,
    eda:         4,
    respiration: 5,
)
~~~

The two Evidence modes of the Privacy Considerations section map to the encoding
as follows. In Disclosure mode, key 3 MUST be present and key 9 MUST be absent.
In Zero-knowledge mode, key 9 and key 7 MUST be present and key 3 MUST be absent,
and no `licet-sample` in a checkpoint may carry key 2. A packet that carries both
key 3 and key 9, or neither, MUST be rejected.

`zkp-proof` carries the proof system identifier and the opaque proof. The proof
system and its verification are outside this document.

`calm-fingerprint-ref` is a digest of the enrolled baseline, not the baseline
itself. The baseline never leaves the Attester. A Verifier confirms that the
distance in key 3 was computed against an endorsed baseline by matching this
digest against the Endorser's record; it does not reconstruct the baseline.

`mahalanobis-distance` is expressed in milli-sigma (thousandths of a standard
deviation) as an unsigned integer, following the integer-scaling convention CPoE
uses for thermal (millidegrees) and inertial (micro-g) samples. Floating-point
encoding MUST NOT be used for this field: appraisal compares it against a policy
threshold, and the comparison has to be reproducible across implementations.

`rsa-cv` is the respiratory sinus arrhythmia coefficient of variation in
milli-units. The illustrative values of {{paced-breathing}} encode as 30 and 400.
Its presence is what allows a Verifier to raise
`licet.respiratory-periodicity-warning`; an Attester that cannot compute it MUST
omit key 5 rather than encode a sentinel.

`baseline-maturity` reuses the CPoE `confidence-tier` enumeration
(`population-reference`, `emerging`, `established`, `mature`) rather than
defining a parallel scale.

`sensor-attestation-ref` is a digest of the sensor-layer attestation evidence and
MUST be present when `attestation-tier` is 4 (L3). At L3 the claim is that the
measurement chain is attested to the silicon boundary, and that claim is not
verifiable without a reference to the sensor attestation that carries it.

### Per-Checkpoint Samples (checkpoint key 100)

Where a LICET measurement accompanies a CPoE session rather than a single
authorization request, each checkpoint MAY carry one sample:

~~~ cddl
licet-sample = {
    1 => cpoe-timestamp,
    ? 2 => uint,                  ; mahalanobis-distance (milli-sigma); Disclosure mode only
    ? 3 => uint,                  ; rsa-cv (milli-units)
}
~~~

In Zero-knowledge mode samples carry no distance, so only `rsa-cv` and the
timestamp appear. A trajectory of samples across checkpoints is what distinguishes a sustained
physiological state from a single favourable reading. A Verifier that receives
per-checkpoint samples MUST appraise the trajectory, not only the packet-level
value in {{licet-evidence}} key 3, where present.

## ZKP Scope Claim Encoding {#zkp-scope-encoding}

{{zkp-scope}} requires a scope claim on every Evidence message carrying a ZKP.
Free-form text does not survive appraisal: two Attesters can describe the same
exclusion in different words, and a Verifier cannot compare them. The claim is
therefore encoded with registered integer properties.

~~~ cddl
zkp-scope = {
  1 => [+ zkp-property],        ; proves
  2 => [+ zkp-property],        ; does-not-prove
}

zkp-property = &(
  measurement-chain-integrity:             1,
  mahalanobis-range:                       2,
  measurement-timestamp:                   3,
  sensor-attestation:                      4,
  intent:                                100,
  absence-of-coercion:                   101,
  absence-of-volitional-vagal-enhancement: 102,
)
~~~

Properties 1-4 are the measurement-chain properties a LICET proof can establish.
Properties 100-102 are the inferential properties it cannot. An Attester MUST
list `intent` and `absence-of-coercion` in key 2 whenever a ZKP is present, and
MUST list `absence-of-volitional-vagal-enhancement` in key 2 whenever the proof
covers `mahalanobis-range`. A property MUST NOT appear in both arrays.

A Verifier that receives a ZKP without a `zkp-scope` claim MUST appraise it as if
key 1 contained only `measurement-chain-integrity` and key 2 contained
`intent`, `absence-of-coercion`, and
`absence-of-volitional-vagal-enhancement`, per {{zkp-scope}}.

## Limitation Flags {#flag-encoding}

The flags of {{appraisal}} are carried in the existing CPoE `limitations` field
(evidence-packet key 8, an array of text strings), not in a LICET extension key.
CPoE already defines that field as the place an Attester records conditions
bounding its own Evidence, and a Verifier that surfaces CPoE limitations will
surface these without LICET-specific code.

Flag values are namespaced:

| Flag value                              | Condition                                     |
|-----------------------------------------|-----------------------------------------------|
| `licet.respiratory-periodicity-warning` | Paced breathing detected via `rsa-cv`         |
| `licet.behavioral-self-report-only`     | Behavioral layer without concurrent T3/T4      |
| `licet.baseline-immature`               | `baseline-maturity` below `established`       |
| `licet.sensor-uncertified`              | `attestation-tier` is 1 or 2                  |

The hyphenated form matches `zkp-scope` and the CDDL naming used throughout this
document.

An Attester MUST emit `licet.sensor-uncertified` whenever `attestation-tier` is 1
or 2, and `licet.baseline-immature` whenever `baseline-maturity` is
`population-reference` or `emerging`. These two are derivable by the Verifier
from the packet, and requiring them of the Attester keeps a Relying Party that
reads only the limitations array from having to re-derive them.

## Privacy of the Encoded Evidence {#encoding-privacy}

The encoding excludes raw physiological samples. An Evidence Packet carries the
distance of a measurement from an enrolled baseline, a digest of that baseline,
and optionally per-modality summary statistics. It does not carry the timeseries
or the baseline itself, so an arrhythmia, a stress response, or any other pattern
that lives in the shape of a signal over time cannot be recovered from it.

The summary statistics of {{licet-evidence}} key 6 are the exception, and this
section does not claim otherwise. A `licet-signal` for the `hr` or `hrv-rmssd`
modality carries a mean, a minimum, and a maximum, and those are physiological
values in the ordinary sense: a mean heart rate is a heart rate. Key 6 is
OPTIONAL for exactly this reason.

This is a constraint on implementations, not only a description. An Attester MUST
NOT place raw samples in a LICET extension key, and MUST NOT use the CPoE
extension range to reintroduce them under another name. Heart rate variability
and electrodermal activity are health data in most jurisdictions; an
authorization decision needs the distance, not the signal.

Of the five `streaming-stats` fields, the minimum and maximum are the most
identifying across repeated sessions, since they track the extremes of a
subject's range rather than its centre. An appraisal policy that thresholds on
Mahalanobis distance alone does not need key 6 at all, and a profile in that
position SHOULD omit it rather than populate it. Key 6 earns its place only where
a Verifier appraises per-modality plausibility, and a profile that includes it
MUST say which modalities it requires and why.

The privacy considerations of {{CPoE-Protocol}} apply to the enclosing Evidence
Packet unchanged.

## IANA Considerations

This document has no IANA actions. The code points listed in {{encoding}} are
extension values within CPoE's extension range and are to be registered on the CPoE
side once that registry exists.


--- back

# Acknowledgments
{:numbered="false"}

The author thanks David Condrey (Writerslogic Inc.) for identifying
paced breathing as the primary limitation of the Mahalanobis design, for the
correction of RFC 9334 composite Attester citations, and for guidance on the
topology-first document structure that informed this draft.
