# ============================================================================
# CRON vs PyTorch Full Transformer Block Empirical Benchmark
# Compares standard PyTorch FP32 Transformer Block vs CRON Native C23 Block
# Target: AMD Zen 4 Host CPU (AVX2 + OpenMP)
# (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
# ============================================================================

import os
import sys
import time
import torch
import torch.nn as nn

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))
import python.cron as cron

class PyTorchTransformerBlock(nn.Module):
    """Reference PyTorch Transformer Block (Llama architecture style)."""
    def __init__(self, dim: int, n_heads: int, mlp_dim: int):
        super().__init__()
        self.attn_norm = nn.LayerNorm(dim, eps=1e-6)
        self.attn = nn.MultiheadAttention(dim, n_heads, batch_first=True, bias=False)
        self.mlp_norm = nn.LayerNorm(dim, eps=1e-6)
        self.gate_proj = nn.Linear(dim, mlp_dim, bias=False)
        self.up_proj = nn.Linear(dim, mlp_dim, bias=False)
        self.down_proj = nn.Linear(mlp_dim, dim, bias=False)
        self.act = nn.SiLU()

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        # Pre-LN Self-Attention
        norm_x = self.attn_norm(x)
        attn_out, _ = self.attn(norm_x, norm_x, norm_x, need_weights=False)
        h = x + attn_out

        # Pre-LN SwiGLU MLP
        norm_h = self.mlp_norm(h)
        mlp_out = self.down_proj(self.act(self.gate_proj(norm_h)) * self.up_proj(norm_h))
        return h + mlp_out

def calculate_model_memory_kb(model: nn.Module) -> float:
    total_bytes = 0
    for p in model.parameters():
        total_bytes += p.numel() * p.element_size()
    for b in model.buffers():
        total_bytes += b.numel() * b.element_size()
    return total_bytes / 1024.0

def benchmark_scenario(name: str, B: int, S: int, D: int, H: int, MLP: int, iterations: int = 50):
    print("=" * 82)
    print(f" WORKLOAD: {name} (B={B}, S={S}, D={D}, H={H}, MLP={MLP})")
    print("=" * 82)

    torch_block = PyTorchTransformerBlock(D, H, MLP).eval()
    cron_block = cron.CronTransformerBlock(D, H, MLP, capacity=2048).eval()

    torch_mem = calculate_model_memory_kb(torch_block)
    cron_mem = calculate_model_memory_kb(cron_block)

    x = torch.randn(B, S, D, dtype=torch.float32)

    # Warmup
    print("  Warming up caches and thread pools...")
    for _ in range(10):
        _ = torch_block(x)
        _ = cron_block(x)

    # Measure PyTorch
    print(f"  Measuring PyTorch Native FP32 ({iterations} runs)...")
    torch_times = []
    for _ in range(iterations):
        t0 = time.perf_counter()
        _ = torch_block(x)
        t1 = time.perf_counter()
        torch_times.append((t1 - t0) * 1e6) # microseconds

    # Measure CRON
    print(f"  Measuring CRON Native C23 ({iterations} runs)...")
    cron_times = []
    for _ in range(iterations):
        t0 = time.perf_counter()
        _ = cron_block(x)
        t1 = time.perf_counter()
        cron_times.append((t1 - t0) * 1e6) # microseconds

    mean_torch_us = sum(torch_times) / len(torch_times)
    mean_cron_us = sum(cron_times) / len(cron_times)
    speedup = mean_torch_us / max(mean_cron_us, 1e-6)
    mem_compression = torch_mem / max(cron_mem, 1e-6)

    print("-" * 82)
    print(f"  {'Metric':<36} | {'PyTorch FP32':<18} | {'CRON C23 Native':<18}")
    print("-" * 82)
    print(f"  {'Mean Wall-Clock Latency':<36} | {mean_torch_us:>14.1f} us | {mean_cron_us:>14.1f} us")
    print(f"  {'Parameter & Buffer Memory':<36} | {torch_mem:>14.1f} KB | {cron_mem:>14.1f} KB")
    print(f"  {'Speedup vs PyTorch':<36} | {'1.00x':>18} | {speedup:>17.2f}x")
    print(f"  {'Memory Footprint Reduction':<36} | {'1.00x':>18} | {mem_compression:>17.2f}x")
    print("-" * 82)

    if speedup > 1.0:
        print(f"  >>> RESULT: CRON Native is {speedup:.2f}x FASTER than PyTorch!")
    else:
        print(f"  >>> RESULT: PyTorch is {1.0 / speedup:.2f}x faster.")
    print()

def main():
    print("=" * 82)
    print(" CRON vs PyTorch FULL TRANSFORMER BLOCK EMPIRICAL BENCHMARK")
    print(" BitNet b1.58 + RingTape Attention + Fast RMSNorm vs PyTorch 2.13.0 (Zen 4)")
    print("=" * 82)

    ver = cron.get_native_version()
    print(f"[CRON Accelerator Loaded] Version: {ver}")
    print(f"[PyTorch Version] {torch.__version__} (Threads: {torch.get_num_threads()})")
    print()

    # Scenario 1: Autoregressive Single-Token Decode (B=1, S=1, D=2048, H=16, MLP=5632)
    benchmark_scenario(
        name="Autoregressive Token Decode (Single-Step Inference)",
        B=1, S=1, D=2048, H=16, MLP=5632,
        iterations=50
    )

    # Scenario 2: Prefill Small Batch (B=4, S=16, D=2048, H=16, MLP=5632)
    benchmark_scenario(
        name="Batched Sequence Prefill",
        B=4, S=16, D=2048, H=16, MLP=5632,
        iterations=30
    )

if __name__ == "__main__":
    main()
