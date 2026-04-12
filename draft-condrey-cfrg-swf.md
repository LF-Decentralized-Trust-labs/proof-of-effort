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
  RFC8126:
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
---

--- abstract

This document specifies the Sequential Work Function (SWF), a
construction that chains iterated Argon2id {{RFC9106}} evaluations
into a Merkle tree and provides probabilistic verification via
Fiat-Shamir-sampled audit proofs. Three modes are defined: Mode 20
(iterated Argon2id), Mode 21 (entangled Argon2id with
inter-invocation dependency), and Mode 10 (SHA-256 with periodic
Argon2id waypoints). The construction provides ASIC resistance
through memory-bandwidth bounds and verification in O(k log n)
time, where k is the sample count and n is the step count.

--- middle

# Introduction {#introduction}

Existing approaches to proving sustained sequential computation
either lack ASIC resistance (iterated SHA-256), require trusted
setup (VDFs with RSA groups {{Boneh2018}}), or cannot efficiently
verify intermediate states. Argon2id {{RFC9106}} resists hardware
acceleration but a single evaluation proves only point-in-time
computation.

The SWF chains iterated Argon2id evaluations into a Merkle tree
over intermediate states, then uses a Fiat-Shamir transform to
derive deterministic sample positions for audit proofs, yielding
probabilistic verification in O(k log n) time. Three modes are
defined ({{algorithm-spec}}).

## Related Work {#related-work}

### Verifiable Delay Functions

Verifiable Delay Functions (VDFs) {{Boneh2018}} provide O(1)
public verification using number-theoretic constructions but require
trusted setup or unproven hardness assumptions. The SWF is not a
VDF; it provides probabilistic verification via Merkle-sampled
proofs in O(k log n) time. This trade-off eliminates trusted setup
and enables memory-hard ASIC resistance.

### Verifiable Delay Tokens

Verifiable Delay Tokens (VDT)
{{I-D.bakshi-vdt-verifiable-delay-token}} use VDFs for
privacy-preserving time enforcement in rate limiting. SWF differs
in primitive (memory-hard Argon2id vs sequential squaring),
use case (authorship attestation vs anonymous rate limiting),
and properties (ASIC resistance via memory bandwidth vs efficient
verification).

### Argon2 Memory-Hard Function

SWF uses Argon2id {{RFC9106}} as a primitive, chaining iterated
evaluations to produce sequential work proofs. Argon2 specifies
single-evaluation password hashing; SWF extends it to multi-step
sequential work with Merkle-committed intermediate states.

# Conventions and Definitions

{::boilerplate bcp14-tagged}

H:
: SHA-256. This specification does not support hash agility;
  future versions may define alternate hash functions.

I2OSP(x, len):
: Integer-to-Octet-String Primitive per {{RFC8017}}.

OS2IP(s):
: Octet-String-to-Integer Primitive per {{RFC8017}}.

HKDF-Expand(prk, info, len):
: HMAC-based KDF Expand step per {{RFC5869}}.

Argon2id(password, salt, t, m, p, len):
: Argon2id evaluation per {{RFC9106}}.

Prover:
: The entity computing the SWF and generating proofs.

Verifier:
: The entity checking SWF proofs.

# Algorithm Specification {#algorithm-spec}

## Mode 20: Argon2id Iterative {#mode-20}

### Construction {#mode-20-construction}

For `swf-argon2id` (algorithm identifier 20):

~~~ pseudocode
state_0  = Argon2id(seed, salt=H(0x00 || "SWF-salt-v1" || seed),
                    t=t, m=m, p=1, len=32)
for i in 1..steps:
    state_i = Argon2id(state_{i-1},
                       salt=H(0x01 || "SWF-salt-v1" || I2OSP(i, 4)),
                       t=t, m=m, p=1, len=32)
merkle_root = MerkleTree(state_0, state_1, ...,
                         state_steps).root
~~~

The 0x00/0x01 type-tag prefixes provide domain separation between
the seed-derived initial salt and step-indexed salts, preventing
collisions even when the seed value equals I2OSP(i, 4) for some i.

### Parameters {#mode-20-params}

