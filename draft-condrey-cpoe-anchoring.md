---
v: 3
docname: draft-condrey-cpoe-anchoring-00
title: "Cryptographic Proof of Effort (CPoE): Temporal Anchoring Extensions"
abbrev: CPoE Anchoring
category: exp
ipr: trust200902
submissiontype: independent
area: Security
workgroup: Individual Submission
keyword:
  - attestation
  - temporal anchoring
  - witness
  - beacon
  - process evidence

date: 2026-05
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
  RFC5869:
  RFC8949:
  CPoE-Protocol:
    title: "Cryptographic Proof of Effort (CPoE): Architecture and Evidence Format"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-05
    seriesinfo:
      Internet-Draft: draft-condrey-cpoe-protocol-00

informative:
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

--- abstract

This document specifies three temporal anchoring extensions for
the Cryptographic Proof of Effort (CPoE) protocol: beacon-anchored
binding using public randomness beacons, witness-anchored nonce
binding using interactive nonce services, and verifier-nonce
binding for supervised authoring. These mechanisms bind CPoE
checkpoints to external time sources with graduated assurance
levels.

--- middle

# Introduction

The Cryptographic Proof of Effort (CPoE) protocol
{{CPoE-Protocol}} defines an evidence format for proving that a
human invested sustained cognitive effort in producing a document.
The core protocol binds checkpoints to each other via hash chains
and to computational effort via a Sequential Work Function (SWF).
However, the core protocol alone cannot bind checkpoints to
external wall-clock time: a dishonest Attester can pre-compute an
entire checkpoint chain and present it later.

This document specifies three temporal anchoring extensions that
address this gap. These extensions form a hierarchy from passive
beacon anchoring through interactive witness nonces to
verifier-controlled nonces, providing increasing levels of
temporal assurance:

- **Beacon-anchored binding** incorporates publicly verifiable
  randomness from external beacon services, proving that a
  checkpoint was created after a specific beacon round.

- **Witness-anchored nonce binding** uses an interactive nonce
  service to bind each checkpoint to a fresh, signed nonce,
  proving real-time creation within seconds.

- **Verifier-nonce binding** allows a supervising Verifier to
  inject challenge nonces into the checkpoint chain, providing
  the strongest anti-precomputation guarantee.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED",
"MAY", and "OPTIONAL" in this document are to be interpreted as
described in BCP 14 {{!RFC2119}} {{!RFC8174}} when, and only when,
they appear in all capitals, as shown here.

# Anchoring Hierarchy {#anchoring-hierarchy}

The three temporal anchoring mechanisms defined in this document
bind checkpoints to external time sources with increasing
assurance:

| Mechanism | Key | Online? | Freshness | Anti-precompute |
|-----------|-----|---------|-----------|-----------------|
| Beacon-anchor | 16 | No (passive) | Minutes | Commit-before-beacon |
| Witness-anchor | 18 | Yes (1 RTT) | Seconds | Signed nonce |
| Verifier-nonce | 17 | Yes (2 RTT) | Immediate | Challenge-response |

An Evidence Packet MAY include any combination of these mechanisms.
When multiple anchoring mechanisms are present, the Verifier
SHOULD use the strongest available: verifier-nonce >
witness-anchor > beacon-anchor. Attesters claiming T2 or higher
assurance SHOULD include a witness-anchor in every checkpoint.
The Evidence Packet MUST NOT claim a tier higher than T1 if any
checkpoint lacks a valid witness-anchor.

# Beacon-Anchored Binding (Optional) {#beacon-binding}

Beacon-anchored binding incorporates externally verifiable
randomness from a public randomness beacon (such as the NIST
Randomness Beacon or drand) into the checkpoint seed derivation.
This establishes a commit-before-beacon temporal ordering: the
Attester commits to checkpoint content via the hash chain before
the beacon value becomes available, then incorporates the beacon
value into the SWF seed for the subsequent checkpoint.

This mechanism is inspired by the temporal ordering properties
described in {{HaberStornetta1991}}, adapted from document
timestamping to process evidence.

## Beacon Integration {#beacon-integration}

The Attester MUST perform beacon integration as follows:

1. Compute the checkpoint content-hash and prev-hash, committing
   to the checkpoint content before the beacon value is known.

2. After the beacon value for the target round becomes available,
   derive the beacon contribution to the SWF seed using HKDF
   {{RFC5869}}:

