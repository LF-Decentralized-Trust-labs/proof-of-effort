// PoSME CUDA Benchmark
// =====================
// Runs the pointer-chase loop on GPU HBM to measure actual
// HBM random-access latency vs system DRAM.
//
// This is the definitive ASIC resistance test: if the pointer-chase
// on HBM is only ~2x faster than on DDR5, the latency-bound claim holds.
//
// BUILD:  nvcc -O3 posme-cuda-bench.cu -o posme-cuda-bench
// RUN:    ./posme-cuda-bench

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define LAMBDA 32
#define BLOCK_SIZE (2 * LAMBDA)

struct Block {
    unsigned char data[LAMBDA];
    unsigned char causal[LAMBDA];
};

// Simple hash: SipHash-like mixing for portability.
// Not cryptographically strong; sufficient for latency measurement.
__host__ __device__
static void fast_hash(unsigned char out[LAMBDA],
                      const unsigned char *in, int in_len) {
    unsigned long long v0 = 0x736f6d6570736575ULL;
    unsigned long long v1 = 0x646f72616e646f6dULL;
    unsigned long long v2 = 0x6c7967656e657261ULL;
    unsigned long long v3 = 0x7465646279746573ULL;

    for (int i = 0; i < in_len; i++) {
        v3 ^= (unsigned long long)in[i];
        for (int r = 0; r < 2; r++) {
            v0 += v1; v2 += v3;
            v1 = (v1 << 13) | (v1 >> 51); v3 = (v3 << 16) | (v3 >> 48);
            v1 ^= v0; v3 ^= v2;
            v0 = (v0 << 32) | (v0 >> 32);
            v2 += v1; v0 += v3;
            v1 = (v1 << 17) | (v1 >> 47); v3 = (v3 << 21) | (v3 >> 43);
            v1 ^= v2; v3 ^= v0;
            v2 = (v2 << 32) | (v2 >> 32);
        }
        v0 ^= (unsigned long long)in[i];
    }

    v2 ^= 0xff;
    for (int r = 0; r < 4; r++) {
        v0 += v1; v2 += v3;
        v1 = (v1 << 13) | (v1 >> 51); v3 = (v3 << 16) | (v3 >> 48);
        v1 ^= v0; v3 ^= v2;
        v0 = (v0 << 32) | (v0 >> 32);
        v2 += v1; v0 += v3;
        v1 = (v1 << 17) | (v1 >> 47); v3 = (v3 << 21) | (v3 >> 43);
        v1 ^= v2; v3 ^= v0;
        v2 = (v2 << 32) | (v2 >> 32);
    }

    unsigned long long h = v0 ^ v1 ^ v2 ^ v3;
    for (int i = 0; i < 8; i++) out[i] = (h >> (i*8)) & 0xff;

    v0 ^= 0xee;
    for (int r = 0; r < 4; r++) {
        v0 += v1; v2 += v3;
        v1 = (v1 << 13) | (v1 >> 51); v3 = (v3 << 16) | (v3 >> 48);
        v1 ^= v0; v3 ^= v2;
        v0 = (v0 << 32) | (v0 >> 32);
        v2 += v1; v0 += v3;
        v1 = (v1 << 17) | (v1 >> 47); v3 = (v3 << 21) | (v3 >> 43);
        v1 ^= v2; v3 ^= v0;
        v2 = (v2 << 32) | (v2 >> 32);
    }

    h = v0 ^ v1 ^ v2 ^ v3;
    for (int i = 0; i < 8; i++) out[8+i] = (h >> (i*8)) & 0xff;

    v1 ^= 0xdd;
    for (int r = 0; r < 4; r++) {
        v0 += v1; v2 += v3;
        v1 = (v1 << 13) | (v1 >> 51); v3 = (v3 << 16) | (v3 >> 48);
        v1 ^= v0; v3 ^= v2;
        v0 = (v0 << 32) | (v0 >> 32);
        v2 += v1; v0 += v3;
        v1 = (v1 << 17) | (v1 >> 47); v3 = (v3 << 21) | (v3 >> 43);
        v1 ^= v2; v3 ^= v0;
        v2 = (v2 << 32) | (v2 >> 32);
    }

    h = v0 ^ v1 ^ v2 ^ v3;
    for (int i = 0; i < 8; i++) out[16+i] = (h >> (i*8)) & 0xff;

    v2 ^= 0xcc;
    for (int r = 0; r < 4; r++) {
        v0 += v1; v2 += v3;
        v1 = (v1 << 13) | (v1 >> 51); v3 = (v3 << 16) | (v3 >> 48);
        v1 ^= v0; v3 ^= v2;
        v0 = (v0 << 32) | (v0 >> 32);
        v2 += v1; v0 += v3;
        v1 = (v1 << 17) | (v1 >> 47); v3 = (v3 << 21) | (v3 >> 43);
        v1 ^= v2; v3 ^= v0;
        v2 = (v2 << 32) | (v2 >> 32);
    }

    h = v0 ^ v1 ^ v2 ^ v3;
    for (int i = 0; i < 8; i++) out[24+i] = (h >> (i*8)) & 0xff;
}

