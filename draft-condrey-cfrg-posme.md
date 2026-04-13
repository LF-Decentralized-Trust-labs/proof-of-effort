---
v: 3
docname: draft-condrey-cfrg-posme-latest
title: "Proof of Sequential Memory Execution (PoSME)"
abbrev: PoSME
category: info
ipr: trust200902
submissiontype: IRTF
area: Security
workgroup: Crypto Forum Research Group
keyword:
  - memory-hard
  - sequential execution
  - causal hash
  - latency-bound
  - ASIC resistance

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
  RFC5869:
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
  CohenPietrzak2018:
    title: "Simple Proofs of Sequential Work"
    target: "https://doi.org/10.1007/978-3-319-78375-8_15"
    author:
      - fullname: Bram Cohen
        initials: B.
        surname: Cohen
      - fullname: Krzysztof Pietrzak
        initials: K.
        surname: Pietrzak
    date: 2018
    seriesinfo:
      "EUROCRYPT 2018, LNCS": "10821, pp. 451-467"
  AlwenBlockPietrzak2017:
    title: "Depth-Robust Graphs and Their Cumulative Memory Complexity"
    target: "https://doi.org/10.1007/978-3-319-78381-9_2"
    author:
      - fullname: Joel Alwen
        initials: J.
        surname: Alwen
      - fullname: Jeremiah Blocki
        initials: J.
        surname: Blocki
      - fullname: Krzysztof Pietrzak
        initials: K.
        surname: Pietrzak
    date: 2017
    seriesinfo:
      "EUROCRYPT 2017, LNCS": "10212, pp. 3-32"
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
  JESD79-5:
    title: "DDR5 SDRAM Standard"
    target: "https://www.jedec.org/standards-documents/docs/jesd79-5d"
    author:
      - org: JEDEC Solid State Technology Association
    date: 2020
    seriesinfo:
      JEDEC: "JESD79-5D"
---

--- abstract

This document defines Proof of Sequential Memory Execution (PoSME),
a cryptographic primitive that fuses sequential computation with
persistent memory evolution. A Prover executes K steps over a
mutable N-block arena, where each step reads data-dependent
addresses via pointer chasing, modifies the arena in-place with
symbiotic binding (data depends on causal hash and vice versa),
and chains a per-block causal hash through every write. The causal
hash creates a dependency web: fabricating any block requires the
full causal ancestry of its writer, recursively. PoSME achieves
latency-bound ASIC resistance (bounded by the DRAM random-access
latency ratio, approximately 2x for DDR5 vs HBM3), asymmetric
verification (O(Q * d * log K * log N) hashes, no arena
allocation), and requires no trusted setup. PoSME is the first
primitive to simultaneously enforce sequential time and persistent
memory occupancy with a single fused mechanism.

--- middle

# Introduction {#introduction}

Existing primitives for proving sequential computation have
complementary weaknesses. Verifiable Delay Functions (VDFs)
{{Boneh2018}} prove sequential time but offer no memory-hardness.
Proofs of Sequential Work (PoSW) {{CohenPietrzak2018}} prove
traversal of a depth-robust graph but operate over static memory.
Memory-hard functions (MHFs) such as Argon2id {{RFC9106}} resist
ASIC acceleration but are single-evaluation primitives with no
chain proof system. Composing these (e.g., chaining Argon2id with
Merkle sampling) produces a construction where sequentiality and
memory-hardness are independent properties; neither reinforces the
other.

PoSME fuses them. A persistent mutable arena IS the computation
state. Each step reads via data-dependent pointer chasing
(sequential because each address depends on the previous read),
modifies the arena in-place (creating evolving state), and chains
a causal hash through every write (creating a dependency web
binding the proof to the full execution history). The data and
causal hash are symbiotically bound: new data depends on the old
causal hash, and the new causal hash depends on the cursor.
Neither can be independently fabricated.

ASIC resistance derives from memory latency, not bandwidth. Each
pointer-chase iteration is bottlenecked by random DRAM access
(~35ns on DDR5 {{JESD79-5}}), with hash computation (~3ns via
BLAKE3) as a minor component. The ASIC advantage is bounded by
the memory latency ratio (approximately 2x for DDR5 vs HBM3),
tighter than the 8-16x bandwidth bounds of Argon2id
{{Biryukov2016}}.

