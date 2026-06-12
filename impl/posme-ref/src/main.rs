// PoSME Reference Implementation
// Proof of Sequential Memory Execution
//
// This implements the core construction from draft-condrey-cfrg-posme.
// The critical path is the step function's pointer-chase loop:
// each read address depends on the previous read's hash,
// forcing sequential DRAM access.

use std::time::Instant;

// ---------------------------------------------------------------------------
// Parameters
// ---------------------------------------------------------------------------

/// Security parameter (bytes). All hashes produce 32-byte output.
const LAMBDA: usize = 32;

/// Block size: 32 bytes data + 32 bytes causal hash.
const BLOCK_SIZE: usize = 2 * LAMBDA;

// ---------------------------------------------------------------------------
// Block and Arena
// ---------------------------------------------------------------------------

/// A single arena block: (data, causal).
#[derive(Clone, Copy)]
struct Block {
    data: [u8; LAMBDA],
    causal: [u8; LAMBDA],
}

impl Block {
    fn zeroed() -> Self {
        Self {
            data: [0u8; LAMBDA],
            causal: [0u8; LAMBDA],
        }
    }

    fn as_bytes(&self) -> [u8; BLOCK_SIZE] {
        let mut out = [0u8; BLOCK_SIZE];
        out[..LAMBDA].copy_from_slice(&self.data);
        out[LAMBDA..].copy_from_slice(&self.causal);
        out
    }
}

/// The mutable arena.
struct Arena {
    blocks: Vec<Block>,
    n: usize,
}

impl Arena {
    fn new(n: usize) -> Self {
        Self {
            blocks: vec![Block::zeroed(); n],
            n,
        }
    }
}

// ---------------------------------------------------------------------------
// Hash helpers
// ---------------------------------------------------------------------------

/// Domain-separated hash. Prepends the tag before hashing.
fn h(tag: &[u8], data: &[u8]) -> [u8; LAMBDA] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(tag);
    hasher.update(data);
    *hasher.finalize().as_bytes()
}

/// Extract 4 bytes from a hash for address derivation (XOF mode).
fn xof_addr(cursor: &[u8; LAMBDA], index: u32) -> [u8; 4] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"PoSME-addr-v1");
    hasher.update(cursor);
    hasher.update(&index.to_be_bytes());
    let hash = hasher.finalize();
    let mut out = [0u8; 4];
    out.copy_from_slice(&hash.as_bytes()[..4]);
    out
}

/// Convert 4 bytes to usize, mod n.
fn addr_mod(bytes: &[u8; 4], n: usize) -> usize {
    let val = u32::from_be_bytes(*bytes) as usize;
    val % n
}

