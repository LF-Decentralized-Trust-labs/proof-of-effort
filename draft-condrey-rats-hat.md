---
v: 3
docname: draft-condrey-rats-hat-latest
title: "Hardware Attestation of Time (HAT): TPM-Based Temporal Binding for Remote Attestation"
abbrev: HAT
category: info
ipr: trust200902
submissiontype: IETF
area: Security
workgroup: Remote ATtestation ProcedureS
keyword:
  - attestation
  - RATS
  - TPM
  - temporal binding
  - hardware time

stand_alone: yes
pi:
  toc: yes
  tocdepth: "3"
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
  RFC8610:
  RFC8949:
  RFC9334:
  TPM2-Structures:
    title: "Trusted Platform Module Library, Part 2: Structures"
    target: "https://trustedcomputinggroup.org/resource/tpm-library-specification/"
    author:
      - org: Trusted Computing Group
    date: 2019
    seriesinfo:
      TCG: "Revision 01.59"
  TPM2-Commands:
    title: "Trusted Platform Module Library, Part 3: Commands"
    target: "https://trustedcomputinggroup.org/resource/tpm-library-specification/"
    author:
      - org: Trusted Computing Group
    date: 2019
    seriesinfo:
      TCG: "Revision 01.59"

informative:
  RFC3161:
  RFC9052:
  RFC9711:
  I-D.birkholz-rats-tuda:
    title: "Time-Based Uni-Directional Attestation"
    author:
      - fullname: Henk Birkholz
        initials: H.
        surname: Birkholz
    date: 2023
    seriesinfo:
      Internet-Draft: draft-birkholz-rats-tuda-07
  CPoE-Protocol:
    title: "Cryptographic Proof of Effort (CPoE): Architecture and Evidence Format"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-protocol-06
---

--- abstract

This document specifies Hardware Attestation of Time (HAT), a
mechanism for binding a computation's duration to hardware-attested
time using a Trusted Platform Module (TPM 2.0). HAT constructs a
"temporal sandwich" by obtaining TPM2\_GetTime() readings before and
after a computation, with both readings signed by an Attestation
Identity Key (AIK). The resulting proof provides hardware-backed
evidence that a computation occupied at least the attested time
interval, even when the platform operator is adversarial. This
document defines the HAT proof construction, wire format, and
verification procedure, and maps HAT to the RATS architecture
(RFC 9334).

--- middle

# Introduction {#introduction}

## The TPM Temporal Attestation Gap {#temporal-gap}

Remote attestation protocols frequently need to verify temporal
claims: that a computation took at least a certain amount of time,
that events occurred in a specific order, or that a process was
active during a claimed interval. Software timestamps are trivially
forgeable by a platform operator who controls the system clock.
RFC 3161 Timestamp Authorities {{RFC3161}} provide trusted
timestamps but require network connectivity, introduce a dependency
on a third-party service, and attest only to the time of a signing
request, not to the duration of a computation.

TPM 2.0 devices contain a monotonic hardware clock that the
platform operator cannot roll back without triggering detectable
state changes (resetCount increments, clock-safe flag transitions).
The TPM2\_GetTime command {{TPM2-Commands}} returns the current
clock value in a signed attestation structure, providing a
hardware-rooted temporal claim. However, no existing specification
defines a standard protocol for using paired TPM2\_GetTime readings
to prove that a computation occupied a specific time interval.

## HAT Solution {#hat-solution}

Hardware Attestation of Time (HAT) fills this gap by defining a
"temporal sandwich" construction. The Attester calls TPM2\_GetTime
immediately before starting a computation, executes the computation,
then calls TPM2\_GetTime again immediately after. Both time
attestations are signed by the same Attestation Identity Key (AIK),
and the pair is packaged as a HAT proof. The Verifier checks both
signatures, validates clock continuity (no reboots, no clock
disturbance), and confirms that the elapsed time meets or exceeds
the expected computation duration.

HAT proofs can optionally bind the computation's input to the
pre-computation time reading, ensuring that the computation could
not have started before the attested time. This prevents
pre-computation attacks where an adversary performs the work in
advance and later claims it occurred during the attested interval.