| Parameter | Description | Constraints |
|---|---|---|
| seed | Opaque input seed | MUST contain >= 128 bits entropy ({{seed-requirements}}) |
| t | Argon2id time cost | MUST be >= 1 |
| m | Argon2id memory cost (KiB) | RECOMMENDED >= 65536 (64 MiB) |
| p | Argon2id parallelism | MUST be 1 (p > 1 undermines sequential guarantee) |
| steps | Number of iterations | MUST be >= 1 |

## Mode 21: Entangled Argon2id {#mode-21}

Mode 21 (`swf-argon2id-entangled`, algorithm identifier 21) is
identical to Mode 20 except seed derivation includes the previous
invocation's final state. For invocation n >= 2:

~~~ pseudocode
seed_n = HKDF-Expand(prev_swf_output,
                     "SWF-entangle-v1" || I2OSP(n, 4), 32)
~~~

Where `prev_swf_output` is state\_steps from invocation n-1 (a
pseudorandom 32-byte Argon2id output). For the first invocation,
the seed is application-provided per {{seed-requirements}}.

Per-step computation (Argon2id iteration, salt derivation, Merkle
tree construction) is identical to Mode 20
({{mode-20-construction}}).

Mode 21 is RECOMMENDED when the SWF is invoked repeatedly and the
consuming protocol requires that invocations cannot be parallelized
or pre-computed. The cryptographic dependency on the previous
invocation's final state eliminates parallel pre-computation.

## Mode 10: SHA-256 with Waypoints {#mode-10}

### Construction {#mode-10-construction}

For `swf-sha256` (algorithm identifier 10):

~~~ pseudocode
state_0  = Argon2id(seed, salt=H(0x00 || "SWF-salt-v1" || seed),
                    t=t, m=m, p=1, len=32)
for i in 1..steps:
    if i mod W == 0:
        state_i = Argon2id(state_{i-1},
                           salt=H(0x01 || "SWF-salt-v1"
                                  || I2OSP(i, 4)),
                           t=1, m=m_waypoint, p=1, len=32)
    else:
        state_i = H(state_{i-1})
merkle_root = MerkleTree(state_0, state_1, ...,
                         state_steps).root
~~~

### Waypoint Logic {#waypoint-logic}

At every W-th step, the Prover MUST compute a full Argon2id
evaluation using m\_waypoint as the memory cost. Custom SHA-256
ASICs achieve approximately 10,000x speedup, but this advantage
applies only between waypoints. Since waypoints dominate total
computation time, the effective ASIC advantage over the full chain
is bounded by the Argon2id memory-bandwidth limit.

When waypoint-interval is absent, the construction reduces to
Argon2id initialization followed by plain iterated SHA-256,
providing substantially weaker ASIC resistance.

### Parameters {#mode-10-params}

| Parameter | Description | Constraints |
|---|---|---|
| seed | Opaque input seed | Application-specific |
| t | Argon2id time cost (init) | MUST be >= 1 |
| m | Argon2id memory cost (init, KiB) | RECOMMENDED >= 65536 |
| W | Waypoint interval | RECOMMENDED >= 100 |
| m\_waypoint | Waypoint memory cost (KiB) | RECOMMENDED >= 32768 |
| steps | Total iterations | MUST be >= W for at least one waypoint |

## Seed Requirements {#seed-requirements}

The seed MUST satisfy the following requirements:

* It MUST contain at least 128 bits of entropy.
* It MUST NOT be fully deterministic, as this would allow
  pre-computation.
* It SHOULD incorporate context that binds the SWF to the
  intended computation (timestamps, nonces, prior state).

# Verification Protocol {#verification}

## Merkle Tree Construction {#merkle-tree-construction}

The Merkle tree uses domain-separated hashing per {{RFC6962}}:

* Leaf nodes: H(0x00 \|\| state\_i) for i in 0..steps.
* Internal nodes: H(0x01 \|\| left\_child \|\| right\_child).
  The 0x01 prefix prevents second-preimage attacks where internal
  nodes masquerade as leaves.
* Padding: If the number of leaves is not a power of 2, the tree
  is padded with sentinel values: pad\_value =
  H(0x02 \|\| I2OSP(steps + 1, 4)), repeated until the count
  reaches the next power of 2. The 0x02 prefix and embedded size
  distinguish padding from data nodes.
* The Merkle root is stored in the process-proof merkle-root field.

## Fiat-Shamir Sampling {#fiat-shamir-sampling}

