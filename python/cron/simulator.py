"""
CRON 4D-Torus Simulator Interface for Python
Allows running .cr or .cl on the 256-Core Neuromorphic Processor and returning telemetry dicts.
"""

import subprocess
import tempfile
import os
import re
from typing import Dict, Any
from .compiler import find_cron_executable

class Simulator:
    """Interface to the 256-Core 4D-Torus Hardware Simulator."""

    def __init__(self, cron_exe: str = None):
        self.cron_exe = cron_exe or find_cron_executable()

    def run(self, source_code: str) -> Dict[str, Any]:
        """Runs a .cr program and parses hardware telemetry output."""
        with tempfile.NamedTemporaryFile(suffix=".cr", mode="w", delete=False, encoding="utf-8") as f_in:
            f_in.write(source_code)
            in_path = f_in.name

        try:
            cmd = [self.cron_exe, "run", in_path]
            res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
            if res.returncode != 0:
                raise RuntimeError(f"Simulation execution failed:\n{res.stderr or res.stdout}")
            
            return self._parse_telemetry(res.stdout)
        finally:
            if os.path.exists(in_path):
                os.remove(in_path)

    def run_cl(self, cl_code: str) -> Dict[str, Any]:
        """Directly runs .cl machine bytecode and parses hardware telemetry."""
        with tempfile.NamedTemporaryFile(suffix=".cl", mode="w", delete=False, encoding="utf-8") as f_in:
            f_in.write(cl_code)
            in_path = f_in.name

        try:
            cmd = [self.cron_exe, "sim", in_path]
            res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
            if res.returncode != 0:
                raise RuntimeError(f"Simulation execution failed:\n{res.stderr or res.stdout}")
            
            return self._parse_telemetry(res.stdout)
        finally:
            if os.path.exists(in_path):
                os.remove(in_path)

    def _parse_telemetry(self, stdout: str) -> Dict[str, Any]:
        telemetry = {
            "total_cycles": 0,
            "active_cores": 256,
            "optical_gemm_ops": 0,
            "reversible_gate_ops": 0,
            "stdp_synapse_updates": 0,
            "mesh_packets_routed": 0,
            "peak_temperature_c": 0,
            "dram_bandwidth_saved_mb": 0.0,
            "raw_output": stdout,
        }

        for line in stdout.splitlines():
            line = line.strip()
            if "Total Execution Cycles:" in line:
                m = re.search(r"(\d+)\s+cycles", line)
                if m: telemetry["total_cycles"] = int(m.group(1))
            elif "Photonic MZI Optical Ops:" in line:
                m = re.search(r"(\d+)\s+ops", line)
                if m: telemetry["optical_gemm_ops"] = int(m.group(1))
            elif "Reversible Gate Ops" in line:
                m = re.search(r"(\d+)\s+ops", line)
                if m: telemetry["reversible_gate_ops"] = int(m.group(1))
            elif "STDP Synapse Adaptations:" in line:
                m = re.search(r"(\d+)\s+updates", line)
                if m: telemetry["stdp_synapse_updates"] = int(m.group(1))
            elif "4D NoC Mesh Packets Routed:" in line:
                m = re.search(r"(\d+)\s+packets", line)
                if m: telemetry["mesh_packets_routed"] = int(m.group(1))
            elif "Peak Core Temperature:" in line:
                m = re.search(r"(\d+)\s+°C", line)
                if m: telemetry["peak_temperature_c"] = int(m.group(1))
            elif "DRAM Bandwidth Saved:" in line:
                m = re.search(r"([\d\.]+)\s+MB", line)
                if m: telemetry["dram_bandwidth_saved_mb"] = float(m.group(1))

        return telemetry