__host__ __device__
static unsigned int addr_from(const unsigned char cursor[LAMBDA], int j, int n) {
    unsigned char buf[LAMBDA + 4];
    memcpy(buf, cursor, LAMBDA);
    buf[LAMBDA] = (j >> 24) & 0xff;
    buf[LAMBDA+1] = (j >> 16) & 0xff;
    buf[LAMBDA+2] = (j >> 8) & 0xff;
    buf[LAMBDA+3] = j & 0xff;
    unsigned char h[LAMBDA];
    fast_hash(h, buf, LAMBDA + 4);
    unsigned int val = ((unsigned int)h[0] << 24) | ((unsigned int)h[1] << 16) |
                       ((unsigned int)h[2] << 8) | (unsigned int)h[3];
    return val % n;
}

// The critical pointer-chase loop, running on GPU with arena in HBM
__global__
void posme_step_kernel(Block *arena, int n, int d, int K,
                       unsigned char *transcript_io) {
    unsigned char cursor[LAMBDA];
    memcpy(cursor, transcript_io, LAMBDA);

    for (int t = 1; t <= K; t++) {
        // Pointer-chase reads -- THIS IS THE LATENCY TEST
        for (int j = 0; j < d; j++) {
            unsigned int a = addr_from(cursor, j, n);
            Block val = arena[a];  // <-- HBM random read

            unsigned char inp[LAMBDA + BLOCK_SIZE];
            memcpy(inp, cursor, LAMBDA);
            memcpy(inp + LAMBDA, val.data, LAMBDA);
            memcpy(inp + LAMBDA + LAMBDA, val.causal, LAMBDA);
            fast_hash(cursor, inp, LAMBDA + BLOCK_SIZE);
        }

        // Symbiotic write
        unsigned int w = addr_from(cursor, d, n);
        Block old = arena[w];

        unsigned char dinp[3 * LAMBDA];
        memcpy(dinp, old.data, LAMBDA);
        memcpy(dinp + LAMBDA, cursor, LAMBDA);
        memcpy(dinp + 2*LAMBDA, old.causal, LAMBDA);
        fast_hash(arena[w].data, dinp, 3 * LAMBDA);

        unsigned char cinp[2 * LAMBDA + 4];
        memcpy(cinp, old.causal, LAMBDA);
        memcpy(cinp + LAMBDA, cursor, LAMBDA);
        cinp[2*LAMBDA] = (t >> 24) & 0xff;
        cinp[2*LAMBDA+1] = (t >> 16) & 0xff;
        cinp[2*LAMBDA+2] = (t >> 8) & 0xff;
        cinp[2*LAMBDA+3] = t & 0xff;
        fast_hash(arena[w].causal, cinp, 2 * LAMBDA + 4);

        // Transcript (simplified)
        unsigned char tinp[LAMBDA + 4 + LAMBDA];
        memcpy(tinp, cursor, LAMBDA);
        tinp[LAMBDA] = (t >> 24) & 0xff;
        tinp[LAMBDA+1] = (t >> 16) & 0xff;
        tinp[LAMBDA+2] = (t >> 8) & 0xff;
        tinp[LAMBDA+3] = t & 0xff;
        memcpy(tinp + LAMBDA + 4, cursor, LAMBDA); // placeholder for root
        fast_hash(cursor, tinp, LAMBDA + 4 + LAMBDA);
    }

    memcpy(transcript_io, cursor, LAMBDA);
}

