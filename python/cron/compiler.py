"""
CRON Python Compiler Interface
Provides seamless compilation of .cr (high-level) and verification of .cl (low-level)
directly from Python scripts, Jupyter Notebooks, and PyTorch pipelines.
"""

import subprocess
import tempfile
import os
import shutil
from typing import Dict, Any, List, Optional

def find_cron_executable() -> str:
    """Finds the 'cron' CLI binary either on PATH or in local target/release or target/debug."""
    # 1. Check PATH
    path_bin = shutil.which("cron")
    if path_bin:
        return path_bin

    # 2. Check current workspace targets (pick the newest binary by mtime)
    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    candidates = [
        os.path.join(base_dir, "target", "debug", "cron.exe"),
        os.path.join(base_dir, "target", "release", "cron.exe"),
        os.path.join(base_dir, "target", "debug", "cron"),
        os.path.join(base_dir, "target", "release", "cron"),
    ]
    existing = [c for c in candidates if os.path.exists(c)]
    if existing:
        existing.sort(key=lambda p: os.path.getmtime(p), reverse=True)
        return existing[0]

    return "cron"

def compile_source(source_code: str) -> str:
    """Compiles .cr source code into machine-native .cl VLIW bytecode."""
    cron_exe = find_cron_executable()
    with tempfile.NamedTemporaryFile(suffix=".cr", mode="w", delete=False, encoding="utf-8") as f_in:
        f_in.write(source_code)
        in_path = f_in.name

    out_cl = in_path.replace(".cr", ".cl")
    try:
        cmd = [cron_exe, "build", in_path, "-o", out_cl]
        res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
        if res.returncode != 0:
            raise RuntimeError(f"CRON compilation error:\n{res.stderr or res.stdout}")
        
        with open(out_cl, "r", encoding="utf-8") as f_out:
            return f_out.read()
    finally:
        if os.path.exists(in_path):
            os.remove(in_path)
        if os.path.exists(out_cl):
            os.remove(out_cl)

def compile_c23(source_code: str) -> str:
    """Transpiles .cr source code into high-performance C23 native source code."""
    cron_exe = find_cron_executable()
    with tempfile.NamedTemporaryFile(suffix=".cr", mode="w", delete=False, encoding="utf-8") as f_in:
        f_in.write(source_code)
        in_path = f_in.name

    out_c = in_path.replace(".cr", ".c")
    try:
        cmd = [cron_exe, "c23", in_path, "-o", out_c]
        res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
        if res.returncode != 0:
            raise RuntimeError(f"CRON C23 transpilation error:\n{res.stderr or res.stdout}")
        
        with open(out_c, "r", encoding="utf-8") as f_out:
            return f_out.read()
    finally:
        if os.path.exists(in_path):
            os.remove(in_path)
        if os.path.exists(out_c):
            os.remove(out_c)

def check_diagnostics(source_code: str) -> List[str]:
    """Verifies linear type safety, region boundaries, and grammar syntax."""
    cron_exe = find_cron_executable()
    with tempfile.NamedTemporaryFile(suffix=".cr", mode="w", delete=False, encoding="utf-8") as f_in:
        f_in.write(source_code)
        in_path = f_in.name

    try:
        cmd = [cron_exe, "check", in_path]
        res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
        if res.returncode != 0:
            return [res.stderr.strip() or res.stdout.strip()]
        return []
    finally:
        if os.path.exists(in_path):
            os.remove(in_path)

def run_esoteric_sim(mode: str = "demo") -> Dict[str, Any]:
    """Runs the Esolang-inspired AI Silicon Coprocessor simulation (Brainfuck, Malbolge, Befunge, Prolog)."""
    cron_exe = find_cron_executable()
    cmd = [cron_exe, "cl-esoteric", mode, "--json"]
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
def _extract_json(text: str) -> Dict[str, Any]:
    import json
    raw = text.strip()
    start = raw.find("{")
    end = raw.rfind("}")
    if start != -1 and end != -1:
        raw = raw[start:end+1]
    return json.loads(raw)

def run_esoteric_sim(program: str = "demo") -> Dict[str, Any]:
    """Runs simulation on the Esoteric Coprocessor and returns telemetry."""
    cron_exe = find_cron_executable()
    cmd = [cron_exe, "cl-esoteric", "sim", "--prog", program, "--json"]
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if res.returncode != 0:
        raise RuntimeError(f"CRON cl-esoteric error:\n{res.stderr or res.stdout}")
    return _extract_json(res.stdout)

