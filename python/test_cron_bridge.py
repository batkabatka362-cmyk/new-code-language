#!/usr/bin/env python3
"""
Test script for cron-py Python / PyTorch bridge
"""

import sys
import os

# Add parent directory to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

try:
    from python.cron import LivingMind, execute_cl
    print("[OK] Successfully imported cron-py module")
    
    mind = LivingMind()
    chems = mind.get_chemicals()
    print(f"[OK] Initial Neuromodulators: Dopamine={chems['dopamine']:.2f}, Serotonin={chems['serotonin']:.2f}")
    
    res = mind.process("Test sensory input from PyTorch tensor stream")
    print(f"[OK] Mind Response: {res}")
    
    regs = execute_cl("B0000: _NO00$000> _NO00$000> _NO00$000> _NO00$000>")
    print(f"[OK] .cl Register Output: R0=0x{regs[0]:04X}")
    print("\n[ALL PYTHON CRON TESTS PASSED 100%]")
except Exception as e:
    print(f"Error during python test: {e}")
    sys.exit(1)
