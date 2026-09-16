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

    # 2. Check current workspace targets
    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    candidates = [
        os.path.join(base_dir, "target", "release", "cron.exe"),
        os.path.join(base_dir, "target", "debug", "cron.exe"),
        os.path.join(base_dir, "target", "release", "cron"),
        os.path.join(base_dir, "target", "debug", "cron"),
    ]
    for c in candidates:
        if os.path.exists(c):
            return c

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
