---
v: 3
docname: draft-condrey-cfrg-swf-latest
title: "Sequential Work Function (SWF): Memory-Hard Iterative Proofs with Merkle-Sampled Verification"
abbrev: SWF
category: info
ipr: trust200902
submissiontype: IRTF
area: Security
workgroup: Crypto Forum Research Group
keyword:
  - memory-hard
  - sequential work
  - Argon2id
  - Merkle tree
  - Fiat-Shamir

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
  RFC8017:
  RFC8610:
  RFC8949:
  RFC9106:

informative:
  RFC6962:
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
  I-D.bakshi-vdt-verifiable-delay-token:
    title: "Verifiable Delay Tokens for Privacy-Preserving Time Enforcement"
    author:
      - fullname: C. Bakshi
        initials: C.
        surname: Bakshi
    date: 2026
    seriesinfo:
      Internet-Draft: draft-bakshi-vdt-verifiable-delay-token-00
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

This document specifies the Sequential Work Function (SWF), a
cryptographic construction that produces verifiable proofs of sustained
sequential computation. The SWF chains iterated memory-hard function
evaluations (Argon2id) into a Merkle tree and provides probabilistic
verification via Fiat-Shamir-sampled audit proofs. Three algorithm modes
are defined: Mode 20 (iterated Argon2id), Mode 21 (entangled Argon2id
with inter-invocation dependency), and Mode 10 (SHA-256 with periodic
Argon2id waypoints for constrained environments). The construction
provides ASIC resistance through memory-bandwidth bounds and enables
efficient verification in O(k log n) time, where k is the sample count
and n is the step count.

--- middle

# Introduction {#introduction}

Many applications require proof that a party performed sustained
sequential computation over a specified duration. Examples include
proof-of-effort systems, temporal binding protocols, and anti-fraud
mechanisms where an adversary might attempt to compress computation
time using specialized hardware. Existing approaches either lack
ASIC resistance (iterated SHA-256), require trusted setup
(Verifiable Delay Functions with RSA groups), or cannot provide
efficient verification of intermediate states.

Memory-hard functions such as Argon2id {{RFC9106}} resist hardware
acceleration by requiring memory bandwidth proportional to
computation time, bounding the advantage of custom ASICs to the
ratio of available memory bandwidth rather than raw ALU throughput.
However, a single memory-hard evaluation proves only a point-in-time
computation, not sustained sequential work over many steps. Chaining
evaluations naively produces a proof that is expensive to verify:
the Verifier must re-execute the entire chain.

The Sequential Work Function (SWF) addresses both problems. It chains
iterated Argon2id evaluations into a Merkle tree over all intermediate
states, then uses a Fiat-Shamir transform to derive deterministic
sample positions for audit proofs. This yields probabilistic
verification in O(k log n) time where k is the number of sampled
transitions and n is the total step count. The construction is NOT a
Verifiable Delay Function in the formal sense {{Boneh2018}}; it does
not provide efficient public verification of the delay claim from the
output alone. Instead, it provides probabilistic verification of
sequential work with tunable detection probability.

Three algorithm modes are specified. Mode 20 (`swf-argon2id`) uses
iterated Argon2id for maximum ASIC resistance. Mode 21
(`swf-argon2id-entangled`) extends Mode 20 by incorporating the
previous invocation's final state into each new seed, creating strict
inter-invocation sequential dependencies that prevent parallel
pre-computation. Mode 10 (`swf-sha256`) uses a single Argon2id
initialization followed by iterated SHA-256 with periodic memory-hard
waypoints, targeting constrained environments where full Argon2id
iteration is impractical.

## Related Work {#related-work}

### Verifiable Delay Functions

Verifiable Delay Functions (VDFs) {{Boneh2018}} provide efficient
public verification of elapsed time from the output alone, typically
using number-theoretic constructions (RSA groups, class groups).
VDFs offer O(1) verification time but require trusted setup or rely
on unproven hardness assumptions.