HAT is designed for scenarios where the platform operator is the
primary adversary, such as proof-of-effort systems, anti-cheat
mechanisms, and temporal attestation in adversarial environments.
The mechanism was originally defined as part of the Cryptographic
Proof of Effort (CPoE) protocol {{CPoE-Protocol}} and is extracted
here as a standalone specification for broader applicability.

## Relationship to RFC 9334 {#rats-relationship}

HAT is designed as a building block within the RATS architecture
{{RFC9334}}. It produces hardware-anchored temporal Evidence that
fits naturally into RATS data flows. The Attester generates HAT
proofs as part of its Evidence. The Verifier appraises HAT proofs
using the AIK certificate chain provided by the Endorser (TPM
manufacturer). Relying Parties consume the temporal attestation
result as part of broader trust decisions.

This document defines HAT independently of any specific consuming
protocol. Any RATS profile that requires hardware-attested duration
measurement can incorporate HAT proofs.

## Related Work {#related-work}

Time-Based Uni-Directional Attestation (TUDA)
{{I-D.birkholz-rats-tuda}} explored TPM-based temporal attestation
using Time Stamp Authorities (TSA). TUDA required synchronization
between TPM tick time and external TSA timestamps via a Sync Base
Protocol, creating Sync Tokens for audit logs.

HAT differs from TUDA in several ways:

- HAT requires no external Time Stamp Authority, relying solely on
  TPM2\_GetTime() for temporal bracketing.
- HAT focuses on computation duration verification rather than
  general audit logging.
- HAT uses TPM resetCount and clock-safe validation rather than TSA
  synchronization.
- HAT is designed for integration with sequential work verification
  rather than general attestation flows.

While TUDA addressed a similar problem space, the draft expired
without progressing to standardization. HAT provides a simpler,
self-contained approach suitable for applications where TSA
infrastructure is unavailable or undesirable.

Timestamp Authorities per {{RFC3161}} provide trusted timestamps
but require network connectivity and attest only to signing time,
not computation duration.

# Conventions and Definitions

{::boilerplate bcp14-tagged}

The following terms are used throughout:

AIK:
: Attestation Identity Key. A restricted signing key in the TPM's
  Endorsement hierarchy, used to sign attestation structures.

TPMS\_TIME\_ATTEST\_INFO:
: The TPM 2.0 attestation structure returned by TPM2\_GetTime,
  containing the clock value, reset count, and clock-safe flag,
  per {{TPM2-Structures}}.

Temporal sandwich:
: The pattern of obtaining TPM time readings before and after a
  computation to bound its duration with hardware-attested time.

Attester:
: The entity generating HAT proofs (per {{RFC9334}}).

Verifier:
: The entity validating HAT proofs (per {{RFC9334}}).

# Architecture {#architecture}

## RATS Entity Roles {#rats-roles}

HAT maps to RATS entity roles {{RFC9334}} as follows:

Attester / Attesting Environment:
: The platform performing the attested computation. The Attesting
  Environment includes the application, the OS TPM interface, and
  the TPM hardware. The Attester generates HAT proofs by
  interacting with the TPM before and after each computation.

Endorser:
: TPM manufacturers that issue Endorsement Key (EK) certificates
  and platform attestation credentials. The Endorser's certificate
  chain establishes that the AIK is bound to genuine TPM hardware.

Verifier:
: The entity that receives HAT proofs, validates AIK signatures
  against the Endorser's certificate chain, and checks temporal
  consistency. The Verifier produces Attestation Results indicating
  whether the temporal claim is valid.

Relying Party:
: The consumer of Attestation Results. The Relying Party uses the
  temporal attestation to make trust decisions (e.g., accepting or
  rejecting a proof-of-effort claim).

Reference Value Provider:
: Provides expected computation durations and clock tolerance
  parameters that the Verifier uses to evaluate HAT proofs.

## HAT in the Attestation Flow {#attestation-flow}

HAT operates within the RATS passport model ({{RFC9334}},
Section 8.1): the Attester generates Evidence containing HAT
proofs and conveys it directly to the Verifier.

