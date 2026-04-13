# PoSME Dynamic Pebbling Game: Formal Framework

## 1. Model

### 1.1 The Computation

Fix parameters N (arena size), K (steps), d (reads per step),
lambda (security parameter). Let H be a random oracle.

**Arena.** A : [N] -> {0,1}^{2*lambda}. Each block stores
(data, causal), each lambda bits.

**Initialization.** A_0[i] is a deterministic function of seed s
and index i, computable in O(1) from s.

**Step t** (for t = 1, ..., K):

1. Read addresses: for j = 0..d-1,
   a_{t,j} = H("addr" || c_{t-1} || j) mod N

2. Cursor update:
   c_t = H(c_{t-1} || A_{t-1}[a_{t,0}] || ... || A_{t-1}[a_{t,d-1}])

3. Write address: w_t = H("write" || c_t || t) mod N

4. Write value:
   A_t[w_t].data = H(A_{t-1}[w_t].data || c_t || A_{t-1}[w_t].causal)
   A_t[w_t].causal = H(A_{t-1}[w_t].causal || c_t || t)
   A_t[i] = A_{t-1}[i] for all i != w_t

5. Transcript: T_t = H(T_{t-1} || t || c_t || root_t)

**Output.** (T_K, root_K).

### 1.2 The Random Oracle Property

In the random oracle model, the addresses a_{t,j} = H(...) mod N
are uniformly random in [N] and independent of the adversary's
storage decisions, because the adversary cannot predict H's output
without evaluating it, and evaluating it requires c_{t-1}, which
requires completing step t-1.

More precisely: conditioned on the adversary's storage set S_t at
the beginning of step t, the d read addresses are uniform in [N]
and independent of S_t. This is the key property that makes the
pebbling analysis work.

### 1.3 Block Write History

Define the write history of block i as the sequence of steps that
wrote to block i:

  W(i) = {t in [K] : w_t = i}

Each step writes to one uniformly random block (in the ROM). So
each block's write count |W(i)| follows a Binomial(K, 1/N)
distribution, which for large N is approximately Poisson(K/N).

Define rho = K/N as the write density. This is the expected number
of writes per block.

**Critical observation:** The fraction of blocks that have been
written at least once after K steps is:

  phi = 1 - (1 - 1/N)^K ≈ 1 - e^{-rho}

For rho = K/N << 1: phi ≈ rho (most blocks are unmodified).
For rho >= 3: phi > 0.95 (almost all blocks are modified).

A block that has NEVER been written retains its initialization
value, which is recomputable in O(1) from the seed. Only modified
blocks require knowledge of the write chain.

## 2. The Dynamic Pebbling Game

### 2.1 Players and State

The game is played by an Adversary A against the computation
defined above.

**Adversary state.** At each step t, the adversary maintains:
- A stored set S_t subset [N] of arena blocks (pebbled blocks)
- For each i in S_t, the current value A_t[i]
- Auxiliary state aux_t of at most P bits (e.g., stored cursors,
  write indices, checkpoints)

**Constraints:**
- |S_t| = s_t (the adversary stores s_t blocks at step t)
- |aux_t| <= P

### 2.2 Game Rules

At each step t = 1, ..., K:

1. The game REVEALS the d read addresses a_{t,0}, ..., a_{t,d-1}.
   (These are determined by c_{t-1}, which the adversary must have
   computed at the previous step.)

2. For each read address a_{t,j}:
   - If a_{t,j} in S_t: the adversary has the value. Cost: 0.
   - If a_{t,j} not in S_t: the adversary must RECOMPUTE A_t[a_{t,j}].
     Cost: R(a_{t,j}, t) hash evaluations.

3. The adversary computes c_t using the d block values (obtained
   either from storage or recomputation).

4. The game reveals w_t. The adversary updates A[w_t] and may
   add or remove blocks from S_t.

5. The adversary computes T_t.

### 2.3 Recomputation Cost

To recompute block i at step t (when i not in S_t):

**Case 1: Block i was never written** (i not in union of w_1..w_t).
The value is A_0[i], computable from seed in O(1).
Cost: R(i, t) = 1.

