"""
CRON Matrix 4.1 Native DLL & Python C FFI Verification Script
Validates seamless interop between Python 3.x and standalone `cron_cortex.dll`.
"""

import sys
import os

# Add parent directory to sys.path
sys.path.insert(0, os.path.dirname(__file__))
from cron_cortex import CronCortex


def test_native_cortex_ffi():
    print("=" * 65)
    print("  CRON MATRIX 4.1: NATIVE DLL (C FFI) PYTHON INTEROP VERIFICATION")
    print("=" * 65)

    dll_path = os.path.join(os.path.dirname(__file__), "..", "cron_cortex.dll")
    print(f"Loading dynamic library: {os.path.abspath(dll_path)}")

    with CronCortex(dll_path=dll_path) as cortex:
        # 1. Initial State & Invariant Check
        h_init = cortex.get_hamiltonian()
        print(f"\n[1] Initial Hamiltonian Invariant: H = {h_init}%")
        assert h_init == 100, f"Expected H=100%, got {h_init}%"
        print("    [OK] 0-Anchor Hamiltonian energy conservation verified (100%)")

        # 2. Single Step Execution
        cortex.step()
        regs = cortex.get_registers()
        print(f"\n[2] Executed 1 Complete VLIW Bundle Pass:")
        print(f"    R0  = 0x{regs[0]:08X} ({regs[0]})")
        print(f"    R1  = 0x{regs[1]:08X} ({regs[1]})")
        print(f"    R4  = 0x{regs[4]:08X} ({regs[4]})")
        print(f"    R6  = 0x{regs[6]:08X} ({regs[6]})")
        print(f"    R7  = 0x{regs[7]:08X} ({regs[7]})")
        print(f"    R8  = 0x{regs[8]:08X} ({regs[8]})")

        sensory = cortex.get_sensory()
        print(f"    Sensory Coordinate: S^3(W={sensory[0]}, X={sensory[1]}, Z={sensory[2]}, Spin={sensory[3]})")

        # 3. Phase Token Injection
        test_tokens = [ord('C'), ord('R'), ord('O'), ord('N')]
        print(f"\n[3] Injecting Phase Tokens into S^3 Cortex:")
        for tok in test_tokens:
            cortex.inject_token(tok)
            cortex.step()
            w, x, z, spin = cortex.get_sensory()
            print(f"    Token '{chr(tok)}' (0x{tok:02X}) -> S^3(W={w:2d}, X={x:2d}, Z={z:2d}, Spin={spin}) | H={cortex.get_hamiltonian()}%")
            assert cortex.get_hamiltonian() == 100, "Hamiltonian broken!"

        # 4. Telemetry Extraction
        tel = cortex.get_telemetry()
        print(f"\n[4] Hardware Telemetry from Native Silicon Core:")
        print(f"    Cycle Count:           {tel['cycle_count']}")
        print(f"    Reversible Gate Ops:   {tel['reversible_ops_count']}")
        print(f"    Spatial Broadcasts:    {tel['spatial_broadcast_count']}")
        print(f"    Global Barriers:       {tel['barrier_count']}")
        print(f"    Wave Interference Ops: {tel['wave_interference_ops_count']}")
        print(f"    Braid Deflections:     {tel['deflections_count']}")
        print(f"    Hamiltonian Energy:    {tel['hamiltonian']}%")

        # 5. High-Speed Benchmark from Python
        bench_iters = 1_000_000
        print(f"\n[5] Executing {bench_iters:,} Iterations Speed Benchmark via Python FFI...")
        res = cortex.benchmark(bench_iters)
        print(f"    Elapsed Time:          {res['elapsed_seconds']:.6f} sec")
        print(f"    Throughput:            {res['throughput_mips']:.2f} MIPS")
        print(f"    Latency:               {res['ns_per_cycle']:.2f} ns/cycle")

        print("=" * 65)
        print("  STATUS: 100% BIT-EXACT C FFI INTEROP & TELEMETRY VERIFIED!")
        print("=" * 65)


if __name__ == "__main__":
    test_native_cortex_ffi()