## Related Work {#related-work}

### Proofs of Sequential Work

PoSW {{CohenPietrzak2018}} proves traversal of a depth-robust
graph via Fiat-Shamir-sampled Merkle proofs. PoSME differs: the
graph is a mutable arena (not a static DAG), the access pattern
is data-dependent (not fixed), and each node carries a causal hash
binding its value to its full write history.

### Memory-Hard Functions

Argon2id {{RFC9106}} resists TMTO via bandwidth-hardness. PoSME
uses Argon2id only for arena initialization. The ongoing
computation uses pointer-chasing with in-place writes, creating
latency-hardness, and causal hashes that amplify TMTO penalties
beyond what bandwidth-hard constructions achieve.

### Cumulative Memory Complexity

Alwen, Blocki, and Pietrzak {{AlwenBlockPietrzak2017}} formalized
cumulative memory complexity for static graph pebbling games.
PoSME's causal dependency DAG is dynamic (edges are created during
execution), requiring a new pebbling framework. The dynamic
pebbling analysis is provided in {{tmto}}.

# Conventions and Definitions {#conventions}

{::boilerplate bcp14-tagged}

H:
: BLAKE3 in XOF mode, producing 32-byte output.

XOF(input, index):
: BLAKE3 XOF evaluated at (input \|\| I2OSP(index, 4)),
  producing 4 bytes.

I2OSP(x, len):
: Integer-to-Octet-String Primitive per {{!RFC8017}}.

MerkleRoot(A):
: Merkle tree root over arena blocks using domain-separated
  hashing per {{RFC6962}}.

MerkleUpdate(root, index, new\_value):
: Incremental Merkle root update at the given index.

Prover:
: The entity executing the PoSME computation and generating proofs.

Verifier:
: The entity checking PoSME proofs.

Arena:
: A mutable array of N blocks, each containing a 32-byte data
  field and a 32-byte causal hash.

Causal hash:
: A per-block running hash chain binding each block's value to
  the cursor of the step that wrote it.

# Construction {#construction}

## Arena Block Format {#block-format}

Each arena block is a pair:

~~~ pseudocode
block = {
    data:   bytes[32],
    causal: bytes[32]
}
~~~

The `data` field stores the block's computational value. The
`causal` field stores the causal hash chain: a running digest
binding the block's current value to the cursor of the step
that last wrote it.

## Arena Initialization {#init}

The arena is initialized deterministically from a public seed s:

~~~ pseudocode
for i in 0..N-1:
    if i == 0:
        A[0].data = H("PoSME-init-v1" || s || I2OSP(0, 4))
    else:
        A[i].data = H("PoSME-init-v1" || s || I2OSP(i, 4)
                      || A[i-1].data
                      || A[floor(i/2)].data)
    A[i].causal = H("PoSME-causal-v1" || s || I2OSP(i, 4))

root_0 = MerkleRoot(A)
T_0 = H("PoSME-transcript-v1" || s || root_0)
~~~

The initialization references both the preceding block and a
logarithmic skip-link (floor(i/2)), ensuring all blocks are
entangled and the arena cannot be initialized in constant space.

The Verifier can independently compute root\_0 and T\_0 from the
seed, providing a trusted anchor for all subsequent verification.

## Step Function {#step-function}

At each step t in {1, ..., K}:

~~~ pseudocode
STEP(t):
    // 1. Determine reads-per-step
    d_t = d
    if t mod C == 0:
        d_t = 4 * d              // committed step

    // 2. Pointer-chase reads (data-dependent)
    cursor = T_{t-1}
    addrs = []
    for j in 0..d_t-1:
        a = OS2IP(XOF(cursor, j)) mod N
        addrs.append(a)
        val = A[a]
        cursor = H(cursor || val.data || val.causal)

    // 3. Write with symbiotic binding
    w = OS2IP(XOF(cursor, d_t)) mod N
    old = A[w]
    new_data = H(old.data || cursor || old.causal)
    new_causal = H(old.causal || cursor || I2OSP(t, 4))
    A[w] = {data: new_data, causal: new_causal}

    // 4. Update commitments
    root_t = MerkleUpdate(root_{t-1}, w, A[w])
    T_t = H(T_{t-1} || I2OSP(t, 4) || cursor || root_t)

    // 5. Log step
    log[t] = {d_t, addrs, reads, w, old, A[w], cursor, root_t}