// CPU version for comparison
void posme_steps_cpu(Block *arena, int n, int d, int K,
                     unsigned char *transcript) {
    unsigned char cursor[LAMBDA];
    memcpy(cursor, transcript, LAMBDA);

    for (int t = 1; t <= K; t++) {
        for (int j = 0; j < d; j++) {
            unsigned int a = addr_from(cursor, j, n);
            Block val = arena[a];

            unsigned char inp[LAMBDA + BLOCK_SIZE];
            memcpy(inp, cursor, LAMBDA);
            memcpy(inp + LAMBDA, val.data, LAMBDA);
            memcpy(inp + LAMBDA + LAMBDA, val.causal, LAMBDA);
            fast_hash(cursor, inp, LAMBDA + BLOCK_SIZE);
        }

        unsigned int w = addr_from(cursor, d, n);
        Block old = arena[w];

        unsigned char dinp[3 * LAMBDA];
        memcpy(dinp, old.data, LAMBDA);
        memcpy(dinp + LAMBDA, cursor, LAMBDA);
        memcpy(dinp + 2*LAMBDA, old.causal, LAMBDA);
        fast_hash(arena[w].data, dinp, 3 * LAMBDA);

        unsigned char cinp[2 * LAMBDA + 4];
        memcpy(cinp, old.causal, LAMBDA);
        memcpy(cinp + LAMBDA, cursor, LAMBDA);
        cinp[2*LAMBDA] = (t >> 24) & 0xff;
        cinp[2*LAMBDA+1] = (t >> 16) & 0xff;
        cinp[2*LAMBDA+2] = (t >> 8) & 0xff;
        cinp[2*LAMBDA+3] = t & 0xff;
        fast_hash(arena[w].causal, cinp, 2 * LAMBDA + 4);

        unsigned char tinp[LAMBDA + 4 + LAMBDA];
        memcpy(tinp, cursor, LAMBDA);
        tinp[LAMBDA] = (t >> 24) & 0xff;
        tinp[LAMBDA+1] = (t >> 16) & 0xff;
        tinp[LAMBDA+2] = (t >> 8) & 0xff;
        tinp[LAMBDA+3] = t & 0xff;
        memcpy(tinp + LAMBDA + 4, cursor, LAMBDA);
        fast_hash(cursor, tinp, LAMBDA + 4 + LAMBDA);
    }

    memcpy(transcript, cursor, LAMBDA);
}

void init_arena(Block *arena, int n) {
    for (int i = 0; i < n; i++) {
        unsigned char seed[8];
        seed[0] = (i >> 24) & 0xff; seed[1] = (i >> 16) & 0xff;
        seed[2] = (i >> 8) & 0xff; seed[3] = i & 0xff;
        seed[4] = 'p'; seed[5] = 'o'; seed[6] = 's'; seed[7] = 'm';
        fast_hash(arena[i].data, seed, 8);
        seed[4] = 'c'; seed[5] = 'a'; seed[6] = 'u'; seed[7] = 's';
        fast_hash(arena[i].causal, seed, 8);
    }
}

double now_sec() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec + ts.tv_nsec * 1e-9;
}

void hex8(const unsigned char *b, char *out) {
    for (int i = 0; i < 8; i++) sprintf(out + 2*i, "%02x", b[i]);
    out[16] = 0;
}