~~~ pseudocode
beacon-key = HKDF-Expand(
    HKDF-Extract(
        salt = "CPoE-Beacon-v1",
        IKM  = beacon-value
    ),
    info = "CPoE-Beacon-Binding" || prev-hash,
    L    = 32
)
~~~

3. Incorporate `beacon-key` into the SWF seed derivation as
   specified in {{CPoE-Protocol}}, Section 5.4. The beacon
   contribution is concatenated after prev-hash (or after
   verifier-nonce, if present) and before any behavioral data
   in the seed.

## CDDL Definition {#beacon-cddl}

The beacon-anchor structure is carried in checkpoint key 16:

~~~ cddl
beacon-anchor = {
    1 => tstr,              ; beacon-source (URI identifying the
                            ;   beacon service, e.g.,
                            ;   "https://drand.cloudflare.com")
    2 => uint,              ; beacon-round (round number)
    3 => bstr .size 32,     ; beacon-value (the randomness output)
}
~~~

## Verification Requirements {#beacon-verification}

Verifiers MUST perform the following checks on beacon-anchored
checkpoints:

1. Retrieve the beacon value for the claimed round from the
   identified beacon service (or a trusted mirror).

2. Verify that the beacon-value in the Evidence matches the
   retrieved value exactly.

3. Verify that the beacon round timestamp is consistent with
   the checkpoint timestamp (allowing for clock skew of up to
   60 seconds).

4. Verify that the checkpoint content-hash was committed (via
   prev-hash in the hash chain) before the beacon round became
   available. The commit timestamp (previous checkpoint) MUST
   precede the beacon round publication time.

5. Recompute the HKDF-derived beacon-key and verify that the
   SWF seed incorporates it correctly.

Verifiers SHOULD cache beacon values to avoid repeated lookups.
If the beacon service is unavailable, the Verifier SHOULD treat
the beacon-anchor as absent rather than rejecting the Evidence
Packet entirely; see {{security-considerations}}.

# Witness-Anchored Nonce Binding {#witness-anchor-binding}

Witness-anchored nonce binding uses an interactive protocol with
an independent witness service to bind each checkpoint to a fresh,
cryptographically signed nonce. This provides second-granularity
temporal assurance and real-time ordering guarantees.

## Witness Service {#witness-service}

A witness service is a stateless HTTPS endpoint that issues
fresh, signed nonces on demand. The witness service:

- Maintains an Ed25519 signing key pair.
- Issues nonces containing the current timestamp and a
  monotonically increasing sequence number.
- Signs each nonce response, enabling offline verification.
- Does not need to store state between requests (nonce
  generation is stateless).

The witness service is declared in the Evidence Packet at key 20:

~~~ cddl
witness-service = {
    1 => tstr,              ; witness-uri (HTTPS endpoint)
    2 => bstr .size 32,     ; witness-public-key (Ed25519)
}
~~~

The witness-anchor structure is carried in each checkpoint at
key 18:

~~~ cddl
witness-anchor = {
    1 => bstr .size 32,     ; nonce (random value from witness)
    2 => uint,              ; witness-timestamp (Unix epoch
                            ;   seconds from the witness service)
    3 => uint,              ; witness-sequence (monotonic counter)
    4 => bstr .size 64,     ; witness-signature (Ed25519 over
                            ;   the nonce response)
}
~~~

## Protocol {#witness-anchor-protocol}

The witness nonce request-response protocol operates as follows:

Step 1: The Attester sends a nonce request to the witness
service, including the session-id and current checkpoint
sequence number:

~~~ http
POST /nonce HTTP/1.1
Host: witness.example.com
Content-Type: application/cbor

{
    "session-id": "<session-uuid>",
    "checkpoint-seq": 5
}
~~~

Step 2: The witness service generates a fresh 32-byte random
nonce, records the current timestamp and sequence counter, and
constructs the signed response:

~~~ pseudocode
nonce = random(32)
timestamp = current_unix_epoch_seconds()
sequence = next_monotonic_counter()
~~~

Step 3: The witness service computes an Ed25519 signature over
the canonical response encoding:

~~~ pseudocode
sign-input = CBOR-encode({
    1 => nonce,
    2 => timestamp,
    3 => sequence,
    4 => session-id,
    5 => checkpoint-seq
})

