# Module `hyper_tree`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct HyperTreeNode`

| Field | Type |
|---|---|
| `core_id` | `u32` |
| `depth` | `u8` |
| `branch_mask` | `u16` |
| `stride_hash` | `u32` |

## ⚡ Functions & Intrinsics

### `fn create_hyper_node(id: u32, tree_depth: u8, mask: u16) -> HyperTreeNode`

### `fn hyper_tree_traverse_step(node: HyperTreeNode, branch_idx: u8) -> u32`

