"""
CRON Programming Language — Python SDK
Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
"""

from .compiler import compile_source, compile_c23, check_diagnostics, find_cron_executable
from .simulator import Simulator
from .photonic import PhotonicBridge
from .kernel import CronKernel, install_kernel_spec

__version__ = "1.0.0"
__all__ = [
    "compile_source",
    "compile_c23",
    "check_diagnostics",
    "find_cron_executable",
    "Simulator",
    "PhotonicBridge",
    "CronKernel",
    "install_kernel_spec",
]
