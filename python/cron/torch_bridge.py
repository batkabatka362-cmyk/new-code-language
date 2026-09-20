# ============================================================================
# CRON PyTorch Acceleration Bridge (torch_bridge.py)
# Drop-in Replacement for torch.nn.Linear and Attention with BitNet b1.58
# Targets: Native C23 / AVX2 / OpenMP Shared Accelerator Library
# ============================================================================

import os
import ctypes
import numpy as np
from typing import Optional, Tuple, Union

try:
    import torch
    import torch.nn as nn
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False
    nn = object # fallback

_LIB_CACHE = None

def find_native_library() -> str:
    """Locates the compiled libcron_native.dll or .so library."""
    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    
    lib_names = [
        "libcron_native.dll",
        "cron_native.dll",
        "libcron_native.so",
        "libcron_native.dylib",
    ]
    
    search_dirs = [
        base_dir,
        os.path.join(base_dir, "python", "cron"),
        os.path.join(base_dir, "target", "release"),
        os.path.join(base_dir, "target", "debug"),
    ]
    
    for d in search_dirs:
        for name in lib_names:
            p = os.path.join(d, name)
            if os.path.exists(p):
                return p
                
    raise FileNotFoundError(
        "Could not find 'libcron_native.dll' (or .so). "
        "Please build it using 'gcc -std=c2x -shared -O3 -mavx2 -mfma src_native/cron_native.c -o libcron_native.dll'."
    )

def get_native_lib():
    """Returns the loaded ctypes CDLL handle with initialized signatures."""
    global _LIB_CACHE
    if _LIB_CACHE is not None:
        return _LIB_CACHE
        
    lib_path = find_native_library()
    if hasattr(os, "add_dll_directory"):
        try:
            os.add_dll_directory(os.path.dirname(lib_path))
        except Exception:
            pass
        import shutil
        gcc_bin = shutil.which("gcc")
        if gcc_bin:
            try:
                os.add_dll_directory(os.path.dirname(gcc_bin))
            except Exception:
                pass
    lib = ctypes.CDLL(lib_path)
    
    # void cron_pack_trits_b158(const float* W, uint8_t* W_packed, float* scale_out, int K, int N)
    lib.cron_pack_trits_b158.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_uint8),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int,
        ctypes.c_int,
    ]
    lib.cron_pack_trits_b158.restype = None

    # void cron_bitnet_gemm_f32(const float* X, const uint8_t* W_packed, const float* bias, float* Y, int M, int K, int N, float scale)
    lib.cron_bitnet_gemm_f32.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_uint8),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_float,
    ]
    lib.cron_bitnet_gemm_f32.restype = None

    # void cron_ringtape_attention_f32(...)
    lib.cron_ringtape_attention_f32.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
    ]
    lib.cron_ringtape_attention_f32.restype = None

    # void cron_fast_rmsnorm_f32(const float* X, const float* W, float* Y, int M, int D, float eps)
    lib.cron_fast_rmsnorm_f32.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_float,
    ]
    lib.cron_fast_rmsnorm_f32.restype = None

    # const char* cron_native_version(void)
    lib.cron_native_version.argtypes = []
    lib.cron_native_version.restype = ctypes.c_char_p

    # uint32_t cron_native_features(void)
    lib.cron_native_features.argtypes = []
    lib.cron_native_features.restype = ctypes.c_uint32

    # void cron_fused_rmsnorm_linear(...)
    lib.cron_fused_rmsnorm_linear.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_float,
    ]
    lib.cron_fused_rmsnorm_linear.restype = None

    # void cron_fused_swiglu(...)
    lib.cron_fused_swiglu.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
    ]
    lib.cron_fused_swiglu.restype = None

    # void cron_flash_attention_2(...)
    lib.cron_flash_attention_2.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_float,
    ]
    lib.cron_flash_attention_2.restype = None

    # void cron_fused_transformer_block(...)
    lib.cron_fused_transformer_block.argtypes = [
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.POINTER(ctypes.c_float),
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
        ctypes.c_int64,
    ]
    lib.cron_fused_transformer_block.restype = None

    _LIB_CACHE = lib
    return lib

def get_native_version() -> str:
    lib = get_native_lib()
    return lib.cron_native_version().decode("utf-8")

