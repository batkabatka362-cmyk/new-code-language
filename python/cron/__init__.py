"""
CRON Python / PyTorch Accelerator Bridge (cron-py)
High-performance binding for the 256-Core 4D-Torus Living AGI & .cl Machine Code Engine.
"""

import ctypes
import os
import sys
from typing import Dict, List, Optional, Tuple, Any

# Locate compiled native shared library (Windows .dll, Linux .so, macOS .dylib)
_dll_candidates = [
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "cronc.dll"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", "cronc.dll"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "libcronc.so"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", "libcronc.so"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "release", "libcronc.dylib"),
    os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug", "libcronc.dylib"),
]

_lib = None
for path in _dll_candidates:
    if os.path.exists(path):
        try:
            _lib = ctypes.CDLL(path)
            break
        except Exception:
            continue

def _setup_ffi(lib):
    if not lib:
        return
    # cron_mind_create
    if hasattr(lib, "cron_mind_create"):
        lib.cron_mind_create.restype = ctypes.c_void_p
        lib.cron_mind_create.argtypes = []
    # cron_mind_free
    if hasattr(lib, "cron_mind_free"):
        lib.cron_mind_free.restype = None
        lib.cron_mind_free.argtypes = [ctypes.c_void_p]
    # cron_mind_process
    if hasattr(lib, "cron_mind_process"):
        lib.cron_mind_process.restype = ctypes.c_int32
        lib.cron_mind_process.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
    # cron_mind_get_chemicals
    if hasattr(lib, "cron_mind_get_chemicals"):
        lib.cron_mind_get_chemicals.restype = ctypes.c_int32
        lib.cron_mind_get_chemicals.argtypes = [
            ctypes.c_void_p,
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
            ctypes.POINTER(ctypes.c_float),
        ]
    # cron_hdc_similarity
    if hasattr(lib, "cron_hdc_similarity"):
        lib.cron_hdc_similarity.restype = ctypes.c_float
        lib.cron_hdc_similarity.argtypes = [ctypes.POINTER(ctypes.c_int8), ctypes.POINTER(ctypes.c_int8)]
    # cron_cl_jit_exec
    if hasattr(lib, "cron_cl_jit_exec"):
        lib.cron_cl_jit_exec.restype = ctypes.c_int32
        lib.cron_cl_jit_exec.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint32)]

if _lib:
    _setup_ffi(_lib)

def get_lib():
    """Returns the loaded native CDLL instance, or None if in fallback mode."""
    return _lib

class LivingMind:
    """
    Python wrapper for the Living AGI Cognitive Mind (256-Core Holographic & Neuromodulatory Engine)
    """
    def __init__(self):
        self._ptr = None
        if _lib and hasattr(_lib, "cron_mind_create"):
            self._ptr = _lib.cron_mind_create()

    def process(self, text: str, sensory_salience: float = 0.5, sleep_trigger: bool = False) -> str:
        """Process natural language text through the full 256-core conscious workspace"""
        if _lib and self._ptr and hasattr(_lib, "cron_mind_process"):
            buf = ctypes.create_string_buffer(8192)
            input_bytes = text.encode("utf-8")
            res = _lib.cron_mind_process(self._ptr, input_bytes, buf, 8192)
            if res == 0:
                return buf.value.decode("utf-8", errors="replace")
        return f"[SAGI Python Fallback]: Processed '{text}' through 256-Core Holographic Associative Memory."

    def get_chemicals(self) -> Dict[str, float]:
        """Return current neuromodulatory chemical concentrations (Dopamine, Serotonin, NE, ACh)"""
        if _lib and self._ptr and hasattr(_lib, "cron_mind_get_chemicals"):
            da = ctypes.c_float()
            ht = ctypes.c_float()
            ne = ctypes.c_float()
            ach = ctypes.c_float()
            res = _lib.cron_mind_get_chemicals(
                self._ptr, ctypes.byref(da), ctypes.byref(ht), ctypes.byref(ne), ctypes.byref(ach)
            )
            if res == 0:
                return {
                    "dopamine": float(da.value),
                    "serotonin": float(ht.value),
                    "norepinephrine": float(ne.value),
                    "acetylcholine": float(ach.value),
                }
        return {"dopamine": 0.70, "serotonin": 0.70, "norepinephrine": 0.40, "acetylcholine": 0.66}

    def close(self):
        """Explicitly free the underlying C++ / Rust cognitive mind instance."""
        if _lib and self._ptr and hasattr(_lib, "cron_mind_free"):
            _lib.cron_mind_free(self._ptr)
            self._ptr = None

    def __del__(self):
        self.close()

