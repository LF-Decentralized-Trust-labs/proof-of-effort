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
a cryptographic primitive combining mutable arena state, data-
dependent pointer-chase addressing, and per-block causal hash
binding in a single step function. A Prover executes K sequential
steps over a mutable N-block arena. Each step reads d blocks at
addresses determined by the previous read's result (pointer
chasing), writes one block with symbiotic binding (new data
depends on old causal hash; new causal hash depends on cursor),
and advances a transcript chain. The construction provides four
properties: (1) unconditional sequential time enforcement
(Omega(K) computation regardless of storage), (2) forgery
prevention via causal hashes (reduces to collision resistance of
H), (3) TMTO resistance scaling linearly with write density rho =
K/N (10x penalty at rho=4 for a zero-storage adversary), and (4)
latency-bound ASIC resistance (approximately 2x for DDR5 vs HBM3,
tighter than the 8-16x of bandwidth-hard constructions).
Verification requires O(Q * d^R * log N) hash evaluations with no
arena allocation. No trusted setup is required.

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

PoSME takes a different approach. A persistent mutable arena IS
the computation state. Each step reads via data-dependent pointer
chasing (sequential because each address depends on the previous
read's result) and modifies the arena in-place. A per-block
causal hash chain binds each block's value to the cursor of the
step that wrote it, preventing forgery (the adversary cannot
produce a valid causal hash without knowing the writer's cursor,
which depends on d other blocks' causal hashes, recursively).
The data and causal hash are symbiotically bound: new data
depends on the old causal hash, and the new causal hash depends
on the cursor.

The primary contribution is latency-bound ASIC resistance. Each
pointer-chase iteration is bottlenecked by random DRAM access
(~35ns on DDR5 {{JESD79-5}}), with hash computation (~3ns via
BLAKE3) as a minor component. The ASIC advantage is bounded by
the memory latency ratio (approximately 2x for DDR5 vs HBM3),
tighter than the 8-16x bandwidth bounds of Argon2id
{{Biryukov2016}}. Memory latency improves more slowly than
bandwidth across technology generations (constrained by signal
propagation and DRAM cell sensing time), making this bound more
durable than bandwidth-based resistance.

## Related Work {#related-work}

### Proofs of Sequential Work

PoSW {{CohenPietrzak2018}} proves traversal of a depth-robust
graph via Fiat-Shamir-sampled Merkle proofs. PoSME differs: the
graph is a mutable arena (not a static DAG), the access pattern
is data-dependent (not fixed), and each node carries a causal hash
binding its value to its full write history.

### Memory-Hard Functions

Argon2id {{RFC9106}} resists TMTO via bandwidth-hardness, with a
single-pass TMTO penalty of approximately 2x. PoSME uses Argon2id
only for arena initialization. The ongoing computation uses
pointer-chasing with in-place writes, creating latency-hardness
(2x ASIC bound vs Argon2id's 8-16x). PoSME's TMTO penalty is
approximately 2+2\*rho for zero-storage adversaries, where
rho = K/N is the write density (10x at rho=4 vs Argon2id's 2x).

### Proofs of Space-Time

Proofs of Space-Time (PoST) {{CohenPietrzak2018}} enforce both
sequential time and persistent storage by requiring a Prover to
repeatedly prove possession of stored data over a sequence of
time intervals. PoST operates over a static graph: the stored
data does not change between proofs, and the graph structure is
fixed before execution. PoSME differs in that the arena is
mutable (each step modifies it), the access pattern is data-
dependent (addresses are determined by arena contents, not
pre-computed), and each block carries a causal hash binding its
current value to its write history. These differences make PoSME
a different construction with different TMTO characteristics,
not a strict improvement over PoST.

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

    // G. Compute and store transcript value for cross-check
    T_c = H(sp.cursor_in || I2OSP(sp.step_id, 4)
            || cursor || sp.root_after)
    // If another challenged step c' has cursor_in == T_c,
    // verify they match. If sp.step_id == K, verify
    // T_c == T_K (the public final transcript).
    stored_transcripts[sp.step_id] = T_c

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

For Q=128, d=8, R=3, N=2^24, K=2^24:

| Operation | Count |
|---|---|
| Root chain verifications | 128 * 2 * 20 = ~5K hashes |
| Arena Merkle verifications | 128 * 512 * 24 = ~1.6M hashes |
| Cursor replays | 128 * 512 * 8 = ~524K hashes |
| Total | ~2.1M hashes, ~6ms |

# Security Analysis {#security}

## Forgery Prevention {#soundness}

The causal hash mechanism prevents block value fabrication.
To forge a block's causal hash, the adversary needs the cursor
of the step that wrote it. That cursor depends on d blocks read
at the writer step, each with their own causal hashes requiring
their own writers' cursors, recursively. Symbiotic binding
strengthens this: forging data requires old\_causal, and forging
old\_causal requires the prior writer's cursor. Neither field
can be independently fabricated.

The root chain commitment ({{root-chain}}) binds the Prover to
ALL K arena roots before challenges are derived. C\_roots is an
input to the Fiat-Shamir challenge derivation, so the Prover
cannot fabricate roots after seeing challenges.

This is a SOUNDNESS property: it prevents the adversary from
producing a valid-looking proof without executing the computation.
It reduces to collision resistance of H: if an adversary produces
T\_K' != T\_K with a valid proof, there exists a step c where the
local state diverges. This requires either a collision in H
(the transcript chain produces the same T\_c from different
inputs) or a collision in the Merkle commitment. Both occur with
probability at most K \* epsilon\_H, where epsilon\_H is the
collision probability of H.

## Recomputation Cost {#recomp-cost}

Separately from forgery prevention, causal hashes impose a
constant-factor increase on the cost of recomputing missing
blocks. Without causal hashes, an adversary recomputing a
missing block traverses its write chain at cost O(rho) hashes
(one per write in the chain). With causal hashes, the adversary
must traverse both the data chain and the causal chain, doubling
the cost to O(2\*rho) per miss.

This is a MODERATE improvement: a 2x constant factor on write
chain traversal, not an exponential blowup. The TMTO penalty
table in {{tmto}} incorporates this factor. The causal hash
mechanism's primary contribution is soundness ({{soundness}}),
not TMTO amplification.

## TMTO Lower Bound {#tmto}

An adversary storing alpha \* N blocks faces a two-layer penalty:

### Sequential Floor {#sequential-floor}

The transcript chain T\_0 through T\_K must be computed
sequentially to produce T\_K before Fiat-Shamir challenges are
derived. This is an Omega(K) lower bound regardless of storage.

### Write Density and Arena Coverage {#write-density}

Each step writes 1 block at a uniformly random address (in the
ROM). After K steps, the fraction of blocks written at least once
is phi = 1 - e^{-rho}, where rho = K/N is the write density.

A block that was never written retains its initialization value,
recomputable in O(1) from the seed. Only written blocks require
write chain traversal for recomputation. The expected write chain
length for a modified block is rho, costing O(rho) hashes.

| rho = K/N | Blocks modified | Recomp cost/miss | TMTO effect |
|---|---|---|---|
| < 1 | < 63% | O(1) | Weak: most blocks free |
| 1 | 63% | O(1) | Moderate |
| 4 | 98% | O(4) | Strong |
| 16 | ~100% | O(16) | Very strong |

K MUST be at least N (rho >= 1) for meaningful TMTO resistance.
Values of rho >= 4 are RECOMMENDED.

### Per-Step Recomputation Cost

An adversary storing alpha \* N arena blocks must still compute
the full K-step transcript chain ({{sequential-floor}}). At
each step, d blocks are read. Each read missing the stored set
(probability 1-alpha per read, since addresses are uniform in
the ROM) requires write-chain traversal at expected cost
2\*rho + 1 hashes. The per-step cost becomes
d \* (1 + (1-alpha) \* (2\*rho + 1)), giving the TMTO ratio:

~~~ artwork
T_adv / T_honest >= 1 + (1-alpha) * (2*rho + 1)
~~~

| rho = K/N | alpha=0 penalty | alpha=0.5 penalty |
|---|---|---|
| 1 | 4x | 2.5x |
| 4 | 10x | 5.5x |
| 16 | 34x | 17.5x |

The penalty scales linearly with rho. Committed steps (4\*d
reads) quadruple the miss rate at committed step positions.

This bound assumes the adversary stores all K cursors
(K \* 32 bytes; 512 MiB for K = 2^26). Storing all cursors is
optimal for the adversary: it eliminates the need for sequential
replay from checkpoints. An adversary who stores cursors at
intervals of L steps instead pays an additional L \* d \*
(1 + (1-alpha) \* (2\*rho + 1)) hashes per cursor miss. Sparse
cursor storage strictly increases the adversary's cost; the
bound above is a LOWER bound on the TMTO penalty.

### Space-Time Product

In the random oracle model, the adversary's space-time product:

~~~ artwork
M * T >= Omega(N * K * d * min(1, rho))
~~~

For rho >= 1 (K >= N), this equals Omega(N \* K \* d), matching
the honest prover. No adversary can reduce both space and time.

### Dynamic Pebbling Game {#pebbling-game}

PoSME's causal DAG is dynamic: edges are created during
execution based on data-dependent addressing. In the random
oracle model, each step creates d edges to uniformly random
targets. The pebbling game:

1. N block nodes (arena) and K step nodes.
2. At step t, the game reveals d random read addresses.
3. To execute step t, the adversary must have pebbles on all
   d read addresses (stored or recomputed at cost O(rho)).
4. The adversary maintains auxiliary state (cursors, write
   index) of at most K \* 32 bytes.

Any adversary storing alpha \* N blocks and all K cursors
performs expected computation:

~~~ artwork
T_adv >= K * d * (1 + (1-alpha) * (2*rho + 1))
~~~

The honest cost is K \* d. The TMTO ratio is
1 + (1-alpha) \* (2\*rho + 1). For alpha=0, rho=4:
the adversary pays 10x honest cost. The penalty is LINEAR
in rho, not exponential.

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
| Steps | K | 4\*N (64M) | MUST be >= N; rho=K/N >= 4 RECOMMENDED ({{write-density}}) |
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
| K | N | Below N, most blocks are never written; TMTO is trivial |
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
| K=4\*N execution time | ~18.8 seconds |
| TMTO penalty (alpha=0) | 10x (rho=4) |
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

The dynamic pebbling game ({{pebbling-game}}) provides a framework
for TMTO analysis, but a machine-checked proof of the space-time
lower bound remains open. The adversary's optimal caching strategy
(which blocks to store, when to checkpoint) has not been formally
optimized. Block access distribution uniformity under hash-derived
addressing requires formal characterization; skewed distributions
may enable hot-block caching. The tight relationship between
committed step frequency C, write density rho, and the optimal
adversary strategy requires further analysis.

# IANA Considerations {#iana-considerations}

This document has no IANA actions.

--- back

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The author thanks the CFRG for foundational work on memory-hard
functions, and the authors of Argon2 for the initialization
primitive used in PoSME.
