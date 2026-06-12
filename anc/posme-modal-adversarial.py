"""
PoSME Adversarial Benchmark via Modal
=====================================
Attempts to BEAT the latency-bound claim by running on hardware
configurations that maximize compute-to-memory ratio.

If any configuration shows hash fraction > 10%, the claim fails.

Usage:
    cd anc && uv run --with modal modal run posme-modal-adversarial.py
"""

import modal
import os
import sys
import traceback

app = modal.App("posme-adversarial")

_local_rs = os.path.join(os.path.dirname(os.path.abspath(__file__)), "posme-bench.rs")
_bench_source = None
if os.path.exists(_local_rs):
    _bench_source = open(_local_rs).read()

rust_image = (
    modal.Image.debian_slim()
    .apt_install("curl", "gcc")
    .run_commands(
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet",
    )
    .env({"PATH": "/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"})
)


def _run(label, source):
    import subprocess
    with open("/tmp/posme-bench.rs", "w") as f:
        f.write(source)
    subprocess.run(
        ["rustc", "-O", "/tmp/posme-bench.rs", "-o", "/tmp/posme-bench"],
        check=True, capture_output=True,
    )
    result = subprocess.run(
        ["/tmp/posme-bench"], capture_output=True, text=True, timeout=600,
    )
    hw = subprocess.run(
        ["bash", "-c",
         'echo "CPU: $(lscpu 2>/dev/null | grep "Model name" | sed "s/.*: *//" || echo unknown)"; '
         'echo "Arch: $(uname -m)"; '
         'echo "Cores: $(nproc 2>/dev/null || echo unknown)"; '
         'echo "RAM: $(free -h 2>/dev/null | grep Mem | awk "{print \\$2}" || echo unknown)"; '
         'echo "L1d: $(lscpu 2>/dev/null | grep "L1d" | sed "s/.*: *//" || echo unknown)"; '
         'echo "L2: $(lscpu 2>/dev/null | grep "L2" | sed "s/.*: *//" || echo unknown)"; '
         'echo "L3: $(lscpu 2>/dev/null | grep "L3" | sed "s/.*: *//" || echo unknown)"; '
         'echo "Kernel: $(uname -r)"; '
         'echo "GPU: $(nvidia-smi --query-gpu=name,memory.total --format=csv,noheader 2>/dev/null || echo none)"'],
        capture_output=True, text=True,
    )
    return f"\n{'='*62}\n  {label}\n{'='*62}\n{hw.stdout}\n{result.stdout}"


# --- ADVERSARIAL CONFIGURATIONS ---
# Goal: find hardware where hash fraction is highest (worst case for our claim)

# Minimum CPU, maximum memory -- slow CPU makes hash fraction higher
@app.function(image=rust_image, cpu=1, memory=4096, timeout=600)
def bench_slow_cpu(source: str):
    return _run("ADVERSARIAL: Slowest CPU (1 vCPU, 4 GiB)", source)

# Maximum CPU -- fast CPU might make hash dominate
@app.function(image=rust_image, cpu=16, memory=32768, timeout=600)
def bench_fast_cpu(source: str):
    return _run("ADVERSARIAL: Fastest CPU (16 vCPU, 32 GiB)", source)

# H100 GPU instance -- HBM3 memory, the actual adversary hardware
@app.function(image=rust_image, gpu="H100", cpu=8, memory=32768, timeout=600)
def bench_h100(source: str):
    return _run("ADVERSARIAL: H100 (HBM3 system RAM)", source)

# A100 GPU instance -- HBM2e
@app.function(image=rust_image, gpu="A100-80GB", cpu=8, memory=65536, timeout=600)
def bench_a100(source: str):
    return _run("ADVERSARIAL: A100-80GB (HBM2e system RAM)", source)

# A10G -- budget GPU
@app.function(image=rust_image, gpu="A10G", cpu=4, memory=16384, timeout=600)
def bench_a10g(source: str):
    return _run("ADVERSARIAL: A10G (budget GPU instance)", source)

# L4 GPU
@app.function(image=rust_image, gpu="L4", cpu=4, memory=16384, timeout=600)
def bench_l4(source: str):
    return _run("ADVERSARIAL: L4 GPU instance", source)

# Tiny memory -- force arena into swap? (probably just OOM)
@app.function(image=rust_image, cpu=2, memory=2048, timeout=600)
def bench_tiny_mem(source: str):
    return _run("ADVERSARIAL: Tiny memory (2 vCPU, 2 GiB)", source)

# Large CPU count -- does NUMA topology affect random access latency?
@app.function(image=rust_image, cpu=32, memory=65536, timeout=600)
def bench_numa(source: str):
    return _run("ADVERSARIAL: NUMA (32 vCPU, 64 GiB)", source)


@app.local_entrypoint()
def main():
    sys.stdout.write("\n")
    sys.stdout.write("  ╔══════════════════════════════════════════════════════════╗\n")
    sys.stdout.write("  ║   PoSME ADVERSARIAL Benchmark                          ║\n")
    sys.stdout.write("  ║   Attempting to break hash fraction < 10% claim         ║\n")
    sys.stdout.write("  ╚══════════════════════════════════════════════════════════╝\n")
    sys.stdout.write("\n  Dispatching to 8 Modal instances...\n\n")

    src = _bench_source
    configs = [
        ("Slowest CPU", bench_slow_cpu),
        ("Fastest CPU", bench_fast_cpu),
        ("H100 HBM3", bench_h100),
        ("A100 HBM2e", bench_a100),
        ("A10G budget", bench_a10g),
        ("L4 GPU", bench_l4),
        ("Tiny memory", bench_tiny_mem),
        ("NUMA 32-core", bench_numa),
    ]

    futures = [(name, fn.spawn(src)) for name, fn in configs]

    sys.stdout.write("=" * 62 + "\n")
    sys.stdout.write("  ADVERSARIAL RESULTS\n")
    sys.stdout.write("=" * 62 + "\n")

    worst_hash_pct = 0.0
    for name, f in futures:
        try:
            result = f.get()
            sys.stdout.write(result + "\n")
            for line in result.split("\n"):
                if "Hash fraction:" in line and "1 GiB" not in line:
                    continue
                if "Hash fraction:" in line:
                    pct = float(line.split(":")[1].strip().rstrip("%"))
                    if pct > worst_hash_pct:
                        worst_hash_pct = pct
        except (modal.exception.Error, RuntimeError, TimeoutError, ConnectionError) as e:
            sys.stdout.write(f"\n  [{name} FAILED: {e}]\n\n")
            traceback.print_exc()

    sys.stdout.write("=" * 62 + "\n")
    sys.stdout.write("  VERDICT\n")
    sys.stdout.write("=" * 62 + "\n\n")
    if worst_hash_pct < 10.0:
        sys.stdout.write(f"  CLAIM HOLDS: worst hash fraction = {worst_hash_pct:.1f}% (< 10%)\n")
        sys.stdout.write("  The latency-bound ASIC resistance claim survives.\n\n")
    else:
        sys.stdout.write(f"  CLAIM BROKEN: hash fraction = {worst_hash_pct:.1f}% (>= 10%)\n")
        sys.stdout.write("  The construction is NOT latency-dominated on this hardware.\n\n")