Merkle proof sample positions MUST be derived deterministically via
Fiat-Shamir transform:

~~~ pseudocode
sample_seed = H(
    "SWF-sample-v1" ||
    I2OSP(proof-algorithm, 2) ||
    CBOR-encode(proof-params) ||
    process-proof.input ||
    merkle_root
)
for j in 0..k-1:
    okm_j   = HKDF-Expand(sample_seed, I2OSP(j, 4), 4)
    index_j = OS2IP(okm_j) mod (steps + 1)
~~~

The DST prefix provides version agility. The algorithm and
parameter binding prevents cross-algorithm confusion attacks where
an adversary exploits a seed that produces the same Merkle root
under a cheaper algorithm. CBOR-encode produces deterministic CBOR
per Section 4.2.1 of {{RFC8949}}.

The Prover MUST include Merkle proofs for exactly these k indices.
The Verifier recomputes the sample positions from the committed root
and seed, then verifies only those proofs.

If the derivation produces duplicate indices, the Prover MUST
continue generating additional indices by incrementing j beyond k-1
until k distinct indices are obtained. The Verifier MUST verify
that all k sample indices are distinct.

Sample indices are in the range \[0, steps\] inclusive. Padded
leaves are never sampled.

## Verification Procedure {#verification-procedure}

### Modes 20/21: Merkle-Sampled Verification {#verify-modes-20-21}

For `swf-argon2id` (20) and `swf-argon2id-entangled` (21), the
Verifier MUST:

1. Recompute Argon2id from the declared seed to obtain state\_0.
2. For each Fiat-Shamir sampled proof in the Merkle tree, verify
   the sibling path against the committed root using tagged hashing
   ({{merkle-tree-construction}}) and recompute the state transition:
   Argon2id(state\_i,
   salt=H(0x01 \|\| "SWF-salt-v1" \|\| I2OSP(i+1, 4)),
   t=t, m=m, p=1, len=32). Verify the result equals
   state\_{i+1}.
3. Verify the final state (state\_steps) by checking its Merkle
   proof against the committed root. If the final-leaf index is
   not included in the Fiat-Shamir sample set, the Verifier MUST
   additionally derive or request a proof for it.

The claimed-duration field is informational; it is not verified by
this specification.

Verifiers MUST verify samples sequentially or limit concurrent
evaluations to avoid excessive memory consumption (each requires
m KiB).

### Mode 10: Deterministic Full-Chain Verification {#verify-mode-10}

For `swf-sha256` (10), the Verifier SHOULD perform deterministic
full-chain verification. The Verifier MUST:

1. Recompute state\_0 = Argon2id(seed,
   salt=H(0x00 \|\| "SWF-salt-v1" \|\| seed),
   t=t, m=m, p=1, len=32).
2. Recompute the full chain sequentially: for i in 1..steps,
   if i mod W == 0, compute state\_i =
   Argon2id(state\_{i-1},
   salt=H(0x01 \|\| "SWF-salt-v1" \|\| I2OSP(i, 4)),
   t=1, m=m\_waypoint, p=1, len=32);
   otherwise, compute state\_i = H(state\_{i-1}).
3. Construct the Merkle tree from all recomputed states using
   tagged hashing ({{merkle-tree-construction}}).
4. Verify the computed Merkle root equals the committed root.

For typical parameters (W=1000, steps=10000), full-chain
verification costs approximately 10 Argon2id evaluations (~1s)
plus 10,000 SHA-256 evaluations (~1ms).

When full-chain verification is impractical, the Verifier MAY fall
back to Merkle-sampled verification per {{fiat-shamir-sampling}}.
In this fallback mode, the Verifier MUST additionally verify all
waypoint transitions by requesting Merkle proofs for index pairs
(iW-1, iW) for i in 1..steps/W.

# Security Analysis {#security-analysis}

## ASIC Resistance and Memory Hardness {#asic-resistance}

The ASIC advantage for Argon2id-based SWF modes (20 and 21) is
bounded by three independent factors:

Time-Memory Tradeoff (TMTO):
: Single-pass Argon2id (t=1) permits at most ~2x reduction in
  time-area product via ranking-based tradeoff attacks
  {{RenDevadas2017}}. Multi-pass reduces this to ~1.33x
  ({{RFC9106}}, Section 7). Using t=1 is recommended because the
  TMTO advantage is offset by the multiplicative effect of iterated
  evaluations: an adversary gaining 2x per step gains 2x overall
  (not 2^steps), while t>1 reduces throughput. Note: the combined
  bound below assumes t=1; higher t values yield a tighter bound.

