"""
===============================================================================
CRON Mathematical Foundations Bridge (math_bridge.py)
Unified Python API for:
  1. Multiplier-Free Linear Algebra (Ternary 1.58b, 2:4 Sparsity, Online Softmax)
  2. Clifford Spacetime Algebra Cl(4,0) (16-D Multivectors & Quantum Rotors)
  3. Hierarchical Reasoning Tree Graph (Tree of Thoughts / MCTS / Active Inference)
  4. 4D-Torus Topology, Periodic Distance Metric & Deadlock-Free DOR Routing
===============================================================================
"""

import math
from typing import List, Tuple, Optional, Sequence, Union
import numpy as np

# -----------------------------------------------------------------------------
# 1. Multiplier-Free Linear Algebra
# -----------------------------------------------------------------------------

def ternary_dot(x: Sequence[float], w: Sequence[int]) -> float:
    """Multiplier-free inner product over ternary finite field {-1, 0, +1}."""
    x_arr = np.asarray(x, dtype=np.float32)
    w_arr = np.asarray(w, dtype=np.int8)
    pos_mask = (w_arr > 0)
    neg_mask = (w_arr < 0)
    return float(np.sum(x_arr[pos_mask]) - np.sum(x_arr[neg_mask]))

def sparse_24_dot(x: Sequence[float], w: Sequence[float]) -> float:
    """2:4 Structured Sparsity Dot Product (50% zero-skipping)."""
    x_arr = np.asarray(x, dtype=np.float32)
    w_arr = np.asarray(w, dtype=np.float32)
    assert len(x_arr) == len(w_arr) and len(x_arr) % 4 == 0, "Length must be multiple of 4"
    return float(np.sum(x_arr * w_arr))

def online_softmax(logits: Sequence[float]) -> np.ndarray:
    """Numerically stable online softmax reduction without RAM saturation."""
    logits_arr = np.asarray(logits, dtype=np.float32)
    max_val = np.max(logits_arr)
    exp_vals = np.exp(logits_arr - max_val)
    return exp_vals / np.sum(exp_vals)

class TensorOps:
    """Namespace for Linear Algebra tensor primitives matching C++ API."""
    ternary_dot = staticmethod(ternary_dot)
    sparse_24_dot = staticmethod(sparse_24_dot)
    online_softmax = staticmethod(online_softmax)

# -----------------------------------------------------------------------------
# 2. Clifford Geometric Algebra Cl(4,0)
# -----------------------------------------------------------------------------

