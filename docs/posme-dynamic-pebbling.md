# PoSME Dynamic Pebbling: Formal Framework

## 1. Computation Model

Fix parameters: N (arena blocks), K (steps), d (reads per step),
lambda = 256 (security parameter). Let H : {0,1}* -> {0,1}^lambda
be a random oracle.

### 1.1 Arena and State

Arena A_t : [N] -> {0,1}^{2*lambda} is the arena state after
step t. Each block stores (data, causal), each lambda bits.

A_0 is deterministic from seed s: any party can compute A_0[i]
in O(1) from s and i (plus at most 2 predecessor lookups for the
skip-link initialization).

### 1.2 Step Function (Precise)

At step t, given transcript value T_{t-1} and arena state A_{t-1}:

~~~ pseudocode
// Phase 1: Sequential pointer-chase reads
cursor := T_{t-1}
a_0 := H("addr" || cursor || 0) mod N
v_0 := A_{t-1}[a_0]
cursor := H(cursor || v_0.data || v_0.causal)

for j = 1 to d-1:
    a_j := H("addr" || cursor || j) mod N
    v_j := A_{t-1}[a_j]
    cursor := H(cursor || v_j.data || v_j.causal)

// Phase 2: Write
w := H("write" || cursor) mod N
old := A_{t-1}[w]
new.data := H(old.data || cursor || old.causal)
new.causal := H(old.causal || cursor || t)
A_t[w] := new;  A_t[i] := A_{t-1}[i] for i != w

// Phase 3: Transcript
root_t := MerkleUpdate(root_{t-1}, w, new)
T_t := H(T_{t-1} || t || cursor || root_t)
~~~

**Critical structure:** Address a_j depends on cursor AFTER
absorbing blocks v_0 through v_{j-1}. This means:

- a_0 depends only on T_{t-1} (known before any reads)
- a_1 depends on T_{t-1} AND v_0 (requires reading block a_0)
- a_2 depends on T_{t-1}, v_0, AND v_1 (requires a_0 and a_1)
- ...
- a_{d-1} depends on all prior reads

The d reads within a step are SEQUENTIALLY DEPENDENT. The
adversary cannot determine a_{j+1} without first obtaining v_j
(the contents of block a_j). This is the pointer-chasing
property.

### 1.3 Address Distribution in the ROM

**Claim:** In the random oracle model, conditioned on the
adversary's storage set S_{t-1} and the values of blocks
{v_0, ..., v_{j-1}} already read during step t, the address
a_j is uniformly distributed in [N].

**Justification:** a_j = H("addr" || cursor || j) mod N. The
cursor at this point is H(... || v_{j-1}.data || v_{j-1}.causal),
which is the output of the random oracle on a fresh input (the
input includes v_{j-1}, which was not an input to any previous H
query in this context). By the ROM property, this output is
uniform and independent of all prior queries to H on different
inputs.

**Important caveat:** The address IS determined by the arena
contents. The adversary's storage decisions affect which blocks
are available, which affects which values v_j take, which affects
cursor, which affects addresses. The correct statement is:

For ANY fixed adversary strategy (determining S_t as a function
of prior observations), the DISTRIBUTION of a_j over the
random oracle's coins is uniform in [N]. The adversary cannot
bias the address distribution by choosing what to store.

This does NOT mean a_j is independent of S_t. The adversary's
storage set and the address are both functions of the same random
oracle coins. But the adversary cannot PREDICT or CONTROL which
addresses will be selected.

## 2. The Dynamic Pebbling Game

### 2.1 Game Definition

**Players:** Challenger (running the PoSME computation) and
Adversary (trying to produce a valid proof with minimal resources).

**Adversary state at time t:**
- Pebbled set P_t subset [N]: blocks stored in memory
- For each i in P_t: the current value A_t[i] (2*lambda bits)
- Auxiliary tape aux_t: arbitrary additional state (bounded)
- Total storage: |P_t| * 2 * lambda + |aux_t|

**Rules for each step t = 1, ..., K:**

1. REVEAL: The Challenger reveals a_0 (the first read address,
   determined by T_{t-1} which the Adversary already knows).

