# ============================================================================
# Clean Unit Test: 4D Sphere Matrix Engine
# ============================================================================

import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from python.cron.hypersphere_matrix import (
    SphereMatrix4D, LayerTransitionMode
)

def run_tests():
    print("=" * 70)
    print("  VERIFYING CORRECTED 4D SPHERE MATRIX ARCHITECTURE")
    print("=" * 70)

    # 1. 4 Heads starting from origin 0
    sm = SphereMatrix4D(transition_mode=LayerTransitionMode.DIRECT_STEP)
    print("\n[1] 4 Heads from Center 0:")
    for name, head in sm.heads.items():
        print(f"  Head {name}: W={head.w}, X={head.x}, Z={head.z}")

    # 2. Coordinate code parsing: e.g. 5667 -> X=56, Z=67
    print("\n[2] Testing Coordinate Code Parsing (e.g., '5667'):")
    x, z = sm.parse_coordinate_code("5667")
    print(f"  Input: '5667' -> Parsed: X={x} (top to half), Z={z} (half to bottom)")
    assert x == 56 and z == 67, "Parsing failed!"
    print("  -> SUCCESS: Coordinate parsing verified.")

    # 3. Curved Needle Collision Verification
    print("\n[3] Testing 'Curved Needle' Non-Collision Theorem:")
    # Place Head X and Head Y on identical surface coordinates X=56, Z=67
    sm.heads['X'].x = 56
    sm.heads['X'].z = 67
    sm.heads['X'].w = 10

    sm.heads['Y'].x = 56
    sm.heads['Y'].z = 67
    sm.heads['Y'].w = 25  # Different radial layer W

    collisions = sm.verify_curved_needle()
    print(f"  Collisions found: {len(collisions)}")
    assert len(collisions) == 0, "Curved needle violated!"
    print("  -> SUCCESS: 0.00% Collision. Differing W layers prevent 4D collision.")

    # 4. Error Dynamics on 0 State (Dead stop vs Phase-Lock)
    print("\n[4] Testing Error Dynamics (4D Line Unfolding & Volume Distribution):")
    # Case A: Dead stop without phase lock -> triggers errors!
    sm_err = SphereMatrix4D()
    sm_err.step_head('X', 0, 0, 0, use_phase_lock=False) # X stops dead
    sm_err.step_head('Y', 1, 2, 3)                       # Y moves
    sm_err.step_head('Z', 1, 2, 3)                       # Z moves
    sm_err.step_head('W', 1, 2, 3)                       # W moves

    err_report = sm_err.check_unfolding_and_volume_errors()
    print("  Without Phase-Lock (Dead stop at 0):")
    print(f"    Line unfolding error: {err_report['line_unfolding_error']}")
    print(f"    Volume distribution error: {err_report['volume_distribution_error']}")
    print(f"    Volume shares: {err_report['volume_shares']}")
    assert err_report['line_unfolding_error'] == True, "Expected unfolding error!"
    assert err_report['volume_distribution_error'] == True, "Expected volume error!"
    print("    -> Confirmed: Line unfolding & Volume deficit errors occurred!")

    # Case B: With Phase-Lock -> Zero errors!
    sm_ok = SphereMatrix4D()
    sm_ok.step_head('X', 0, 0, 0, use_phase_lock=True)  # X uses Phase-Lock Orbit
    sm_ok.step_head('Y', 1, 2, 3)
    sm_ok.step_head('Z', 1, 2, 3)
    sm_ok.step_head('W', 1, 2, 3)

    ok_report = sm_ok.check_unfolding_and_volume_errors()
    print("  With Phase-Lock Orbit at 0:")
    print(f"    Line unfolding error: {ok_report['line_unfolding_error']}")
    print(f"    Volume distribution error: {ok_report['volume_distribution_error']}")
    print(f"    Volume shares: {ok_report['volume_shares']}")
    assert ok_report['line_unfolding_error'] == False, "Unexpected error!"
    assert ok_report['volume_distribution_error'] == False, "Unexpected error!"
    print("    -> Confirmed: Exact 25.0% volume balance & 0 unfolding error!")

    # 5. Sphere Matrix Multiplication (2D, 3D, 4D)
    print("\n[5] Testing Sphere Matrix Multiplication (2D, 3D, 4D):")
    vec = [100, 200, 300, 400]
    for dim in [2, 3, 4]:
        out = sm_ok.multiply_sphere_matrix(vec, dimension=dim)
        recovered = sm_ok.multiply_sphere_matrix(out, dimension=dim)
        print(f"  {dim}D Sphere Matrix: In={vec} -> Out={out} -> Recovered={recovered}")
        assert vec == recovered, f"{dim}D multiplication recovery failed!"
    print("  -> SUCCESS: All 2D, 3D, 4D Sphere Matrix operations verified!")

    print("\n" + "=" * 70)
    print("  ALL CORRECTED ARCHITECTURE TESTS PASSED SUCCESSFULLY (100%)")
    print("=" * 70)

if __name__ == "__main__":
    run_tests()
