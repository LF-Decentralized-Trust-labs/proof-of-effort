---
v: 3
docname: draft-condrey-hat-00
date: 2026-05
title: "Hardware Attestation of Time (HAT): TPM-Based Temporal Binding for Remote Attestation"
abbrev: HAT
category: info
ipr: trust200902
submissiontype: independent
area: Security
workgroup: Individual Submission
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
  I-D.birkholz-rats-tuda:
    title: "Time-Based Uni-Directional Attestation"
    author:
      - fullname: Henk Birkholz
        initials: H.
        surname: Birkholz
    date: 2023
    seriesinfo:
      Internet-Draft: draft-birkholz-rats-tuda-07
---

--- abstract

This document specifies Hardware Attestation of Time (HAT), a
mechanism for binding computation duration to TPM 2.0 hardware-
attested time. HAT obtains TPM2\_GetTime() readings before and
after a computation, both signed by an Attestation Identity Key
(AIK), producing a "temporal sandwich" that proves minimum elapsed
time even when the platform operator is adversarial. This document
defines the proof construction, wire format, and verification
procedure, and maps HAT to the RATS architecture (RFC 9334).

--- middle

# Introduction {#introduction}

## The TPM Temporal Attestation Gap {#temporal-gap}

Remote attestation protocols verify temporal claims: that a
computation took at least a certain duration, that events occurred
in order, or that a process was active during a claimed interval.
Software timestamps are trivially forgeable. {{RFC3161}} Timestamp
Authorities provide trusted timestamps but require network
connectivity and attest only to signing time, not computation
duration.

TPM 2.0 devices contain a monotonic hardware clock that cannot be
rolled back without triggering detectable state changes. The
TPM2\_GetTime command {{TPM2-Commands}} returns the current clock
value in a signed attestation structure. No existing specification
standardizes paired TPM2\_GetTime readings for computation duration
proof.

## HAT Solution {#hat-solution}

HAT defines a "temporal sandwich": the Attester calls TPM2\_GetTime
before a computation, executes it, then calls TPM2\_GetTime after.
Both attestations are signed by the same AIK. The Verifier validates
signatures, clock continuity, and elapsed time.

HAT proofs can optionally bind the computation's input to T\_before,
preventing pre-computation attacks.

HAT is designed for sequential computation verification systems and
other scenarios where the platform operator is adversarial.

## Related Work {#related-work}

TUDA {{I-D.birkholz-rats-tuda}} specified TPM-based temporal
attestation using Time Stamp Authorities (TSA) and a Sync Base
Protocol. HAT differs in requiring no external TSA, focusing on
computation duration rather than audit logging, and using
resetCount/clock-safe validation rather than TSA synchronization.

# Conventions and Definitions

{::boilerplate bcp14-tagged}

AIK:
: Attestation Identity Key. A restricted signing key in the TPM's
  Endorsement hierarchy, used to sign attestation structures.

TPMS\_TIME\_ATTEST\_INFO:
: TPM 2.0 attestation structure returned by TPM2\_GetTime,
  containing clock value, reset count, and clock-safe flag,
  per {{TPM2-Structures}}.

Temporal sandwich:
: Paired TPM time readings bounding a computation's duration.

# Architecture {#architecture}

## RATS Entity Roles {#rats-roles}

HAT maps to RATS entity roles {{RFC9334}} as follows:

Attester / Attesting Environment:
: The platform performing the attested computation. Generates HAT
  proofs by calling TPM2\_GetTime before and after each computation.

Endorser:
: TPM manufacturers issuing EK certificates and platform
  attestation credentials. The certificate chain establishes AIK
  binding to genuine TPM hardware.

Verifier:
: Receives HAT proofs, validates AIK signatures against the
  Endorser's certificate chain, and checks temporal consistency.

Relying Party:
: Consumes Attestation Results per {{RFC9334}}.

Reference Value Provider:
: Supplies the expected computation duration and clock tolerance
  parameters that the Verifier uses to evaluate HAT time deltas.
  The Reference Value Provider conveys these values via one of:
  (a) the consuming protocol's Evidence structure (e.g., declared
  parameters from which duration is derived),
  (b) an out-of-band policy configuration on the Verifier, or
  (c) a CoRIM reference values manifest per draft-ietf-rats-corim.
  The specific conveyance mechanism is defined by the consuming
  protocol.

## HAT in the Attestation Flow {#attestation-flow}

HAT operates within the RATS passport model ({{RFC9334}},
Section 8.1):

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

## Trust Model {#trust-model}

HAT implements a trust inversion: the Attester operator is the
primary adversary, analogous to anti-cheat attestation. The trust
anchors are:

* The TPM hardware, providing a monotonic clock resistant to
  rollback.
* The AIK, a restricted signing key bound to the TPM.
* The Endorser's certificate chain, binding the AIK to genuine
  hardware.