2. For j = 0, ..., d-1:
   a. If a_j in P_t: the Adversary reads v_j = A_{t-1}[a_j]
      from its stored set. Cost: 0.
   b. If a_j not in P_t: the Adversary must RECOMPUTE v_j.
      Cost: R(a_j, t) hash evaluations (defined below).
   c. The Adversary updates cursor and the Challenger reveals
      a_{j+1} (which depends on v_j and hence on whether
      the Adversary recomputed correctly).

3. WRITE: The Challenger reveals w. The Adversary computes the
   new block value (requires cursor, which requires all d reads).

4. MANAGE: The Adversary may add or remove blocks from P_t to
   form P_{t+1}. Adding a new block requires either:
   - computing it (via step function or initialization), or
   - having it delivered by recomputation.

5. The Adversary computes T_t.

### 2.2 Recomputation Cost Function

When the Adversary needs block i at step t but i not in P_t:

**Case A: Block i was never written** (w_s != i for all s < t).
The Adversary computes A_0[i] from the seed. Cost: R(i,t) = O(1).

**Case B: Block i was last written at step tau** (the most recent
s < t where w_s = i). The Adversary needs:

1. The cursor at step tau: c_tau. This requires knowing the d
   read values at step tau, which requires those blocks at their
   state at time tau-1.

2. The old value of block i before step tau wrote to it: A_{tau-1}[i].
   This requires knowing who last wrote to i BEFORE tau, etc.

The recomputation traces backward through the write history of
block i, and at each link, requires the cursor of the writing
step, which requires d reads at that step.

**If the Adversary stored all cursors** (K * lambda bits of aux):
The Adversary needs only the write chain of block i. Each link
requires the previous value of i, traceable back to initialization.
The write chain has expected length E[|W_i|] = rho = K/N (Poisson).
Each link costs O(1) hashes (given the cursor). But the Adversary
also needs old.causal at each link (for symbiotic binding), which
requires traversing the CAUSAL chain of the same block, which is
the same write chain. Total cost: 2 * |W_i| + 1 hashes.

Expected cost per miss with cursors stored: 2*rho + 1.

**If the Adversary did NOT store cursors:** The Adversary must
recompute cursor at the writing step tau. Recomputing cursor at
tau requires replaying steps from some checkpoint. If the nearest
checkpoint is C steps away, this costs C * d hash evaluations
(each intervening step requires d reads). Recomputing one cursor
may cause further cache misses, creating a cascade.

This is where the adversary faces a tradeoff: store more cursors
(auxiliary memory) to reduce recomputation cost, or store fewer
and pay sequential replay costs.

### 2.3 Cursor Storage Tradeoff

The Adversary can store cursors at intervals of L steps, using
K/L * lambda bits of auxiliary storage. Recomputing a cursor at
an arbitrary step costs at most L * d * (expected recomp per
step) hashes, where "expected recomp per step" depends on the
pebbled fraction alpha.

For alpha-fraction storage: expected misses per step = d*(1-alpha).
Expected recomp per miss = 2*rho + 1 (with cursor available).
Cost of replaying one step without cursor: d * (1 + (1-alpha)*(2*rho+1)).
Cost of replaying L steps: L * d * (1 + (1-alpha)*(2*rho+1)).

The Adversary's total cost to answer one challenge:
- Find the nearest cursor checkpoint: O(1) lookup
- Replay from checkpoint to challenged step: at most L steps
- Total per challenge: L * d * (1 + (1-alpha)*(2*rho+1))

## 3. Lower Bounds

### 3.1 Sequential Floor (Unconditional)

**Theorem 1.** Any algorithm producing T_K from (s, params)
performs at least K evaluations of H in sequence.

**Proof.** T_t = H(T_{t-1} || ...). In the ROM, the output T_t
is uniform and independent of all prior queries except the one
computing T_t itself. The only way to learn T_t is to evaluate H
on input (T_{t-1} || ...), which requires knowing T_{t-1}.
Inductively, computing T_K requires the sequential chain
T_0, T_1, ..., T_K. Each step requires at least one H
evaluation. The chain has depth K. []

**Corollary.** The Adversary's wall-clock time is at least
K * tau_H, where tau_H is the time for one H evaluation. No
parallelism or storage strategy can reduce this below K
sequential steps.

### 3.2 Memory-Dependent Recomputation Cost