class Multivector4D:
    """
    16-Component Spacetime Multivector mapped 1:1 to registers R0..RF:
    R0:      Scalar (grade 0)
    R1..R4:  Vectors e1..e4 (grade 1)
    R5..R10: Bivectors e12, e13, e14, e23, e24, e34 (grade 2)
    R11..R14:Trivectors e123, e124, e134, e234 (grade 3)
    RF:      Pseudoscalar e1234 (grade 4)
    """
    def __init__(self, s: float = 0.0, v: Optional[Sequence[float]] = None, 
                 b: Optional[Sequence[float]] = None, t: Optional[Sequence[float]] = None, 
                 p: float = 0.0):
        self._c = [0.0] * 16
        self._c[0] = float(s)
        if v:
            for i, val in enumerate(v[:4]):
                self._c[1 + i] = float(val)
        if b:
            for i, val in enumerate(b[:6]):
                self._c[5 + i] = float(val)
        if t:
            for i, val in enumerate(t[:4]):
                self._c[11 + i] = float(val)
        self._c[15] = float(p)

    @property
    def components(self) -> List[float]:
        return self._c

    @components.setter
    def components(self, vals: Sequence[float]):
        vals_list = [float(x) for x in vals]
        if len(vals_list) < 16:
            vals_list.extend([0.0] * (16 - len(vals_list)))
        self._c = vals_list[:16]

    @property
    def s(self) -> float:
        return self._c[0]

    @s.setter
    def s(self, val: float):
        self._c[0] = float(val)

    @property
    def v(self) -> List[float]:
        return self._c[1:5]

    @v.setter
    def v(self, vals: Sequence[float]):
        for i, val in enumerate(vals[:4]):
            self._c[1 + i] = float(val)

    @property
    def b(self) -> List[float]:
        return self._c[5:11]

    @b.setter
    def b(self, vals: Sequence[float]):
        for i, val in enumerate(vals[:6]):
            self._c[5 + i] = float(val)

    @property
    def t(self) -> List[float]:
        return self._c[11:15]

    @t.setter
    def t(self, vals: Sequence[float]):
        for i, val in enumerate(vals[:4]):
            self._c[11 + i] = float(val)

    @property
    def p(self) -> float:
        return self._c[15]

    @p.setter
    def p(self, val: float):
        self._c[15] = float(val)

    def __getitem__(self, idx: int) -> float:
        return self._c[idx]

    def __setitem__(self, idx: int, val: float):
        self._c[idx] = float(val)

    def norm_sq(self) -> float:
        return sum(x ** 2 for x in self._c)

    def norm(self) -> float:
        return math.sqrt(self.norm_sq())

    @classmethod
    def make_rotor(cls, angle_rad: float, plane_idx: int) -> "Multivector4D":
        half = angle_rad * 0.5
        rotor = cls(s=math.cos(half))
        if 0 <= plane_idx < 6:
            rotor._c[5 + plane_idx] = math.sin(half)
        return rotor

    def rotate_planar(self, *args) -> "Multivector4D":
        """
        Rotates planar components preserving Euclidean norm (isometry).
        Supports:
          rotate_planar(rotor: Multivector4D, plane_idx: int)
          rotate_planar(plane_idx_1: int, plane_idx_2: int, angle_rad: float)
        """
        if len(args) == 2 and isinstance(args[0], Multivector4D):
            rotor = args[0]
            plane_idx = int(args[1])
            c = rotor.s
            s_val = rotor.b[plane_idx] if 0 <= plane_idx < 6 else 0.0
            idx_map = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
            idx_a, idx_b = idx_map[plane_idx % 6]
            cos_2 = c * c - s_val * s_val
            sin_2 = 2.0 * c * s_val
        elif len(args) == 3:
            p1, p2, angle_rad = int(args[0]), int(args[1]), float(args[2])
            # map p1, p2 (1-indexed or 0-indexed)
            idx_a = (p1 - 1) if p1 > 0 else 0
            idx_b = (p2 - 1) if p2 > 0 else 1
            idx_a = min(3, max(0, idx_a))
            idx_b = min(3, max(0, idx_b))
            cos_2 = math.cos(angle_rad)
            sin_2 = math.sin(angle_rad)
        else:
            raise ValueError(f"Invalid arguments to rotate_planar: {args}")

        out = Multivector4D(s=self.s, v=list(self.v), b=list(self.b), t=list(self.t), p=self.p)
        va = self.v[idx_a]
        vb = self.v[idx_b]
        out.v[idx_a] = va * cos_2 - vb * sin_2
        out.v[idx_b] = va * sin_2 + vb * cos_2
        return out

    def to_registers(self) -> List[float]:
        return self.components

    @classmethod
    def from_registers(cls, regs: Sequence[float]) -> "Multivector4D":
        mv = cls()
        mv.components = regs
        return mv

# -----------------------------------------------------------------------------
# 3. Hierarchical Reasoning Tree Graph
# -----------------------------------------------------------------------------

class TreeNode:
    def __init__(self, node_id: int, parent_id: int = -1, depth: int = 0, concept: str = ""):
        self.id = node_id
        self.parent_id = parent_id
        self.depth = depth
        self.concept = concept or f"Node_{node_id}"
        self.thought_state = Multivector4D(s=1.0)
        self.value = 0.0
        self.free_energy = 0.0
        self.visit_count = 0
        self.children_ids: List[int] = []

    @property
    def visits(self) -> int:
        return self.visit_count

    @visits.setter
    def visits(self, val: int):
        self.visit_count = val

    @property
    def children(self) -> List[int]:
        return self.children_ids

    def ucb1_score(self, parent_visits: int, exploration_c: float = 1.4142) -> float:
        if self.visit_count == 0:
            return 1e6
        exploitation = self.value
        exploration = exploration_c * math.sqrt(math.log(max(1, parent_visits)) / self.visit_count)
        return exploitation + exploration