/// Encode u32 as 4-byte big-endian.
fn i2osp(x: u32) -> [u8; 4] {
    x.to_be_bytes()
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

/// Initialize the arena deterministically from seed.
/// A[0].data = H("init" || seed || 0)
/// A[i].data = H("init" || seed || i || A[i-1].data || A[i/2].data)
/// A[i].causal = H("causal" || seed || i)
fn initialize(arena: &mut Arena, seed: &[u8]) {
    let n = arena.n;

    // Block 0
    let mut inp = Vec::with_capacity(64);
    inp.extend_from_slice(seed);
    inp.extend_from_slice(&i2osp(0));
    arena.blocks[0].data = h(b"PoSME-init-v1", &inp);
    arena.blocks[0].causal = {
        let mut v = Vec::new();
        v.extend_from_slice(seed);
        v.extend_from_slice(&i2osp(0));
        h(b"PoSME-causal-v1", &v)
    };

    // Blocks 1..N-1
    for i in 1..n {
        let prev_data = arena.blocks[i - 1].data;
        let skip_data = arena.blocks[i / 2].data;

        let mut inp = Vec::with_capacity(128);
        inp.extend_from_slice(seed);
        inp.extend_from_slice(&i2osp(i as u32));
        inp.extend_from_slice(&prev_data);
        inp.extend_from_slice(&skip_data);
        arena.blocks[i].data = h(b"PoSME-init-v1", &inp);

        let mut cinp = Vec::new();
        cinp.extend_from_slice(seed);
        cinp.extend_from_slice(&i2osp(i as u32));
        arena.blocks[i].causal = h(b"PoSME-causal-v1", &cinp);
    }
}

// ---------------------------------------------------------------------------
// Step Function -- THE CRITICAL PATH
// ---------------------------------------------------------------------------

/// Execute one PoSME step. Returns (cursor, write_addr).
///
/// This is the "heartbeat" of the system. The inner loop MUST be
/// strictly sequential: each hash depends on the previous read,
/// each read address depends on the previous hash.
#[inline(never)] // Prevent the compiler from reordering across steps
fn posme_step(
    arena: &mut Arena,
    transcript_prev: &[u8; LAMBDA],
    t: u32,
    d: usize,
) -> ([u8; LAMBDA], usize) {
    let n = arena.n;
    let mut cursor: [u8; LAMBDA] = *transcript_prev;

    // Phase 1: Pointer-chase reads (data-dependent, strictly sequential)
    //
    //   cursor = H(cursor || val.data || val.causal)
    //
    // This is the line where latency-hardness lives. Each iteration:
    //   1. Derive address from cursor (hash, ~3ns)
    //   2. Read block from arena at that address (DRAM, ~35ns)
    //   3. Hash cursor with block data+causal (~3ns)
    //   4. Loop: next address depends on step 3's output
    //
    // Steps 1-3 are serialized by data dependency. No hardware
    // can execute step 2 of iteration j+1 before step 3 of
    // iteration j completes.
    for j in 0..d {
        let addr_bytes = xof_addr(&cursor, j as u32);
        let a = addr_mod(&addr_bytes, n);

        // THE MEMORY READ -- this is the latency bottleneck
        let val = arena.blocks[a];

        // THE HASH -- cursor depends on what we just read
        let mut inp = [0u8; LAMBDA + BLOCK_SIZE];
        inp[..LAMBDA].copy_from_slice(&cursor);
        inp[LAMBDA..LAMBDA + LAMBDA].copy_from_slice(&val.data);
        inp[LAMBDA + LAMBDA..].copy_from_slice(&val.causal);
        cursor = h(b"", &inp);
    }

    // Phase 2: Symbiotic write
    let w_bytes = xof_addr(&cursor, d as u32);
    let w = addr_mod(&w_bytes, n);
    let old = arena.blocks[w];

    // new_data = H(old_data || cursor || old_causal)
    let mut dinp = [0u8; 3 * LAMBDA];
    dinp[..LAMBDA].copy_from_slice(&old.data);
    dinp[LAMBDA..2 * LAMBDA].copy_from_slice(&cursor);
    dinp[2 * LAMBDA..].copy_from_slice(&old.causal);
    let new_data = h(b"PoSME-write-v1", &dinp);

    // new_causal = H(old_causal || cursor || t)
    let mut cinp = [0u8; 2 * LAMBDA + 4];
    cinp[..LAMBDA].copy_from_slice(&old.causal);
    cinp[LAMBDA..2 * LAMBDA].copy_from_slice(&cursor);
    cinp[2 * LAMBDA..].copy_from_slice(&i2osp(t));
    let new_causal = h(b"PoSME-causal-v1", &cinp);

    arena.blocks[w] = Block {
        data: new_data,
        causal: new_causal,
    };

    (cursor, w)
}

/// Compute the transcript chain value for step t.
fn transcript_hash(
    t_prev: &[u8; LAMBDA],
    t: u32,
    cursor: &[u8; LAMBDA],
    root: &[u8; LAMBDA],
) -> [u8; LAMBDA] {
    let mut inp = [0u8; 3 * LAMBDA + 4];
    inp[..LAMBDA].copy_from_slice(t_prev);
    inp[LAMBDA..LAMBDA + 4].copy_from_slice(&i2osp(t));
    inp[LAMBDA + 4..2 * LAMBDA + 4].copy_from_slice(cursor);
    inp[2 * LAMBDA + 4..].copy_from_slice(root);
    h(b"PoSME-transcript-v1", &inp)
}

// ---------------------------------------------------------------------------
// Merkle tree (minimal, for root computation)
// ---------------------------------------------------------------------------

fn merkle_root(arena: &Arena) -> [u8; LAMBDA] {
    let n = arena.n;
    let mut leaves: Vec<[u8; LAMBDA]> = Vec::with_capacity(n);
    for i in 0..n {
        leaves.push(h(b"\x00", &arena.blocks[i].as_bytes()));
    }
    build_tree(&leaves)
}

fn build_tree(leaves: &[[u8; LAMBDA]]) -> [u8; LAMBDA] {
    if leaves.len() == 1 {
        return leaves[0];
    }
    let mut level = leaves.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity((level.len() + 1) / 2);
        let mut i = 0;
        while i < level.len() {
            if i + 1 < level.len() {
                let mut inp = [0u8; 2 * LAMBDA];
                inp[..LAMBDA].copy_from_slice(&level[i]);
                inp[LAMBDA..].copy_from_slice(&level[i + 1]);
                next.push(h(b"\x01", &inp));
            } else {
                let pad = h(b"\x02", &i2osp(level.len() as u32));
                let mut inp = [0u8; 2 * LAMBDA];
                inp[..LAMBDA].copy_from_slice(&level[i]);
                inp[LAMBDA..].copy_from_slice(&pad);
                next.push(h(b"\x01", &inp));
            }
            i += 2;
        }
        level = next;
    }
    level[0]
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    // Small parameters for reference testing.
    // Production: N=2^24, K=4*N, d=8.
    let n: usize = std::env::var("POSME_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1 << 16); // default 64K blocks = 4 MiB
    let k: u32 = std::env::var("POSME_K")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(n as u32); // default rho = 1
    let d: usize = 8;
    let seed = b"posme-reference-v1";

    println!("PoSME Reference Implementation");
    println!("===============================");
    println!("Arena: {} blocks ({} MiB)", n, n * BLOCK_SIZE / (1 << 20));
    println!("Steps: {} (rho = {:.1})", k, k as f64 / n as f64);
    println!("Reads/step: {}", d);
    println!();

    // --- Initialize ---
    let t0 = Instant::now();
    let mut arena = Arena::new(n);
    initialize(&mut arena, seed);
    let init_time = t0.elapsed();
    println!("Initialization: {:.3}s", init_time.as_secs_f64());

    // --- Initial root and transcript ---
    let t0 = Instant::now();
    let root_0 = merkle_root(&arena);
    let merkle_time = t0.elapsed();
    println!("Initial Merkle root: {:.3}s", merkle_time.as_secs_f64());

    let mut t_prev = {
        let mut inp = Vec::new();
        inp.extend_from_slice(seed);
        inp.extend_from_slice(&root_0);
        h(b"PoSME-transcript-v1", &inp)
    };

    println!("T_0: {}", hex(&t_prev));
    println!();

    // --- Execute K steps (core loop timing) ---
    println!("Executing {} steps...", k);
    let exec_start = Instant::now();

    // We skip Merkle updates in the timing loop because the
    // reference Merkle implementation is O(N) per step (a real
    // implementation uses O(log N) incremental updates). The
    // step function timing is what matters for latency validation.
    let placeholder_root = [0u8; LAMBDA];

    let report_interval = std::cmp::max(1, k / 10);

    for t in 1..=k {
        let (cursor, _w) = posme_step(&mut arena, &t_prev, t, d);
        t_prev = transcript_hash(&t_prev, t, &cursor, &placeholder_root);

        if t % report_interval == 0 || t == k {
            let elapsed = exec_start.elapsed().as_secs_f64();
            let avg_ns = elapsed / t as f64 * 1e9;
            let pct = t as f64 / k as f64 * 100.0;
            println!(
                "  Step {}/{} ({:.0}%) -- avg {:.0} ns/step, {:.2}s elapsed",
                t, k, pct, avg_ns, elapsed
            );
        }
    }

    let total_time = exec_start.elapsed();
    let avg_step_ns = total_time.as_secs_f64() / k as f64 * 1e9;

    println!();
    println!("T_K: {}", hex(&t_prev));
    println!();
    println!("=== Timing Results ===");
    println!("Total execution: {:.3}s", total_time.as_secs_f64());
    println!("Average step: {:.0} ns", avg_step_ns);
    println!(
        "  Hash component (BLAKE3, est. {}x ~3ns): ~{:.0} ns",
        d + 3, // d cursor hashes + addr hash + write data + write causal
        (d as f64 + 3.0) * 3.0
    );
    println!(
        "  Memory component (remainder): ~{:.0} ns",
        avg_step_ns - (d as f64 + 3.0) * 3.0
    );
    println!(
        "  Hash fraction: {:.1}%",
        ((d as f64 + 3.0) * 3.0) / avg_step_ns * 100.0
    );
    println!();

    // Latency analysis
    let theoretical_min_ns = 35.0 * d as f64;
    println!("=== Latency Analysis ===");
    println!(
        "Theoretical min ({}x 35ns DRAM): {:.0} ns",
        d, theoretical_min_ns
    );
    println!("Measured avg: {:.0} ns", avg_step_ns);
    if avg_step_ns > theoretical_min_ns {
        println!(
            "Ratio: {:.2}x (overhead from hash + cache effects)",
            avg_step_ns / theoretical_min_ns
        );
    } else {
        println!(
            "Note: measured < theoretical -- arena likely fits in cache ({} MiB arena)",
            n * BLOCK_SIZE / (1 << 20)
        );
    }
    println!();

    // TMTO analysis
    let rho = k as f64 / n as f64;
    println!("=== TMTO Analysis (rho = {:.1}) ===", rho);
    for alpha in [0.0_f64, 0.25, 0.5, 0.75] {
        let ratio = 1.0 + (1.0 - alpha) * (2.0 * rho + 1.0);
        println!("  alpha={:.2}: {:.1}x penalty", alpha, ratio);
    }
}

fn hex(bytes: &[u8; LAMBDA]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