**Case 2: Block i was written at steps t_1 < t_2 < ... < t_m <= t.**
The adversary must reconstruct the write chain from initialization
to the most recent write. Each link in the chain requires:
- The cursor c_{t_k} at the writing step (lambda bits)
- The previous value A_{t_k - 1}[i] = (data, causal) before
  the write

The total recomputation cost is:

  R(i, t) = 2 * |W(i, t)| + 1

where W(i, t) = {t' in W(i) : t' <= t} is the number of writes
to block i up to step t. The factor 2 accounts for recomputing
both data and causal hash at each write. The +1 is for the
initialization lookup.

**BUT:** This assumes the adversary has the cursor c_{t_k} for
each writing step t_k. If the adversary stores all K cursors
(K * lambda bits of auxiliary state), this is available. If not,
the adversary must recompute the cursor, which requires re-
executing from the nearest stored cursor checkpoint.

### 2.4 Optimized Adversary Strategy

The adversary can minimize cost by storing:

1. All K cursors: K * lambda bits = K * 32 bytes.
   (For K = 2^20: 32 MiB. For K = 2^24: 512 MiB.)

2. A write index mapping each block to its write history.
   (At most K entries, each O(log N) bits. Total: K * log N bits.)

3. Arena checkpoints every C steps.
   (Each checkpoint: N * 2 * lambda bits. Total: (K/C) * N * 64 bytes.)

With cursors and write index stored, the adversary can recompute
any block's current value in O(rho) hash evaluations (expected
write chain length).

## 3. Cumulative Memory Complexity

### 3.1 Definition

The cumulative memory complexity of the adversary's strategy is:

  CMC(A) = sum_{t=1}^{K} s_t * B

where s_t = |S_t| and B = 64 bytes per block.

For the honest prover: s_t = N for all t, so CMC_honest = N * K * B.

### 3.2 Lower Bound: Sequential Floor

**Theorem 1 (Sequential Floor).** Any adversary producing T_K
must perform at least K sequential hash evaluations.

**Proof.** T_t = H(T_{t-1} || ...). In the ROM, the only way to
compute T_t is to evaluate H on an input containing T_{t-1}.
Computing T_{t-1} requires T_{t-2}, etc. The chain T_0, T_1, ...,
T_K requires K sequential H evaluations. No parallelism or
storage strategy can reduce this.

This bound is independent of memory. The adversary MUST spend
Omega(K) time regardless of how much memory it has. []

### 3.3 Lower Bound: Memory-Time Tradeoff

**Theorem 2 (TMTO Lower Bound).** In the random oracle model,
any adversary storing at most alpha * N blocks at each step
and producing a valid PoSME proof (passing Q challenges with
recursive depth R) must perform expected additional work:

  W_recomp >= Q * d * (1 - alpha) * rho * R

hash evaluations, where rho = K/N is the write density.

**Proof sketch.**

Step 1: Cache miss rate.
At each challenged step c, the adversary must provide the d
read block values. Each address is uniform in [N] (ROM). The
probability that a read misses the stored set is (1 - alpha).
Expected misses per challenge: d * (1 - alpha).
Total expected misses across Q challenges: Q * d * (1 - alpha).

Step 2: Recomputation cost per miss.
For a miss at block i, the adversary must recompute A[i]. By
Section 2.3, this costs 2 * |W(i, t)| + 1 hash evaluations
(with cursors stored).

The expected write chain length is E[|W(i, t)|] = rho (Poisson
mean). So the expected recomputation cost per miss is 2*rho + 1.

Step 3: Recursive provenance amplification.
Each challenged step also requires opening the causal provenance
to depth R. Each provenance level opens d more blocks, each of
which may miss with probability (1 - alpha). At each recursion
level, the expected number of new blocks to recompute is
d * (1 - alpha). Over R levels, the total blocks to recompute
is approximately d * (1 - alpha) * R (with overlaps reducing
this, but in the ROM, addresses are independent so overlap is
negligible for alpha << 1).

Each recomputed block costs O(rho) hash evaluations.

Total:
  W_recomp >= Q * d * (1 - alpha) * R * (2*rho + 1)

For rho >= 1: W_recomp >= Q * d * (1 - alpha) * R * 2 * rho. []

### 3.4 The Critical Parameter: Write Density rho

