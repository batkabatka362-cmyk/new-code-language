# ============================================================================
# CRON 4D Sphere Matrix Engine
# Module: python.cron.hypersphere_matrix
# Description:
#   Implements the 4-Head Sphere Matrix Architecture originating from center (0):
#     1. 4 Heads (X, Y, Z, W) starting from origin 0
#     2. Radial layer W in [0, 99] (depth from center to surface)
#     3. Dual Rubik-cube axis mapping:
#        - X: Top to half (0..99, north pole to equator)
#        - Z: Half to bottom (0..99, equator to south pole, right hemisphere)
#        - Packed string format (e.g., '5667' -> X=56, Z=67) & 14-bit (X << 7) | Z
#     4. Next location steering & 2D / 3D / 4D Sphere Matrix Multiplication:
#        - Transition mode A: Complete wrap around (360 deg) before next layer
#        - Transition mode B: Direct layer step
#        - Error tracking: 4D line unfolding & volume distribution on idle 0
#        - Direct Sphere Matrix multiplication mode (ignoring distortion)
# ============================================================================

from dataclasses import dataclass, field
from enum import Enum
from typing import List, Tuple, Dict, Optional


# Bitwise Hardware Masks
MASK_7BIT = 0x7F       # 7-bit mask (0..127, covers 0..99)
MASK_14BIT = 0x3FFF    # 14-bit packed surface mask (0..16383)
COORD_SHIFT = 7        # Shift for 14-bit packing: (X << 7) | Z
MAX_LAYER = 99         # Radial layer bound W in [0, 99]
MAX_ANGLE = 100        # Angular resolution for X, Z in [0, 99]


class LayerTransitionMode(Enum):
    COMPLETE_WRAP = "complete_wrap"   # Wrap 360 degrees on layer before advancing W
    DIRECT_STEP = "direct_step"       # Advance W directly without full wrap


@dataclass
class HeadState:
    """State vector of one of the 4 heads (X, Y, Z, W)."""
    name: str              # 'X', 'Y', 'Z', or 'W'
    w: int = 0             # Radial layer W in [0, 99] (distance from center 0)
    x: int = 0             # Top to half angle X in [0, 99]
    z: int = 0             # Half to bottom angle Z in [0, 99]
    phase: int = 0         # Internal hardware phase counter (for Phase-Lock Orbit)
    path_length: int = 0   # Mechanical path length (L)
    wrapped_angle: int = 0 # Accumulated angular traversal on current layer
    trajectories: List[Tuple[int, int, int]] = field(default_factory=list)

    def __post_init__(self):
        if not self.trajectories:
            self.trajectories.append((self.w, self.x, self.z))

    @property
    def packed_code(self) -> str:
        """Formatted 4-digit code e.g. '5667' for X=56, Z=67."""
        return f"{self.x:02d}{self.z:02d}"

    @property
    def packed_14bit(self) -> int:
        """14-bit packed hardware coordinate C = (X << 7) | Z."""
        return ((self.x & MASK_7BIT) << COORD_SHIFT) | (self.z & MASK_7BIT)


