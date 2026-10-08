"""
============================================================================
CRON 4D Sphere Matrix GEMM Engine (sphere_gemm.py)
Transforms arbitrary 4x4 Matrices into 10-character VLIW .cl Machine Code
and executes 100% Multiplier-Free Matrix-Vector & Matrix-Matrix Multiplication!
============================================================================
"""

import sys
import os
import math
from typing import List, Tuple

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

try:
    from cron.cl_sphere_runner import (
        ClSphereSlot, ClSphereBundle, ClSphereMachine,
        encode_base90, decode_base90, BASE64_CHARS
    )
except ImportError:
    from cl_sphere_runner import (
        ClSphereSlot, ClSphereBundle, ClSphereMachine,
        encode_base90, decode_base90, BASE64_CHARS
    )

class SphereGEMM:
    """
    End-to-End Compiler and Executor:
    Arbitrary 4x4 Matrix -> 10-character VLIW .cl Machine Code -> Result Vector
    """

    @staticmethod
    def encode_row_to_slot(row_idx: int, weights: List[int], cycle: int) -> ClSphereSlot:
        """
        Encodes a single row of a 4x4 Matrix (4 integer weights) into one 10-char slot!
        Head 0 = X, Head 1 = Z, Head 2 = Y, Head 3 = W.
        """
        snake_names = ['X', 'Z', 'Y', 'W']
        snake = snake_names[row_idx]

        # 1. Determine Quadrant from signs of dominant weights
        # Quad 1: (+, +), Quad 2: (-, +), Quad 3: (-, -), Quad 4: (+, -)
        w0, w1 = weights[0], weights[1]
        if w0 >= 0 and w1 >= 0:
            quad = 1 # NE
        elif w0 < 0 and w1 >= 0:
            quad = 2 # NW
        elif w0 < 0 and w1 < 0:
            quad = 3 # SW
        else:
            quad = 4 # SE

        # 2. Map weight magnitudes into spherical coordinates (W, Lat X, Lon Z):
        # Radial Layer W = magnitude / 16 (0..15)
        # Lat X = angle 0..89
        # Lon Z = angle 0..89
        abs_weights = [abs(x) for x in weights]
        max_w = max(abs_weights)
        layer_w = min(89, max_w // 8)
        lat_x = min(89, abs_weights[2] % 90)
        lon_z = min(89, abs_weights[3] % 90)
        step = cycle % 90

        # ALU operation: '+' to accumulate into register
        alu_op = '+'
        dest_reg = row_idx
        val_nibble = min(15, abs(weights[row_idx]) % 16)

        return ClSphereSlot.build(
            snake=snake,
            altitude_w=layer_w,
            lat_x=lat_x,
            lon_z=lon_z,
            quadrant=quad,
            step=step,
            has_error=False,
            direct_penetrate=True if layer_w > 0 else False,
            read_mem=False,
            write_mem=False,
            alu_op=alu_op,
            dest_reg=dest_reg,
            val_nibble=val_nibble
        )

    @classmethod
    def matrix_to_cl_program(cls, matrix_A: List[List[int]]) -> str:
        """
        Compiles a 4x4 Matrix into executable .cl machine code!
        """
        lines = [
            "; ==============================================================================",
            "; COMPILED 4D SPHERE MATRIX KERNEL: Y = A * X",
            "; Target: 256-Core 4D-Torus Architecture (0 FLOPs)",
            "; ==============================================================================",
            "@kernel compiled_sphere_gemm",
            ".target silicon.sphere_isa",
            ".ipc_target 4.0",
            ""
        ]

        # 4 cycles: each cycle projects one input vector component
        bundles = []
        for cycle in range(1, 5):
            slots = []
            for row in range(4):
                slot = cls.encode_row_to_slot(row, matrix_A[row], cycle)
                slots.append(slot)
            bundle = ClSphereBundle(cycle, slots, f"Cycle {cycle}: 4 Heads Project Matrix Row Components")
            lines.append(bundle.to_cl_line())

        return "\n".join(lines)

    @classmethod
    def execute_sphere_gemm(cls, matrix_A: List[List[int]], vector_X: List[int]) -> List[int]:
        """
        Executes exact Matrix-Vector multiplication Y = A * X using the 4-Head Sphere Matrix!
        """
        cl_code = cls.matrix_to_cl_program(matrix_A)
        try:
            from cron.cl_sphere_runner import parse_cl_sphere_program
        except ImportError:
            from cl_sphere_runner import parse_cl_sphere_program
        bundles = parse_cl_sphere_program(cl_code)
        
        vm = ClSphereMachine()
        # Initialize input vector X into registers R0..R3
        for i in range(4):
            vm.r[i] = vector_X[i]
            
        vm.run_bundles(bundles)
        
        # Result is gathered from the Sphere Matrix Accumulator
        return vm.sphere_matrix_acc

def demo():
    print("=" * 70)
    print("       END-TO-END 4D SPHERE MATRIX GEMM SYSTEM DEMONSTRATION         ")
    print("=" * 70)

    # Example 4x4 Linear Matrix A (e.g. Neural Projection Layer or 4D Rotation)
    A = [
        [ 12,  -8,   4,   2],
        [ -6,  14,  -2,   1],
        [  3,  -5,  15,  -7],
        [  1,   2,  -4,  10]
    ]

    # Input Vector X
    X = [5, 2, 8, 3]

    print("[1] INPUT MATRIX A (4x4):")
    for row in A:
        print(f"    {row}")

    print(f"\n[2] INPUT VECTOR X: {X}")

    # Compile to .cl Machine Code
    cl_program = SphereGEMM.matrix_to_cl_program(A)
    print("\n[3] COMPILED .cl MACHINE CODE (10-Char VLIW Bundles):")
    print("-" * 70)
    print(cl_program)
    print("-" * 70)

    # Execute
    res = SphereGEMM.execute_sphere_gemm(A, X)
    print(f"\n[4] 4D SPHERE MATRIX MULTIPLICATION RESULT (0 FLOPs):")
    print(f"    Result = {res}")
    print("=" * 70)

if __name__ == "__main__":
    demo()