signature = Ed25519-Sign(witness-private-key, sign-input)
~~~

Step 4: The witness service returns the signed nonce response:

~~~ http
HTTP/1.1 200 OK
Content-Type: application/cbor

{
    "nonce": "<32 bytes>",
    "timestamp": 1748131200,
    "sequence": 928471,
    "signature": "<64 bytes>"
}
~~~

Step 5: The Attester incorporates the witness nonce into the
SWF seed derivation. The nonce is concatenated after prev-hash
and before any beacon-anchor value or behavioral data:

~~~ pseudocode
seed = H(
    "CPoE-SWF-Seed-v1" ||
    I2OSP(mode, 1) ||
    prev-hash ||
    witness-nonce ||
    [beacon-anchor if present] ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state) ||
    [edit-graph-hash if present]
)
~~~

The Attester stores the complete witness-anchor structure in the
checkpoint at key 18.

## Witness Nonce Expiry {#witness-nonce-expiry}

Each witness nonce has a time-to-live (TTL) of 120 seconds from
the witness-timestamp. The Attester MUST incorporate the nonce
into a checkpoint within this window.

Verifiers MUST reject a witness-anchor where the checkpoint
timestamp exceeds the witness-timestamp by more than 120 seconds.
To accommodate clock skew between the Attester and witness
service, Verifiers SHOULD allow a tolerance of up to 5 seconds
for the checkpoint timestamp preceding the witness-timestamp
(i.e., the checkpoint timestamp MAY be up to 5 seconds before
the witness-timestamp due to clock differences).

If the Attester cannot reach the witness service within the
checkpoint interval, the Attester SHOULD proceed without a
witness-anchor for that checkpoint. Evidence Packets containing
checkpoints without witness-anchors MUST NOT claim T2 or higher
assurance.

## Witness Duplicate and Ordering Checks {#witness-ordering}

Verifiers MUST enforce monotonic ordering of witness-anchor
fields within an Evidence Packet:

1. The witness-sequence values MUST be strictly monotonically
   increasing across consecutive checkpoints that contain
   witness-anchors.

2. The witness-timestamp values MUST be monotonically
   non-decreasing across consecutive checkpoints.

3. No two checkpoints within the same Evidence Packet MAY
   contain the same nonce value.

4. The witness-sequence gap between consecutive witness-anchored
   checkpoints SHOULD be 1. Gaps greater than 1 indicate that
   the witness service issued nonces to other sessions between
   the Attester's requests, which is expected in multi-tenant
   deployments. Gaps of 0 or negative values MUST cause
   rejection.

## Security Properties {#witness-anchor-security}

Witness-anchored nonce binding provides the following security
properties:

Anti-precomputation:
: The Attester cannot precompute SWF proofs before obtaining the
  witness nonce, because the nonce is incorporated into the SWF
  seed. An adversary must wait for each nonce before beginning
  the SWF computation for that checkpoint.

Real-time ordering:
: The witness-timestamp and witness-sequence establish a total
  ordering of nonce issuance across all sessions. A Verifier can
  determine that checkpoint N was created after checkpoint N-1
  with second-granularity precision.

Anti-selective-omission:
: Combined with the witness transparency log
  ({{witness-transparency-log}}), the witness service records the
  number of nonces issued per session. A Verifier can detect if
  the Attester omitted trailing checkpoints by comparing the
  Evidence Packet's checkpoint count against the witness log's
  recorded nonce count.

Graceful degradation:
: If the witness service is temporarily unavailable, the Attester
  can continue producing checkpoints without witness-anchors. The
  resulting Evidence degrades to T1 assurance for the affected
  checkpoints but remains valid.

Complement to SWF:
: Witness nonces and SWF proofs provide independent temporal
  guarantees. SWF proves minimum computation time per checkpoint;
  witness nonces prove real-time existence at nonce issuance.
  Together, they establish both that the work was done and when
  it was done.

## Witness-Anchor Relationships {#witness-anchor-relationships}

The witness-anchor mechanism complements other CPoE components:

- **SWF proofs** establish minimum computation time; witness
  nonces establish wall-clock time of creation.
- **Beacon anchors** provide coarse-grained (minutes) temporal
  binding; witness anchors provide fine-grained (seconds)
  binding.