The SWF is NOT a VDF. SWF provides probabilistic verification via
Merkle-sampled audit proofs in O(k log n) time, where k is the
sample count. This trade-off (larger proofs, probabilistic
verification) eliminates trusted setup and enables memory-hard ASIC
resistance.

### Verifiable Delay Tokens

Verifiable Delay Tokens (VDT)
{{I-D.bakshi-vdt-verifiable-delay-token}} use VDFs for
privacy-preserving time enforcement in rate limiting and abuse
prevention. VDT targets anonymous token issuance with unlinkability
between issuance and redemption.

SWF differs from VDT in:

- Primitive: Memory-hard Argon2id vs VDF sequential squaring.
- Use case: Authorship attestation vs anonymous rate limiting.
- Properties: ASIC resistance via memory bandwidth vs efficient
  verification.
- Privacy: Not privacy-preserving (deterministic from seed).

VDT and SWF address different application domains and are
complementary rather than competing approaches.

### Argon2 Memory-Hard Function

Argon2 {{RFC9106}} is the foundational memory-hard function
underlying SWF Modes 20 and 21. SWF uses Argon2id as a primitive,
not a replacement. The SWF construction chains iterated Argon2id
evaluations to produce sequential work proofs, whereas Argon2
specifies single-evaluation password hashing and proof-of-work.

# Conventions and Definitions

{::boilerplate bcp14-tagged}

The following notation is used throughout:

H:
: The hash function (SHA-256 unless otherwise specified).

I2OSP(x, len):
: Integer-to-Octet-String Primitive, encoding integer x as a
  big-endian byte string of length len, per {{RFC8017}}.

OS2IP(s):
: Octet-String-to-Integer Primitive, converting byte string s to
  an integer, per {{RFC8017}}.

HKDF-Expand(prk, info, len):
: HMAC-based Key Derivation Function Expand step, per {{RFC5869}}.

Argon2id(password, salt, t, m, p, len):
: Argon2id evaluation with time cost t, memory cost m KiB,
  parallelism p, and output length len bytes, per {{RFC9106}}.

Prover:
: The entity computing the SWF and generating proofs.

Verifier:
: The entity checking SWF proofs.

# Algorithm Specification {#algorithm-spec}

The SWF is computed by iterating a core function over a sequence of
steps, collecting all intermediate states into a Merkle tree, and
committing to the tree root. The Prover then derives deterministic
sample positions and provides Merkle proofs for the sampled state
transitions.

## Mode 20: Argon2id Iterative {#mode-20}

### Construction {#mode-20-construction}

For `swf-argon2id` (algorithm identifier 20), the SWF is computed
as follows:

~~~ pseudocode
hash_len = output_length(H)          ; 32, 48, or 64 bytes
state_0  = Argon2id(seed, salt=H(0x00 || "CPoE-salt-v1" || seed),
                    t=t, m=m, p=1, len=hash_len)
for i in 1..steps:
    state_i = Argon2id(state_{i-1},
                       salt=H(0x01 || "CPoE-salt-v1" || I2OSP(i, 4)),
                       t=t, m=m, p=1, len=hash_len)
merkle_root = MerkleTree(state_0, state_1, ...,
                         state_steps).root
~~~

Each step is a full Argon2id evaluation bounded by memory bandwidth,
ensuring ASIC resistance at every link in the chain.

The salt for state\_0 MUST be derived from the seed:
salt = H(0x00 \|\| "CPoE-salt-v1" \|\| seed). For subsequent steps
i >= 1, the salt MUST be H(0x01 \|\| "CPoE-salt-v1" \|\| I2OSP(i, 4)),
where I2OSP encodes i as a 4-byte big-endian integer per {{RFC8017}}.

The 0x00 and 0x01 type-tag prefixes provide unambiguous domain
separation between the seed-derived initial salt and step-indexed
salts, preventing collisions even when the seed value equals
I2OSP(i, 4) for some i.