class SphereMatrix4D:
    """
    4D Sphere Matrix Engine tracking 4 heads (X, Y, Z, W) starting from center 0.
    """

    def __init__(self, transition_mode: LayerTransitionMode = LayerTransitionMode.DIRECT_STEP):
        self.transition_mode = transition_mode
        # 4 Heads originating from center point 0
        # Heads X, Y, Z, W are spaced symmetrically in initial W layers to satisfy curved needle
        self.heads: Dict[str, HeadState] = {
            'X': HeadState(name='X', w=0, x=0,  z=0),
            'Y': HeadState(name='Y', w=1, x=25, z=25),
            'Z': HeadState(name='Z', w=2, x=50, z=50),
            'W': HeadState(name='W', w=3, x=75, z=75),
        }
        self.current_tact: int = 0

    @staticmethod
    def parse_coordinate_code(code_str: str) -> Tuple[int, int]:
        """
        Parses 4-digit coordinate string e.g. '5667' -> (56, 67)
        where X=56 (top to half) and Z=67 (half to bottom).
        """
        assert len(code_str) == 4, f"Code must be 4 digits, got {code_str}"
        x = int(code_str[:2])
        z = int(code_str[2:])
        assert 0 <= x <= 99 and 0 <= z <= 99, f"Coordinates out of [0, 99]: X={x}, Z={z}"
        return x, z

    def step_head(self, head_name: str, dw: int, dx: int, dz: int, use_phase_lock: bool = True) -> Dict[str, any]:
        """
        Advances a head by (dW, dX, dZ).
        If (dW, dX, dZ) == (0, 0, 0):
          - If use_phase_lock is True: Phase-Lock Orbit activates (||Delta|| = 1, path_length += 1).
          - If use_phase_lock is False: Dead stop (creates line unfolding & volume error).
        """
        head = self.heads[head_name]

        if dw == 0 and dx == 0 and dz == 0:
            if use_phase_lock:
                head.phase += 1
                head.path_length += 1
                head.trajectories.append((head.w, head.x, head.z))
                return {"head": head_name, "status": "Phase-Lock Orbit", "length": head.path_length}
            else:
                # Dead stop without phase lock
                head.trajectories.append((head.w, head.x, head.z))
                return {"head": head_name, "status": "Dead Stop (Void/Deficit Error)", "length": head.path_length}

        # Handle layer transition mode
        if self.transition_mode == LayerTransitionMode.COMPLETE_WRAP:
            head.wrapped_angle += abs(dx) + abs(dz)
            if head.wrapped_angle >= 100:
                head.w = min(MAX_LAYER, max(0, head.w + dw))
                head.wrapped_angle = 0
        else:
            # Direct step into layer
            head.w = min(MAX_LAYER, max(0, head.w + dw))

        head.x = (head.x + dx) % MAX_ANGLE
        head.z = (head.z + dz) % MAX_ANGLE
        head.path_length += 1
        head.trajectories.append((head.w, head.x, head.z))

        return {
            "head": head_name,
            "pos": (head.w, head.x, head.z),
            "packed_code": head.packed_code,
            "packed_14bit": head.packed_14bit,
            "length": head.path_length
        }

    def verify_curved_needle(self) -> List[Dict]:
        """
        Verifies the 'Curved Needle' theorem:
        If X_i == X_j and Z_i == Z_j, then |W_i - W_j| >= 1.
        Returns list of collisions (must be EMPTY).
        """
        collisions = []
        head_list = list(self.heads.values())
        for i in range(len(head_list)):
            for j in range(i + 1, len(head_list)):
                h1, h2 = head_list[i], head_list[j]
                if h1.x == h2.x and h1.z == h2.z:
                    diff_w = abs(h1.w - h2.w)
                    if diff_w < 1:
                        collisions.append({
                            "head_1": h1.name, "head_2": h2.name,
                            "pos_1": (h1.w, h1.x, h1.z), "pos_2": (h2.w, h2.x, h2.z)
                        })
        return collisions

    def check_unfolding_and_volume_errors(self) -> Dict[str, any]:
        """
        Evaluates the 2 known error conditions:
          1. 4D line unfolding error: divergence in path lengths (L_i != L_j)
          2. Sphere volume distribution error: deviation from 25.0% equal volume
        """
        lengths = [h.path_length for h in self.heads.values()]
        total_len = sum(lengths)

        max_len = max(lengths)
        min_len = min(lengths)
        line_unfolding_error = (max_len != min_len)

        if total_len == 0:
            volume_shares = {h.name: 25.0 for h in self.heads.values()}
            volume_error = False
        else:
            volume_shares = {h.name: (h.path_length / total_len) * 100.0 for h in self.heads.values()}
            volume_error = any(abs(share - 25.0) > 1e-4 for share in volume_shares.values())

        return {
            "path_lengths": {h.name: h.path_length for h in self.heads.values()},
            "line_unfolding_error": line_unfolding_error,
            "volume_distribution_error": volume_error,
            "volume_shares": volume_shares
        }

    def multiply_sphere_matrix(self, input_vector: List[int], dimension: int = 4) -> List[int]:
        """
        Executes Sphere Matrix Multiplication across 2D, 3D, or 4D:
        Uses the 4-head coordinates (W, X, Z) directly as the geometric matrix operator.
        In direct matrix mode, geometric line distortion errors are bypassed.
        """
        assert dimension in (2, 3, 4), "Dimension must be 2, 3, or 4"
        n = len(input_vector)
        output = [0] * n

        head_names = ['X', 'Y', 'Z', 'W'][:dimension]

        for i in range(n):
            head_idx = i % dimension
            head = self.heads[head_names[head_idx]]

            if dimension == 2:
                # 2D Sphere Matrix: Uses clock angle X
                matrix_weight = (head.x & MASK_7BIT)
            elif dimension == 3:
                # 3D Sphere Matrix: Uses dual angles X and Z
                matrix_weight = head.packed_14bit & 0x0FFF
            else:
                # 4D Sphere Matrix: Uses radial depth W combined with 14-bit surface
                matrix_weight = ((head.w & MASK_7BIT) << 8) | (head.packed_14bit & 0x00FF)

            # Reversible bitwise multiplication/transformation along sphere geodesic
            output[i] = input_vector[i] ^ matrix_weight

        return output