- **HAT proofs** ({{CPoE-Protocol}}) bind time to hardware
  attestation; witness anchors bind time to an independent
  third-party service.

Deployments requiring high assurance SHOULD combine witness
anchoring with at least one additional temporal mechanism.

## Witness Transparency Log {#witness-transparency-log}

Witness services SHOULD maintain an append-only transparency log
of all nonce issuance events. Each log entry records:

~~~ cddl
witness-log-entry = {
    1 => uint,              ; witness-sequence
    2 => uint,              ; witness-timestamp
    3 => bstr .size 32,     ; nonce-hash (H(nonce))
    4 => bstr .size 32,     ; session-id-pseudonym
                            ;   (H("WitnessLog-v1" || session-id))
    5 => uint,              ; checkpoint-seq
}
~~~

The session-id-pseudonym is derived by hashing the session-id
with a domain separation tag, preventing the log from revealing
raw session identifiers while still enabling correlation of
entries belonging to the same session.

Verifiers MAY request inclusion proofs from the transparency log
to verify that:

1. The witness nonce was genuinely issued by the witness service
   (not fabricated by the Attester).

2. The total number of nonces issued for a session matches the
   number of witness-anchored checkpoints in the Evidence Packet
   (anti-selective-omission).

3. The chronological ordering of nonce issuance is consistent
   with the checkpoint ordering in the Evidence.

The transparency log is OPTIONAL. When available, it provides
the strongest anti-selective-omission guarantee. When
unavailable, Verifiers fall back to signature verification of
individual witness-anchors.

# Verifier-Nonce Binding (Optional) {#verifier-nonce-binding}

Verifier-nonce binding allows a supervising Verifier to inject
challenge nonces directly into the Attester's checkpoint chain.
This provides the strongest temporal binding: the Verifier
controls when nonces are issued and can verify that each
checkpoint incorporates the most recent challenge.

This mechanism is appropriate for supervised authoring scenarios
such as proctored examinations, editorial workflows, or
contractual writing engagements where a Verifier is actively
monitoring the authoring session.

## Protocol {#verifier-nonce-protocol}

The verifier-nonce protocol operates as follows:

Step 1: The Verifier generates a fresh 32-byte random nonce and
sends it to the Attester over a secure channel:

~~~ pseudocode
verifier-nonce = random(32)
~~~

Step 2: The Attester acknowledges receipt and begins checkpoint
computation.

Step 3: The Attester incorporates the verifier-nonce into the
SWF seed derivation. The verifier-nonce is concatenated
immediately after prev-hash, before any witness-anchor or
beacon-anchor value:

~~~ pseudocode
seed = H(
    "CPoE-SWF-Seed-v1" ||
    I2OSP(mode, 1) ||
    prev-hash ||
    verifier-nonce ||
    [witness-nonce if present] ||
    [beacon-anchor if present] ||
    CBOR-encode(jitter-binding.intervals) ||
    CBOR-encode(physical-state) ||
    [edit-graph-hash if present]
)
~~~

Step 4: The Attester stores the verifier-nonce in the checkpoint
at key 17 (as a 32-byte bstr).

Step 5: The Verifier issues a new nonce for the next checkpoint
interval, creating a request-response dependency chain: each
checkpoint depends on the Verifier's most recent nonce, and the
Verifier does not issue the next nonce until the previous
checkpoint is received or the interval expires.

The concatenation order when multiple anchoring mechanisms are
present MUST be: prev-hash || verifier-nonce || witness-nonce ||
beacon-anchor || behavioral-data. This ordering ensures that the
strongest anchoring mechanism has the earliest influence on the
seed.

## Security Properties {#verifier-nonce-security}

Verifier-nonce binding provides the following security
properties:

Immediate freshness:
: Each checkpoint is bound to a nonce that the Verifier generated
  at a known time. The Attester cannot precompute any checkpoint
  before receiving the corresponding nonce.

Request-response dependency chain:
: The Verifier gates nonce issuance on receipt of the previous
  checkpoint, creating a strict alternating dependency: Verifier
  issues nonce, Attester produces checkpoint, Verifier issues
  next nonce. This prevents batch precomputation.

Pace verification:
: The Verifier observes the real-time pace of checkpoint
  production, enabling detection of anomalous patterns (e.g.,
  checkpoints produced faster than the SWF minimum duration).

