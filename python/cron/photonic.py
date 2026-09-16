"""
CRON Photonic Hardware Bridge Interface
Bridges CRON wave_t (amplitudes, phases) with NumPy arrays and physical Silicon Photonic WDM chip.
Integrates directly with Desktop/light chip/cron_photonic_bridge.py.
"""

import sys
import os
from typing import List, Dict, Any, Tuple

# Add Desktop/light chip to sys.path if present
LIGHT_CHIP_DIR = os.path.abspath(os.path.join(os.environ.get("USERPROFILE", "C:\\Users\\local_ybjuj27"), "Desktop", "light chip"))
if os.path.exists(LIGHT_CHIP_DIR) and LIGHT_CHIP_DIR not in sys.path:
    sys.path.insert(0, LIGHT_CHIP_DIR)

class PhotonicBridge:
    """Co-simulation bridge between CRON Optical Tensor ops and Silicon Photonic Hardware."""

    def __init__(self):
        self._native_bridge = None
        try:
            from cron_photonic_bridge import CronPhotonicBridge
            self._native_bridge = CronPhotonicBridge()
            self.mode = "hardware_coprocessor"
        except Exception as e:
            self.mode = "pure_python_emulation"

    def execute_optical_gemm(
        self,
        amplitudes: List[int],
        phases: List[int],
        transformation: str = "attention"
    ) -> Dict[str, Any]:
        """Executes an optical GEMM operation through the WDM photonic mesh."""
        if self._native_bridge is not None:
            raw_res = self._native_bridge.execute_cron_optical_gemm(
                query_amps=amplitudes,
                query_phases=phases,
                transformation=transformation
            )
            raw_res["mode"] = self.mode
            raw_res["output_amps"] = raw_res.get("cron_output_vec4_i8", [])
            return raw_res
        
        # Software fallback emulation
        import math
        out_amps = []
        out_phases = []
        for a, p in zip(amplitudes, phases):
            rad = (p / 128.0) * math.pi
            trans = math.cos(rad * 0.5) ** 2
            out_amps.append(int(a * trans))
            out_phases.append((p + 32) % 256)

        return {
            "output_amps": out_amps,
            "output_phases": out_phases,
            "energy_pj": 0.05 * len(amplitudes),
            "mode": self.mode
        }