Memory Bandwidth:
: Each step is bounded by memory bandwidth, not ALU throughput.
  Consumer DDR4 provides ~25 GB/s {{JESD79-4}}; DDR5 ~50 GB/s
  {{JESD79-5}}. HBM3 provides ~800 GB/s per stack {{JESD238}} at
  substantially higher cost. Effective economic advantage: 3-4x
  amortized over device cost.

Silicon Optimization:
: Custom ASICs eliminating instruction decode and optimizing
  Blake2b provide 1.5-2x advantage for the same memory bandwidth
  {{Biryukov2016}}{{RenDevadas2017}}.

Combined Advantage:
: Multiplicative combination yields an upper bound of 8-16x for a
  fully optimized ASIC versus consumer DDR4. Verifiers SHOULD use
  a conservative ASIC advantage factor of 10x. This factor should
  be re-evaluated as memory technology evolves.

For `swf-sha256` (Mode 10), SHA-256 iterations between waypoints
have ASIC advantage exceeding 10,000x. The memory-hard waypoints
({{mode-10-construction}}) bound the effective ASIC advantage to
the Argon2id limit at waypoint steps, which dominate total
computation time.

Reference wall-clock times on DDR4 (~25 GB/s {{JESD79-4}}): each
Argon2id step with t=1, m=65536 KiB requires approximately 100ms
{{Biryukov2016}}. For Mode 10, initial Argon2id requires 50-100ms;
SHA-256 steps add approximately 0.1ms per 1000 steps.

## Attack Costs {#attack-costs}

### Skipping Detection {#skipping-detection}

An adversary who skips fraction f of steps is detected with
probability 1-(1-f)^k, where k is the number of sampled proofs.
With k=20 and f=0.1, detection probability exceeds 0.878. With
k=100 and f=0.05, detection probability exceeds 0.994.

This bound holds in the random oracle model; biasing all k samples
away from skipped steps requires inverting H.

### Seed Grinding Resistance {#seed-grinding}

A grinding adversary tries multiple seeds, selecting one where
Fiat-Shamir samples avoid skipped steps. This strategy is strictly
anti-profitable:

Theorem: For any fraction f in (0,1) of skipped steps and sample
count k >= 2, the expected total work of a grinding adversary
strictly exceeds honest computation.

Proof sketch: Let n be the step count. The expected total work is:

~~~ pseudocode
W_grind = (1-f)*n * (1-f)^{-k} = n * (1-f)^{1-k}
~~~

Since k >= 2, (1-f)^{1-k} = 1 / (1-f)^{k-1} > 1 for all
f in (0,1). Therefore W\_grind > n = W\_honest.

For k=20, f=0.10: W\_grind is approximately 8.2n. For k=100,
f=0.05: W\_grind is approximately 131n.

### Forgery Cost {#forgery-cost}

The minimum forgery cost for n steps is bounded by:

~~~ artwork
C_swf >= n * t_step / advantage_factor

where:
  n = number of steps
  t_step = wall-clock time per Argon2id evaluation
  advantage_factor = ASIC advantage (<=10x recommended)
~~~

For Mode 20 with typical parameters (t=1, m=65536 KiB, 90 steps),
minimum computation time is approximately 9 seconds on consumer
hardware, or approximately 0.9 seconds with 10x ASIC advantage.

For Mode 10, forgery cost is dominated by waypoint evaluations
(steps/W Argon2id evaluations).

# Wire Format {#wire-format}

The SWF proof structure is encoded in CBOR {{RFC8949}} using the
following CDDL {{RFC8610}} definition:

~~~ cddl
process-proof = {
    1 => proof-algorithm,
    2 => proof-params,
    3 => bstr,                    ; input (max 64 bytes)
    4 => bstr,                    ; merkle-root (32 bytes)
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
    7 => uint,                    ; sample-count (k)
}

merkle-proof = {
    1 => uint,                    ; leaf-index
    2 => [+ bstr .size 32],       ; sibling-path (max depth 64)
    3 => bstr,                    ; leaf-value (32 bytes)
}