def synthesize_esoteric_coprocessor() -> str:
    """Synthesizes Verilog-2001 RTL for the Esoteric AI Coprocessor."""
    cron_exe = find_cron_executable()
    cmd = [cron_exe, "cl-esoteric", "synth"]
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if res.returncode != 0:
        raise RuntimeError(f"CRON cl-esoteric synth error:\n{res.stderr or res.stdout}")
    return res.stdout

def tile_gemm_systolic(m: int = 64, n: int = 64, k: int = 64, unroll: int = 4, subbyte: bool = False, emit_cr: bool = False) -> Dict[str, Any]:
    """Derives polyhedral loop tiling with Befunge 2D systolic wavefront dataflow."""
    cron_exe = find_cron_executable()
    cmd = [
        cron_exe, "cl-tile", "gemm",
        "--m", str(m),
        "--n", str(n),
        "--k", str(k),
        "--unroll", str(unroll),
        "--systolic",
        "--json"
    ]
    if subbyte:
        cmd.append("--subbyte")
    if emit_cr:
        cmd.append("--cr")
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if res.returncode != 0:
        raise RuntimeError(f"CRON cl-tile error:\n{res.stderr or res.stdout}")
    return _extract_json(res.stdout)

def analyze_optical_budget(file_or_code: str, mesh_dim: int = 16, wdm_channels: int = 8, topology: str = "clemens") -> Dict[str, Any]:
    """Analyzes photonic insertion loss, WDM lambda allocation, and laser power budget."""
    cron_exe = find_cron_executable()
    is_file = os.path.exists(file_or_code)
    target_path = file_or_code
    temp_f = None
    if not is_file:
        suffix = ".cl" if ("B0" in file_or_code or "_" in file_or_code) else ".cr"
        temp_f = tempfile.NamedTemporaryFile(suffix=suffix, mode="w", delete=False, encoding="utf-8")
        temp_f.write(file_or_code)
        temp_f.close()
        target_path = temp_f.name

    try:
        cmd = [
            cron_exe, "cl-optic", target_path,
            "--mesh", str(mesh_dim),
            "--lambda", str(wdm_channels),
            "--topology", topology,
            "--json"
        ]
        res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
        if res.returncode != 0:
            raise RuntimeError(f"CRON cl-optic error:\n{res.stderr or res.stdout}")
        return _extract_json(res.stdout)
    finally:
        if temp_f and os.path.exists(temp_f.name):
            try:
                os.remove(temp_f.name)
            except OSError:
                pass

def synthesize_optical_verilog(file_or_code: str, mesh_dim: int = 16, wdm_channels: int = 8) -> str:
    """Synthesizes Verilog RTL for closed-loop laser regulation controller."""
    cron_exe = find_cron_executable()
    is_file = os.path.exists(file_or_code)
    target_path = file_or_code
    temp_f = None
    if not is_file:
        suffix = ".cl" if ("B0" in file_or_code or "_" in file_or_code) else ".cr"
        temp_f = tempfile.NamedTemporaryFile(suffix=suffix, mode="w", delete=False, encoding="utf-8")
        temp_f.write(file_or_code)
        temp_f.close()
        target_path = temp_f.name

    try:
        cmd = [
            cron_exe, "cl-optic", target_path,
            "--mesh", str(mesh_dim),
            "--lambda", str(wdm_channels),
            "--synth-verilog"
        ]
        res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
        if res.returncode != 0:
            raise RuntimeError(f"CRON cl-optic synth error:\n{res.stderr or res.stdout}")
        return res.stdout
    finally:
        if temp_f and os.path.exists(temp_f.name):
            try:
                os.remove(temp_f.name)
            except OSError:
                pass

def run_comparison_benchmark(workload: str = "all", baseline: str = "all", json_output: bool = True) -> Any:
    """Runs the competitive benchmark validation engine comparing CRON against PyTorch/CUDA and Mojo."""
    cron_exe = find_cron_executable()
    cmd = [cron_exe, "cl-compare", workload, "--baseline", baseline]
    if json_output:
        cmd.append("--json")
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if res.returncode != 0:
        raise RuntimeError(f"CRON cl-compare error:\n{res.stderr or res.stdout}")
    if json_output:
        return _extract_json(res.stdout)
    return res.stdout

def generate_benchmark_whitepaper(output_path: Optional[str] = None) -> str:
    """Generates the authoritative technical whitepaper in Markdown format."""
    cron_exe = find_cron_executable()
    cmd = [cron_exe, "cl-compare", "all", "--whitepaper"]
    if output_path:
        cmd.extend(["-o", output_path])
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if res.returncode != 0:
        raise RuntimeError(f"CRON whitepaper generation error:\n{res.stderr or res.stdout}")
    if output_path and os.path.exists(output_path):
        with open(output_path, "r", encoding="utf-8") as f:
            return f.read()
    return res.stdout