~~~

### Pointer-Chase Addressing {#address-gen}

Read addresses are data-dependent: address j+1 depends on the
value and causal hash of the block read at address j. This creates
a pointer-chasing chain within each step that is inherently
sequential.

### Symbiotic Binding {#symbiotic-binding}

The data update incorporates the old causal hash:

~~~ pseudocode
new_data = H(old_data || cursor || old_causal)
~~~

This creates bidirectional dependency: data depends on causal
history, and causal hash depends on cursor (which depends on
data). Neither can be independently fabricated. An adversary
forging data must know old\_causal; forging causal must know the
cursor; computing the cursor requires reading d blocks with
their causal hashes.

### Committed Steps {#committed-steps}

Every C-th step reads 4\*d blocks instead of d. These committed
steps have 4x the causal dependencies of normal steps, creating
hard checkpoints in the computation. An adversary who attempts
to reconstruct a committed step from a checkpoint must trace
4\*d causal chains instead of d, amplifying the reconstruction
cost by 4x at these points.

The committed step interval C SHOULD be chosen so that the
expected number of committed steps among Q challenges is at
least 4: C <= K / (4 \* Q / K) = K^2 / (4 \* Q).

### Transcript Chain {#transcript-chain}

The transcript chain T\_t binds all steps causally:

~~~ pseudocode
T_t = H(T_{t-1} || I2OSP(t, 4) || cursor || root_t)
~~~

T\_t incorporates root\_t (the Merkle root after the write) and
cursor (which depends on the arena state at step t). Computing
T\_t requires computing all prior steps.

## Root Chain Commitment {#root-chain}

The Prover commits to the sequence of ALL K arena roots:

~~~ pseudocode
R = [root_0, root_1, ..., root_K]
C_roots = MerkleRoot(R)
~~~

This root chain commitment binds the Prover to a specific
sequence of arena states BEFORE Fiat-Shamir challenges are
derived. The challenges depend on (T\_K, C\_roots), and both
must be fixed before the Prover knows which steps will be
challenged.

## Proof Generation {#proof-gen}

~~~ pseudocode
PROVE(K, Q, R_depth):
    C_roots = MerkleRoot([root_0, ..., root_K])
    challenges = FS(T_K, C_roots, Q)
    proof = {params, T_K, C_roots, step_proofs: []}

    for c in challenges:
        sp = make_step_proof(c, R_depth)
        proof.step_proofs.append(sp)
    return proof

make_step_proof(step, depth):
    sp = {
        step_id: step,
        cursor_in: T_{step-1},
        cursor_out: log[step].cursor,
        root_before: root_{step-1},
        root_after: log[step].root_t,
        root_chain_paths: [
            MerklePath(C_roots, step-1),
            MerklePath(C_roots, step)
        ],
        reads: [],
        write: {addr, old, new,
                merkle_path: MerklePath(root_{step-1}, w)},
        writers: []
    }
    for j in 0..d_t-1:
        sp.reads.append({
            addr, block, merkle_path:
                MerklePath(root_{step-1}, addr)})
        if depth > 0:
            ws = last_writer(addr, step)
            if ws == 0:
                sp.writers.append({type: "init",
                    init_path: MerklePath(root_0, addr)})
            else:
                sp.writers.append({type: "step",
                    proof: make_step_proof(ws, depth-1)})
        else:
            sp.writers.append({type: "leaf",
                writer_step: last_writer(addr, step),
                merkle_path: MerklePath(
                    root_{ws}, addr)})
    return sp
~~~

# Verification {#verification}

## Verification Procedure {#verify-procedure}

The Verifier receives (seed, params, T\_K, C\_roots, proof):

~~~ pseudocode
VERIFY(seed, params, T_K, C_roots, proof):
    // 1. Trusted anchor
    root_0 = compute_init_root(seed, params.N)
    T_0 = H("PoSME-transcript-v1" || seed || root_0)

    // 2. Verify root_0 in root chain
    assert MerkleVerify(C_roots, 0, root_0,
                        proof.root_0_path)

    // 3. Recompute challenges
    challenges = FS(T_K, C_roots, params.Q)

    // 4. Verify each challenged step
    for sp in proof.step_proofs:
        verify_step(sp, C_roots, root_0, params)