proof-algorithm = &(
    swf-sha256:             10,
    swf-argon2id:           20,
    swf-argon2id-entangled: 21,
)
~~~

The sample-count (proof-params key 7) MUST be at least 20. Values
below 100 are NOT RECOMMENDED for production use.

For Mode 10, `waypoint-interval` (key 5) and `waypoint-memory`
(key 6) MUST be present in proof-params. Verifiers MUST reject
Mode 10 proofs that omit these parameters.

# Security Considerations {#security-considerations}

Applications MUST choose k based on their required detection
probability ({{skipping-detection}}).

Seed requirements are defined in {{seed-requirements}}. A
predictable seed defeats temporal binding by enabling
pre-computation.

Verification of Modes 20 and 21 requires k Argon2id evaluations,
each consuming m KiB. Verifiers SHOULD implement rate limiting on
proof submission and MAY reject proofs with parameters exceeding
configured memory limits.

# IANA Considerations {#iana-considerations}

## SWF Proof Algorithm Registry {#swf-registry}

IANA is requested to create a new "SWF Proof Algorithm" registry
with the following initial entries:

| Value | Name | Reference |
|---|---|---|
| 10 | swf-sha256 | This document, {{mode-10}} |
| 20 | swf-argon2id | This document, {{mode-20}} |
| 21 | swf-argon2id-entangled | This document, {{mode-21}} |

Registration policy: Specification Required per {{RFC8126}}.
Values 0-9 are Reserved. Values 10-255 are available for
registration. Values 256+ are Private Use.

--- back

# SWF Test Vectors {#test-vectors}
{:numbered="false"}

The following test vectors use the type-tagged salt derivation
(0x00/0x01 prefixes) as specified in {{mode-20-construction}}.
All vectors use SHA-256 (H = SHA-256).

## swf-sha256 (Mode 10) Test Vector {#test-vector-mode10}
{:numbered="false"}

~~~ test-vectors
Seed: "cpoe-genesis-v1"
Seed (hex): 63706f652d67656e657369732d7631
Salt: H(0x00 || "SWF-salt-v1" || seed)  [H = SHA-256]

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

Salt (hex): d0b1de5e520255be2ab0c75b5b6f08cc
             fcf6023c9c3cccdd263ce152192e5cb3

Intermediate States:
  state_0 (Argon2id):
    96a66581b8db69eb85ab99241c453ac4
    279e06b85eea84565880bf20274383f1
  state_1000 (waypoint, Argon2id):
    67055d129375e4a1327be707d36d4867
    484621948772825475be2e5e7f69209f
  state_5000 (waypoint, Argon2id):
    82f8275a18a95f729cedc7dec7fc50ca
    34ff15d40fa5bbff1b83342af678631f
  state_9999 (SHA-256):
    9ced542457d0ee3576a1849c1c618eca
    c7ef7881685938f99fd9f0137c1daff9
  state_10000 (waypoint, Argon2id, final):
    bcc6aade8854903b0504b2a475f139cc
    8d6ad2386ec586d0673e451c12d73605
~~~

## swf-argon2id (Mode 20) Test Vector {#test-vector-mode20}
{:numbered="false"}

Implementers should verify state\_0 matches the Mode 10 vector
above (identical Argon2id initialization).

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
           salt=H(0x00 || "SWF-salt-v1" || seed)):
    96a66581b8db69eb85ab99241c453ac4
    279e06b85eea84565880bf20274383f1
  state_1 (Argon2id, state_0 as password,
           salt=H(0x01 || "SWF-salt-v1" || I2OSP(1, 4))):
    59ef8f4ededbd8e2a3e0472859968078
    d85e662a89721fb1e77c324f5e01dc2c
  state_2 (Argon2id, state_1 as password,
           salt=H(0x01 || "SWF-salt-v1" || I2OSP(2, 4))):
    0256acada5c209e8e423a0bbcd202829
    8ef785f638f8934ba38745cef131d3c9
  state_3 (Argon2id, state_2 as password,
           salt=H(0x01 || "SWF-salt-v1" || I2OSP(3, 4))):
    bd0363655ff2b96db5e93f9dca5cc445
    bc2dc2fd33d51f89f390b359b6625533
~~~

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The author thanks the participants of the CFRG for their work on
memory-hard functions and the authors of Argon2 for the
foundational construction.
