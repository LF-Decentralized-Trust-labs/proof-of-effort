[//]: # (SPDX-License-Identifier: Apache-2.0)

# Architecture

This document explains how CPoP maps onto the IETF RATS architecture
([RFC 9334](https://www.rfc-editor.org/rfc/rfc9334)) and where it diverges
from conventional remote attestation.

## The Trust Inversion

In traditional RATS deployments the adversary is external — malware, network
attackers, supply-chain compromise. The Attester operator is trusted.

CPoP inverts this. The Attester is the author, and the author is the party
whose claims need verification. The primary adversary *is* the Attester
operator. This shapes every design decision:

- Evidence must be unforgeable by the entity generating it.
- Temporal claims are bound to physical constraints (timing jitter,
  memory-hard functions) the Attester cannot cheaply circumvent.
- Behavioral entropy must be computationally expensive to simulate.
- Hardware trust anchors (TPM, Secure Enclave) only help when they are
  genuinely inaccessible to the operator.

RFC 9334 Section 3 defines Attester roles functionally without requiring the
operator to be trusted, so this inversion is architecturally valid. CPoP is
essentially a RATS profile where the standard Attester-operator trust
assumption is explicitly relaxed — analogous to anti-cheat or DRM attestation
patterns.

## RATS Role Mapping

```
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
```

The roles, per the [RATS Entity Roles](../draft-condrey-cpop-protocol.md#rats-entity-roles) section of the protocol draft:

**Attester / Attesting Environment (AE)**
: The authoring application, OS entropy interfaces, and hardware Secure
Elements when available. Unlike conventional RATS, the Attester is operated
by the entity whose claims are being verified.

**Verifier**
: Receives Evidence Packets, evaluates chain integrity, temporal ordering,
behavioral entropy, and content binding per the appraisal procedures in
`draft-condrey-cpop-appraisal`.

**Relying Party**
: Consumes the Attestation Result to make trust decisions — publishers,
content platforms, credential issuers, regulatory systems.

**Endorser**
: Hardware manufacturers issuing TPM endorsement certificates and platform
attestation credentials (relevant at T3/T4 tiers only).

**Reference Value Provider**
: Two forms in CPoP. First, the specification itself (SWF parameters, forensic
thresholds). Second, per-author behavioral baselines accumulated across
sessions. Baselines are self-asserted and MUST NOT be treated as trusted
Reference Values — they are corroborative, not authoritative.

**Target Environment**
: The document editor session and its content state. Measured by the AE at
each checkpoint via content hashing and behavioral telemetry capture.

## Passport Model

CPoP follows the RATS passport model (RFC 9334 §8.1). The Evidence Packet
travels with the content as a self-contained credential:

1. The Attester collects behavioral telemetry during content creation and
   generates an Evidence Packet (`.cpop`) containing SWF proofs, jitter
   bindings, and physical state markers.
2. The Verifier appraises chain integrity, temporal ordering, behavioral
   entropy, and content binding.
3. The Verifier produces a Writers Authenticity Report (`.cwar`) with forensic
   scores and forgery cost estimates.
4. The Relying Party consumes the result for trust decisions.

In some deployments, the Relying Party receives the Evidence Packet alongside
content and forwards it to a Verifier. This resembles the background-check
model (RFC 9334 §8.2) operationally, but the packet still originates from
the Attester and travels with the content, keeping it architecturally
consistent with passport model semantics.

## Evidence Format

CPoP Evidence Packets use a domain-specific CBOR structure (tag `1129336656` /
`CPOP`) rather than CWT/JWT-wrapped EAT tokens. The reasons are structural:

- EAT ([RFC 9711](https://www.rfc-editor.org/rfc/rfc9711)) models entity
  state as claims at a point in time. CPoP Evidence models a *continuous
  physical process* as an ordered sequence of checkpoints with SWF proofs,
  behavioral entropy, and physical state bindings.
- The checkpoint chain (sequential process-proofs, hash-linked deltas,
  jitter bindings) has no natural representation in the EAT claims model.
- Wrapping each checkpoint as a separate EAT would break the cryptographic
  chain integrity that is central to CPoP's security properties.

The Writers Authenticity Report (tag `1129791826` / `CWAR`) *is* EAT-compatible. It
carries standard EAT claims (verdict, tier, forensic assessments) and
implements the EAR compatibility mapping defined in the appraisal draft.
Generic EAT tooling can consume WARs but cannot parse Evidence Packets
without CPoP-specific support.

## Attestation Tiers

The protocol defines four deployment tiers with increasing security properties:

| Tier | Name | Trust Anchor | Characteristics |
| ---- | ---- | ------------ | --------------- |
| T1 | Software-Only | None | SWF proofs + behavioral entropy. No hardware binding. Lowest forgery cost. |
| T2 | Attested Software | Software key | Signed evidence with key bound to author identity. Key is software-controlled. |
| T3 | Hardware-Bound | TPM/SE | Evidence signing key held in hardware. Attester cannot extract the key. |
| T4 | Hardware-Hardened | TPM/SE + sealed | Hardware-bound keys with sealed storage and platform attestation. Highest forgery cost. |

Verifiers assess evidence against the claimed tier. Higher tiers impose
greater constraints on the Attester and raise the cost of forgery, but
require hardware support that may not be universally available.

## Wire Format

The formal data model is specified in CDDL at
[`cddl/cpop.cddl`](../cddl/cpop.cddl):

| Structure            | CBOR Tag       | Mnemonic | Direction         |
| -------------------- | -------------- | -------- | ----------------- |
| `evidence-packet`    | `1129336656`   | `CPOP`   | Attester → Verifier |
| `attestation-result` | `1129791826`   | `CWAR`   | Verifier → RP     |

Integer-keyed maps throughout. Timestamps in milliseconds, entropy in
centibits (1/100 bit).

## Further Reading

- [`draft-condrey-cpop-protocol`](../draft-condrey-cpop-protocol.md) —
  Protocol specification, threat model, attestation tiers, wire format.
- [`draft-condrey-cpop-appraisal`](../draft-condrey-cpop-appraisal.md) —
  Verifier appraisal logic, forensic scoring, security model.
- [RFC 9334](https://www.rfc-editor.org/rfc/rfc9334) — RATS Architecture.
- [RFC 9711](https://www.rfc-editor.org/rfc/rfc9711) — Entity Attestation Token (EAT).
- [RFC 8610](https://www.rfc-editor.org/rfc/rfc8610) — CDDL specification.