~~~
+----------+                      +-----------+
| Attester |                      |  Verifier |
|          |                      |           |
| 1. TPM2_GetTime (T_before)      |           |
| 2. Execute computation          |           |
| 3. TPM2_GetTime (T_after)       |           |
| 4. Package HAT proof            |           |
|          |                      |           |
|          |--- Evidence -------->|           |
|          |   (includes HAT)     |           |
|          |                      | 5. Verify |
|          |                      |    sigs   |
|          |                      | 6. Check  |
|          |                      |    clock  |
|          |                      | 7. Verify |
|          |                      |    delta  |
|          |                      |           |
+----------+                      +-----+-----+
                                        |
                                        v
                                  Attestation
                                   Result
~~~

The Attester performs steps 1-4 locally. The Evidence (including
the HAT proof) is conveyed to the Verifier, which performs
signature verification, clock consistency checks, and time delta
validation.

## Trust Model {#trust-model}

HAT implements a critical trust inversion relative to traditional
remote attestation. In conventional RATS deployments, the Attester
operator is trusted and the adversary is external (malware,
network attackers). HAT is designed for scenarios where the
Attester operator is the primary adversary, analogous to anti-cheat
and DRM attestation patterns.

The trust anchors in HAT are:

* The TPM hardware, which provides a monotonic clock that the
  platform operator cannot roll back without detectable state
  changes.
* The AIK, which is a restricted signing key that can only be
  used for attestation operations within the TPM. The platform
  operator cannot extract or forge AIK signatures.
* The Endorser's certificate chain, which binds the AIK to
  genuine TPM hardware from a known manufacturer.

The Attester operator controls the software stack and can choose
when to call TPM2\_GetTime, but cannot forge the TPM's clock
readings or AIK signatures. The Verifier's trust in HAT proofs
derives from trust in the TPM hardware and the Endorser's
certificate chain, not from trust in the Attester operator.

# TPM Procedure {#tpm-procedure}

## TPM2\_GetTime() Bracketing {#tpm-bracketing}

The Attester MUST obtain TPM-attested time readings before and
after each computation to be temporally attested:

~~~ pseudocode
T_before = TPM2_GetTime(aikHandle, qualifyingData_before)

... execute computation ...

T_after  = TPM2_GetTime(aikHandle, qualifyingData_after)
~~~

The `aikHandle` MUST refer to the same AIK for both calls within
a single HAT proof. Using different AIKs would prevent the
Verifier from confirming that both readings originate from the
same TPM.

Each TPM2\_GetTime call returns a TPMS\_ATTEST structure
{{TPM2-Structures}} containing:

* `clockInfo.clock`: The TPM's monotonic millisecond counter.
  This counter increments while the platform is powered and the
  TPM is operational. It does not reset on reboot (only on
  owner-clear events, which increment resetCount).
* `clockInfo.resetCount`: Incremented on each TPM2\_Startup(CLEAR).
  A change between T\_before and T\_after indicates a platform
  reboot occurred during the computation.
* `clockInfo.restartCount`: Incremented on each
  TPM2\_Startup(STATE), typically from hibernation/resume.
* `clockInfo.safe`: A boolean flag indicating whether the TPM
  clock value is reliable. Set to NO if the TPM detects a clock
  discontinuity (e.g., the platform was powered off and the
  TPM's internal battery-backed RTC was disturbed).
* `firmwareVersion`: The TPM firmware version. A mismatch between
  readings would indicate a firmware update during computation
  (extremely unlikely and suspicious).

The qualifying data (qualifyingData) is an optional caller-provided
nonce included in the attestation structure and covered by the AIK
signature. Applications SHOULD include qualifying data that binds
the HAT proof to the specific computation:

* For T\_before: the qualifying data SHOULD include the
  computation's input seed or a hash thereof, preventing the
  HAT proof from being reused with a different computation.
* For T\_after: the qualifying data SHOULD include the
  computation's output or a commitment to it (e.g., a Merkle
  root), binding the temporal claim to the specific result.