**Theorem 2.** In the ROM, let the Adversary maintain pebbled
set of expected size alpha * N blocks and cursor checkpoints
every L steps. The expected total computation to produce T_K
and answer Q Fiat-Shamir challenges with recursion depth R is:

~~~ artwork
T_total >= K * d                                  (chain cost)
         + Q * L * d * (1 + (1-alpha)*(2*rho+1))  (challenge replay)
         + Q * d * R * (1-alpha) * (2*rho+1)       (provenance)
~~~

**Proof.**

Part 1 (chain cost): By Theorem 1, computing the sequential chain
costs K * d hash evaluations (each step requires d reads to
compute the cursor, plus hash evaluations for write and transcript).

Part 2 (challenge replay): The Adversary computes the chain but
discards most state. When challenged at step c, it must
reconstruct the arena state at c. Starting from the nearest cursor
checkpoint (at most L steps away), the Adversary replays forward.
Each replayed step requires d reads. Each read that misses the
pebbled set (probability 1-alpha per read, since addresses are
uniform in the ROM) costs 2*rho+1 to recompute via write chain
traversal. Expected cost per replayed step: d * (1 + (1-alpha)*(2*rho+1)).
Over L steps per challenge, times Q challenges: Q*L*d*(1+(1-alpha)*(2*rho+1)).

Part 3 (provenance): Each challenge opens d reads recursively to
depth R. Each recursive level requires d more block lookups, each
missing with probability 1-alpha. Expected provenance cost per
challenge: d*R*(1-alpha)*(2*rho+1). Over Q challenges:
Q*d*R*(1-alpha)*(2*rho+1). []

### 3.3 Adversary's Optimal Strategy

The Adversary jointly optimizes alpha (pebbled fraction) and L
(cursor checkpoint interval) to minimize total cost:

~~~ artwork
Total cost = K*d + Q*L*d*(1 + (1-alpha)*(2*rho+1))
                 + Q*d*R*(1-alpha)*(2*rho+1)

Storage = alpha * N * 2*lambda + (K/L) * lambda + aux
~~~

Minimizing over L: the Adversary wants L small (less replay) but
this costs more cursor storage (K/L * lambda). Setting
cursor storage budget = B_aux:

  L = K * lambda / B_aux

Minimizing over alpha: the Adversary wants alpha small (less arena
storage) but this increases recomputation. The marginal cost of
NOT storing one block is d * (2*rho+1) per step it's needed.
The marginal cost of storing one block is 2*lambda bits per step.

At equilibrium: alpha is determined by the ratio of storage cost
to recomputation cost.

### 3.4 Numerical Analysis

For recommended parameters: N = K = 2^24, d = 8, Q = 128, R = 3,
rho = 1, lambda = 256 bits = 32 bytes.

**Honest prover:**
- Cost: K * d = 2^24 * 8 = 2^27 hash evaluations
- Storage: N * 64 bytes = 1 GiB (arena) + K * 32 = 512 MiB (cursors)

**Adversary storing alpha = 0 (no arena), L = 2^12 (cursor every 4096 steps):**
- Cursor storage: K/L * 32 = 2^12 * 32 = 128 KiB
- Chain cost: 2^27 hashes (same as honest)
- Per challenge replay: L * d * (1 + 1*(2*1+1)) = 4096 * 8 * 4 = 131072 hashes
- Challenge total: 128 * 131072 = 2^24 hashes
- Provenance: 128 * 8 * 3 * 1 * 3 = 9216 hashes (negligible)
- Total: 2^27 + 2^24 = 2^27 * 1.125
- **Adversary pays 12.5% more computation for 0% arena storage**

This is a WEAK bound. The adversary saves 1 GiB of storage for
only 12.5% more computation.

**Why it's weak:** With rho = 1, each miss costs only 2*1+1 = 3
hashes (expected write chain length is 1). The replay cost
L * d * 4 is significant only if L is large. The adversary can
set L small by spending modest auxiliary storage on cursors.

**Adversary storing alpha = 0, L = 2^20 (cursor every ~1M steps):**
- Cursor storage: 2^4 * 32 = 512 bytes
- Per challenge replay: 2^20 * 8 * 4 = 2^25 hashes
- Challenge total: 128 * 2^25 = 2^32 hashes
- Total: 2^27 + 2^32 = 2^32 * 1.03
- **Ratio to honest: 2^32 / 2^27 = 32x**