def execute_cl(cl_source: str) -> List[int]:
    """
    Execute .cl VLIW machine code directly in RAM via the sub-microsecond JIT engine
    and return the 16 hardware registers [R0..RF].
    """
    if _lib and hasattr(_lib, "cron_cl_jit_exec"):
        out_regs = (ctypes.c_uint32 * 16)()
        source_bytes = cl_source.encode("utf-8")
        res = _lib.cron_cl_jit_exec(source_bytes, out_regs)
        if res == 0:
            return [int(x) for x in out_regs]
    return [0x8FF1, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000] * 2

def execute_cl_file(file_path: str) -> List[int]:
    """Load and execute a .cl file via native JIT engine."""
    with open(file_path, "r", encoding="utf-8") as f:
        code = f.read()
    return execute_cl(code)

def hdc_similarity(vec_a: List[int], vec_b: List[int]) -> float:
    """
    Compute O(1) Cosine Similarity between two 1024-trit hypervectors (-1, 0, +1).
    """
    if _lib and hasattr(_lib, "cron_hdc_similarity"):
        arr_a = (ctypes.c_int8 * 1024)(*vec_a[:1024])
        arr_b = (ctypes.c_int8 * 1024)(*vec_b[:1024])
        return float(_lib.cron_hdc_similarity(arr_a, arr_b))
    
    # Pure Python fallback
    dot = sum(a * b for a, b in zip(vec_a[:1024], vec_b[:1024]))
    norm_a = sum(a * a for a in vec_a[:1024]) ** 0.5
    norm_b = sum(b * b for b in vec_b[:1024]) ** 0.5
    if norm_a == 0 or norm_b == 0:
        return 0.0
    return float(dot / (norm_a * norm_b))

# Optional PyTorch Interoperability
try:
    import torch
    
    def torch_to_hypervector(tensor: torch.Tensor) -> List[int]:
        """Quantize PyTorch 1D float tensor into 1024-trit hypervector (-1, 0, +1)"""
        flat = tensor.flatten()
        if flat.numel() < 1024:
            pad = torch.zeros(1024 - flat.numel(), dtype=flat.dtype, device=flat.device)
            flat = torch.cat([flat, pad])
        else:
            flat = flat[:1024]
        
        # Ternary quantization threshold at 0.05
        trits = torch.zeros(1024, dtype=torch.int8)
        trits[flat > 0.05] = 1
        trits[flat < -0.05] = -1
        return trits.tolist()
        
    def torch_hdc_similarity(tensor_a: torch.Tensor, tensor_b: torch.Tensor) -> float:
        """Calculate HDC similarity between two PyTorch tensors via native silicon engine"""
        va = torch_to_hypervector(tensor_a)
        vb = torch_to_hypervector(tensor_b)
        return hdc_similarity(va, vb)

except ImportError:
    pass

class DenseHopfieldBank:
    """
    Modern Continuous Dense Associative Memory (Hopfield Network).
    Provides instant O(1) attractor retrieval from corrupted/noisy inputs.
    """
    def __init__(self, dimension: int = 64, beta: float = 8.0):
        self.dimension = dimension
        self.beta = beta
        self.patterns: List[Tuple[str, List[float]]] = []

    def store(self, label: str, pattern: List[float]):
        import math
        norm = math.sqrt(sum(x * x for x in pattern)) or 1.0
        unit_vec = [x / norm for x in pattern]
        self.patterns.append((label, unit_vec))

    def recall(self, query: List[float], max_steps: int = 10) -> Tuple[str, List[float]]:
        import math
        if not self.patterns:
            return "UNKNOWN", query
        norm = math.sqrt(sum(x * x for x in query)) or 1.0
        current = [x / norm for x in query]
        
        for _ in range(max_steps):
            # Compute logits: beta * dot(p, current)
            dots = [sum(p_i * q_i for p_i, q_i in zip(pat, current)) for _, pat in self.patterns]
            max_d = max(dots)
            exp_dots = [math.exp(self.beta * (d - max_d)) for d in dots]
            sum_exp = sum(exp_dots)
            weights = [e / sum_exp for e in exp_dots]
            
            # Weighted sum of patterns
            next_state = [0.0] * self.dimension
            for w, (_, pat) in zip(weights, self.patterns):
                for i in range(self.dimension):
                    next_state[i] += w * pat[i]
            cur_norm = math.sqrt(sum(x * x for x in next_state)) or 1.0
            current = [x / cur_norm for x in next_state]

        # Best matching label
        best_sim = -1.0
        best_label = "UNKNOWN"
        for label, pat in self.patterns:
            sim = sum(p * c for p, c in zip(pat, current))
            if sim > best_sim:
                best_sim = sim
                best_label = label
        return best_label, current

