#!/usr/bin/env python3
"""
================================================================================
CRON SOVEREIGN ECOSYSTEM MASTER AUDIT RUNNER
Runs all 5 debugging layers (D1-D5), C++20, and Python live benchmarks in one command.
================================================================================
"""

import sys
import subprocess
import time
import os

def run_step(step_name, cmd, cwd=None):
    print(f"\n[RUNNING] {step_name}...")
    t0 = time.perf_counter()
    res = subprocess.run(cmd, cwd=cwd, shell=isinstance(cmd, str))
    elapsed = (time.perf_counter() - t0)
    if res.returncode != 0:
        print(f"[FAILED] {step_name} failed with exit code {res.returncode}")
        sys.exit(res.returncode)
    print(f"[PASSED]  {step_name} completed in {elapsed:.2f}s")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    print("=" * 80)
    print(" CRON SOVEREIGN LANGUAGE: MASTER ECOSYSTEM REGRESSION AUDIT (PHASES D1 - D5)")
    print("=" * 80)

    # 1. D1: Finite-difference audit
    run_step("Phase D1: Finite-Difference Gradient & Numerical Precision Audit",
             ["cargo", "test", "-p", "cronc", "--test", "test_finite_difference_gradient_audit"], cwd=root)

    # 2. D2: Cross-backend parity
    run_step("Phase D2: Cross-Backend Parity (5-way check)",
             ["cargo", "test", "-p", "cronc", "--test", "test_cross_backend_parity"], cwd=root)

    # 3. D3: Memory safety & FFI
    run_step("Phase D3: Memory Safety & FFI 100k Leak Audit",
             ["cargo", "test", "-p", "cronc", "--test", "test_memory_safety_and_ffi"], cwd=root)

    # 4. D4: Shape checker & circular dependency
    run_step("Phase D4: Dimension Fuzzing & Circular Graph Diagnostics",
             ["cargo", "test", "-p", "cronc", "--test", "test_shape_fuzzing_and_diagnostics"], cwd=root)

    # 5. C++20 / CUDA turnkey suite
    cpp_exe = os.path.join(root, "examples", "cpp", "test_cron_cpp20.exe")
    run_step("Phase D5.1: C++20 & CUDA Build",
             ["g++", "-std=c++20", "-O3", "examples/cpp/test_cron_cpp20.cpp", "-Iexamples/cpp", "-o", cpp_exe], cwd=root)
    run_step("Phase D5.2: C++20 & CUDA Interop Suite Run", [cpp_exe], cwd=root)

    # 6. Python ctypes live inference benchmark
    run_step("Phase D5.3: Python Live C-ABI Benchmark (100k calls)",
             [sys.executable, "examples/python/demo_c_abi_live_inference.py"], cwd=root)

    print("\n" + "=" * 80)
    print(" [ALL 100% GREEN] COMPLETE CRON ECOSYSTEM AUDITED SUCCESSFULLY!")
    print("=" * 80)

if __name__ == "__main__":
    main()