The Argon2id output length (`len`) MUST equal the output length of H
to ensure SWF state sizes are consistent with the selected hash
algorithm.

The parallelism parameter p MUST be 1. Using p > 1 would allow
intra-step parallelism, undermining the sequential work guarantee.

### Parameters {#mode-20-params}

The following parameters control Mode 20 computation:

| Parameter | Description | Constraints |
|---|---|---|
| seed | Opaque input seed | Application-specific; MUST contain sufficient entropy (see {{seed-requirements}}) |
| t | Argon2id time cost | MUST be >= 1 |
| m | Argon2id memory cost (KiB) | RECOMMENDED >= 65536 (64 MiB) |
| p | Argon2id parallelism | MUST be 1 |
| steps | Number of iterations | MUST be >= 1 |
| hash\_len | Output length per step | MUST equal output\_length(H) |

Expected wall-clock time on reference hardware (DDR4, approximately
25 GB/s memory bandwidth {{JESD79-4}}): each Argon2id step with
t=1, m=65536 KiB requires approximately 100ms {{Biryukov2016}}.

## Mode 21: Entangled Argon2id {#mode-21}

### Construction {#mode-21-construction}

Mode 21 (`swf-argon2id-entangled`, algorithm identifier 21) extends
Mode 20 by incorporating the previous invocation's final state into
the seed derivation for the current invocation. The per-step
computation is identical to Mode 20; only the seed derivation differs.

When a protocol invokes the SWF multiple times in sequence (e.g.,
at periodic intervals), Mode 21 requires that each invocation's
seed includes the final state from the immediately preceding
invocation:

~~~ pseudocode
seed_n = application_seed_derivation(
    ...,
    prev_swf_output    ; state_steps from invocation n-1
)
~~~

This creates a strict cryptographic dependency chain: invocation n
cannot begin until invocation n-1 has completed, because the seed
for invocation n depends on the final output of invocation n-1.
This eliminates parallel pre-computation across invocations.

For the first invocation in a sequence, the seed derivation MUST
NOT include a previous SWF output (there is none). Subsequent
invocations MUST include the previous invocation's final state.

The per-step algorithm (Argon2id iteration, salt derivation,
Merkle tree construction) is identical to Mode 20
({{mode-20-construction}}).

### When to Use Mode 21 {#mode-21-usage}

Mode 21 is RECOMMENDED when the SWF is invoked repeatedly and the
consuming protocol requires that invocations cannot be parallelized
or pre-computed. Without entanglement (Mode 20), an adversary with
sufficient hardware could pre-compute multiple invocations in
parallel, then present them as sequential. Mode 21 prevents this
by chaining the SWF outputs cryptographically.

## Mode 10: SHA-256 with Waypoints {#mode-10}

### Construction {#mode-10-construction}

For `swf-sha256` (algorithm identifier 10), the SWF uses a single
Argon2id initialization followed by iterated SHA-256 hashing with
periodic memory-hard waypoints:

~~~ pseudocode
hash_len = 32                        ; SHA-256 fixed
state_0  = Argon2id(seed, salt=H(0x00 || "CPoE-salt-v1" || seed),
                    t=t, m=m, p=1, len=hash_len)
for i in 1..steps:
    if i mod W == 0:
        state_i = Argon2id(state_{i-1},
                           salt=H(0x01 || "CPoE-salt-v1"
                                  || I2OSP(i, 4)),
                           t=1, m=m_waypoint, p=1, len=hash_len)
    else:
        state_i = H(state_{i-1})
merkle_root = MerkleTree(state_0, state_1, ...,
                         state_steps).root
~~~

### Waypoint Logic {#waypoint-logic}

At every W-th step (where W is the waypoint-interval parameter),
the Prover MUST compute a full Argon2id evaluation instead of
SHA-256, using waypoint-memory (m\_waypoint) as the memory cost.