If the TPM is unavailable or returns an error, the Attester MUST
NOT generate a HAT proof. The consuming protocol determines
whether to proceed without temporal attestation (at a lower
assurance level) or to abort.

The Attester SHOULD minimize the delay between the T\_before call
and the start of computation, and between the end of computation
and the T\_after call. Unnecessary delays inflate the attested
interval without corresponding computation, reducing the
precision of the temporal binding.

## AIK Signature Requirements {#aik-requirements}

The AIK used for HAT proofs MUST satisfy the following requirements:

Key Attributes:
: The AIK MUST be a restricted signing key created with
  `TPMA_OBJECT.restricted` set and `TPMA_OBJECT.sign` set, per
  {{TPM2-Structures}}. The `restricted` attribute ensures the key
  can only sign TPM-generated attestation structures, preventing
  the platform operator from using the AIK to sign arbitrary data
  that could be confused with genuine attestation.

Key Hierarchy:
: The AIK SHOULD reside in the Endorsement hierarchy, linked to the
  TPM's Endorsement Key (EK). This provides the strongest binding
  to the TPM manufacturer's certificate chain. AIKs in the Platform
  or Storage hierarchies MAY be used but offer weaker provenance
  guarantees.

Certificate Chain:
: The AIK certificate chain MUST root to a known TPM manufacturer's
  Endorsement Key certificate. The Verifier MUST validate the full
  certificate chain before accepting HAT proofs. The chain
  establishes that the AIK is bound to genuine TPM hardware and
  has not been extracted or cloned.

Signature Scheme:
: The AIK signature scheme MUST be one of the following TPM 2.0
  approved schemes:

  * RSASSA-PKCS1-v1\_5 with SHA-256 (TPM\_ALG\_RSASSA with
    TPM\_ALG\_SHA256)
  * RSAPSS with SHA-256 (TPM\_ALG\_RSAPSS with TPM\_ALG\_SHA256)
  * ECDSA with P-256 (TPM\_ALG\_ECDSA with TPM\_ALG\_SHA256
    on TPM\_ECC\_NIST\_P256)

  The Verifier MUST support at least RSASSA-PKCS1-v1\_5 with
  SHA-256 and ECDSA with P-256.

Key Conveyance:
: The AIK public key and certificate chain MUST be conveyed to the
  Verifier. This MAY be done out-of-band (e.g., during device
  enrollment) or inline as part of the Evidence structure. The
  consuming protocol defines the specific conveyance mechanism.

## Clock-Safe Validation {#clock-safe}

The `clockInfo.safe` field in the TPMS\_TIME\_ATTEST\_INFO structure
indicates whether the TPM considers its clock value reliable.

The TPM sets `safe` to YES (true) during normal operation when the
clock has been continuously maintained. The TPM sets `safe` to NO
(false) when it detects a clock discontinuity, which can occur in
the following scenarios:

* The platform lost power and the TPM's battery-backed RTC was
  disturbed or absent.
* The TPM detected a hardware anomaly affecting clock integrity.
* The TPM was explicitly informed of a clock discontinuity via
  TPM2\_ClockSet (though this command is typically restricted
  to the platform hierarchy).

When `safe` is NO, the TPM clock value may have jumped forward or
backward by an unknown amount. The temporal binding is unreliable
because the attested interval may not correspond to actual elapsed
time.

The Verifier MUST check T\_before.clockInfo.safe:

* If T\_before.clockInfo.safe is NO, the Verifier MUST reject the
  HAT proof. A clock discontinuity before the computation means
  the temporal binding is unreliable from the start.
* If T\_after.clockInfo.safe is NO but T\_before.clockInfo.safe
  was YES, the Verifier SHOULD reject the HAT proof. A clock
  discontinuity during the computation invalidates the time delta.
* If both readings have safe = YES, the clock was continuously
  maintained throughout the attested interval.

## resetCount and restartCount Checks {#reset-checks}

The `clockInfo.resetCount` field increments on each
TPM2\_Startup(CLEAR), which occurs when the platform performs a
full reset (power cycle, hardware reset). The
`clockInfo.restartCount` field increments on each
TPM2\_Startup(STATE), which occurs when the platform resumes from
hibernation.