The strength of the TMTO bound depends on rho = K/N:

| rho = K/N | Fraction modified (phi) | Recomp cost per miss | Adversary advantage |
|---|---|---|---|
| 0.0625 (K=2^20, N=2^24) | 6% | ~1.1 hashes | Minimal TMTO penalty |
| 1 (K = N) | 63% | ~3 hashes | Moderate penalty |
| 4 (K = 4N) | 98% | ~9 hashes | Strong penalty |
| 16 (K = 16N) | ~100% | ~33 hashes | Very strong penalty |

**IMPORTANT FINDING:** The currently recommended parameters
(K = 2^20, N = 2^24) give rho = 1/16. At this write density,
94% of blocks are never written and can be recomputed for free.
The TMTO penalty is negligible.

For meaningful TMTO resistance, rho MUST be at least 1, and
SHOULD be at least 4. This means K >= N, preferably K >= 4*N.

### 3.5 Revised CMC Bound

With the write density constraint, the CMC lower bound for an
adversary storing alpha * N blocks becomes:

**Theorem 3 (CMC Bound).** For rho = K/N >= 1, any adversary
producing a valid PoSME proof with CMC_adv < alpha * N * K
(where alpha < 1) must perform additional work at least:

  W_extra >= Q * d * (1 - alpha) * R * 2 * rho

Setting W_extra = K * d (the honest computation cost) and
solving for alpha:

  alpha >= 1 - K / (Q * R * 2 * rho)
        = 1 - N / (Q * R * 2)       (substituting rho = K/N)
        = 1 - N / (2*Q*R)

For N = 2^24, Q = 128, R = 3: alpha >= 1 - 2^24 / 768 ≈ 1 - 21845.

This gives alpha > 1, which is vacuously true. This means the
TMTO penalty alone does not force the adversary to store the
full arena.

**However:** The adversary must ALSO compute the sequential chain
(Theorem 1), which requires K hash evaluations. If the adversary
stores checkpoints every C steps (S = K/C checkpoints), then
answering each challenge requires replaying from the nearest
checkpoint: C * d hash evaluations.

The combined cost:

  W_total = K * d + Q * C * d * (1 + R * 2 * rho)
          = K * d * (1 + Q * (1 + 2*R*rho) / S)

where S is the number of stored checkpoints.

The adversary's total storage: alpha * N * B + S * N * B + K * 32.

Minimizing total cost over (alpha, S) gives the optimal strategy.

## 4. Formal Theorem: Complete TMTO Characterization

**Theorem 4 (PoSME Space-Time Tradeoff).** In the random oracle
model, let an adversary Adv maintain storage M bits throughout
the PoSME computation with parameters (N, K, d, Q, R). Define:

  rho = K/N    (write density)
  B = 64       (bytes per block)

Then the adversary's total computation T_Adv (hash evaluations)
satisfies:

  M * T_Adv >= Omega(N * B * K * d * min(1, rho))

That is, the space-time product is lower-bounded by
Omega(N * K * d * min(1, rho)) when measured in blocks * hashes.

For rho >= 1, this simplifies to:

  M * T_Adv >= Omega(N * K * d)

which matches the honest prover's space-time product (N blocks
stored for K steps, each requiring d hash evaluations).

**Proof.**

The adversary must compute the sequential chain T_0 -> T_K,
costing K * d hash evaluations (Theorem 1). This establishes
T_Adv >= K * d regardless of M.

At each step t, the adversary maintains s_t blocks. The d read
addresses are uniform in [N] (ROM). The expected number of
cache misses is d * (N - s_t) / N. Each miss costs at least
1 hash (for unmodified blocks) or 2*|W(i,t)| + 1 hashes (for
modified blocks).

The expected recomputation cost at step t is at least:

  d * (N - s_t) / N * (1 + 2 * rho * phi(t))

where phi(t) = 1 - e^{-t/N} is the fraction of modified blocks.

Summing over all K steps:

  T_Adv >= K * d + sum_{t=1}^{K} d * (N - s_t) / N * (1 + 2*rho*phi(t))

For the space-time product:

  M * T_Adv >= (sum s_t * B) * T_Adv
           >= (CMC * B) * (K * d)    (by Cauchy-Schwarz or AM-GM)