These waypoints bound the ASIC advantage: custom SHA-256 ASICs
achieve approximately 10,000x speedup over general-purpose CPUs,
but this advantage applies only to the SHA-256 steps between
waypoints. The Argon2id waypoints are memory-bandwidth-bound and
resist acceleration. Since the waypoints dominate total computation
time (each waypoint costs orders of magnitude more than the
intervening SHA-256 steps), the effective ASIC advantage over the
full chain is bounded by the Argon2id memory-bandwidth limit.

When waypoint-interval is absent, there are no waypoints and the
construction reduces to Argon2id initialization followed by plain
iterated SHA-256. This provides substantially weaker ASIC resistance.

### Parameters {#mode-10-params}

| Parameter | Description | Constraints |
|---|---|---|
| seed | Opaque input seed | Application-specific |
| t | Argon2id time cost (init) | MUST be >= 1 |
| m | Argon2id memory cost (init, KiB) | RECOMMENDED >= 65536 |
| W | Waypoint interval | RECOMMENDED >= 100 |
| m\_waypoint | Waypoint memory cost (KiB) | RECOMMENDED >= 32768 |
| steps | Total iterations | MUST be >= W for at least one waypoint |

For Mode 10, the initial Argon2id evaluation requires approximately
50-100ms; subsequent SHA-256 steps add approximately 0.1ms per 1000
steps. The Argon2id waypoints each add approximately 50-100ms
depending on m\_waypoint.

## Seed Requirements {#seed-requirements}

The SWF accepts an opaque seed as input. The seed derivation is
application-specific; this document defines the SWF algorithm
independently of how seeds are produced.

The seed MUST satisfy the following requirements:

* It MUST contain at least 128 bits of entropy.
* It MUST NOT be fully deterministic (i.e., predictable from
  public information alone), as this would allow pre-computation.
* It SHOULD incorporate context that binds the SWF to the
  intended computation (timestamps, nonces, prior state).

The Cryptographic Proof of Effort (CPoE) protocol {{CPoE-Protocol}}
defines one concrete seed derivation incorporating behavioral
entropy, content hashes, and optional hardware-attested time.

The test vectors in {{test-vectors}} use a simplified fixed seed
for implementation validation. Production deployments MUST use
application-specific seed derivation meeting the entropy
requirements above.

# Verification Protocol {#verification}

## Merkle Tree Construction {#merkle-tree-construction}

The SWF Merkle tree is constructed over all intermediate states
using domain-separated hashing following the tagged hash
construction from {{RFC6962}}:

* Leaf nodes: H(0x00 \|\| state\_i) for i in 0..steps, where
  leaf-index = i and leaf-value = state\_i. The total number of
  leaves is (steps + 1). The 0x00 prefix tag identifies leaf-level
  hashes.
* Internal nodes: H(0x01 \|\| left\_child \|\| right\_child). The
  0x01 prefix tag identifies internal node hashes. This domain
  separation prevents second-preimage attacks on the tree structure
  where an adversary constructs an internal node that masquerades
  as a leaf or vice versa.
* Padding: If the number of leaves is not a power of 2, the tree
  is padded with sentinel values: pad\_value =
  H(0x02 \|\| I2OSP(steps + 1, 4)), repeated until the count
  reaches the next power of 2. The 0x02 prefix tag and embedded
  tree size make padding nodes cryptographically distinguishable
  from both leaf and internal nodes and bind the padding to the
  specific chain length.
* The Merkle root is stored in the process-proof merkle-root field.

The final state (state\_steps) is the leaf at index "steps" and is
verified by checking its Merkle proof against the committed root.

## Fiat-Shamir Sampling {#fiat-shamir-sampling}

Merkle proof sample positions MUST be derived deterministically via
Fiat-Shamir transform:

~~~ pseudocode
sample_seed = H(
    "CPoE-Fiat-Shamir-v1" ||
    I2OSP(proof-algorithm, 2) ||
    CBOR-encode(proof-params) ||
    process-proof.input ||
    merkle_root
)
for j in 0..k-1:
    okm_j   = HKDF-Expand(sample_seed, I2OSP(j, 4), 4)
    index_j = OS2IP(okm_j) mod (steps + 1)
