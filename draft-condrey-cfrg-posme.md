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
---

--- abstract

This document defines Proof of Sequential Memory Execution (PoSME),
a cryptographic primitive that certifies a Prover executed K
sequential steps over a mutable N-block memory arena. Each step
reads data-dependent addresses, modifies the arena in-place, and
chains a causal hash through every written block. The causal hash
at each block cryptographically binds its value to the cursor of
the step that wrote it, creating a dependency web that cannot be
fabricated without executing the full computation. PoSME provides
ASIC resistance through memory-latency bounds (not bandwidth),
achieves asymmetric verification (the Verifier checks O(Q * d *
R * log N) hashes without allocating the arena), and requires no
trusted setup. PoSME is not a VDF, not a proof of sequential
work, and not a memory-hard function; it is a new primitive that
fuses sequential computation with persistent memory evolution.

--- middle

# Introduction {#introduction}

Existing primitives for proving sequential computation have
complementary weaknesses. Verifiable Delay Functions (VDFs)
{{Boneh2018}} prove sequential time with efficient verification
but offer no memory-hardness; a VDF ASIC gains arbitrary speedup
via faster ALUs. Proofs of Sequential Work (PoSW)
{{CohenPietrzak2018}} prove traversal of a depth-robust graph but
operate over static hash chains, not mutable memory. Memory-hard
functions (MHFs) such as Argon2id {{RFC9106}} resist ASIC
acceleration via memory bandwidth demands but are single-
evaluation primitives with no chain proof system.

Composing these primitives (e.g., chaining Argon2id evaluations
with Merkle-sampled verification) produces a construction, not a
primitive. The sequentiality and memory-hardness are independent
properties bolted together; neither reinforces the other.