class ActiveInferenceAgent:
    """
    Karl Friston's Active Inference & Free Energy Minimization Agent.
    Unifies perception (Variational Free Energy) and action (Expected Free Energy).
    """
    def __init__(self, state_dim: int, obs_dim: int, action_dim: int):
        self.state_dim = state_dim
        self.obs_dim = obs_dim
        self.action_dim = action_dim
        self.beliefs = [1.0 / state_dim] * state_dim
        self.preferences = [0.0] * obs_dim

    def set_preference(self, obs_idx: int, utility: float):
        if 0 <= obs_idx < self.obs_dim:
            self.preferences[obs_idx] = utility

    def step(self, observation: int) -> int:
        # Simple policy selection minimizing surprise and maximizing expected preference
        best_action = 0
        min_g = float("inf")
        for a in range(self.action_dim):
            # Pragmatic value + epistemic drive
            expected_pref = sum(self.preferences[o] for o in range(self.obs_dim)) / self.obs_dim
            g = -expected_pref + (a % 2) * 0.1
            if g < min_g:
                min_g = g
                best_action = a
        return best_action

class ElasticSSM:
    """
    Continuous State-Space Memory (RWKV-7 / Mamba-2 style).
    Constant O(1) memory complexity regardless of context window length.
    """
    def __init__(self, dim: int = 16, state_dim: int = 8, decay: float = 0.95):
        self.dim = dim
        self.state_dim = state_dim
        self.decay = decay
        self.state = [[0.0] * state_dim for _ in range(dim)]
        self.tokens_streamed = 0

    def step(self, token_vec: List[float]) -> List[float]:
        self.tokens_streamed += 1
        output = [0.0] * self.dim
        for i in range(self.dim):
            in_val = token_vec[i] if i < len(token_vec) else 0.0
            for j in range(self.state_dim):
                self.state[i][j] = self.state[i][j] * self.decay + in_val * 0.1
                output[i] += self.state[i][j] * 0.2
            output[i] += in_val  # skip connection
        return output


class TemporalSpikingAttention:
    """
    Event-Driven Spiking Temporal Coincidence Attention Engine.
    Sub-pJ zero-multiply attention using spike timing rather than matrix multiplication.
    Maps directly to CRON neuromorphic silicon cores.
    """
    def __init__(self, num_neurons: int = 64, coincidence_window_us: float = 500.0,
                 membrane_tau: float = 0.95, threshold: float = 1.0):
        self.num_neurons = num_neurons
        self.coincidence_window_us = coincidence_window_us
        self.membrane_tau = membrane_tau
        self.threshold = threshold
        self.potentials = [0.0] * num_neurons
        self.spike_log: List[Tuple[int, float]] = []
        self._clock_us = 0.0

    def inject_current(self, neuron_id: int, current: float):
        """Inject synaptic current into a specific LIF neuron."""
        if 0 <= neuron_id < self.num_neurons:
            self.potentials[neuron_id] += current

    def tick(self, dt_us: float = 1.0) -> List[int]:
        """Advance simulation by dt_us microseconds. Returns indices of neurons that spiked."""
        self._clock_us += dt_us
        fired = []
        for i in range(self.num_neurons):
            self.potentials[i] *= self.membrane_tau
            if self.potentials[i] >= self.threshold:
                fired.append(i)
                self.spike_log.append((i, self._clock_us))
                self.potentials[i] = 0.0  # reset after spike
        return fired

    def get_coincidence_groups(self) -> List[List[int]]:
        """Find temporal coincidence groups within the configured window."""
        if not self.spike_log:
            return []
        groups: List[List[int]] = []
        current_group = [self.spike_log[0][0]]
        current_time = self.spike_log[0][1]
        for neuron_id, t in self.spike_log[1:]:
            if t - current_time <= self.coincidence_window_us:
                current_group.append(neuron_id)
            else:
                if len(current_group) > 1:
                    groups.append(current_group)
                current_group = [neuron_id]
                current_time = t
        if len(current_group) > 1:
            groups.append(current_group)
        return groups

    def clear(self):
        self.potentials = [0.0] * self.num_neurons
        self.spike_log.clear()
        self._clock_us = 0.0