The Attester operator controls the software stack and can choose
when to call TPM2\_GetTime, but cannot forge clock readings or AIK
signatures.

# TPM Procedure {#tpm-procedure}

## TPM2\_GetTime() Bracketing {#tpm-bracketing}

The Attester MUST obtain TPM-attested time readings before and
after each computation:

~~~ pseudocode
T_before = TPM2_GetTime(aikHandle, qualifyingData_before)

... execute computation ...

T_after  = TPM2_GetTime(aikHandle, qualifyingData_after)
~~~

The `aikHandle` MUST refer to the same AIK for both calls.

Each TPM2\_GetTime call returns a TPMS\_ATTEST structure
{{TPM2-Structures}} containing:

* `clockInfo.clock`: Monotonic millisecond counter. Does not reset
  on reboot; only on owner-clear (which increments resetCount).
* `clockInfo.resetCount`: Increments on TPM2\_Startup(CLEAR).
* `clockInfo.restartCount`: Increments on TPM2\_Startup(STATE)
  (hibernation/resume).
* `clockInfo.safe`: Indicates clock reliability. Set to NO on
  clock discontinuity.
* `firmwareVersion`: TPM firmware version.

Applications SHOULD include qualifying data binding the HAT proof
to the computation. Use cases where replay is not a threat (e.g.,
public timestamping) may omit qualifying data.

* T\_before qualifyingData: computation input seed or hash thereof.
* T\_after qualifyingData: computation output or commitment (e.g.,
  Merkle root).

If the TPM is unavailable, the Attester MUST NOT generate a HAT
proof.

The Attester SHOULD minimize delay between T\_before and computation
start, and between computation end and T\_after.

## AIK Signature Requirements {#aik-requirements}

Key Attributes:
: The AIK MUST be a restricted signing key with
  `TPMA_OBJECT.restricted` and `TPMA_OBJECT.sign` set per
  {{TPM2-Structures}}.

Key Hierarchy:
: The AIK SHOULD reside in the Endorsement hierarchy. AIKs in the
  Platform or Storage hierarchies MAY be used but offer weaker
  provenance.

Certificate Chain:
: The AIK certificate chain MUST root to a known TPM
  manufacturer's EK certificate. The Verifier MUST validate the
  full chain before accepting HAT proofs.

Signature Scheme:
: The AIK MUST use one of:

  * RSASSA-PKCS1-v1\_5 with SHA-256
  * RSAPSS with SHA-256
  * ECDSA with P-256

  The Verifier MUST support at least RSASSA-PKCS1-v1\_5 with
  SHA-256 and ECDSA with P-256.

Key Conveyance:
: The AIK public key and certificate chain MUST be conveyed to the
  Verifier, either out-of-band or inline per the consuming
  protocol.

Discrete TPM:
: For high-assurance scenarios, Verifiers SHOULD require
  attestation that the TPM is a discrete hardware module, not a
  vTPM.

## Clock-Safe and Reset Semantics {#clock-safe}

The `clockInfo.safe` field indicates clock reliability. The TPM
sets `safe` to NO on clock discontinuity:

* Platform power loss with disturbed or absent battery-backed RTC.
* Hardware anomaly affecting clock integrity.
* Explicit TPM2\_ClockSet (typically restricted to platform
  hierarchy).

A resetCount mismatch between T\_before and T\_after indicates a
platform reboot during computation. A reboot resets volatile TPM
state and allows the operator to manipulate the computation
environment. A restartCount mismatch indicates hibernation/resume,
which preserves the clock but allows state inspection.

For multi-invocation chains, a resetCount change between T\_after
of invocation n-1 and T\_before of invocation n breaks temporal
chain continuity.

# Wire Format {#wire-format}

## hat-proof CDDL {#hat-cddl}

The HAT proof is encoded in CBOR {{RFC8949}} per {{RFC8610}}:

~~~ cddl
hat-proof = {
    1 => bstr,     ; time-before (TPMS_TIME_ATTEST_INFO)
    2 => bstr,     ; time-after (TPMS_TIME_ATTEST_INFO)
    3 => bstr,     ; sig-before (AIK signature)
    4 => bstr,     ; sig-after (AIK signature)
}
~~~

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

The raw TPMS\_ATTEST structure from TPM2\_GetTime before the
computation, marshaled as TPM2B\_ATTEST per {{TPM2-Structures}}.
MUST have `magic` = TPM\_GENERATED\_VALUE (0xff544347) and
`type` = TPM\_ST\_ATTEST\_TIME (0x8019).

### time-after (Key 2) {#field-time-after}

Same format as time-before, from the post-computation
TPM2\_GetTime call.

### sig-before (Key 3) {#field-sig-before}

The TPMT\_SIGNATURE over the time-before TPMS\_ATTEST structure.
For ECDSA: concatenation of r and s values per TPM2 Part 2.
For RSA: the raw signature value.

### sig-after (Key 4) {#field-sig-after}