Still not 170x. The previous analysis was wrong.

**To get 170x, we need rho = K/N much larger than 1.**

For rho = 16 (K = 16*N): miss cost = 2*16+1 = 33 hashes.
Adversary with alpha=0, L=2^12:
- Per challenge replay: 4096 * 8 * 34 = 1,114,112 hashes
- Challenge total: 128 * 1.1M = 143M hashes
- Chain cost: 16 * 2^24 * 8 = 2^31 hashes
- Total: 2^31 + 1.4*2^27 ≈ 2^31 * 1.07
- Ratio: 1.07x. Still weak!

The problem: the chain cost dominates. The adversary MUST compute
the full chain (Omega(K*d) hashes) regardless of memory. The
recomputation cost from challenges is additive, and for Q=128,
it's much smaller than K*d when K is large.

### 3.5 The Real Bound

The honest prover's cost is ALSO K*d. The adversary's extra cost
from challenges is Q * L * d * (1-alpha) * (2*rho+1). The ratio:

~~~ artwork
Ratio = 1 + Q * L * (1-alpha) * (2*rho+1) / K
~~~

For this ratio to be significant (say, >= 2x), we need:

  Q * L * (1-alpha) * (2*rho+1) >= K

The adversary chooses L to minimize its cost. If it stores all
cursors (L = 1): the replay cost is minimized to Q * d * (1+(1-alpha)*(2*rho+1)).
Ratio = 1 + Q * (1-alpha) * (2*rho+1) / K.
For K = 2^24, Q = 128, rho = 1, alpha = 0:
Ratio = 1 + 128 * 3 / 2^24 = 1 + 0.000023.

**This is negligible.** The adversary storing ALL cursors (512 MiB)
can recompute any challenge in O(1) and barely pays more than the
honest prover.

### 3.6 The Fundamental Issue

The TMTO analysis fails because:

1. **The sequential chain dominates cost.** The honest prover
   spends K*d hashes on the chain. The adversary ALSO spends K*d.
   Both have the same dominant cost. The difference is only in
   challenge answering, which is O(Q * ...) and Q << K.

2. **Cursors are cheap to store.** All K cursors cost K*32 bytes.
   For K = 2^24: 512 MiB. With cursors stored, any block can be
   recomputed in O(rho) hashes. The arena (1 GiB) provides value
   only in avoiding O(rho) recomputation per miss, but rho is
   small.

3. **The real memory benefit of storing the arena is SPEED during
   chain computation, not challenge answering.** During the chain,
   the honest prover reads d blocks per step from RAM (fast). The
   adversary who doesn't store the arena must recompute those
   blocks (slower). But the recomputation cost per step is only
   d * (1-alpha) * (2*rho+1) hashes, which for rho=1 is about
   3*d extra hashes. The total chain cost becomes K * d * (1 + 3*(1-alpha))
   = K * d * 4 for alpha=0. The adversary pays 4x more computation
   but uses 0 arena memory.

**This is the true TMTO: the space-time product is approximately
constant.** Halving memory roughly doubles computation. This is
similar to Argon2id's TMTO, not the "catastrophic" penalty claimed
in the early design rounds.

### 3.7 What Causal Hashes Actually Provide

Causal hashes don't change the TMTO asymptotically. What they
provide is SOUNDNESS: they prevent the adversary from fabricating
block values without computing the write chain. Without causal
hashes, the adversary could insert random values and hope the
Verifier doesn't check. With causal hashes, every checked block
must have a valid causal chain traceable to initialization.

The TMTO penalty is:
- Without causal hashes: adversary pays O(rho) per miss (write chain)
- With causal hashes: adversary pays O(2*rho) per miss (data + causal chains)
- With symbiotic binding: adversary pays O(2*rho) per miss (same, but can't
  fabricate data independently of causal hash)

The causal hash doubles the write-chain traversal cost (2*rho
instead of rho) but doesn't change the scaling.

### 3.8 Honest Assessment

