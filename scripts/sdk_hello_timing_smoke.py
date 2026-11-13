#!/usr/bin/env python3
"""Cold/warm/gate wall-clock for packaged SDK `syx run hello.sy`.

Usage: sdk_hello_timing_smoke.py <syx-bin> <hello.sy> [--gate-s 5.0]
Exit non-zero if gate run is slower than --gate-s (embed/rkyv miss signal).
"""
from __future__ import annotations

import argparse
import subprocess
import sys
import time


def timed_run(bin_path: str, hello: str, label: str) -> float:
    t0 = time.perf_counter()
    result = subprocess.run([bin_path, "run", hello], capture_output=True, text=True)
    dt = time.perf_counter() - t0
    sys.stdout.write(result.stdout)
    sys.stderr.write(result.stderr)
    print(f"sdk_hello_{label}_s={dt:.3f}")
    if result.returncode != 0:
        raise SystemExit(result.returncode)
    return dt


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("syx_bin")
    parser.add_argument("hello_sy")
    parser.add_argument("--gate-s", type=float, default=5.0)
    args = parser.parse_args()

    timed_run(args.syx_bin, args.hello_sy, "cold")
    timed_run(args.syx_bin, args.hello_sy, "warm")
    gate = timed_run(args.syx_bin, args.hello_sy, "gate")
    if gate >= args.gate_s:
        print(
            f"ERROR: packaged syx run hello too slow ({gate:.3f}s ≥ {args.gate_s}s) — "
            "likely embed/rkyv miss",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
