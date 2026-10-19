# Module `mcts_quantum`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct QuantumBranchNode`

| Field | Type |
|---|---|
| `action_id` | `u32` |
| `amplitude_real` | `i16` |
| `amplitude_imag` | `i16` |
| `visit_count` | `u32` |
| `accumulated_reward` | `i32` |

### `struct SuperpositionPlanTree`

| Field | Type |
|---|---|
| `branches` | `[QuantumBranchNode; 8]` |
| `branch_count` | `u32` |
| `coherence_time_cycles` | `u32` |

## ⚡ Functions & Intrinsics

### `fn init_superposition_tree(coherence_cycles: u32) -> SuperpositionPlanTree`

### `fn branch_in_superposition(tree: linear SuperpositionPlanTree, action: u32, amp_r: i16, amp_i: i16) -> linear SuperpositionPlanTree`

### `fn collapse_optimal_decision(tree: linear SuperpositionPlanTree) -> u32`