Same format as sig-before, covering the time-after attestation.

## Embedding in Consuming Protocols {#embedding}

The consuming protocol defines:

* The position within its Evidence structure for the HAT proof.
* AIK conveyance mechanism.
* Computation binding (e.g., incorporating T\_before into seed
  derivation).

For standalone use, a HAT proof MAY be wrapped in a COSE\_Sign1
envelope {{RFC9052}} with the AIK certificate chain in the
unprotected header.

# Verification {#verification}

## Verification Procedure {#verification-procedure}

The Verifier MUST perform the following checks:

1. **Signature verification**: Verify sig-before and sig-after
   against the AIK public key. Validate the AIK certificate chain
   to a known TPM manufacturer root. Reject on failure.

2. **Attestation type check**: Verify both TPMS\_ATTEST structures
   have `type` = TPM\_ST\_ATTEST\_TIME and `magic` =
   TPM\_GENERATED\_VALUE.

3. **resetCount consistency**: Verify resetCount is identical in
   T\_before and T\_after. Reject on mismatch.

4. **Clock-safe validation**: Verify T\_before.clockInfo.safe is
   true; MUST reject if false. If T\_after.clockInfo.safe is false,
   SHOULD reject.

5. **Time delta check**: delta = T\_after.clock - T\_before.clock
   MUST be >= the expected computation duration obtained from the
   Reference Value Provider ({{rats-roles}}). SHOULD flag deltas
   exceeding a configurable upper threshold (default: 10x expected
   duration).

6. **Temporal chain continuity** (multi-invocation): T\_before.clock
   of invocation n MUST be strictly greater than T\_after.clock of
   invocation n-1.

7. **Computation binding** (application-specific): If the consuming
   protocol binds the computation seed to T\_before, verify this
   binding.

NOTE: HAT proves a lower bound on computation duration (the TPM
clock interval). It does not prove a tight bound; the Attester may
introduce delays between T\_before/T\_after and the actual
computation.

## Error Handling {#error-handling}

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
| Time delta implausibly large (>threshold) | SHOULD flag as warning |
| Temporal chain continuity violation | MUST reject |
| firmwareVersion mismatch between readings | SHOULD reject |

Warnings SHOULD be included in the Attestation Result.

# Security Considerations {#security-considerations}

## TPM Clock Manipulation {#clock-manipulation}

* **Voltage glitching**: Targeted voltage manipulation may cause
  clock skips. Requires physical access and specialized equipment.
* **TPM reset attacks**: Power-cycling resets volatile state,
  detectable via resetCount.
* **VM-based attacks**: vTPMs do not provide discrete TPM hardware
  guarantees. See {{aik-requirements}} for discrete TPM
  requirements.

## AIK Compromise {#aik-compromise}

AIK extraction (via side-channel or fault injection) enables
forging HAT proofs with arbitrary timestamps. Verifiers SHOULD
support AIK revocation.

## Replay Attacks {#replay-attacks}

Without qualifying data binding, HAT proofs can be replayed for
different computations. Applications SHOULD include
computation-specific qualifying data. For multi-invocation chains,
temporal chain continuity prevents insertion of old proofs.

## Clock Drift and Resolution {#clock-drift}

TPM clocks lack mandated accuracy; typical drift is seconds per
day. Verifiers SHOULD use a tolerance factor (5-10%) when comparing
deltas to expected durations.

The TPM 2.0 specification defines `clockInfo.clock` as a
millisecond counter, but actual update granularity is
implementation-dependent. Some TPMs update the clock at intervals
of 10ms or coarser. For computations shorter than approximately
100ms, clock resolution may dominate the measurement error. HAT
is best suited for computations lasting at least one second.

## Scope Limitations {#scope-limitations}

This specification covers TPM 2.0 only. Other hardware platforms
(ARM TrustZone, Intel SGX, Apple Secure Enclave) are out of scope.

# Privacy Considerations {#privacy-considerations}

## Device Identification {#device-identification}

AIKs are device-specific and potentially identifying. To mitigate
correlation:

* **Privacy CA**: Issues AIK certificates without revealing the EK.
* **Direct Anonymous Attestation (DAA)**: Proves TPM authenticity
  without persistent identifiers.

## Uptime Leakage {#uptime-leakage}

TPM clock readings reveal platform uptime, a fingerprinting vector.
When uptime exposure is unacceptable, the consuming protocol MAY
strip absolute clock values, retaining only the time delta.

## Data Minimization {#data-minimization}

The TPMS\_ATTEST structure contains fields beyond what HAT
verification requires (firmwareVersion, qualifiedSigner). The
consuming protocol MAY define a reduced format including only
clock values, resetCount, safe flag, and the AIK signature.

# IANA Considerations {#iana-considerations}

This document has no IANA actions.

--- back

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The author thanks the Trusted Computing Group for the TPM 2.0
specification and the IETF RATS working group for the remote
attestation architecture that informed this design.