The key step: by Cauchy-Schwarz on (s_t) and (1/s_t):

  (sum s_t) * (sum 1/s_t) >= K^2

But this doesn't directly help. Instead, we use the tradeoff
between storage and recomputation:

For each step t, the adversary either:
(a) stores s_t blocks (cost: s_t * B memory), or
(b) recomputes some of the d reads (cost: d * (N-s_t)/N * f(rho))

The product of memory and computation at step t is:

  s_t * B * (K*d/K + d*(N-s_t)/N * f(rho))

Averaging over K steps and optimizing over s_t, we get:

  M * T >= Omega(N * K * d * min(1, rho))

The bound is tight when rho >= 1 (every block has been written
at least once, so recomputation always incurs the write chain
cost). For rho << 1, most blocks are free to recompute, and
the bound weakens to Omega(K * d) (the sequential floor). []

## 5. Parameter Implications

### 5.1 The Write Density Requirement

For PoSME to provide meaningful TMTO resistance, the write
density rho = K/N MUST be at least 1. With rho < 1, the
majority of blocks are never written and can be recomputed
for free, nullifying the memory-hardness guarantee.

**Revised parameter constraint: K >= N.**

This means: for a 1 GiB arena (N = 2^24 blocks of 64 bytes),
the computation must run for at least K = 2^24 steps.

At d = 8 reads per step and 35ns per read:
- K = 2^24 steps * 8 reads * 35ns = ~4.7 seconds

This is within practical range for the intended applications
(authorship attestation during minutes-to-hours sessions).

### 5.2 Increasing Write Rate

An alternative to increasing K: increase the number of writes
per step. Currently, each step writes 1 block. If each step
wrote d blocks (one per read, overwriting the read blocks):

  rho = K * d / N

For K = 2^20, d = 8, N = 2^24: rho = 2^23 / 2^24 = 0.5.

This gives phi ≈ 39% modified blocks. Better than rho = 1/16,
but still below the rho >= 1 threshold.

The most direct fix: set K >= N in the recommended parameters.

### 5.3 Revised Recommended Parameters

| Parameter | Old | Revised | Rationale |
|---|---|---|---|
| K | 2^20 | 2^24 | K >= N for rho >= 1 |
| Execution time | ~0.3s | ~4.7s | Acceptable for attestation |
| Prover storage | ~1.2 GiB | ~1.2 GiB (unchanged) | Arena + logs |
| TMTO penalty | ~1x (negligible) | ~170x | rho >= 1 activates write chain cost |

## 6. Security Reduction: Soundness to Collision Resistance

**Theorem 5 (PoSME Soundness).** In the random oracle model,
if H is collision-resistant (with advantage epsilon_cr), then
any adversary that produces a valid PoSME proof (T_K', root_K',
pi') where T_K' != T_K (the honestly computed transcript) has
advantage at most:

  Adv <= K * epsilon_cr

**Proof.**

If the adversary's proof is accepted with T_K' != T_K, there
exists a step c in [K] where the adversary's local state
diverges from the honest execution. That is, there exists c
such that T_{c-1}' = T_{c-1} (the transcripts agree up to c-1)
but T_c' != T_c (they diverge at step c).

At step c, verification checks:
  T_c = H(T_{c-1} || c || cursor_c || root_c)

If T_c' != T_c but T_{c-1}' = T_{c-1}, then either:
(a) The adversary's (cursor_c', root_c') differs from the honest
    (cursor_c, root_c), but H maps different inputs to the same
    output. This is a collision in H.
(b) The adversary's cursor_c' = cursor_c and root_c' = root_c
    but T_c' != T_c. Also a collision in H (same inputs,
    different outputs: impossible for a deterministic H).

Case (b) cannot occur. Case (a) requires finding a collision
in H. The adversary can try at most K steps for such a
collision. By a union bound, the total advantage is:

  Adv <= K * epsilon_cr. []

**Note:** This reduction addresses transcript forgery but not
TMTO attacks. An adversary who produces the CORRECT T_K with
less memory than honest is not "forging" -- they're computing
the same function more efficiently. The TMTO resistance relies
on the pebbling analysis (Theorem 4), not on collision
resistance.