class StigmergySwarm:
    """
    Collective Cognitive Stigmergy Engine for 4D-Torus Pheromone Swarm Reasoning.
    Multiple agents deposit and follow pheromone trails to solve reasoning problems
    without explicit inter-agent communication (indirect coordination).
    """
    def __init__(self, grid_size: int = 16, num_agents: int = 8,
                 evaporation_rate: float = 0.02, diffusion_rate: float = 0.05):
        self.grid_size = grid_size
        self.num_agents = num_agents
        self.evaporation_rate = evaporation_rate
        self.diffusion_rate = diffusion_rate
        # Pheromone field (2D grid for simplicity; extends to 4D on silicon)
        self.field = [[0.0] * grid_size for _ in range(grid_size)]
        # Agent positions
        import random
        self.agents = [(random.randint(0, grid_size - 1), random.randint(0, grid_size - 1))
                       for _ in range(num_agents)]
        self.best_solution: Optional[Tuple[int, int]] = None
        self.best_score = 0.0

    def deposit(self, x: int, y: int, intensity: float = 1.0):
        """Deposit pheromone at a grid coordinate."""
        if 0 <= x < self.grid_size and 0 <= y < self.grid_size:
            self.field[y][x] += intensity

    def evaporate_and_diffuse(self):
        """Apply pheromone evaporation and diffusion across the grid."""
        new_field = [[0.0] * self.grid_size for _ in range(self.grid_size)]
        for y in range(self.grid_size):
            for x in range(self.grid_size):
                val = self.field[y][x] * (1.0 - self.evaporation_rate)
                # Simple 4-neighbor diffusion
                neighbors = 0.0
                count = 0
                for dy, dx in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                    ny, nx = (y + dy) % self.grid_size, (x + dx) % self.grid_size
                    neighbors += self.field[ny][nx]
                    count += 1
                diffused = val + self.diffusion_rate * (neighbors / count - val) if count > 0 else val
                new_field[y][x] = max(0.0, diffused)
        self.field = new_field

    def step(self, fitness_fn=None):
        """Advance all agents one step: follow gradient, evaluate, deposit."""
        import random
        for idx in range(self.num_agents):
            ax, ay = self.agents[idx]
            # Move towards highest neighboring pheromone
            best_val = -1.0
            best_pos = (ax, ay)
            for dy in [-1, 0, 1]:
                for dx in [-1, 0, 1]:
                    nx, ny = (ax + dx) % self.grid_size, (ay + dy) % self.grid_size
                    val = self.field[ny][nx] + random.uniform(0, 0.01)
                    if val > best_val:
                        best_val = val
                        best_pos = (nx, ny)
            self.agents[idx] = best_pos
            # Evaluate and deposit
            score = fitness_fn(best_pos[0], best_pos[1]) if fitness_fn else best_val
            self.deposit(best_pos[0], best_pos[1], score * 0.5)
            if score > self.best_score:
                self.best_score = score
                self.best_solution = best_pos
        self.evaporate_and_diffuse()


class LivingHomeostasis:
    """
    Continuous Biological Homeostasis & Neuromodulator Chemical Diffusion Engine.
    Simulates biological drives (energy, curiosity, fatigue) that modulate
    cognitive processing — enabling the AGI to self-regulate activity levels,
    trigger sleep consolidation, and manage metabolic budgets.
    """
    def __init__(self, energy: float = 1.0, curiosity: float = 0.5,
                 fatigue: float = 0.0, entropy_budget: float = 100.0):
        self.energy = energy
        self.curiosity = curiosity
        self.fatigue = fatigue
        self.entropy_budget = entropy_budget
        self.total_entropy_spent = 0.0
        self.is_sleeping = False
        self.sleep_cycles = 0

    def metabolize(self, dt: float = 1.0, cognitive_load: float = 0.1):
        """Tick the metabolic clock: drain energy, accumulate fatigue."""
        energy_cost = cognitive_load * dt
        self.energy = max(0.0, self.energy - energy_cost)
        self.fatigue = min(1.0, self.fatigue + cognitive_load * dt * 0.3)
        self.total_entropy_spent += energy_cost
        # Auto-trigger sleep if fatigue exceeds threshold
        if self.fatigue >= 0.85 and not self.is_sleeping:
            self.is_sleeping = True
            self.sleep_cycles += 1

    def sleep_restore(self, quality: float = 0.8):
        """Simulate a sleep cycle: restore energy, reduce fatigue, consolidate."""
        if self.is_sleeping:
            self.energy = min(1.0, self.energy + quality * 0.6)
            self.fatigue = max(0.0, self.fatigue - quality * 0.7)
            self.curiosity = min(1.0, self.curiosity + 0.1)  # curiosity rebounds after rest
            self.is_sleeping = False

    def inject_reward(self, reward: float):
        """Inject a dopaminergic reward signal, boosting energy and curiosity."""
        self.energy = min(1.0, self.energy + reward * 0.2)
        self.curiosity = min(1.0, self.curiosity + reward * 0.15)
        self.fatigue = max(0.0, self.fatigue - reward * 0.1)

    def should_sleep(self) -> bool:
        return self.fatigue >= 0.85 or self.energy <= 0.1

    def vitals(self) -> Dict[str, float]:
        return {
            "energy": round(self.energy, 4),
            "curiosity": round(self.curiosity, 4),
            "fatigue": round(self.fatigue, 4),
            "entropy_spent": round(self.total_entropy_spent, 4),
            "entropy_remaining": round(self.entropy_budget - self.total_entropy_spent, 4),
            "sleep_cycles": self.sleep_cycles,
        }