The Verifier MUST verify that resetCount is identical in both
T\_before and T\_after. A resetCount mismatch indicates that the
platform rebooted during the computation. A reboot resets the
TPM's volatile state and may allow the platform operator to
manipulate the computation environment (e.g., restart with
different software, replay a saved state).

The Verifier SHOULD verify that restartCount is identical in both
readings. A restartCount change indicates the platform hibernated
and resumed during the computation. While hibernation preserves
the TPM clock, it provides an opportunity for the platform
operator to inspect and potentially modify computation state.

For multi-invocation protocols where HAT proofs are chained across
successive computations, the Verifier MUST also verify that the
resetCount in T\_before of invocation n matches the resetCount in
T\_after of invocation n-1. A resetCount change between
invocations indicates a platform reboot that may break the
temporal chain.

# Wire Format {#wire-format}

## hat-proof CDDL {#hat-cddl}

The HAT proof structure is encoded in CBOR {{RFC8949}} using the
following CDDL {{RFC8610}} definition:

~~~ cddl
hat-proof = {
    1 => bstr,     ; time-before (TPMS_TIME_ATTEST_INFO)
    2 => bstr,     ; time-after (TPMS_TIME_ATTEST_INFO)
    3 => bstr,     ; sig-before (AIK signature)
    4 => bstr,     ; sig-after (AIK signature)
}
~~~

The following CBOR diagnostic notation shows an example HAT proof
(values truncated for readability):

~~~ cbor-diag
{
  1: h'ff20...0140',    / time-before: TPMS_ATTEST /
  2: h'ff20...0280',    / time-after:  TPMS_ATTEST /
  3: h'3045...9a7b',    / sig-before: ECDSA-P256   /
  4: h'3045...c4e2'     / sig-after:  ECDSA-P256   /
}
~~~

HAT proofs MUST use deterministic CBOR encoding per Section 4.2.1
of {{RFC8949}}.

## Field Semantics {#field-semantics}

### time-before (Key 1) {#field-time-before}

The `time-before` field MUST contain the raw TPMS\_ATTEST structure
returned by TPM2\_GetTime before the computation, marshaled as a
TPM2B\_ATTEST byte string per {{TPM2-Structures}}. The
TPMS\_ATTEST structure includes:

* `magic`: MUST be TPM\_GENERATED\_VALUE (0xff544347).
* `type`: MUST be TPM\_ST\_ATTEST\_TIME (0x8019).
* `qualifiedSigner`: Name of the signing key (AIK).
* `extraData`: The qualifying data provided by the caller.
* `clockInfo`: The TPMS\_CLOCK\_INFO containing clock, resetCount,
  restartCount, and safe fields.
* `firmwareVersion`: TPM firmware version.
* `attested`: The TPMS\_TIME\_ATTEST\_INFO containing the time
  value.

The maximum size of a TPMS\_ATTEST structure is implementation-
dependent but typically does not exceed 256 bytes.

### time-after (Key 2) {#field-time-after}

The `time-after` field has the same format as `time-before` and
MUST contain the TPMS\_ATTEST structure from the TPM2\_GetTime call
after the computation.

### sig-before (Key 3) {#field-sig-before}

The `sig-before` field MUST contain the TPMT\_SIGNATURE structure
produced by the TPM when signing the T\_before attestation. The
signature covers the entire TPMS\_ATTEST structure in
`time-before`. The signature scheme is determined by the AIK's key
attributes (see {{aik-requirements}}).

For ECDSA signatures, the TPMT\_SIGNATURE is marshaled as the
concatenation of the r and s values per the TPM2 Part 2
specification. For RSA signatures, it is the raw signature value.

### sig-after (Key 4) {#field-sig-after}

The `sig-after` field has the same format as `sig-before` and MUST
contain the TPMT\_SIGNATURE for the T\_after attestation.

## Embedding in Consuming Protocols {#embedding}

HAT proofs are designed to be embedded within larger Evidence
structures defined by consuming protocols. The consuming protocol
defines:

* The key or position within its Evidence structure where the
  HAT proof is stored (e.g., CPoE uses checkpoint key 15).
