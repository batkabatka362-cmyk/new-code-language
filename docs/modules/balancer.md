# Module `balancer`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct MeshPartitionConfig`

| Field | Type |
|---|---|
| `total_cores` | `i32` |
| `tensor_m` | `i32` |
| `tensor_k` | `i32` |
| `tensor_n` | `i32` |
| `layers` | `i32` |
| `tp` | `i32` |
| `pp` | `i32` |
| `dp` | `i32` |

### `struct SpatialWorkloadPlan`

| Field | Type |
|---|---|
| `total_cores` | `i32` |
| `bubble_fraction` | `f32` |
| `imbalance_ratio` | `f32` |
| `estimated_speedup` | `f32` |

## ⚡ Functions & Intrinsics

### `fn evaluate_partition_plan(cfg: MeshPartitionConfig) -> SpatialWorkloadPlan`

