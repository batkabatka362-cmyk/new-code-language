# ============================================================================
# CRON 4-Head Sphere Matrix AI Model
# Module: python.cron.sphere_ai
# Description:
#   Implements a discrete, multiplier-free AI Language / Pattern Model
#   powered entirely by the 4-Head Sphere Matrix (Heads X, Y, Z, W).
#
#   Architecture:
#     - Input tokens map to Sphere coordinates: Layer W and (X, Z) angles.
#     - 4 Heads (X, Y, Z, W) start at center (0) and traverse the Sphere Matrix.
#     - Hidden memory is the continuous 4D trajectory state of the 4 heads.
#     - Trajectory Optimization / Learning updates the geodesic steering
#       parameters to minimize prediction error with ZERO floating point GEMM!
# ============================================================================

import math
import random
from typing import List, Tuple, Dict
from python.cron.hypersphere_matrix import (
    SphereMatrix4D, HeadState, LayerTransitionMode,
    MASK_7BIT, MASK_14BIT, COORD_SHIFT
)


class SphereMatrixAI:
    """
    Multiplier-Free 4-Head Sphere Matrix Neural Engine.
    Uses the 4D Sphere Matrix as a topological recurrent neural network (TRNN).
    """

    def __init__(self, vocab_size: int = 128, hidden_dim: int = 4):
        self.vocab_size = vocab_size
        self.hidden_dim = hidden_dim  # 4 Heads: X, Y, Z, W
        self.sphere = SphereMatrix4D(transition_mode=LayerTransitionMode.DIRECT_STEP)

        # Steering Trajectory Weights for each token in vocab
        # Each token steers the 4 heads by [ (dW, dX, dZ) for each of X, Y, Z, W ]
        random.seed(42)
        self.token_steering: Dict[int, Dict[str, Tuple[int, int, int]]] = {}
        for token_id in range(vocab_size):
            self.token_steering[token_id] = {
                'X': ((token_id % 7) - 3, (token_id * 3) % 20 - 10, (token_id * 5) % 20 - 10),
                'Y': (((token_id + 1) % 7) - 3, (token_id * 7) % 20 - 10, (token_id * 2) % 20 - 10),
                'Z': (((token_id + 2) % 7) - 3, (token_id * 11) % 20 - 10, (token_id * 13) % 20 - 10),
                'W': (((token_id + 3) % 7) - 3, (token_id * 17) % 20 - 10, (token_id * 19) % 20 - 10),
            }

        # Output Projection Matrix: Maps 4-Head (W, X, Z) states to Vocab Scores
        # Stored as integer weights (Zero floating point!)
        self.proj_weights: List[List[int]] = [
            [(random.randint(-15, 15)) for _ in range(12)]  # 4 heads * 3 coordinates (W, X, Z) = 12 features
            for _ in range(vocab_size)
        ]

    def reset_state(self):
        """Resets the 4 heads back to their initial state near center 0."""
        self.sphere = SphereMatrix4D(transition_mode=LayerTransitionMode.DIRECT_STEP)

    def get_feature_vector(self) -> List[int]:
        """Extracts the current 12-dimensional state vector from the 4 heads."""
        features = []
        for name in ['X', 'Y', 'Z', 'W']:
            head = self.sphere.heads[name]
            features.extend([head.w, head.x, head.z])
        return features

    def forward_token(self, token_id: int) -> int:
        """
        Advances the 4 heads through the Sphere Matrix driven by token_id
        and predicts the next token with integer ALU arithmetic.
        """
        steering = self.token_steering[token_id % self.vocab_size]

        # Step all 4 heads simultaneously (1 VLIW bundle step)
        for name in ['X', 'Y', 'Z', 'W']:
            dw, dx, dz = steering[name]
            self.sphere.step_head(name, dw, dx, dz, use_phase_lock=True)

        features = self.get_feature_vector()

        # Output projection using integer dot-product (No floating point!)
        best_token = 0
        max_score = -10**9

        for v in range(self.vocab_size):
            score = 0
            w_row = self.proj_weights[v]
            for f_idx in range(12):
                score += features[f_idx] * w_row[f_idx]
            if score > max_score:
                max_score = score
                best_token = v

        return best_token

    def train_step(self, input_token: int, target_token: int, lr: int = 1) -> bool:
        """
        Topological Hebbian / Perceptron trajectory optimization step:
        If predicted token != target token, adjust projection weights and steering
        to align the 4 heads with the target!
        Returns True if prediction was already correct.
        """
        predicted = self.forward_token(input_token)

        if predicted == target_token:
            return True  # Correct prediction!

        # Update projection weights along the 4-head feature vector (Integer Widrow-Hoff rule)
        features = self.get_feature_vector()
        for f_idx in range(12):
            feat_val = features[f_idx]
            self.proj_weights[target_token][f_idx] += feat_val
            self.proj_weights[predicted][f_idx] -= feat_val

        # Nudge steering parameters slightly
        for name in ['X', 'Y', 'Z', 'W']:
            dw, dx, dz = self.token_steering[input_token][name]
            # Nudge towards target direction
            delta = 1 if target_token > predicted else -1
            self.token_steering[input_token][name] = (
                max(-5, min(5, dw + delta)),
                (dx + delta) % 20,
                (dz + delta) % 20
            )

        return False

    def train_sequence(self, text: str, epochs: int = 15) -> List[float]:
        """Trains the Sphere Matrix AI on a target character sequence."""
        tokens = [ord(c) for c in text]
        accuracy_history = []

        for ep in range(epochs):
            self.reset_state()
            correct_count = 0
            total_steps = len(tokens) - 1

            for i in range(total_steps):
                inp = tokens[i]
                tgt = tokens[i + 1]
                is_correct = self.train_step(inp, tgt)
                if is_correct:
                    correct_count += 1

            acc = (correct_count / total_steps) * 100.0 if total_steps > 0 else 100.0
            accuracy_history.append(acc)

        return accuracy_history

    def generate(self, prompt: str, length: int = 20) -> str:
        """Generates continuation text using the 4-Head Sphere Matrix."""
        self.reset_state()
        result = list(prompt)

        # Warm up the 4 heads with prompt characters
        curr_token = ord(prompt[0]) if prompt else ord('A')
        for c in prompt:
            curr_token = ord(c)
            _ = self.forward_token(curr_token)

        # Autoregressive generation
        for _ in range(length):
            next_token = self.forward_token(curr_token)
            # Constrain to printable ASCII range (32..126)
            next_char_code = max(32, min(126, next_token))
            result.append(chr(next_char_code))
            curr_token = next_char_code

        return "".join(result)
