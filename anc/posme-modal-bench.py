"""
PoSME Multi-Architecture Benchmark via Modal
=============================================
Usage:
    cd anc && uv run --with modal modal run posme-modal-bench.py

Requires: Modal account (https://modal.com). Run `modal setup` first.
"""

import modal
import os
import sys

app = modal.App("posme-bench")

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
         'echo "RAM: $(free -h 2>/dev/null | grep Mem | awk "{print \\$2}" || echo unknown)"; '
         'echo "Kernel: $(uname -r)"'],
        capture_output=True, text=True,
    )
    return f"\n{'='*62}\n  {label}\n{'='*62}\n{hw.stdout}\n{result.stdout}"


@app.function(image=rust_image, cpu=4, memory=4096, timeout=600)
def bench_cpu_standard(source: str):
    return _run("Standard CPU (4 vCPU, 4 GiB)", source)


@app.function(image=rust_image, cpu=8, memory=16384, timeout=600)
def bench_cpu_highmem(source: str):
    return _run("High-Memory CPU (8 vCPU, 16 GiB)", source)


@app.function(image=rust_image, cpu=2, memory=8192, timeout=600)
def bench_cpu_large_arena(source: str):
    return _run("Large Arena CPU (2 vCPU, 8 GiB)", source)


@app.function(image=rust_image, gpu="T4", cpu=4, memory=16384, timeout=600)
def bench_gpu_t4(source: str):
    return _run("GPU Instance (T4, system RAM)", source)


@app.local_entrypoint()
def main():
    sys.stdout.write("\n  PoSME Multi-Architecture Benchmark\n")
    sys.stdout.write("  Dispatching to Modal cloud instances...\n\n")

    src = _bench_source
    futures = [
        bench_cpu_standard.spawn(src),
        bench_cpu_highmem.spawn(src),
        bench_cpu_large_arena.spawn(src),
        bench_gpu_t4.spawn(src),
    ]

    results = []
    for f in futures:
        try:
            results.append(f.get())
        except Exception as e:
            results.append(f"\n  [FAILED: {e}]\n")

    sys.stdout.write("\n" + "=" * 62 + "\n")
    sys.stdout.write("  POSME MULTI-ARCHITECTURE BENCHMARK RESULTS\n")
    sys.stdout.write("=" * 62 + "\n")
    for r in results:
        sys.stdout.write(r + "\n")
    sys.stdout.write("=" * 62 + "\n")
    sys.stdout.write("  INTERPRETATION\n")
    sys.stdout.write("=" * 62 + "\n\n")
    sys.stdout.write("  If hash fraction < 10% across ALL architectures above,\n")
    sys.stdout.write("  the latency-bound ASIC resistance claim is validated.\n\n")
    sys.stdout.write("  Absolute step times (ns) will vary -- that's expected.\n")
    sys.stdout.write("  The hash FRACTION should be consistently low (<10%).\n\n")