* How the AIK public key and certificate chain are conveyed.
* Any additional binding between the HAT proof and the
  computation (e.g., incorporating T\_before into a seed
  derivation).

For standalone use outside a consuming protocol, a HAT proof
MAY be wrapped in a COSE\_Sign1 envelope {{RFC9052}} with the
AIK certificate chain in the unprotected header.

# Verification {#verification}

## Verification Procedure {#verification-procedure}

The Verifier MUST perform the following checks on each HAT proof:

1. **Signature verification**: Verify both signatures (sig-before,
   sig-after) against the AIK public key. The Verifier MUST
   validate the AIK certificate chain back to a known TPM
   manufacturer's root certificate. If either signature fails
   verification, the Verifier MUST reject the HAT proof.

2. **Attestation type check**: Verify that both TPMS\_ATTEST
   structures have `type` = TPM\_ST\_ATTEST\_TIME and `magic` =
   TPM\_GENERATED\_VALUE. This confirms the structures were
   generated by TPM2\_GetTime and not by another TPM command.

3. **resetCount consistency**: Verify that resetCount is identical
   in both T\_before and T\_after. If they differ, a platform reboot
   occurred during the computation, and the Verifier MUST reject
   the HAT proof.

4. **Clock-safe validation**: Verify T\_before.clockInfo.safe is
   true. If false, the TPM clock may have been disturbed and
   the Verifier MUST reject the HAT proof. If
   T\_after.clockInfo.safe is false but T\_before was true, the
   Verifier SHOULD reject the HAT proof.

5. **Time delta check**: Compute the elapsed time:
   delta = T\_after.clock - T\_before.clock. The delta MUST be
   greater than or equal to the expected computation duration for
   the declared parameters. This bounds the minimum computation
   time to what the TPM's hardware clock observed. The Verifier
   SHOULD also check that the delta is not implausibly large
   (e.g., more than 10x the expected duration), which may indicate
   the computation was paused or the platform was suspended.

6. **Temporal chain continuity** (multi-invocation): For HAT proofs
   chained across successive computations, the Verifier MUST verify
   that T\_before.clock of invocation n is strictly greater than
   T\_after.clock of invocation n-1. This ensures temporal ordering
   at the hardware level.

7. **Computation binding** (application-specific): If the consuming
   protocol binds the computation's seed to T\_before (e.g., by
   incorporating T\_before.attestation into a seed derivation), the
   Verifier MUST verify this binding. This confirms the computation
   could not have started before the attested time.

## Error Handling {#error-handling}

The following error conditions are defined. For each, the specified
Verifier behavior applies:

| Error Condition | Verifier Behavior |
|---|---|
| sig-before or sig-after fails verification | MUST reject |
| AIK certificate chain invalid or untrusted | MUST reject |
| TPMS\_ATTEST type is not TPM\_ST\_ATTEST\_TIME | MUST reject |
| resetCount mismatch between T\_before and T\_after | MUST reject |
| T\_before.clockInfo.safe is false | MUST reject |
| T\_after.clockInfo.safe is false | SHOULD reject |
| restartCount mismatch | SHOULD reject |
| Time delta less than expected duration | MUST reject |
| Time delta implausibly large (>10x expected) | SHOULD flag as warning |
| Temporal chain continuity violation | MUST reject |
| firmwareVersion mismatch between readings | SHOULD reject |

When the Verifier rejects a HAT proof, the consuming protocol
determines the consequences (e.g., rejecting the entire Evidence,
downgrading the assurance tier, requesting re-attestation).

When the Verifier issues a warning (e.g., implausibly large delta),
the Attestation Result SHOULD include the warning for the Relying
Party to evaluate.

# Security Considerations {#security-considerations}

## TPM Clock Manipulation {#clock-manipulation}

The TPM clock is designed to be tamper-resistant, but physical
attacks remain possible:

* **Voltage glitching**: Targeted voltage manipulation of the TPM
  chip may cause clock skips or incorrect readings. This requires
  physical access to the TPM and specialized equipment.
