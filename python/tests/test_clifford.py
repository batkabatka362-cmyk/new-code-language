# ============================================================================
# CRON 4D Clifford Algebra Cl(4,0) Test Suite
# Tests: Multivector4D, Rotor4D, Spacetime Tensor Folding, CliffordLinear
# ============================================================================

import math
import unittest
import numpy as np

from cron.clifford import (
    Multivector4D,
    Rotor4D,
    fold_tensor_4d,
    blade_index,
    blade_sign,
    BLADE_SCALAR,
    BLADE_E1,
    BLADE_E2,
    BLADE_E12,
    BLADE_E1234,
)

try:
    import torch
    from cron.clifford import CliffordLinear
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False


class TestCliffordAlgebra(unittest.TestCase):
    def test_blade_index_and_sign(self):
        # Basis vector squares e_i^2 = +1 in Euclidean Cl(4,0)
        for i in [1, 2, 4, 8]:
            self.assertEqual(blade_index(i, i), 0)
            self.assertEqual(blade_sign(i, i), 1.0)

        # Anti-symmetry: e1*e2 = -e2*e1
        self.assertEqual(blade_index(1, 2), 3)
        self.assertEqual(blade_sign(1, 2), 1.0)
        self.assertEqual(blade_sign(2, 1), -1.0)

        # Bivector square: e12*e12 = -1
        self.assertEqual(blade_index(3, 3), 0)
        self.assertEqual(blade_sign(3, 3), -1.0)

        # Pseudoscalar square: e1234*e1234 = +1
        self.assertEqual(blade_index(15, 15), 0)
        self.assertEqual(blade_sign(15, 15), 1.0)

    def test_multivector_geometric_product(self):
        e1 = Multivector4D.vector(1.0, 0.0, 0.0, 0.0)
        e2 = Multivector4D.vector(0.0, 1.0, 0.0, 0.0)

        # e1 * e1 = 1
        e1_sq = e1 * e1
        self.assertAlmostEqual(e1_sq.blades[BLADE_SCALAR], 1.0, places=5)

        # e1 * e2 = e12
        e12 = e1 * e2
        self.assertAlmostEqual(e12.blades[BLADE_E12], 1.0, places=5)

        # e2 * e1 = -e12
        e21 = e2 * e1
        self.assertAlmostEqual(e21.blades[BLADE_E12], -1.0, places=5)

    def test_multivector_reversion_and_norm(self):
        mv = Multivector4D()
        mv.blades[BLADE_SCALAR] = 2.0
        mv.blades[BLADE_E1] = 1.0
        mv.blades[BLADE_E12] = 3.0 # Grade 2
        mv.blades[BLADE_E1234] = 4.0 # Grade 4

        rev = mv.reverse()
        # Scalar and pseudoscalar invariant, bivector flips
        self.assertEqual(rev.blades[BLADE_SCALAR], 2.0)
        self.assertEqual(rev.blades[BLADE_E1], 1.0)
        self.assertEqual(rev.blades[BLADE_E12], -3.0)
        self.assertEqual(rev.blades[BLADE_E1234], 4.0)

        self.assertAlmostEqual(mv.norm(), math.sqrt(4 + 1 + 9 + 16), places=5)

    def test_rotor_planar_rotation_and_length_preservation(self):
        # 90 deg rotation in plane (1, 2)
        r = Rotor4D.from_plane_angle((1, 2), math.pi * 0.5)
        v = np.array([1.0, 0.0, 0.0, 0.0], dtype=np.float32)
        v_rot = r.sandwich_vector(v)

        # (1, 0, 0, 0) -> (0, 1, 0, 0)
        self.assertAlmostEqual(v_rot[0], 0.0, places=4)
        self.assertAlmostEqual(v_rot[1], 1.0, places=4)
        self.assertAlmostEqual(v_rot[2], 0.0, places=4)
        self.assertAlmostEqual(v_rot[3], 0.0, places=4)

        # Arbitrary vector length preservation
        v_arb = np.array([0.7, -1.2, 3.4, 0.5], dtype=np.float32)
        v_arb_rot = r.sandwich_vector(v_arb)
        self.assertAlmostEqual(np.linalg.norm(v_arb), np.linalg.norm(v_arb_rot), places=5)

    def test_rotor_so4_matrix_orthogonality(self):
        for angle in [0.1, 0.5, 1.2, math.pi]:
            r = Rotor4D.from_plane_angle((1, 4), angle)
            M = r.to_rotation_matrix()
            # M @ M^T == I
            np.testing.assert_allclose(M @ M.T, np.eye(4), atol=1e-5)
            self.assertAlmostEqual(np.linalg.det(M), 1.0, places=5)

    def test_tensor_folding_batch(self):
        rotors = [
            Rotor4D.from_plane_angle((1, 2), math.pi * 0.5),
            Rotor4D.from_plane_angle((3, 4), math.pi * 0.5),
        ]
        inputs = np.array([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ], dtype=np.float32)

        folded = fold_tensor_4d(inputs, rotors)
        self.assertEqual(folded.shape, inputs.shape)

        # First rotated in (1,2) -> (0, 1, 0, 0)
        self.assertAlmostEqual(folded[0, 1], 1.0, places=4)
        # Second rotated in (3,4) -> (0, 0, 0, 1)
        self.assertAlmostEqual(folded[1, 3], 1.0, places=4)

    @unittest.skipUnless(TORCH_AVAILABLE, "PyTorch required for CliffordLinear test")
    def test_clifford_linear_layer(self):
        layer = CliffordLinear(in_features=16, out_features=32, bias=True)
        x = torch.randn(2, 8, 16)
        out = layer(x)

        # Shape verification
        self.assertEqual(out.shape, (2, 8, 32))

        # Autograd backward verification
        loss = out.sum()
        loss.backward()

        self.assertIsNotNone(layer.angles.grad)
        self.assertIsNotNone(layer.scales.grad)
        self.assertIsNotNone(layer.bias.grad)
        self.assertFalse(torch.isnan(layer.angles.grad).any())


if __name__ == "__main__":
    unittest.main()