~~~

The sample seed MUST incorporate a versioned domain separation tag
("CPoE-Fiat-Shamir-v1") followed by the full proof context: the
algorithm identifier, all parameters, the SWF input seed, and the
Merkle root. The DST prefix provides version agility and prevents
cross-protocol confusion. The algorithm and parameter binding
prevents cross-algorithm confusion attacks where an adversary
exploits a seed that produces the same Merkle root under a cheaper
algorithm. CBOR-encode produces deterministic CBOR per Section 4.2.1
of {{RFC8949}}.

Where k is the number of required samples. HKDF-Expand is
instantiated with H as the underlying hash function per {{RFC5869}}.
I2OSP and OS2IP are the Integer-to-Octet-String and
Octet-String-to-Integer primitives per {{RFC8017}}.

The Prover MUST include Merkle proofs for exactly these indices. The
Verifier recomputes the sample positions from the committed root and
seed, then verifies only those proofs.

If the derivation produces duplicate indices, the Prover MUST
continue generating additional indices by incrementing j beyond k-1
until k distinct indices are obtained. The Verifier MUST verify
that all k sample indices are distinct.

Sample indices are in the range \[0, steps\] inclusive. Padded Merkle
tree leaves (indices greater than steps) are never sampled by this
derivation.

## Verification Procedure {#verification-procedure}

### Modes 20/21: Merkle-Sampled Verification {#verify-modes-20-21}

For `swf-argon2id` (20) and `swf-argon2id-entangled` (21), the
Verifier MUST:

1. Recompute Argon2id from the declared seed to obtain state\_0.
2. For each Fiat-Shamir sampled proof in the Merkle tree, verify
   the sibling path against the committed root using tagged hashing
   ({{merkle-tree-construction}}) and recompute the state transition:
   Argon2id(state\_i,
   salt=H(0x01 \|\| "CPoE-salt-v1" \|\| I2OSP(i+1, 4)),
   t=t, m=m, p=1, len=hash\_len). Verify the result equals
   state\_{i+1}.
3. Verify the final state (state\_steps) by checking its Merkle
   proof against the committed root. If the final-leaf index is
   not included in the Fiat-Shamir sample set, the Verifier MUST
   additionally derive or request a proof for it.

Each sampled proof requires one Argon2id evaluation to verify.
Verification cost is O(k) Argon2id evaluations, each requiring m
KiB of memory. Verifiers MUST verify samples sequentially or limit
concurrent evaluations to avoid excessive memory consumption.

### Mode 10: Deterministic Full-Chain Verification {#verify-mode-10}

For `swf-sha256` (10), the Verifier SHOULD perform deterministic
full-chain verification rather than Merkle-sampled verification.
The Verifier MUST:

1. Recompute state\_0 = Argon2id(seed,
   salt=H(0x00 \|\| "CPoE-salt-v1" \|\| seed),
   t=t, m=m, p=1, len=32).
2. Recompute the full chain sequentially: for i in 1..steps,
   if i mod W == 0, compute state\_i =
   Argon2id(state\_{i-1},
   salt=H(0x01 \|\| "CPoE-salt-v1" \|\| I2OSP(i, 4)),
   t=1, m=m\_waypoint, p=1, len=32);
   otherwise, compute state\_i = H(state\_{i-1}).
3. Construct the Merkle tree from all recomputed states using
   tagged hashing ({{merkle-tree-construction}}).
4. Verify the computed Merkle root equals the committed root.

Full-chain verification requires steps/W Argon2id evaluations plus
steps SHA-256 evaluations. For typical parameters (W=1000,
steps=10000), this costs approximately 10 Argon2id evaluations
(~1 second) plus 10,000 SHA-256 evaluations (~1 millisecond).