int main() {
    int n = 1 << 20;  // 64 MiB arena (fits in GPU memory)
    int d = 8;
    int K = 1 << 16;  // 64K steps

    printf("\n");
    printf("  ╔══════════════════════════════════════════════════════════╗\n");
    printf("  ║   PoSME CUDA Latency Benchmark                        ║\n");
    printf("  ║   CPU (DRAM) vs GPU (HBM) pointer-chase comparison     ║\n");
    printf("  ╠══════════════════════════════════════════════════════════╣\n");
    printf("  ║  Arena: %d blocks (%d MiB)                       ║\n", n, n * BLOCK_SIZE / (1<<20));
    printf("  ║  Steps: %d    Reads/step: %d                         ║\n", K, d);
    printf("  ╚══════════════════════════════════════════════════════════╝\n\n");

    size_t arena_bytes = (size_t)n * sizeof(Block);

    // --- CPU benchmark ---
    Block *h_arena = (Block*)malloc(arena_bytes);
    init_arena(h_arena, n);

    unsigned char transcript[LAMBDA];
    memset(transcript, 0x42, LAMBDA);

    printf("  ── CPU (System DRAM) ─────────────────────────────────────\n");
    double t0 = now_sec();
    posme_steps_cpu(h_arena, n, d, K, transcript);
    double cpu_time = now_sec() - t0;
    double cpu_ns = cpu_time / K * 1e9;

    char hex[17];
    hex8(transcript, hex);
    printf("    Steps: %d\n", K);
    printf("    Total: %.3f s\n", cpu_time);
    printf("    Avg step: %.0f ns\n", cpu_ns);
    printf("    T_K: %s...\n\n", hex);

    // --- GPU benchmark ---
    Block *d_arena;
    unsigned char *d_transcript;
    cudaMalloc(&d_arena, arena_bytes);
    cudaMalloc(&d_transcript, LAMBDA);

    // Re-init arena and transcript
    init_arena(h_arena, n);
    cudaMemcpy(d_arena, h_arena, arena_bytes, cudaMemcpyHostToDevice);

    unsigned char gpu_transcript[LAMBDA];
    memset(gpu_transcript, 0x42, LAMBDA);
    cudaMemcpy(d_transcript, gpu_transcript, LAMBDA, cudaMemcpyHostToDevice);

    cudaDeviceSynchronize();

    printf("  ── GPU (HBM) ─────────────────────────────────────────────\n");

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);

    cudaEventRecord(start);
    posme_step_kernel<<<1, 1>>>(d_arena, n, d, K, d_transcript);
    cudaEventRecord(stop);
    cudaEventSynchronize(stop);

    float gpu_ms;
    cudaEventElapsedTime(&gpu_ms, start, stop);
    double gpu_time = gpu_ms / 1000.0;
    double gpu_ns = gpu_time / K * 1e9;

    cudaMemcpy(gpu_transcript, d_transcript, LAMBDA, cudaMemcpyDeviceToHost);
    hex8(gpu_transcript, hex);

    printf("    Steps: %d\n", K);
    printf("    Total: %.3f s\n", gpu_time);
    printf("    Avg step: %.0f ns\n", gpu_ns);
    printf("    T_K: %s...\n\n", hex);

    // --- Comparison ---
    double ratio = cpu_ns / gpu_ns;

    printf("  ── ASIC Resistance Result ────────────────────────────────\n\n");
    printf("    CPU step:   %.0f ns (system DRAM)\n", cpu_ns);
    printf("    GPU step:   %.0f ns (HBM)\n", gpu_ns);
    printf("    Speedup:    %.2fx\n\n", ratio);

    if (ratio < 3.0) {
        printf("    CLAIM HOLDS: GPU (HBM) advantage is %.1fx (< 3x)\n", ratio);
        printf("    The latency-bound ASIC resistance claim is confirmed.\n");
        printf("    An adversary with HBM gains only %.1fx, not 8-16x.\n\n", ratio);
    } else if (ratio < 10.0) {
        printf("    MARGINAL: GPU advantage is %.1fx (3-10x range)\n", ratio);
        printf("    Higher than expected. May indicate cache effects or\n");
        printf("    hash computation dominating on slow GPU cores.\n\n");
    } else {
        printf("    CLAIM BROKEN: GPU advantage is %.1fx (>= 10x)\n", ratio);
        printf("    The construction is NOT latency-dominated.\n\n");
    }

    cudaFree(d_arena);
    cudaFree(d_transcript);
    free(h_arena);

    return 0;
}