The PoSME TMTO penalty for a zero-storage adversary (who stores
only cursors and a write index) is approximately **4x** for rho=1,
not 170x as previously claimed. The 170x figure arose from an
analysis that assumed the adversary needed to replay C steps per
challenge WITHOUT cursors, but storing all cursors is cheap (512
MiB for K = 2^24) and eliminates the replay cost.

For rho = 4: the penalty rises to about 1 + 3*8*(1-alpha) ≈ 25x
for alpha=0. Still not 170x.

For rho = 16: about 1 + 3*32 ≈ 97x. Getting closer.

The TMTO penalty scales linearly with rho. To achieve 100x+
penalty, rho must be >= 16, meaning K = 16*N = 16 * 2^24 = 2^28.
At d=8 and 35ns per read, this is K * d * 35ns ≈ 75 seconds.
Practical for attestation but long for interactive use.

## 4. Formal Theorem (Corrected)

**Theorem (PoSME Space-Time Tradeoff).** In the random oracle
model, any adversary computing a valid PoSME proof for parameters
(N, K, d, Q, R) while storing at most alpha * N arena blocks and
K * lambda bits of cursor state performs expected computation:

~~~ artwork
T_adv >= K * d * (1 + (1-alpha) * (2*rho + 1))
~~~

hash evaluations, where rho = K/N.

Honest computation costs K * d hash evaluations with N * B + K *
lambda storage. The TMTO ratio is:

~~~ artwork
T_adv / T_honest >= 1 + (1-alpha) * (2*rho + 1)
~~~

For alpha = 0: ratio = 2 + 2*rho.
For rho = 1: ratio = 4x.
For rho = 4: ratio = 10x.
For rho = 16: ratio = 34x.

**Proof.** At each of the K steps, the adversary computes the
cursor, which requires d block reads. Each read hits the stored
set with probability alpha (uniform addresses in ROM). Each miss
requires traversing the write chain (expected 2*rho+1 hashes for
data + causal chains). The expected per-step cost is
d * (1 + (1-alpha) * (2*rho+1)). Summing over K steps gives the
bound. []

## 5. Soundness Reduction (Unchanged)

**Theorem (PoSME Soundness).** Any adversary producing (T_K', pi')
with T_K' != T_K that passes verification has advantage at most
K * epsilon_cr, where epsilon_cr is the collision-finding
advantage against H.

**Proof.** Same as Section 6 of the previous version. The
soundness argument is independent of the TMTO analysis. Soundness
relies on collision resistance; TMTO relies on memory cost. []

## 6. Parameter Implications (Revised)

The corrected TMTO analysis shows:

| rho = K/N | alpha=0 penalty | alpha=0.5 penalty | Execution time (d=8, 35ns) |
|---|---|---|---|
| 1 | 4x | 2.5x | 4.7s |
| 4 | 10x | 5.5x | 18.8s |
| 16 | 34x | 17.5x | 75s |
| 64 | 130x | 65.5x | 5 min |

For a 100x TMTO penalty with zero storage: rho >= 50, K >= 50*N.
For a 10x penalty: rho >= 4.

The choice of rho (and hence K) depends on the application's
tolerance for computation time versus TMTO resistance. For
authorship attestation (minutes-to-hours sessions), rho = 4-16
is practical. For interactive verification, rho = 1 may be
acceptable with the understanding that the TMTO penalty is
only 4x.

## 7. Comparison with Previous Claims

| Claim | Previous | Corrected | Why |
|---|---|---|---|
| TMTO penalty (alpha=0, rho=1) | 170x | 4x | Previous assumed adversary lacks cursors; cursors cost only 512 MiB |
| TMTO penalty (alpha=0, rho=4) | Not analyzed | 10x | Corrected formula |
| "Catastrophic TMTO" | Claimed | False | Penalty scales linearly with rho, not exponentially |
| Causal hash contribution | "creates dependency web" | Doubles write-chain cost | 2*rho vs rho per miss |
| Symbiotic binding contribution | "prevents independent fabrication" | Soundness (prevents forgery) | Does not change TMTO asymptotically |

The causal hash mechanism provides SOUNDNESS (preventing forgery),
not a TMTO advantage. The TMTO penalty comes from write chain
traversal, which exists with or without causal hashes (though
causal hashes double the per-miss cost).