Full-chain verification provides deterministic guarantees: all
waypoints AND all SHA-256 transitions are verified with zero false
negatives.

When full-chain verification is impractical (e.g., constrained
Verifier environments), the Verifier MAY fall back to
Merkle-sampled verification per {{fiat-shamir-sampling}}. In this
fallback mode, the Verifier MUST additionally verify all waypoint
transitions by requesting Merkle proofs for index pairs (iW-1, iW)
for i in 1..steps/W. This ensures memory-hard waypoints are always
verified regardless of the Fiat-Shamir sample draw.

# Security Analysis {#security-analysis}

## ASIC Resistance {#asic-resistance}

The ASIC advantage for Argon2id-based SWF modes (20 and 21) is
bounded by three independent factors:

Time-Memory Tradeoff (TMTO):
: Single-pass Argon2id (t=1) permits at most ~2x reduction in
  time-area product via ranking-based tradeoff attacks
  {{RenDevadas2017}}. Multi-pass reduces this to ~1.33x
  ({{RFC9106}}, Section 7). Using t=1 is recommended because the
  TMTO advantage is offset by the multiplicative effect of iterated
  evaluations: an adversary gaining 2x per step gains 2x overall
  (not 2^steps), while t>1 would reduce Prover throughput and
  therefore the achievable step count per time interval.

Memory Bandwidth:
: Each Argon2id step is bounded by memory bandwidth, not ALU
  throughput. Consumer DDR4 provides ~25 GB/s {{JESD79-4}}; DDR5
  provides ~50 GB/s {{JESD79-5}}. HBM3 (available in datacenter
  ASICs) provides ~800 GB/s per stack {{JESD238}} but at
  substantially higher cost per device. The effective economic
  advantage is approximately 3-4x when amortized over device cost.

Silicon Optimization:
: Custom Argon2id ASICs can eliminate instruction decode overhead
  and optimize the Blake2b core, providing an estimated 1.5-2x
  advantage over general-purpose CPUs for the same memory bandwidth
  {{Biryukov2016}}{{RenDevadas2017}}.

Combined Advantage:
: The multiplicative combination of these factors yields an upper
  bound of approximately 8-16x for a fully optimized ASIC versus
  consumer DDR4 hardware. However, the economic cost of such
  hardware ensures that the forgery cost in absolute terms remains
  substantial. Verifiers SHOULD use a conservative ASIC advantage
  factor of 10x when computing cost estimates.

For `swf-sha256` (Mode 10), SHA-256 iterations between waypoints
have an ASIC advantage exceeding 10,000x. The memory-hard waypoints
({{mode-10-construction}}) ensure that the effective ASIC advantage
over the full Mode 10 chain is bounded by the Argon2id advantage at
waypoint steps, since waypoints dominate the total computation time.

## Memory Hardness {#memory-hardness}

For Modes 20 and 21, every SWF step is a full Argon2id evaluation
bounded by memory bandwidth (approximately 50 GB/s for DDR5
{{JESD79-5}}), not ALU throughput. ASICs provide minimal advantage
per step, and this resistance compounds across all steps in the
chain.

The memory-hard nature of Argon2id ensures that reducing memory per
evaluation forces a disproportionate increase in computation time;
the best known TMTO attacks achieve at most a ~2x reduction in
time-area product for single-pass Argon2id, decreasing to ~1.33x
with multiple passes ({{RFC9106}}, Section 7). The raw
memory-bandwidth advantage of custom hardware (HBM3 at ~800 GB/s
versus consumer DDR4 at ~25 GB/s) provides an additional 3-4x
speedup when amortized over device cost.

For Mode 10, memory-hard waypoints at every W-th step bound the
ASIC advantage at those steps to the Argon2id limit.

## Attack Costs {#attack-costs}

### Skipping Detection {#skipping-detection}

An adversary who skips fraction f of steps will be detected with
probability 1-(1-f)^k, where k is the number of sampled proofs.
With k=20 and f=0.1, detection probability exceeds 0.878. With
k=100 and f=0.05, detection probability exceeds 0.994. This
detection probability applies independently to each invocation.