verify_step(sp, C_roots, root_0, params):
    // A. Verify roots are in the root chain
    assert MerkleVerify(C_roots, sp.step_id - 1,
                        sp.root_before,
                        sp.root_chain_paths[0])
    assert MerkleVerify(C_roots, sp.step_id,
                        sp.root_after,
                        sp.root_chain_paths[1])

    // B. Determine if committed step
    d_t = params.d
    if sp.step_id mod params.C == 0:
        d_t = 4 * params.d

    // C. Verify read Merkle proofs
    for j in 0..d_t-1:
        assert MerkleVerify(sp.root_before,
            sp.reads[j].addr, sp.reads[j].block,
            sp.reads[j].merkle_path)

    // D. Replay pointer-chase
    cursor = sp.cursor_in
    for j in 0..d_t-1:
        a = OS2IP(XOF(cursor, j)) mod N
        assert a == sp.reads[j].addr
        cursor = H(cursor || sp.reads[j].block.data
                           || sp.reads[j].block.causal)

    // E. Verify symbiotic write
    w = OS2IP(XOF(cursor, d_t)) mod N
    assert w == sp.write.addr
    assert MerkleVerify(sp.root_before, w,
                        sp.write.old, sp.write.merkle_path)
    assert sp.write.new.data == H(sp.write.old.data
                                   || cursor
                                   || sp.write.old.causal)
    assert sp.write.new.causal == H(sp.write.old.causal
                                     || cursor
                                     || I2OSP(sp.step_id, 4))

    // F. Verify Merkle root update
    assert sp.root_after == MerkleUpdate(
        sp.root_before, w, sp.write.new)

    // G. Verify transcript chain
    assert H(sp.cursor_in || I2OSP(sp.step_id, 4)
             || cursor || sp.root_after) is consistent

    // H. Recursive causal provenance
    for j in 0..d_t-1:
        verify_writer(sp.writers[j], sp.reads[j],
                      C_roots, root_0, params)
~~~

## Verification Cost {#verify-cost}

For Q challenges with recursion depth R:

- Root chain proofs: O(Q * log K) per challenged step
- Arena Merkle proofs: O(Q * d^R * log N)
- Cursor replays: O(Q * d^R * d)
- No arena memory allocation

For Q=128, d=8, R=3, N=2^24, K=2^20:

| Operation | Count |
|---|---|
| Root chain verifications | 128 * 2 * 20 = ~5K hashes |
| Arena Merkle verifications | 128 * 512 * 24 = ~1.6M hashes |
| Cursor replays | 128 * 512 * 8 = ~524K hashes |
| Total | ~2.1M hashes, ~6ms |

# Security Analysis {#security}

## Soundness via Causal Web {#soundness}

The causal hash mechanism prevents fabrication. To forge a
block's causal hash, the adversary needs the cursor of the step
that wrote it. That cursor depends on d blocks read at the
writer step, each with their own causal hashes requiring their
own writers' cursors, recursively.

Symbiotic binding strengthens this: forging data requires
old\_causal (the block's causal history), and forging old\_causal
requires the prior writer's cursor. The bidirectional dependency
eliminates the possibility of independently fabricating either
field.

The root chain commitment ({{root-chain}}) binds the Prover to
ALL K arena roots before challenges are derived. The Prover
cannot fabricate roots after seeing challenges because C\_roots
is an input to the Fiat-Shamir challenge derivation.

Under collision-resistant H, the probability of passing
verification while having fabricated fraction delta of steps:

~~~ artwork
Pr[accept] <= (1 - delta)^{Q * D_eff} + negl(lambda)
~~~

## Security Reduction {#reduction}

PoSME soundness reduces to collision resistance of H. If an
adversary produces a valid proof with T\_K' != T\_K (the honest
transcript), there exists a step c where the adversary's local
state differs from the honest execution but verification passes.
This requires either:

1. A collision in H (the transcript chain produces the same T\_c
   from different inputs), or
2. A collision in the Merkle commitment (different arena states
   produce the same root).

Both occur with probability at most epsilon\_H (the collision
probability of H). The reduction loses a factor of K (the
number of steps where the divergence could occur).

## TMTO Lower Bound {#tmto}

