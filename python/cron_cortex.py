"""
CRON Matrix 4.1 Sovereign Cortex Native C FFI Python Wrapper
High-performance Python bindings for standalone `cron_cortex.dll` / `libcron_cortex.so`.
Enables calling bare-metal 4D Torus & Clifford S^3 Geodesic Kinematics directly from Python.
Zero-Python overhead in the silicon core: 100% Native C23 + AVX-512.
"""

import ctypes
import os
import sys
import time
from typing import Dict, Any, Tuple, List, Union


class CronCortexTelemetry(ctypes.Structure):
    _fields_ = [
        ("cycle_count", ctypes.c_uint64),
        ("optical_gemm_count", ctypes.c_uint64),
        ("reversible_ops_count", ctypes.c_uint64),
        ("stdp_updates_count", ctypes.c_uint64),
        ("spatial_broadcast_count", ctypes.c_uint64),
        ("barrier_count", ctypes.c_uint64),
        ("wave_interference_ops_count", ctypes.c_uint64),
        ("deflections_count", ctypes.c_uint64),
        ("hamiltonian", ctypes.c_uint32),
        ("sensory_w", ctypes.c_uint32),
        ("sensory_x", ctypes.c_uint32),
        ("sensory_z", ctypes.c_uint32),
        ("sensory_spin", ctypes.c_uint32),
    ]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "cycle_count": self.cycle_count,
            "optical_gemm_count": self.optical_gemm_count,
            "reversible_ops_count": self.reversible_ops_count,
            "stdp_updates_count": self.stdp_updates_count,
            "spatial_broadcast_count": self.spatial_broadcast_count,
            "barrier_count": self.barrier_count,
            "wave_interference_ops_count": self.wave_interference_ops_count,
            "deflections_count": self.deflections_count,
            "hamiltonian": self.hamiltonian,
            "sensory_coordinate": {
                "w": self.sensory_w,
                "x": self.sensory_x,
                "z": self.sensory_z,
                "spin": self.sensory_spin,
            },
        }