This bound holds under the random oracle model for H. The Prover
commits to the Merkle root before sample positions are derived via
the Fiat-Shamir transform. Finding a root that biases all k samples
away from skipped steps requires inverting H, which is
computationally infeasible under standard assumptions.

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

The grinding overhead grows rapidly: for k=20 and f=0.10,
W\_grind is approximately 8.2n (8.2x honest work); for k=100 and
f=0.05, W\_grind is approximately 131n. Grinding is
counterproductive because each trial requires computing the full
honest chain to obtain the Merkle root from which sample positions
are derived.

### Forgery Cost {#forgery-cost}

The minimum forgery cost for an SWF proof of n steps is bounded by:

~~~ artwork
C_swf >= n * t_step / advantage_factor

where:
  n = number of steps
  t_step = wall-clock time per Argon2id evaluation
  advantage_factor = ASIC advantage (<=10x recommended)
~~~

For Mode 20 with typical parameters (t=1, m=65536 KiB, 90 steps),
the minimum computation time is approximately 9 seconds on consumer
hardware, or approximately 0.9 seconds with a 10x ASIC advantage.
The memory requirement (64 MiB per step, sequential) prevents
trivial parallelization.

For Mode 10, the forgery cost is dominated by the number of
waypoint evaluations (steps/W Argon2id evaluations) rather than the
SHA-256 iterations between waypoints.

# Wire Format {#wire-format}

The SWF proof structure is encoded in CBOR {{RFC8949}} using the
following CDDL {{RFC8610}} definition:

~~~ cddl
process-proof = {
    1 => proof-algorithm,
    2 => proof-params,
    3 => bstr,                    ; input (max 64 bytes)
    4 => bstr,                    ; merkle-root (max 64 bytes)
    5 => [+ merkle-proof],        ; sampled-proofs (max 1000)
    6 => uint,                    ; claimed-duration (ms)
}

proof-params = {
    1 => uint,                    ; time-cost
    2 => uint,                    ; memory-cost (KiB)
    3 => uint,                    ; parallelism
    4 => uint,                    ; steps
    ? 5 => uint,                  ; waypoint-interval (Mode 10)
    ? 6 => uint,                  ; waypoint-memory (KiB, Mode 10)
}

merkle-proof = {
    1 => uint,                    ; leaf-index
    2 => [+ bstr .size 32],       ; sibling-path (max depth 64)
    3 => bstr,                    ; leaf-value (max 64 bytes)
}

proof-algorithm = &(
    swf-sha256:             10,
    swf-argon2id:           20,
    swf-argon2id-entangled: 21,
)
~~~

The `input` field (key 3) contains the seed used for the SWF
computation. The `merkle-root` field (key 4) contains the root of
the Merkle tree over all intermediate states. The `sampled-proofs`
field (key 5) contains Merkle proofs for the Fiat-Shamir-derived
sample indices plus any mandatory waypoint proofs (for Mode 10
sampled verification). The `claimed-duration` field (key 6) is the
Prover's self-reported wall-clock computation time in milliseconds.

For Mode 10, the `waypoint-interval` (key 5) and `waypoint-memory`
(key 6) parameters MUST be present in proof-params. Verifiers MUST
reject Mode 10 proofs that omit these parameters.

# Security Considerations {#security-considerations}

The SWF construction provides probabilistic verification, not
deterministic proof of sequential work. The detection probability
for an adversary skipping fraction f of steps is 1-(1-f)^k, where
k is the sample count. Applications MUST choose k based on their
required detection probability.

The seed MUST NOT be fully deterministic. A predictable seed allows
pre-computation of the entire SWF chain before the intended
computation window, defeating the temporal binding property.
Applications SHOULD incorporate fresh randomness, timestamps, or
external nonces into the seed derivation.

