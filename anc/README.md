# PoSME Reference Benchmark

Ancillary material for **draft-condrey-cfrg-posme**
(Proof of Sequential Memory Execution).

## Quick Start

**macOS:**
```
./run-bench.sh
```
Pre-compiled signed binaries are included for ARM64 and x86-64.
No installation required.

**Windows:**
Double-click `run-bench.bat`. A pre-compiled binary is included.

**Linux / Other:**
```
./run-bench.sh
```
Compiles from source automatically. Installs Rust via rustup if
not present.

**Manual compilation (any platform):**
```
rustc -O posme-bench.rs -o posme-bench
./posme-bench
```
Single file. Zero external dependencies.

## What It Measures

The benchmark runs the PoSME step function at increasing arena
sizes and reports per-step timing with a hash/memory breakdown.

**Results that are consistent across machines:**
- Hash fraction (< 10% at 1 GiB arena)
- Step time increases with arena size (cache-to-DRAM transition)
- TMTO penalty matches formula: 1 + (1 - alpha)(2 * rho + 1)

**Results that vary by machine:**
- Absolute step time (ns) -- depends on CPU, memory speed, cache
- Total execution time (s) -- depends on step time and step count

The benchmark validates the construction's key claim: the
computational bottleneck is memory random-access latency, not
hash throughput. An ASIC that computes hashes 100x faster gains
almost nothing because hashing is < 10% of step cost.

## Files

| File | Description |
|------|-------------|
| `posme-bench.rs` | Source code (single file, no dependencies) |
| `run-bench.sh` | Runner script for macOS / Linux |
| `run-bench.bat` | Runner script for Windows |
| `posme-bench-macos-arm64` | Pre-compiled binary (Apple Silicon, signed) |
| `posme-bench-macos-x64` | Pre-compiled binary (Intel Mac, signed) |
| `posme-bench-windows-x64.exe` | Pre-compiled binary (Windows x64) |

## Parameters

The benchmark runs five configurations:

| Label | Arena | Steps | Rho | Expected Behavior |
|-------|-------|-------|-----|-------------------|
| L2-resident | 1 MiB | 16K | 1.0 | Fast (all cache hits) |
| L3-resident | 4 MiB | 64K | 1.0 | Fast (L3 hits) |
| L3-edge | 16 MiB | 256K | 1.0 | Transition zone |
| DRAM-spill | 64 MiB | 1M | 1.0 | Slower (cache misses begin) |
| Production | 1 GiB | 16M | 1.0 | Memory-bound (validates claim) |

## Runtime

Expect 1-5 minutes total depending on hardware. The production
configuration (1 GiB arena, 16M steps) takes 15-60 seconds.

## License

Apache-2.0. See the repository root for full license text.