* **TPM reset attacks**: An attacker with physical access could
  power-cycle the TPM to reset volatile state. This is detectable
  via resetCount changes.
* **VM-based attacks**: Running the attested computation inside a
  virtual machine where the hypervisor controls the virtual TPM
  (vTPM). vTPMs do not provide the same hardware guarantees as
  discrete TPMs. Verifiers SHOULD require attestation that the
  TPM is a discrete hardware module, not a vTPM, for high-
  assurance scenarios.

## AIK Compromise {#aik-compromise}

If the AIK private key is extracted from the TPM (e.g., via side-
channel attacks or hardware fault injection), the attacker can
forge HAT proofs with arbitrary timestamps. This requires invasive
hardware attacks, typically costing USD 10,000-100,000+ per device.

Verifiers SHOULD monitor for key compromise indicators and support
AIK revocation. The Endorser's certificate chain provides a
revocation path.

## Replay Attacks {#replay-attacks}

Without qualifying data binding, an attacker could replay a
previously captured HAT proof to attest a different computation.
Applications SHOULD include computation-specific data in the
TPM2\_GetTime qualifying data field to prevent replay.

For multi-invocation chains, the temporal chain continuity check
(T\_before.clock > preceding T\_after.clock) prevents simple
insertion of old HAT proofs into a new chain.

## Clock Drift {#clock-drift}

The TPM clock and the system wall clock may drift relative to each
other. The TPM specification does not mandate a specific clock
accuracy, though most implementations maintain accuracy within
a few seconds per day. Verifiers SHOULD account for reasonable
clock drift when comparing HAT time deltas to expected computation
durations, using a tolerance factor (e.g., 5-10%).

## Scope Limitations {#scope-limitations}

This specification defines HAT exclusively for TPM 2.0 devices.
Other hardware platforms with monotonic clocks (ARM TrustZone
trusted timers, Intel SGX trusted time, Apple Secure Enclave) are
out of scope for this version. Future work may extend HAT to
additional hardware platforms.

# Privacy Considerations {#privacy-considerations}

## Device Identification {#device-identification}

TPM Endorsement Keys (EKs) and AIKs are device-specific and
potentially device-identifying. A HAT proof that includes the AIK
public key or certificate chain allows the Verifier (and any party
who obtains the Evidence) to correlate multiple computations to the
same device.

To mitigate this, implementations SHOULD use privacy-preserving
AIK provisioning mechanisms:

* **Privacy CA**: A Privacy Certificate Authority issues AIK
  certificates that attest to the AIK's validity without
  revealing the EK, preventing cross-Verifier correlation.
* **Direct Anonymous Attestation (DAA)**: DAA protocols allow
  the Attester to prove TPM authenticity without revealing a
  persistent device identifier.

## Uptime Leakage {#uptime-leakage}

TPM clock readings reveal the platform's uptime since the last
reset. This is a fingerprinting vector that can distinguish
devices and track platform usage patterns.

Applications SHOULD consider whether uptime information is
sensitive in their deployment context. When privacy is critical,
the consuming protocol MAY strip or redact the absolute clock
values from the HAT proof before conveying it, retaining only the
time delta.

## Data Minimization {#data-minimization}

The TPMS\_ATTEST structure contains fields beyond what HAT
verification requires (e.g., firmwareVersion, qualifiedSigner).
Applications SHOULD consider whether conveying these fields is
necessary. When not required, the consuming protocol MAY define a
reduced attestation format that includes only the fields needed
for HAT verification (clock values, resetCount, safe flag, and
the AIK signature over the full structure).

# IANA Considerations {#iana-considerations}

This document has no IANA actions.

Consuming protocols that embed HAT proofs in their Evidence
structures handle their own IANA registrations for the embedding
mechanism (e.g., CBOR map keys, media types).

--- back

# Acknowledgements {#acknowledgements}
{:numbered="false"}

HAT was originally specified as part of the Cryptographic Proof
of Effort (CPoE) protocol. The author thanks the Trusted
Computing Group for the TPM 2.0 specification and the
participants of the IETF RATS working group for the remote
attestation architecture that informed this design.
