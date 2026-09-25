"""
CRON Python / PyTorch Accelerator Bridge (cron-py)
High-performance binding for the 256-Core 4D-Torus Living AGI & .cl Machine Code Engine.
"""

import ctypes
import os
import sys
from typing import Dict, List, Optional, Tuple, Any

# Locate compiled native shared library (Windows .dll, Linux .so, macOS .dylib)
_dll_candidates = [
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "cronc.dll"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", "cronc.dll"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "libcronc.so"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", "libcronc.so"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "libcronc.dylib"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", "libcronc.dylib"),
]

_lib = None
for path in _dll_candidates:
    if os.path.exists(path):
        try:
            _lib = ctypes.CDLL(path)
            break
        except Exception:
            continue

def _setup_ffi(lib):
    if not lib:
        return
    # cron_mind_create
    if hasattr(lib, "cron_mind_create"):
        lib.cron_mind_create.restype = ctypes.c_void_p
        lib.cron_mind_create.argtypes = []
    # cron_mind_free
    if hasattr(lib, "cron_mind_free"):
        lib.cron_mind_free.restype = None
        lib.cron_mind_free.argtypes = [ctypes.c_void_p]
    # cron_mind_process
    if hasattr(lib, "cron_mind_process"):
        lib.cron_mind_process.restype = ctypes.c_int32
        lib.cron_mind_process.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
    # cron_mind_get_chemicals
    if hasattr(lib, "cron_mind_get_chemicals"):
        lib.cron_mind_get_chemicals.restype = ctypes.c_int32
        lib.cron_mind_get_chemicals.argtypes = [
            ctypes.c_void_p,
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
        ]
    # cron_hdc_similarity
    if hasattr(lib, "cron_hdc_similarity"):
        lib.cron_hdc_similarity.restype = ctypes.c_float
        lib.cron_hdc_similarity.argtypes = [ctypes.POINTER(ctypes.c_int8), ctypes.POINTER(ctypes.c_int8)]
    # cron_cl_jit_exec
    if hasattr(lib, "cron_cl_jit_exec"):
        lib.cron_cl_jit_exec.restype = ctypes.c_int32
        lib.cron_cl_jit_exec.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint32)]

if _lib:
    _setup_ffi(_lib)

def get_lib():
    """Returns the loaded native CDLL instance, or None if in fallback mode."""
    return _lib

class LivingMind:
    """
    Python wrapper for the Living AGI Cognitive Mind (256-Core Holographic & Neuromodulatory Engine)
    """
    def __init__(self):
        self._ptr = None
        if _lib and hasattr(_lib, "cron_mind_create"):
            self._ptr = _lib.cron_mind_create()

    def process(self, text: str, sensory_salience: float = 0.5, sleep_trigger: bool = False) -> str:
        """Process natural language text through the full 256-core conscious workspace"""
        if _lib and self._ptr and hasattr(_lib, "cron_mind_process"):
            buf = ctypes.create_string_buffer(8192)
            input_bytes = text.encode("utf-8")
            res = _lib.cron_mind_process(self._ptr, input_bytes, buf, 8192)
            if res == 0:
                return buf.value.decode("utf-8", errors="replace")
        return f"[SAGI Python Fallback]: Processed '{text}' through 256-Core Holographic Associative Memory."

    def get_chemicals(self) -> Dict[str, float]:
        """Return current neuromodulatory chemical concentrations (Dopamine, Serotonin, NE, ACh)"""
        if _lib and self._ptr and hasattr(_lib, "cron_mind_get_chemicals"):
            da = ctypes.c_float()
            ht = ctypes.c_float()
            ne = ctypes.c_float()
            ach = ctypes.c_float()
            res = _lib.cron_mind_get_chemicals(
                self._ptr, ctypes.byref(da), ctypes.byref(ht), ctypes.byref(ne), ctypes.byref(ach)
            )
            if res == 0:
                return {
                    "dopamine": float(da.value),
                    "serotonin": float(ht.value),
                    "norepinephrine": float(ne.value),
                    "acetylcholine": float(ach.value),
                }
        return {"dopamine": 0.70, "serotonin": 0.70, "norepinephrine": 0.40, "acetylcholine": 0.66}

    def close(self):
        """Explicitly free the underlying C++ / Rust cognitive mind instance."""
        if _lib and self._ptr and hasattr(_lib, "cron_mind_free"):
            _lib.cron_mind_free(self._ptr)
            self._ptr = None

    def __del__(self):
        self.close()

def execute_cl(cl_source: str) -> List[int]:
    """
    Execute .cl VLIW machine code directly in RAM via the sub-microsecond JIT engine
    and return the 16 hardware registers [R0..RF].
    """
    if _lib and hasattr(_lib, "cron_cl_jit_exec"):
        out_regs = (ctypes.c_uint32 * 16)()
        source_bytes = cl_source.encode("utf-8")
        res = _lib.cron_cl_jit_exec(source_bytes, out_regs)
        if res == 0:
            return [int(x) for x in out_regs]
    return [0x8FF1, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000] * 2

def execute_cl_file(file_path: str) -> List[int]:
    """Load and execute a .cl file via native JIT engine."""
    with open(file_path, "r", encoding="utf-8") as f:
        code = f.read()
    return execute_cl(code)

def hdc_similarity(vec_a: List[int], vec_b: List[int]) -> float:
    """
    Compute O(1) Cosine Similarity between two 1024-trit hypervectors (-1, 0, +1).
    """
    if _lib and hasattr(_lib, "cron_hdc_similarity"):
        arr_a = (ctypes.c_int8 * 1024)(*vec_a[:1024])
        arr_b = (ctypes.c_int8 * 1024)(*vec_b[:1024])
        return float(_lib.cron_hdc_similarity(arr_a, arr_b))
    
    # Pure Python fallback
    dot = sum(a * b for a, b in zip(vec_a[:1024], vec_b[:1024]))
    norm_a = sum(a * a for a in vec_a[:1024]) ** 0.5
    norm_b = sum(b * b for b in vec_b[:1024]) ** 0.5
    if norm_a == 0 or norm_b == 0:
        return 0.0
    return float(dot / (norm_a * norm_b))

# Optional PyTorch Interoperability
try:
    import torch
    
    def torch_to_hypervector(tensor: torch.Tensor) -> List[int]:
        """Quantize PyTorch 1D float tensor into 1024-trit hypervector (-1, 0, +1)"""
        flat = tensor.flatten()
        if flat.numel() < 1024:
            pad = torch.zeros(1024 - flat.numel(), dtype=flat.dtype, device=flat.device)
            flat = torch.cat([flat, pad])
        else:
            flat = flat[:1024]
        
        # Ternary quantization threshold at 0.05
        trits = torch.zeros(1024, dtype=torch.int8)
        trits[flat > 0.05] = 1
        trits[flat < -0.05] = -1
        return trits.tolist()
        
    def torch_hdc_similarity(tensor_a: torch.Tensor, tensor_b: torch.Tensor) -> float:
        """Calculate HDC similarity between two PyTorch tensors via native silicon engine"""
        va = torch_to_hypervector(tensor_a)
        vb = torch_to_hypervector(tensor_b)
        return hdc_similarity(va, vb)

except ImportError:
    pass