def pack_ternary_weights(weight: Union[torch.Tensor, np.ndarray]) -> Tuple[torch.Tensor, float]:
    """
    Quantizes an [N, K] weight matrix into packed BitNet 1.58b ternary {-1, 0, +1} format.
    Returns: (packed_weights_tensor_uint8, scale_factor_float)
    """
    if not TORCH_AVAILABLE:
        raise RuntimeError("PyTorch is required for pack_ternary_weights")
        
    lib = get_native_lib()
    if not isinstance(weight, torch.Tensor):
        weight = torch.tensor(weight, dtype=torch.float32)
        
    weight = weight.contiguous().float()
    N, K = weight.shape
    total_elements = N * K
    packed_bytes = (total_elements + 3) // 4
    
    packed_w = torch.zeros(packed_bytes, dtype=torch.uint8)
    scale_out = ctypes.c_float(1.0)
    
    w_ptr = ctypes.cast(weight.data_ptr(), ctypes.POINTER(ctypes.c_float))
    packed_ptr = ctypes.cast(packed_w.data_ptr(), ctypes.POINTER(ctypes.c_uint8))
    
    lib.cron_pack_trits_b158(w_ptr, packed_ptr, ctypes.byref(scale_out), K, N)
    
    return packed_w, scale_out.value