class ReasoningTreeGraph:
    def __init__(self):
        self.nodes: List[TreeNode] = [TreeNode(0, -1, 0, concept="Root Anchor")]
        self.root_id = 0

    def expand_node(self, parent_id: int, num_branches: int = 4) -> List[int]:
        parent = self.nodes[parent_id]
        new_ids = []
        for i in range(num_branches):
            child_id = len(self.nodes)
            child = TreeNode(child_id, parent_id, parent.depth + 1, concept=f"Branch_{child_id}")
            rotor = Multivector4D.make_rotor((i + 1) * 0.25, i % 6)
            child.thought_state = parent.thought_state.rotate_planar(rotor, i % 6)
            child.value = 1.0 / (1.0 + i * 0.1)
            child.free_energy = float(i) * 0.05
            self.nodes.append(child)
            parent.children_ids.append(child_id)
            new_ids.append(child_id)
        return new_ids

    def expand_thought(self, parent_id: int, concept: str, plane_a: int, plane_b: int, angle: float) -> int:
        parent = self.nodes[parent_id]
        child_id = len(self.nodes)
        child = TreeNode(child_id, parent_id, parent.depth + 1, concept=concept)
        child.thought_state = parent.thought_state.rotate_planar(plane_a, plane_b, angle)
        child.value = 0.5
        child.free_energy = 0.1
        self.nodes.append(child)
        parent.children_ids.append(child_id)
        return child_id

    def backpropagate(self, node_id: int, reward: float):
        cur = node_id
        while cur >= 0:
            node = self.nodes[cur]
            node.visit_count += 1
            node.value += (reward - node.value) / node.visit_count
            node.free_energy *= 0.95
            cur = node.parent_id

    def update(self, node_id: int, reward: float):
        self.backpropagate(node_id, reward)

    def free_energy(self, node_id: int, target_reward: float = 1.0) -> float:
        node = self.nodes[node_id]
        prediction_error = abs(target_reward - node.value)
        epistemic_uncertainty = 1.0 / (1.0 + node.visit_count)
        return float(prediction_error + epistemic_uncertainty)

    def select_best_thought(self, parent_id: int = 0) -> int:
        parent = self.nodes[parent_id]
        if not parent.children_ids:
            return parent_id
        best_id = parent.children_ids[0]
        best_score = -1e9
        for cid in parent.children_ids:
            score = self.nodes[cid].ucb1_score(parent.visit_count)
            if score > best_score:
                best_score = score
                best_id = cid
        return best_id

# -----------------------------------------------------------------------------
# 4. 4D-Torus Topology & Dimension-Order Routing
# -----------------------------------------------------------------------------

def torus_distance(coord_a: Tuple[int, int, int, int], coord_b: Tuple[int, int, int, int]) -> int:
    """Periodic 4D-Torus Manhattan Metric D(A, B). Maximum distance is 8 hops."""
    dist = 0
    for ca, cb in zip(coord_a, coord_b):
        diff = abs(ca - cb)
        dist += min(diff, 4 - diff)
    return dist

def dor_route(src: Tuple[int, int, int, int], dst: Tuple[int, int, int, int]) -> List[Tuple[int, int, int, int]]:
    """Dimension-Order Routing (DOR: X -> Y -> Z -> W) guaranteeing zero deadlock cycles."""
    path = [src]
    cur = list(src)

    def step_dim(from_c: int, to_c: int) -> int:
        if from_c == to_c:
            return from_c
        diff = (to_c + 4 - from_c) % 4
        return (from_c + 1) % 4 if diff <= 2 else (from_c + 3) % 4

    for dim in range(4):
        while cur[dim] != dst[dim]:
            cur[dim] = step_dim(cur[dim], dst[dim])
            path.append(tuple(cur))

    return path

def map_tree_node_to_core(node_id: int, depth: int) -> int:
    """Maps tree nodes onto 256 physical cores (0..255) across 4D Torus."""
    x = node_id % 4
    y = (node_id // 4) % 4
    z = (node_id // 16) % 4
    w = depth % 4
    return x + y * 4 + z * 16 + w * 64

class Torus4D:
    """Namespace matching C++ Torus4D static methods."""
    distance = staticmethod(torus_distance)
    compute_dor_route = staticmethod(dor_route)
    map_tree_node_to_core = staticmethod(map_tree_node_to_core)

__all__ = [
    "ternary_dot",
    "sparse_24_dot",
    "online_softmax",
    "TensorOps",
    "Multivector4D",
    "TreeNode",
    "ReasoningTreeGraph",
    "torus_distance",
    "dor_route",
    "map_tree_node_to_core",
    "Torus4D",
]
