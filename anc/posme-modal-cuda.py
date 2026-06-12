"""
PoSME CUDA Latency Benchmark via Modal
=======================================
Compiles and runs the CUDA benchmark on GPU instances to measure
actual HBM vs DRAM pointer-chase latency ratio.

Usage:
    cd anc && uv run --with modal modal run posme-modal-cuda.py
"""

import modal
import os
import sys
import traceback

app = modal.App("posme-cuda")

_local_cu = os.path.join(os.path.dirname(os.path.abspath(__file__)), "posme-cuda-bench.cu")
_cuda_source = None
if os.path.exists(_local_cu):
    _cuda_source = open(_local_cu).read()

cuda_image = (
    modal.Image.from_registry("nvidia/cuda:12.4.0-devel-ubuntu22.04", add_python="3.11")
)


def _run(label, source):
    import subprocess
    with open("/tmp/posme-cuda-bench.cu", "w") as f:
        f.write(source)
    compile_result = subprocess.run(
        ["nvcc", "-O3", "/tmp/posme-cuda-bench.cu", "-o", "/tmp/posme-cuda-bench"],
        capture_output=True, text=True, timeout=120,
    )
    if compile_result.returncode != 0:
        return f"\n  [{label} COMPILE FAILED]\n{compile_result.stderr}\n"
    result = subprocess.run(
        ["/tmp/posme-cuda-bench"],
        capture_output=True, text=True, timeout=600,
    )
    gpu_info = subprocess.run(
        ["nvidia-smi", "--query-gpu=name,memory.total,memory.free",
         "--format=csv,noheader"],
        capture_output=True, text=True,
    )
    return f"\n{'='*62}\n  {label}\n  GPU: {gpu_info.stdout.strip()}\n{'='*62}\n{result.stdout}"


@app.function(image=cuda_image, gpu="T4", timeout=600)
def bench_t4(source: str):
    return _run("T4 (GDDR6, 16 GB)", source)


@app.function(image=cuda_image, gpu="A100-80GB", timeout=600)
def bench_a100(source: str):
    return _run("A100-80GB (HBM2e)", source)


@app.function(image=cuda_image, gpu="H100", timeout=600)
def bench_h100(source: str):
    return _run("H100 (HBM3, 80 GB)", source)


@app.function(image=cuda_image, gpu="L4", timeout=600)
def bench_l4(source: str):
    return _run("L4 (GDDR6, 24 GB)", source)


@app.local_entrypoint()
def main():
    sys.stdout.write("\n")
    sys.stdout.write("  ╔══════════════════════════════════════════════════════════╗\n")
    sys.stdout.write("  ║   PoSME CUDA Latency Benchmark (CPU vs GPU HBM)        ║\n")
    sys.stdout.write("  ║   The definitive ASIC resistance test                   ║\n")
    sys.stdout.write("  ╚══════════════════════════════════════════════════════════╝\n\n")

    src = _cuda_source
    configs = [
        ("T4", bench_t4),
        ("A100", bench_a100),
        ("H100", bench_h100),
        ("L4", bench_l4),
    ]

    futures = [(name, fn.spawn(src)) for name, fn in configs]

    for name, f in futures:
        try:
            sys.stdout.write(f.get() + "\n")
        except (modal.exception.Error, RuntimeError, TimeoutError, ConnectionError) as e:
            sys.stdout.write(f"\n  [{name} FAILED: {e}]\n\n")
            traceback.print_exc()