Verification of Modes 20 and 21 requires k Argon2id evaluations,
each consuming m KiB of memory. This creates an asymmetric
denial-of-service vector: a malicious Prover can submit proofs that
are expensive for the Verifier to check. Verifiers SHOULD implement
rate limiting on proof submission and MAY reject proofs with
parameters exceeding configured memory limits.

The domain separation tags used in this specification
("CPoE-salt-v1", "CPoE-Fiat-Shamir-v1") contain the "CPoE" prefix
for historical compatibility with the Cryptographic Proof of Effort
protocol {{CPoE-Protocol}} in which the SWF was originally defined.

# IANA Considerations {#iana-considerations}

## SWF Proof Algorithm Registry {#swf-registry}

IANA is requested to create a new "SWF Proof Algorithm" registry
with the following initial entries:

| Value | Name | Reference |
|---|---|---|
| 10 | swf-sha256 | This document, {{mode-10}} |
| 20 | swf-argon2id | This document, {{mode-20}} |
| 21 | swf-argon2id-entangled | This document, {{mode-21}} |

The registration policy for this registry is Specification Required
per RFC 8126.

Values 0-9 are Reserved. Values 10-255 are available for
registration. Values 256 and above are for Private Use.

--- back

# SWF Test Vectors {#test-vectors}
{:numbered="false"}

The following test vectors validate SWF implementations.

These test vectors use the type-tagged salt derivation (0x00/0x01
prefixes) as specified in {{mode-20-construction}}. All vectors use
SHA-256 (H = SHA-256, hash\_len = 32).

## swf-sha256 (Mode 10) Test Vector {#test-vector-mode10}
{:numbered="false"}

This vector uses the `swf-sha256` construction with memory-hard
waypoints (W=1000, waypoint-memory=32768 KiB): Argon2id
initialization, then iterated SHA-256 with Argon2id waypoints at
every 1000th step.

~~~ test-vectors
Seed: "cpoe-genesis-v1"
Seed (hex): 7769746e657373642d67656e657369732d7631
Salt: H(0x00 || "CPoE-salt-v1" || seed)  [H = SHA-256]

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

This vector uses the `swf-argon2id` construction: iterated Argon2id
evaluations with type-tagged salts. Each step feeds the previous
state as the password input. Implementers should verify state\_0
matches the Mode 10 vector above (identical Argon2id initialization).

~~~ test-vectors
Seed: "cpoe-genesis-v1"
Seed (hex): 7769746e657373642d67656e657369732d7631

Argon2id Parameters (per step):
  Time Cost (t): 1
  Memory Cost (m): 65536 KiB
  Parallelism (p): 1
  Output Length: 32 bytes

Steps: 3

Intermediate States:
  state_0 (Argon2id, seed as password,
           salt=H(0x00 || "CPoE-salt-v1" || seed)):
    55518d63068b5f245d9dccf5919cbcdc
    1fa1b3256e89a5c1eb7a7b37609b323f
  state_1 (Argon2id, state_0 as password,
           salt=H(0x01 || "CPoE-salt-v1" || I2OSP(1, 4))):
    6a6df1cfbce07c09036526e19f7b6e73
    ef2ce911d1ea77a66bb23bde5b033a79
  state_2 (Argon2id, state_1 as password,
           salt=H(0x01 || "CPoE-salt-v1" || I2OSP(2, 4))):
    bfa124c53651b2aedc79f48ec562342f
    91efc8bc61cd8f833a5e63efbb41af44
  state_3 (Argon2id, state_2 as password,
           salt=H(0x01 || "CPoE-salt-v1" || I2OSP(3, 4))):
    bdd55e641b507d2d2d49cb67cb34c78d
    92952ce025ef1b22a906f4721bcceb7c
~~~

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The SWF was originally specified as part of the Cryptographic Proof
of Effort (CPoE) protocol. The author thanks the participants of
the CFRG for their work on memory-hard functions and the authors
of Argon2 for the foundational construction on which the SWF is
built.