def fused_rmsnorm_linear(x: torch.Tensor, gamma: torch.Tensor, weight: torch.Tensor, eps: float = 1e-5) -> torch.Tensor:
    """Single-pass Fused RMSNorm + Linear GEMM with zero intermediate DRAM writeback."""
    lib = get_native_lib()
    x = x.contiguous().float()
    gamma = gamma.contiguous().float()
    weight = weight.contiguous().float()
    
    orig_shape = x.shape
    d_in = orig_shape[-1]
    total_tokens = x.numel() // d_in
    d_out = weight.shape[0] if weight.dim() == 2 and weight.shape[1] == d_in else weight.shape[-1]
    
    out = torch.empty((*orig_shape[:-1], d_out), dtype=torch.float32)
    lib.cron_fused_rmsnorm_linear(
        ctypes.cast(x.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(gamma.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(weight.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(out.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        total_tokens,
        d_in,
        d_out,
        eps
    )
    return out

def fused_swiglu(x: torch.Tensor, w_gate: torch.Tensor, w_up: torch.Tensor) -> torch.Tensor:
    """In-register Fused Dual-Projection SwiGLU Gated Activation."""
    lib = get_native_lib()
    x = x.contiguous().float()
    w_gate = w_gate.contiguous().float()
    w_up = w_up.contiguous().float()
    
    orig_shape = x.shape
    d_in = orig_shape[-1]
    total_tokens = x.numel() // d_in
    d_hidden = w_gate.shape[0]
    
    out = torch.empty((*orig_shape[:-1], d_hidden), dtype=torch.float32)
    lib.cron_fused_swiglu(
        ctypes.cast(x.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(w_gate.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(w_up.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(out.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        total_tokens,
        d_in,
        d_hidden
    )
    return out

def flash_attention_2(q: torch.Tensor, k: torch.Tensor, v: torch.Tensor, scale: Optional[float] = None) -> torch.Tensor:
    """Online Softmax FlashAttention-2 Forward Pass in O(N) memory."""
    lib = get_native_lib()
    q = q.contiguous().float()
    k = k.contiguous().float()
    v = v.contiguous().float()
    
    batch, heads, seq, d = q.shape
    sm_scale = scale if scale is not None else (1.0 / (d ** 0.5))
    
    out = torch.empty_like(q)
    lib.cron_flash_attention_2(
        ctypes.cast(q.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(k.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(v.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        ctypes.cast(out.data_ptr(), ctypes.POINTER(ctypes.c_float)),
        batch,
        heads,
        seq,
        d,
        sm_scale
    )
    return out

if TORCH_AVAILABLE:
    class CronBitLinear(nn.Module):
        """
        Drop-in replacement for `torch.nn.Linear` accelerated by the CRON Native C23 Engine.
        Uses packed 1.58-bit ternary weights {-1, 0, +1} without floating multipliers.
        """
        def __init__(self, in_features: int, out_features: int, bias: bool = True):
            super().__init__()
            self.in_features = in_features
            self.out_features = out_features
            
            packed_bytes = (in_features * out_features + 3) // 4
            self.register_buffer("weight_packed", torch.zeros(packed_bytes, dtype=torch.uint8))
            self.register_buffer("scale", torch.tensor(1.0, dtype=torch.float32))
            
            if bias:
                self.bias = nn.Parameter(torch.zeros(out_features, dtype=torch.float32))
            else:
                self.register_parameter("bias", None)
                
            self._lib = get_native_lib()

        @classmethod
        def from_linear(cls, linear: nn.Linear) -> "CronBitLinear":
            """Converts an existing torch.nn.Linear layer to CronBitLinear."""
            mod = cls(linear.in_features, linear.out_features, bias=(linear.bias is not None))
            packed_w, scale = pack_ternary_weights(linear.weight.data)
            mod.weight_packed.copy_(packed_w)
            mod.scale.fill_(scale)
            if linear.bias is not None:
                mod.bias.data.copy_(linear.bias.data)
            return mod

        def forward(self, x: torch.Tensor) -> torch.Tensor:
            orig_shape = x.shape
            K = self.in_features
            N = self.out_features
            
            x_2d = x.view(-1, K).contiguous().float()
            M = x_2d.shape[0]
            
            y_2d = torch.empty((M, N), dtype=torch.float32, device=x.device)
            
            x_ptr = ctypes.cast(x_2d.data_ptr(), ctypes.POINTER(ctypes.c_float))
            w_ptr = ctypes.cast(self.weight_packed.data_ptr(), ctypes.POINTER(ctypes.c_uint8))
            y_ptr = ctypes.cast(y_2d.data_ptr(), ctypes.POINTER(ctypes.c_float))
            
            bias_ptr = None
            if self.bias is not None:
                b = self.bias.contiguous().float()
                bias_ptr = ctypes.cast(b.data_ptr(), ctypes.POINTER(ctypes.c_float))
                
            self._lib.cron_bitnet_gemm_f32(
                x_ptr,
                w_ptr,
                bias_ptr,
                y_ptr,
                M,
                K,
                N,
                float(self.scale.item())
            )
            
            out_shape = list(orig_shape[:-1]) + [N]
            return y_2d.view(*out_shape)

        def extra_repr(self) -> str:
            return f"in_features={self.in_features}, out_features={self.out_features}, scale={self.scale.item():.4f}, precision=b1.58"

    class CronRMSNorm(nn.Module):
        """
        Fast Vectorized Root Mean Square Normalization (RMSNorm)
        Accelerated by CRON AVX2/FMA Native C23 Engine.
        """
        def __init__(self, dim: int, eps: float = 1e-6):
            super().__init__()
            self.dim = dim
            self.eps = eps
            self.weight = nn.Parameter(torch.ones(dim, dtype=torch.float32))
            self._lib = get_native_lib()

        def forward(self, x: torch.Tensor) -> torch.Tensor:
            orig_shape = x.shape
            D = self.dim
            x_2d = x.view(-1, D).contiguous().float()
            M = x_2d.shape[0]

            y_2d = torch.empty((M, D), dtype=torch.float32, device=x.device)
            w_vec = self.weight.contiguous().float()

            x_ptr = ctypes.cast(x_2d.data_ptr(), ctypes.POINTER(ctypes.c_float))
            w_ptr = ctypes.cast(w_vec.data_ptr(), ctypes.POINTER(ctypes.c_float))
            y_ptr = ctypes.cast(y_2d.data_ptr(), ctypes.POINTER(ctypes.c_float))

            self._lib.cron_fast_rmsnorm_f32(x_ptr, w_ptr, y_ptr, M, D, float(self.eps))

            return y_2d.view(*orig_shape)

        def extra_repr(self) -> str:
            return f"dim={self.dim}, eps={self.eps}"

    class CronRingTapeAttention(nn.Module):
        """
        RingTape O(1) Streaming FlashAttention Kernel.
        Zero dynamic heap allocation, sliding-window circular buffer.
        """
        def __init__(self, dim: int, n_heads: int, capacity: int = 2048):
            super().__init__()
            self.dim = dim
            self.n_heads = n_heads
            self.head_dim = dim // n_heads
            self.capacity = capacity
            
            # Linear projections using CronBitLinear
            self.q_proj = CronBitLinear(dim, dim, bias=False)
            self.k_proj = CronBitLinear(dim, dim, bias=False)
            self.v_proj = CronBitLinear(dim, dim, bias=False)
            self.out_proj = CronBitLinear(dim, dim, bias=False)

            # Circular KV tape buffer [B, H, Capacity, D_head]
            self.register_buffer("k_tape", torch.zeros((1, n_heads, capacity, self.head_dim), dtype=torch.float32))
            self.register_buffer("v_tape", torch.zeros((1, n_heads, capacity, self.head_dim), dtype=torch.float32))
            self.register_buffer("tape_head", torch.zeros(1, dtype=torch.long))

            self._lib = get_native_lib()

        def reset_tape(self, batch_size: int = 1):
            self.k_tape = torch.zeros((batch_size, self.n_heads, self.capacity, self.head_dim), dtype=torch.float32)
            self.v_tape = torch.zeros((batch_size, self.n_heads, self.capacity, self.head_dim), dtype=torch.float32)
            self.tape_head.zero_()

        def forward(self, x: torch.Tensor) -> torch.Tensor:
            B, S, D = x.shape
            H = self.n_heads
            HD = self.head_dim

            # Project Q, K, V
            q = self.q_proj(x).view(B, S, H, HD).transpose(1, 2).contiguous() # [B, H, S, HD]
            k = self.k_proj(x).view(B, S, H, HD).transpose(1, 2).contiguous() # [B, H, S, HD]
            v = self.v_proj(x).view(B, S, H, HD).transpose(1, 2).contiguous() # [B, H, S, HD]

            # If batch size changed or tape uninitialized, resize
            if self.k_tape.shape[0] != B:
                self.reset_tape(B)

            # Write incoming K, V into circular tape
            cur_head = int(self.tape_head.item())
            for s in range(S):
                pos = (cur_head + s) % self.capacity
                self.k_tape[:, :, pos, :] = k[:, :, s, :]
                self.v_tape[:, :, pos, :] = v[:, :, s, :]
            new_head = (cur_head + S - 1) % self.capacity
            self.tape_head.fill_((cur_head + S) % self.capacity)

            out = torch.empty((B, H, S, HD), dtype=torch.float32, device=x.device)

            q_ptr = ctypes.cast(q.contiguous().data_ptr(), ctypes.POINTER(ctypes.c_float))
            kt_ptr = ctypes.cast(self.k_tape.contiguous().data_ptr(), ctypes.POINTER(ctypes.c_float))
            vt_ptr = ctypes.cast(self.v_tape.contiguous().data_ptr(), ctypes.POINTER(ctypes.c_float))
            out_ptr = ctypes.cast(out.data_ptr(), ctypes.POINTER(ctypes.c_float))

            self._lib.cron_ringtape_attention_f32(
                q_ptr,
                kt_ptr,
                vt_ptr,
                out_ptr,
                B, H, S, HD,
                self.capacity,
                new_head
            )

            # Reshape back to [B, S, D]
            out_proj_in = out.transpose(1, 2).contiguous().view(B, S, D)
            return self.out_proj(out_proj_in)

    class CronTransformerBlock(nn.Module):
        """
        Unified End-to-End LLM Transformer Block.
        Combines:
          1. CronRMSNorm (AVX2 FMA)
          2. CronRingTapeAttention (AVX2 Streaming FlashAttention)
          3. CronRMSNorm (AVX2 FMA)
          4. BitNet SwiGLU MLP (Multiplier-Free AVX2)
        """
        def __init__(self, dim: int, n_heads: int, mlp_dim: int, capacity: int = 2048):
            super().__init__()
            self.dim = dim
            self.n_heads = n_heads
            self.mlp_dim = mlp_dim

            self.attn_norm = CronRMSNorm(dim)
            self.attn = CronRingTapeAttention(dim, n_heads, capacity)

            self.mlp_norm = CronRMSNorm(dim)
            self.up_proj = CronBitLinear(dim, mlp_dim, bias=False)
            self.gate_proj = CronBitLinear(dim, mlp_dim, bias=False)
            self.down_proj = CronBitLinear(mlp_dim, dim, bias=False)
            self.act = nn.SiLU()

        def forward(self, x: torch.Tensor) -> torch.Tensor:
            # Pre-LN Self-Attention with Residual
            h = x + self.attn(self.attn_norm(x))

            # Pre-LN SwiGLU MLP with Residual
            norm_h = self.mlp_norm(h)
            mlp_out = self.down_proj(self.act(self.gate_proj(norm_h)) * self.up_proj(norm_h))
            return h + mlp_out

    def replace_torch_layers(model: nn.Module) -> nn.Module:
        """
        Recursively traverses any PyTorch model and replaces all `torch.nn.Linear` layers
        with CRON multiplier-free `CronBitLinear` layers in place.
        """
        for name, child in model.named_children():
            if isinstance(child, nn.Linear):
                setattr(model, name, CronBitLinear.from_linear(child))
            else:
                replace_torch_layers(child)
        return model
else:
    class CronBitLinear:
        pass
    class CronRMSNorm:
        pass
    class CronRingTapeAttention:
        pass
    class CronTransformerBlock:
        pass
    def replace_torch_layers(model):
        pass