An adversary storing alpha \* N blocks faces a two-layer penalty:

### Sequential Floor

The transcript chain T\_0 through T\_K must be computed
sequentially to produce T\_K before Fiat-Shamir challenges are
derived. This is an Omega(K) lower bound regardless of storage.

### Reconstruction Amplification

After computing the chain, the adversary discards state. When
challenged, it must reconstruct from checkpoints. With S stored
arena snapshots at spacing C = K/S:

~~~ artwork
T_adv = K * d + Q * (K/S) * d * R

Ratio: T_adv / T_honest = 1 + Q * R / S
~~~

For Q=128, R=3, S=2: the adversary pays 193x honest cost.

Committed steps amplify this further: if a challenged step is
committed (4\*d reads), the reconstruction cost at that step is
4x higher. The expected number of committed steps among Q
challenges is Q \* d / C.

### Dynamic Pebbling Framework

PoSME's causal DAG is dynamic: edges are created during
execution based on data-dependent addressing. In the random
oracle model, each step creates d edges to unpredictable targets.
The resulting DAG has expected depth K and expected in-degree d
at each node.

A pebbling adversary storing alpha \* N pebbles incurs expected
recomputation per challenge proportional to the causal cone size,
which grows as d^R for recursion depth R. The space-time product
for the adversary is:

~~~ artwork
S * T >= Omega(N + K * d * R)
~~~

This bound is informal; a formal proof within the dynamic
pebbling framework remains open.

## ASIC Resistance {#asic-resistance}

PoSME is latency-bound. Each pointer-chase iteration costs:

| Component | Consumer DDR5 | ASIC (HBM3) | Ratio |
|---|---|---|---|
| Memory read | ~35ns | ~20ns | 1.75x |
| BLAKE3 hash | ~3ns | ~0.3ns | 10x |
| **Total** | **~38ns** | **~20.3ns** | **~1.9x** |

The bottleneck is memory latency in both cases. The ASIC
advantage is bounded by the DRAM random-access latency ratio:
approximately 1.75-2x for DDR5 vs HBM3 {{JESD79-5}}, or up to
3x with aggressive controller optimization.

This is tighter than bandwidth-hard constructions (8-16x for
Argon2id {{Biryukov2016}}{{RenDevadas2017}}) and more durable
across technology generations because memory latency improves
more slowly than bandwidth.

## Sequentiality {#sequentiality}

Intra-step: The d reads form a pointer-chasing chain; read j+1's
address depends on read j's result.

Inter-step: T\_t feeds into address generation for step t+1.

Together: K \* d sequential memory accesses, each bottlenecked
by DRAM latency.

# Wire Format {#wire-format}

The PoSME proof is encoded in CBOR {{RFC8949}} per {{RFC8610}}:

~~~ cddl
posme-proof = {
    1 => posme-params,
    2 => bstr .size 32,           ; final-transcript (T_K)
    3 => bstr .size 32,           ; root-chain-commitment
    4 => [+ step-proof],          ; challenged-steps
}

posme-params = {
    1 => uint,                    ; arena-blocks (N)
    2 => uint,                    ; total-steps (K)
    3 => uint,                    ; reads-per-step (d)
    4 => uint,                    ; challenges (Q)
    5 => uint,                    ; recursion-depth (R)
    6 => uint,                    ; committed-step-interval (C)
}

step-proof = {
    1 => uint,                    ; step-id
    2 => bstr .size 32,           ; cursor-in
    3 => bstr .size 32,           ; cursor-out
    4 => bstr .size 32,           ; root-before
    5 => bstr .size 32,           ; root-after
    6 => [+ bstr .size 32],       ; root-chain-paths
    7 => [+ read-witness],        ; reads
    8 => write-witness,           ; write
    9 => [* writer-proof],        ; recursive provenance
}

read-witness = {
    1 => uint,                    ; address
    2 => bstr .size 32,           ; data
    3 => bstr .size 32,           ; causal-hash
    4 => [+ bstr .size 32],       ; merkle-path
}

write-witness = {
    1 => uint,                    ; address
    2 => bstr .size 32,           ; old-data
    3 => bstr .size 32,           ; old-causal
    4 => bstr .size 32,           ; new-data
    5 => bstr .size 32,           ; new-causal
    6 => [+ bstr .size 32],       ; merkle-path
}