class CronCortex:
    """Python interface to the native CRON Matrix 4.1 C FFI engine."""

    def __init__(self, dll_path: str = None):
        if dll_path is None:
            # Look in current working directory, script directory, or parent directory
            candidates = [
                "cron_cortex.dll",
                os.path.join(os.path.dirname(__file__), "cron_cortex.dll"),
                os.path.join(os.path.dirname(__file__), "..", "cron_cortex.dll"),
                "libcron_cortex.so",
                os.path.join(os.path.dirname(__file__), "libcron_cortex.so"),
            ]
            dll_path = next((c for c in candidates if os.path.exists(c)), None)
            if not dll_path:
                raise FileNotFoundError(
                    "Could not find 'cron_cortex.dll' (or .so). "
                    "Run 'cron matrix4 native --dll' or pass explicit dll_path."
                )

        self._lib = ctypes.CDLL(os.path.abspath(dll_path))
        self._setup_prototypes()
        self._handle = self._lib.cron_cortex_create()
        if not self._handle:
            raise MemoryError("Failed to allocate native CronSiliconCore via cron_cortex_create.")

    def _setup_prototypes(self):
        # cron_cortex_create
        self._lib.cron_cortex_create.argtypes = []
        self._lib.cron_cortex_create.restype = ctypes.c_void_p

        # cron_cortex_destroy
        self._lib.cron_cortex_destroy.argtypes = [ctypes.c_void_p]
        self._lib.cron_cortex_destroy.restype = None

        # cron_cortex_reset
        self._lib.cron_cortex_reset.argtypes = [ctypes.c_void_p]
        self._lib.cron_cortex_reset.restype = None

        # cron_cortex_step
        self._lib.cron_cortex_step.argtypes = [ctypes.c_void_p]
        self._lib.cron_cortex_step.restype = None

        # cron_cortex_run
        self._lib.cron_cortex_run.argtypes = [ctypes.c_void_p, ctypes.c_uint64]
        self._lib.cron_cortex_run.restype = None

        # cron_cortex_get_reg
        self._lib.cron_cortex_get_reg.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
        self._lib.cron_cortex_get_reg.restype = ctypes.c_uint32

        # cron_cortex_set_reg
        self._lib.cron_cortex_set_reg.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint32]
        self._lib.cron_cortex_set_reg.restype = None

        # cron_cortex_inject_token
        self._lib.cron_cortex_inject_token.argtypes = [ctypes.c_void_p, ctypes.c_uint8]
        self._lib.cron_cortex_inject_token.restype = None

        # cron_cortex_get_hamiltonian
        self._lib.cron_cortex_get_hamiltonian.argtypes = [ctypes.c_void_p]
        self._lib.cron_cortex_get_hamiltonian.restype = ctypes.c_uint32

        # cron_cortex_get_sensory
        self._lib.cron_cortex_get_sensory.argtypes = [
            ctypes.c_void_p,
            ctypes.POINTER(ctypes.c_uint32),
            ctypes.POINTER(ctypes.c_uint32),
            ctypes.POINTER(ctypes.c_uint32),
            ctypes.POINTER(ctypes.c_uint32),
        ]
        self._lib.cron_cortex_get_sensory.restype = None

        # cron_cortex_get_telemetry
        self._lib.cron_cortex_get_telemetry.argtypes = [
            ctypes.c_void_p,
            ctypes.POINTER(CronCortexTelemetry),
        ]
        self._lib.cron_cortex_get_telemetry.restype = None

    def close(self):
        if self._handle:
            self._lib.cron_cortex_destroy(self._handle)
            self._handle = None

    def __del__(self):
        self.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def reset(self):
        """Resets silicon registers, hydraulics, and wave states to default."""
        self._lib.cron_cortex_reset(self._handle)

    def step(self):
        """Executes 1 complete VLIW bundle cycle across all 256 cores."""
        self._lib.cron_cortex_step(self._handle)

    def run(self, cycles: int):
        """Executes N VLIW bundle cycles consecutively at hardware speed."""
        self._lib.cron_cortex_run(self._handle, ctypes.c_uint64(cycles))

    def get_register(self, reg_idx: int) -> int:
        """Reads 32-bit register R[0..15]."""
        if not 0 <= reg_idx < 16:
            raise IndexError("Register index must be between 0 and 15")
        return self._lib.cron_cortex_get_reg(self._handle, reg_idx)

    def set_register(self, reg_idx: int, val: int):
        """Writes 32-bit value to register R[0..15]."""
        if not 0 <= reg_idx < 16:
            raise IndexError("Register index must be between 0 and 15")
        self._lib.cron_cortex_set_reg(self._handle, reg_idx, val & 0xFFFFFFFF)

    def get_registers(self) -> List[int]:
        """Reads all 16 registers [R0..R15]."""
        return [self.get_register(i) for i in range(16)]

    def inject_token(self, token: Union[int, str, bytes]):
        """Injects a phase token byte into sensory input register R1."""
        if isinstance(token, str):
            byte_val = ord(token[0]) if token else 0
        elif isinstance(token, bytes):
            byte_val = token[0] if token else 0
        else:
            byte_val = int(token) & 0xFF
        self._lib.cron_cortex_inject_token(self._handle, byte_val)

    def get_hamiltonian(self) -> int:
        """Returns 0-Anchor Hamiltonian energy conservation invariant (H == 100%)."""
        return self._lib.cron_cortex_get_hamiltonian(self._handle)

    def get_sensory(self) -> Tuple[int, int, int, int]:
        """Returns (W, X, Z, Spin) coordinate on 4D S^3 sphere."""
        w = ctypes.c_uint32()
        x = ctypes.c_uint32()
        z = ctypes.c_uint32()
        spin = ctypes.c_uint32()
        self._lib.cron_cortex_get_sensory(
            self._handle,
            ctypes.byref(w),
            ctypes.byref(x),
            ctypes.byref(z),
            ctypes.byref(spin),
        )
        return (w.value, x.value, z.value, spin.value)

    def get_telemetry(self) -> Dict[str, Any]:
        """Returns hardware execution counters and state telemetry."""
        tel = CronCortexTelemetry()
        self._lib.cron_cortex_get_telemetry(self._handle, ctypes.byref(tel))
        return tel.to_dict()

    def benchmark(self, iterations: int = 1_000_000) -> Dict[str, float]:
        """Runs empirical speed test directly against host native DLL."""
        t_start = time.perf_counter()
        self.run(iterations)
        elapsed = time.perf_counter() - t_start

        # 5 VLIW bundles per iteration in cortex_program
        total_bundles = iterations * 5
        mips = (total_bundles / max(elapsed, 1e-9)) / 1e6
        ns_per_cycle = (elapsed / max(total_bundles, 1)) * 1e9

        return {
            "iterations": iterations,
            "elapsed_seconds": elapsed,
            "throughput_mips": mips,
            "ns_per_cycle": ns_per_cycle,
        }
