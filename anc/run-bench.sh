#!/bin/bash
# PoSME Benchmark Runner
# Just run: ./run-bench.sh
# Or double-click in a file manager (macOS/Linux).
#
# Pre-compiled binaries are included for macOS (ARM64/x64) and
# Windows (x64). Other platforms compile from source automatically
# (installs Rust if needed).

set -e
cd "$(dirname "$0")"

ARCH=$(uname -m)
OS=$(uname -s)
BIN=""

if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ] && [ -x "./posme-bench-macos-arm64" ]; then
        BIN="./posme-bench-macos-arm64"
    elif [ -x "./posme-bench-macos-x64" ]; then
        BIN="./posme-bench-macos-x64"
    fi
fi

if [ -z "$BIN" ]; then
    if ! command -v rustc &>/dev/null; then
        echo ""
        echo "  Rust compiler not found. Installing via rustup ..."
        echo ""
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet
        . "$HOME/.cargo/env"
    fi
    echo ""
    echo "  Compiling posme-bench.rs for $OS/$ARCH ..."
    rustc -O posme-bench.rs -o posme-bench 2>&1
    BIN="./posme-bench"
fi

echo ""
echo "  Running benchmark (this takes 1-5 minutes) ..."
echo ""
$BIN

read -p "  Press Enter to exit." _