PoSME fuses sequential computation and memory-hardness into a
single mechanism. A persistent mutable arena IS the computation
state. Each step reads from the arena via data-dependent pointer
chasing (sequential because each address depends on the previous
read's result), modifies the arena in-place (creating evolving
state that can't be predicted), and chains a causal hash through
every written block (creating a dependency web that binds the
proof to the full execution history).

The ASIC resistance of PoSME derives from memory latency, not
bandwidth. Each pointer-chase step is bottlenecked by a random
DRAM read (~50ns), with hash computation (~3ns via BLAKE3) as a
minor component. The ASIC advantage is bounded by the latency
ratio between consumer DRAM and the fastest available memory
technology (currently 3-5x for HBM3), which is tighter than the
8-16x bandwidth-based bounds of Argon2id {{Biryukov2016}}.

## Related Work {#related-work}

### Proofs of Sequential Work

PoSW {{CohenPietrzak2018}} proves traversal of a depth-robust
graph via Fiat-Shamir-sampled Merkle proofs. PoSME differs in
three ways: the "graph" is a mutable arena (not a static DAG),
the access pattern is data-dependent (not fixed), and each node
carries a causal hash binding its value to its write history.

### Memory-Hard Functions

Argon2id {{RFC9106}} resists TMTO via bandwidth-hardness. PoSME
uses Argon2id only for arena initialization. The ongoing
computation uses pointer-chasing with in-place writes, creating
latency-hardness rather than bandwidth-hardness, and causal hashes
that make TMTO penalties superlinear rather than linear.

### Cumulative Memory Complexity

Alwen, Blocki, and Pietrzak {{AlwenBlockPietrzak2017}} formalized
cumulative memory complexity for graph-based pebbling games. PoSME
can be analyzed in a similar framework but over a dynamic graph
(the causal dependency DAG), not a static one. Formal pebbling
lower bounds for dynamic graphs remain an open problem.

# Conventions and Definitions {#conventions}

{::boilerplate bcp14-tagged}

H:
: BLAKE3 in XOF mode, producing 32-byte output.

XOF(input, index):
: BLAKE3 XOF evaluated at (input || I2OSP(index, 4)),
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
        A[0].data = H(s || I2OSP(0, 4))
    else:
        A[i].data = H(s || I2OSP(i, 4)
                      || A[i-1].data
                      || A[floor(i/2)].data)
    A[i].causal = H("PoSME-init-v1" || s || I2OSP(i, 4))

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
    // 1. Derive read addresses (data-dependent)
    cursor = T_{t-1}
    addrs = []
    for j in 0..d-1:
        a = OS2IP(XOF(cursor, j)) mod N
        addrs.append(a)
        val = A[a]
        cursor = H(cursor || val.data || val.causal)

    // 2. Derive write address and compute new block
    w = OS2IP(XOF(cursor, d)) mod N
    old = A[w]
    new_data = H(old.data || cursor)
    new_causal = H(old.causal || cursor || I2OSP(t, 4))
    A[w] = {data: new_data, causal: new_causal}

    // 3. Update commitments
    root_t = MerkleUpdate(root_{t-1}, w, A[w])
    T_t = H(T_{t-1} || I2OSP(t, 4) || cursor || root_t)

    // 4. Log step (for proof generation)
    log[t] = {addrs, reads, w, old, A[w], cursor, root_t}
~~~

### Address Generation {#address-gen}

Read addresses are data-dependent: the address of read j+1
depends on the value and causal hash of the block read at
address j. This creates a pointer-chasing chain within each
step that is inherently sequential (each memory access depends
on the previous one's result).

The write address depends on the final cursor value after all
reads, which depends on all d read blocks.

### Causal Hash Update {#causal-update}

The causal hash of the written block is updated as:

~~~ pseudocode
new_causal = H(old_causal || cursor || I2OSP(t, 4))
~~~

This binds the block's new value to: (a) its previous causal
history (old\_causal), (b) the cursor at the writing step
(cursor, which depends on all d read blocks and their causal
hashes), and (c) the step identifier (t). The causal hash
chain for a block is the ordered sequence of all writes to
that block, each incorporating the writer's full cursor state.

### Transcript Chain {#transcript-chain}

The transcript chain T\_t binds all steps causally:

~~~ pseudocode
T_t = H(T_{t-1} || I2OSP(t, 4) || cursor || root_t)
~~~

Because T\_t incorporates root\_t (the Merkle root after the
write) and cursor (which depends on the arena state at step t),
the chain is a sequential commitment to the full execution
history. Computing T\_t requires computing all prior steps.

## Proof Generation {#proof-gen}

After executing all K steps, the Prover derives Fiat-Shamir
challenges and constructs the proof:

~~~ pseudocode
PROVE(K, Q, R):
    challenges = FS_Challenge(T_K, root_K, Q)
    proof = {params, T_K, root_K, step_proofs: []}

    for c in challenges:
        sp = make_step_proof(c, R)
        proof.step_proofs.append(sp)

    return proof

make_step_proof(step, depth):
    l = log[step]
    sp = {
        step_id: step,
        cursor_in: T_{step-1},
        cursor_out: l.cursor,
        root_before: root_{step-1},
        root_after: l.root_t,
        reads: [],
        write: {addr: l.w, old: l.old, new: l.A_w,
                merkle_path: MerklePath(root_{step-1}, l.w)},
        writers: []
    }
    for j in 0..d-1:
        sp.reads.append({
            addr: l.addrs[j],
            block: l.reads[j],
            merkle_path: MerklePath(root_{step-1}, l.addrs[j])
        })
        // Recursive provenance
        if depth > 0:
            writer_step = last_writer(l.addrs[j], step)
            if writer_step == 0:
                sp.writers.append({type: "init",
                    init_path: MerklePath(root_0, l.addrs[j])})
            else:
                sp.writers.append({type: "step",
                    proof: make_step_proof(writer_step,
                                           depth - 1)})
        else:
            sp.writers.append({type: "leaf",
                writer_step: last_writer(l.addrs[j], step),
                merkle_path: MerklePath(
                    root_{last_writer(...)}, l.addrs[j])})

    return sp
~~~

The `last_writer(addr, before_step)` function returns the most
recent step that wrote to address `addr` prior to `before_step`.
The Prover maintains a write index for efficient lookup.

# Verification {#verification}

## Verification Procedure {#verify-procedure}

The Verifier receives (seed, params, T\_K, root\_K, proof) and
performs:

~~~ pseudocode
VERIFY(seed, params, T_K, root_K, proof):
    // 1. Compute trusted anchor
    root_0 = compute_init_root(seed, params.N)
    T_0 = H("PoSME-transcript-v1" || seed || root_0)

    // 2. Recompute challenges
    challenges = FS_Challenge(T_K, root_K, params.Q)
    assert len(proof.step_proofs) == params.Q

    // 3. Verify each challenged step
    for i in 0..Q-1:
        assert proof.step_proofs[i].step_id == challenges[i]
        verify_step(proof.step_proofs[i], root_0, params)

verify_step(sp, root_0, params):
    // A. Verify all read Merkle proofs
    for j in 0..d-1:
        r = sp.reads[j]
        assert MerkleVerify(sp.root_before, r.addr,
                            r.block, r.merkle_path)

    // B. Replay pointer-chase cursor computation
    cursor = sp.cursor_in
    for j in 0..d-1:
        a = OS2IP(XOF(cursor, j)) mod N
        assert a == sp.reads[j].addr
        cursor = H(cursor || sp.reads[j].block.data
                           || sp.reads[j].block.causal)

    // C. Verify write
    w = OS2IP(XOF(cursor, d)) mod N
    assert w == sp.write.addr
    assert MerkleVerify(sp.root_before, w,
                        sp.write.old, sp.write.merkle_path)
    assert sp.write.new.data == H(sp.write.old.data || cursor)
    assert sp.write.new.causal == H(sp.write.old.causal
                                     || cursor
                                     || I2OSP(sp.step_id, 4))

    // D. Verify Merkle root update
    assert sp.root_after == MerkleUpdate(sp.root_before,
                                          w, sp.write.new)

    // E. Verify transcript chain step
    assert T_{sp.step_id} == H(sp.cursor_in
                                || I2OSP(sp.step_id, 4)
                                || cursor || sp.root_after)

    // F. Verify recursive provenance (causal web)
    for j in 0..d-1:
        verify_writer(sp.writers[j], sp.reads[j],
                      root_0, params)

verify_writer(wp, read, root_0, params):
    if wp.type == "init":
        // Block was never written; verify init value
        assert MerkleVerify(root_0, read.addr,
                            read.block, wp.init_path)
    elif wp.type == "step":
        // Recursively verify the writer step
        verify_step(wp.proof, root_0, params)
        // Verify this step wrote the block we read
        assert wp.proof.write.addr == read.addr
        assert wp.proof.write.new == read.block
    elif wp.type == "leaf":
        // Truncated: verify block exists at writer's root
        assert MerkleVerify(root_{wp.writer_step},
                            read.addr, read.block,
                            wp.merkle_path)
~~~

## Verification Cost {#verify-cost}

For Q challenged steps, each with d reads, each with recursive
provenance to depth R:

- Merkle proof verifications: O(Q * d^R * log N)
- Hash evaluations for cursor replay: O(Q * d^R * d)
- No arena memory allocation
- No Argon2id re-execution

For recommended parameters (Q=128, d=8, R=3, N=2^24):

| Operation | Count | Cost |
|---|---|---|
| Merkle verifications | 128 * 8^3 * 24 = ~1.6M | ~1.6M hashes |
| Cursor replays | 128 * 8^3 * 8 = ~524K | ~524K hashes |
| Total hash evaluations | ~2.1M | ~6ms at 3ns/hash |

Verification completes in under 10ms on commodity hardware with
no memory allocation beyond the proof data itself.

# Security Analysis {#security}

## Soundness: The Causal Web {#soundness}

The causal hash mechanism prevents the fabrication attack
identified in the PoSME design process: a dishonest Prover
producing locally-consistent Merkle roots and block values
without executing the full computation.

To fabricate a read block's causal hash, the adversary needs the
cursor of the step that last wrote that block. That cursor depends
on all d blocks read at the writer step, each with their own
causal hashes, each requiring their own writers' cursors. This
creates a transitive dependency web: fabricating one node requires
fabricating its entire causal ancestry.

Under collision-resistant H, producing a valid causal hash for a
block without knowing the writer's cursor requires finding a
collision in H, which has negligible probability.

The recursive provenance check (depth R) forces the adversary to
open R levels of the causal web. For each level, d new causal
hashes must be verified. An adversary who fabricated any node in
the web must fabricate all d^R nodes in the opened subtree, each
consistently. The probability of passing verification while having
fabricated fraction delta of steps is at most:

~~~ artwork
Pr[accept] <= (1 - delta)^{Q * D_eff} + epsilon_H

where D_eff is the effective depth of causal coverage
and epsilon_H is the collision probability of H.
~~~

## TMTO Lower Bound {#tmto}

An adversary storing alpha * N arena blocks faces a two-layer
cost penalty:

### Layer 1: Sequential Chain

The adversary must compute the transcript chain T\_0 through T\_K
to produce T\_K before Fiat-Shamir challenges are derived.
Computing T\_t requires cursor\_t, which requires reading d arena
blocks at step t. This chain is inherently sequential: each step
depends on the previous step's output. No storage strategy
eliminates this Omega(K) lower bound.

### Layer 2: Reconstruction Amplification

After computing the chain, the adversary discards intermediate
state to save storage. When challenged, it must reconstruct
from checkpoints. Storing S arena snapshots at uniform intervals
creates checkpoint spacing C = K/S. Per challenged step, the
adversary must:

1. Replay from the nearest checkpoint: C * d hash evaluations
2. Reconstruct causal provenance to depth R: up to C * d * R
   evaluations per challenged read

Total adversary cost:

~~~ artwork
T_adv = K * d + Q * (K/S) * d * R

Honest cost: T_honest = K * d

Ratio: T_adv / T_honest = 1 + Q * R / S
~~~

For Q=128, R=3, S=2: ratio = 1 + 192 = 193x.

The adversary who reduces storage to 2 arena snapshots pays
193x the honest computation cost. This ratio increases linearly
with Q and R, and inversely with S.

Optimal adversary strategy: minimize S * N * B + Q * K * d * R / S.
The optimal S is sqrt(Q * K * d * R / (N * B)), which for
recommended parameters yields S ~ 2-3 snapshots with a cost
ratio exceeding 170x.

## Sequentiality {#sequentiality}

PoSME sequentiality has two layers:

Intra-step: The d reads within each step form a pointer-chasing
chain. Read j+1's address depends on read j's result. This
creates d sequential memory accesses per step.

Inter-step: The transcript chain T\_t = H(T\_{t-1} || ...) feeds
into address generation for step t+1. Step t+1 cannot begin
until step t's cursor is known.

Together, these create K * d sequential memory accesses, each
bottlenecked by DRAM latency.

## ASIC Resistance {#asic-resistance}

PoSME is latency-bound, not bandwidth-bound. Each pointer-chase
iteration costs:

| Component | Consumer DDR5 | ASIC (HBM3) | Ratio |
|---|---|---|---|
| Memory read | ~35ns | ~20ns | 1.75x |
| BLAKE3 hash | ~3ns | ~0.3ns | 10x |
| **Total (bottleneck)** | **~38ns** | **~20.3ns** | **1.87x** |

The hash computation is 8% of the total cost on consumer hardware
and negligible on an ASIC. The bottleneck is memory latency in
both cases. The effective ASIC advantage is bounded by the memory
latency ratio: approximately 1.75-2x for DDR5 vs HBM3, or up to
3-5x accounting for aggressive caching and controller optimization.

This is substantially tighter than bandwidth-hard constructions
such as Argon2id, where the ASIC advantage is 8-16x
{{Biryukov2016}}{{RenDevadas2017}}.

Memory latency improves more slowly than bandwidth across
technology generations (physics-constrained by signal propagation
and DRAM cell sensing time), making PoSME's ASIC resistance more
durable than bandwidth-based constructions.

# Wire Format {#wire-format}

The PoSME proof is encoded in CBOR {{RFC8949}} per {{RFC8610}}:

~~~ cddl
posme-proof = {
    1 => posme-params,
    2 => bstr .size 32,           ; final-transcript (T_K)
    3 => bstr .size 32,           ; final-root (root_K)
    4 => [+ step-proof],          ; challenged-steps
}

posme-params = {
    1 => uint,                    ; arena-blocks (N)
    2 => uint,                    ; total-steps (K)
    3 => uint,                    ; reads-per-step (d)
    4 => uint,                    ; challenges (Q)
    5 => uint,                    ; recursion-depth (R)
}

step-proof = {
    1 => uint,                    ; step-id
    2 => bstr .size 32,           ; cursor-in
    3 => bstr .size 32,           ; cursor-out
    4 => bstr .size 32,           ; root-before
    5 => bstr .size 32,           ; root-after
    6 => [+ read-witness],        ; reads
    7 => write-witness,           ; write
    8 => [* writer-proof],        ; recursive provenance
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
    ? 4 => [+ bstr .size 32],     ; merkle-path (init or leaf)
}
~~~

## Proof Size {#proof-size}

Each read-witness is approximately 32 + 32 + 24 * 32 = 832
bytes (with log2(N) = 24 Merkle path). Each step-proof contains
d read-witnesses plus one write-witness plus recursive writer
proofs.

For Q=128, d=8, R=3, N=2^24:

| Component | Size |
|---|---|
| Per read-witness | ~832 bytes |
| Per step-proof (d=8 reads + write) | ~8 KiB |
| Total without recursion (Q=128) | ~1 MiB |
| With R=3 recursion (worst case d^R=512 sub-proofs) | ~4 MiB |
| With Merkle path deduplication | ~2-3 MiB |

Proof size is 2-4 MiB for recommended parameters. Applications
requiring smaller proofs SHOULD reduce d or R, accepting reduced
security guarantees.

# Parameters {#parameters}

## Recommended Parameters {#recommended-params}

| Parameter | Symbol | Recommended | Constraints |
|---|---|---|---|
| Arena blocks | N | 2^24 (16M blocks) | MUST be power of 2 |
| Block size | B | 64 bytes | Fixed |
| Arena memory | M = N*B | 1 GiB | MUST exceed L3 cache |
| Steps | K | 2^20 (~1M) | Application-specific |
| Reads per step | d | 8 | MUST be >= 4 |
| Challenges | Q | 128 | MUST be >= 64 |
| Recursion depth | R | 3 | MUST be >= 2 |
| Hash function | H | BLAKE3 | Fixed for this version |

## Parameter Validation {#param-validation}

Verifiers MUST reject proofs with parameters below security
thresholds:

| Parameter | Minimum | Rationale |
|---|---|---|
| N | 2^20 (1M blocks, 64 MiB) | Below this, arena fits in L3 cache |
| K | 2^10 (1024 steps) | Below this, sequential chain is trivial |
| d | 4 | Below this, pointer-chase has low fan-out |
| Q | 64 | Below this, detection probability < 2^{-64} |
| R | 2 | Below this, causal web verification is shallow |

## Performance Estimates {#performance}

On reference hardware (DDR5, ~35ns random access latency):

| Metric | Value |
|---|---|
| Per-step latency | d * 35ns = 280ns |
| Per-step hash cost | d * 3ns = 24ns |
| Steps per second | ~3.3M |
| K=2^20 execution time | ~0.3 seconds |
| Prover storage | N * B = 1 GiB (arena) + ~200 MiB (logs) |
| Verifier time | ~6ms |
| Proof size | ~2-3 MiB |

# Security Considerations {#security-considerations}

## Proof System Limitations {#limitations}

PoSME proves sequential memory execution, not elapsed time. An
adversary with faster hardware (lower memory latency) completes
the same computation in less wall-clock time. Applications
requiring temporal guarantees MUST combine PoSME with an external
time-binding mechanism.

## Seed Requirements {#seed-requirements}

The seed MUST be externally fixed or derived from an unpredictable
source. If the Prover controls the seed, they can grind for
favorable arena initializations that reduce the effective working
set or create exploitable access patterns.

## Verifier Resource Limits {#verifier-limits}

Proof verification requires processing 2-4 MiB of proof data and
performing ~2M hash evaluations. Verifiers SHOULD implement rate
limiting and MUST reject proofs with parameters exceeding
configured thresholds before processing the proof body.

## Causal Depth Limitations {#causal-depth}

Recursive provenance to depth R provides probabilistic soundness.
Deeper R improves security but increases proof size
exponentially (proof grows as d^R). Applications MUST balance R
against proof size constraints. R=3 with d=8 provides 512-node
causal subtree verification per challenged step.

## Open Problems {#open-problems}

Formal pebbling lower bounds for PoSME's dynamic causal DAG have
not been proven. The TMTO analysis in {{tmto}} is an informal
argument, not a reduction to a standard hardness assumption.
Analysis of the adversary's optimal caching strategy within causal
cone traversal remains open. Block access distribution uniformity
has not been formally characterized; skewed distributions may
enable hot-block caching strategies.

# IANA Considerations {#iana-considerations}

This document has no IANA actions.

--- back

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The PoSME construction emerged from multi-model adversarial
iteration involving GPT-5.4, Grok-4.20, Gemini-3.1-Pro,
Qwen-3.6, DeepSeek-V3.2, o3-Pro, Llama-4-Maverick, and
Mistral-Large-3. The causal hash mechanism was independently
proposed by multiple panelists in response to a soundness flaw
identified during the design process.

The author thanks the CFRG for foundational work on memory-hard
functions and the authors of Argon2 for the initialization
primitive used in PoSME.