writer-proof = {
    1 => uint,                    ; type (0=init, 1=step, 2=leaf)
    ? 2 => uint,                  ; writer-step-id
    ? 3 => step-proof,            ; recursive step proof
    ? 4 => [+ bstr .size 32],     ; merkle-path
}
~~~

# Parameters {#parameters}

## Recommended Parameters {#recommended-params}

| Parameter | Symbol | Recommended | Constraints |
|---|---|---|---|
| Arena blocks | N | 2^24 (16M blocks) | MUST be power of 2 |
| Block size | B | 64 bytes | Fixed |
| Arena memory | M | 1 GiB | MUST exceed L3 cache |
| Steps | K | 2^20 (~1M) | Application-specific |
| Reads per step | d | 8 | MUST be >= 4 |
| Challenges | Q | 128 | MUST be >= 64 |
| Recursion depth | R | 3 | MUST be >= 2 |
| Committed interval | C | 2^12 (4096) | MUST be >= 256 |
| Hash function | H | BLAKE3 | Fixed |

## Parameter Validation {#param-validation}

Verifiers MUST reject proofs with parameters below these minimums:

| Parameter | Minimum | Rationale |
|---|---|---|
| N | 2^20 (64 MiB) | Below this, arena fits in L3 cache |
| K | 2^10 | Below this, sequential chain is trivial |
| d | 4 | Below this, causal fan-out is insufficient |
| Q | 64 | Below this, detection probability < 2^{-64} |
| R | 2 | Below this, causal verification is shallow |
| C | 256 | Below this, committed steps are too frequent |

## Performance Estimates {#performance}

On reference hardware (DDR5, ~35ns random access latency):

| Metric | Value |
|---|---|
| Per-step latency | d * 35ns = 280ns |
| Per-step hash cost | d * 3ns = 24ns |
| K=2^20 execution time | ~0.3 seconds |
| Prover storage | 1 GiB (arena) + ~200 MiB (logs) |
| Verifier time | ~6ms |
| Proof size | ~2-4 MiB |

# Security Considerations {#security-considerations}

## Work vs. Time {#work-vs-time}

PoSME proves sequential memory execution, not elapsed time. An
adversary with faster memory (lower latency) completes the same
computation in less wall-clock time. The ASIC advantage is bounded
(approximately 2x) but nonzero. Applications requiring temporal
guarantees MUST combine PoSME with an external time-binding
mechanism such as hardware-attested timestamps.

Hardware-independent time-binding is fundamentally impossible:
deterministic computation produces identical output regardless of
hardware speed, and self-reported timing is forgeable.

## Seed Requirements {#seed-requirements}

The seed MUST be externally fixed or derived from an unpredictable
source. A Prover-controlled seed enables grinding for favorable
arena initializations with reduced effective working sets.

## Verification Complexity {#verify-complexity}

O(1) verification under hash-only assumptions is impossible for
sequential pointer-chasing computations. K adaptive state
transitions inject K \* log(d) bits of entropy; a constant-size
hash-only proof cannot certify this. The tightest achievable
verification without algebraic assumptions is O(Q \* d^R \* log N)
as specified in this document. O(log^2 K) verification is
achievable via FRI/STARK-based commitment (requiring field
arithmetic but no trusted setup) and is left as a future
optimization.

## Verifier Resource Limits {#verifier-limits}

Verifiers SHOULD implement rate limiting and MUST reject proofs
with parameters exceeding configured thresholds before allocating
resources for verification.

## Open Problems {#open-problems}

The TMTO analysis is an informal argument, not a formal reduction
within the dynamic pebbling framework. The cumulative memory
complexity of PoSME's dynamic causal DAG has not been formally
proven. Block access distribution uniformity under hash-derived
addressing has not been formally characterized; skewed
distributions may enable hot-block caching strategies. The
interaction between committed steps and the optimal adversary
checkpoint strategy requires further analysis.

# IANA Considerations {#iana-considerations}

This document has no IANA actions.

--- back

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The PoSME construction emerged from 58 rounds of multi-model
adversarial iteration. The causal hash mechanism, symbiotic
binding, and committed steps were independently proposed and
stress-tested across this process. The author thanks the CFRG
for foundational work on memory-hard functions.
