# ============================================================================
# CRON Sovereign Interop: Standalone Python Runtime (cron_runtime.py)
# Zero-Copy C FFI, DLPack Bridge & BitNet b1.58 Inference Engine
# ============================================================================
import ctypes
import os
import sys
import numpy as np

def find_cron_lib() -> ctypes.CDLL:
    names = ["cronc.dll", "libcronc.so", "libcronc.dylib"]
    search_paths = [
        ".",
        os.path.dirname(__file__),
        os.path.join(os.path.dirname(__file__), "..", "target", "release"),
        os.path.join(os.path.dirname(__file__), "..", "target", "debug"),
    ]
    for path in search_paths:
        for name in names:
            full = os.path.join(path, name)
            if os.path.exists(full):
                return ctypes.CDLL(os.path.abspath(full))
    raise FileNotFoundError("Could not find native CRON library (cronc.dll / libcronc.so)")

class CronRuntime:
    def __init__(self, lib_path: str = None):
        self.lib = ctypes.CDLL(lib_path) if lib_path else find_cron_lib()
        self._init_signatures()

    def _init_signatures(self):
        if hasattr(self.lib, "cron_mind_create"):
            self.lib.cron_mind_create.restype = ctypes.c_void_p
            self.lib.cron_mind_free.argtypes = [ctypes.c_void_p]
        if hasattr(self.lib, "cron_cl_jit_exec"):
            self.lib.cron_cl_jit_exec.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint32)]
            self.lib.cron_cl_jit_exec.restype = ctypes.c_int32

    def run_jit(self, cl_code: str) -> np.ndarray:
        out = (ctypes.c_uint32 * 16)()
        res = self.lib.cron_cl_jit_exec(cl_code.encode("utf-8"), out)
        if res != 0:
            raise RuntimeError(f"CRON JIT execution failed with code {res}")
        return np.array(list(out), dtype=np.uint32)