Complementarity with beacon:
: Verifier-nonce and beacon-anchor are independent mechanisms.
  When both are present, the Verifier can verify temporal binding
  via both its own nonce and the public beacon, providing defense
  in depth against a compromised Verifier or an unavailable
  beacon.

# Security Considerations {#security-considerations}

## Beacon Threat Model {#sec-beacon-threats}

Beacon-anchored binding defends against pre-computation attacks
by requiring checkpoint content to be committed before the
beacon value is known. However, the granularity is limited to
the beacon's round interval (typically 30 seconds to several
minutes). An adversary who can predict or influence the beacon
output can defeat this mechanism. Verifiers SHOULD use beacons
with publicly verifiable randomness generation (e.g., threshold
BLS signatures) and SHOULD NOT rely on beacon anchoring as the
sole temporal assurance for T2+ Evidence.

## Witness Threat Model {#sec-witness-threats}

Witness-anchored nonce binding assumes an honest witness service
that issues nonces in real time and does not collude with the
Attester. A compromised witness service can issue backdated or
pre-arranged nonces, defeating the temporal guarantee.
Deployments SHOULD use witness services operated by independent
third parties. The transparency log ({{witness-transparency-log}})
provides a detection mechanism: collusion between the Attester
and witness service is detectable if the log entries are
inconsistent with independently observed wall-clock time.

## Verifier Threat Model {#sec-verifier-threats}

Verifier-nonce binding assumes an honest Verifier. If the
Verifier colludes with the Attester, the Verifier can issue
nonces in advance, defeating the anti-precomputation property.
This is acceptable in the supervised authoring model because the
Verifier is the Relying Party; collusion between Attester and
Relying Party is outside the threat model.

## Graceful Degradation {#sec-degradation}

When temporal anchoring services are unavailable, CPoE Evidence
degrades gracefully:

- If the beacon service is unreachable, the Attester omits the
  beacon-anchor from affected checkpoints. The SWF proof and
  hash chain remain valid.

- If the witness service is unreachable, the Attester omits the
  witness-anchor from affected checkpoints. The Evidence Packet
  MUST NOT claim T2 or higher assurance for these checkpoints.

- If the Verifier cannot issue nonces (network failure), the
  supervised session pauses or continues without verifier-nonce
  binding, depending on local policy.

In all cases, the core CPoE Evidence (hash chain, SWF proofs,
behavioral data) remains valid. Temporal anchoring adds assurance
but is not required for basic evidence integrity.

## Interaction with SWF {#sec-swf-interaction}

All three anchoring mechanisms interact with the SWF by
modifying the seed derivation. This means that temporal anchors
are cryptographically bound to the SWF computation: changing
the anchor value changes the seed, which changes the entire
SWF output chain. An adversary cannot substitute anchor values
after SWF computation without recomputing the full sequential
chain.

This binding is one-directional: the anchor influences the SWF
seed, but the SWF output does not influence the anchor. The
temporal guarantee comes from the anchor; the computational
cost guarantee comes from the SWF. Together, they establish
both when and how long the work was performed.

# IANA Considerations

This document has no IANA actions. The CDDL types defined here
(beacon-anchor, witness-service, witness-anchor,
witness-log-entry) are scoped to the CPoE evidence format
registered in {{CPoE-Protocol}}.

# Privacy Considerations {#privacy-considerations}

Witness services observe the session-id and checkpoint sequence
numbers of every nonce request. This reveals:

- The existence of an authoring session.
- The real-time pace of checkpoint production.
- The approximate duration of the session.

The witness transparency log uses pseudonymized session IDs
(derived via hashing with a domain separation tag) to prevent
direct correlation with Evidence Packets by parties who do not
possess the original session-id.

Verifier-nonce binding reveals authoring pace to the Verifier,
which is inherent in the supervised authoring model. The Verifier
observes when each checkpoint is produced and can infer writing
speed, pause patterns, and session duration. Attesters should be
aware that verifier-nonce binding provides the Verifier with
real-time behavioral telemetry as a side effect of the temporal
binding protocol.

Beacon-anchored binding does not reveal information to the beacon
service, as the Attester only reads public beacon values without
interacting with the service.

--- back

# Acknowledgments
{:numbered="false"}

The temporal anchoring hierarchy builds on foundational work in
digital timestamping by Haber and Stornetta
{{HaberStornetta1991}}.
