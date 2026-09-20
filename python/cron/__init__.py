"""
CRON Programming Language — Python SDK
Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
"""

from .compiler import (
    compile_source,
    compile_c23,
    check_diagnostics,
    find_cron_executable,
    run_esoteric_sim,
    synthesize_esoteric_coprocessor,
    tile_gemm_systolic,
    analyze_optical_budget,
    synthesize_optical_verilog,
    run_comparison_benchmark,
    generate_benchmark_whitepaper,
)
from .torch_bridge import (
    CronBitLinear,
    CronRMSNorm,
    CronRingTapeAttention,
    CronTransformerBlock,
    replace_torch_layers,
    pack_ternary_weights,
    find_native_library,
    get_native_version,
    fused_rmsnorm_linear,
    fused_swiglu,
    flash_attention_2,
)
from .simulator import Simulator
from .photonic import PhotonicBridge
from .kernel import CronKernel, install_kernel_spec

__version__ = "1.0.0"
__all__ = [
    "compile_source",
    "compile_c23",
    "check_diagnostics",
    "find_cron_executable",
    "run_esoteric_sim",
    "synthesize_esoteric_coprocessor",
    "tile_gemm_systolic",
    "analyze_optical_budget",
    "synthesize_optical_verilog",
    "run_comparison_benchmark",
    "generate_benchmark_whitepaper",
    "CronBitLinear",
    "CronRMSNorm",
    "CronRingTapeAttention",
    "CronTransformerBlock",
    "replace_torch_layers",
    "pack_ternary_weights",
    "find_native_library",
    "get_native_version",
    "fused_rmsnorm_linear",
    "fused_swiglu",
    "flash_attention_2",
    "Simulator",
    "PhotonicBridge",
    "CronKernel",
    "install_kernel_spec",
]

