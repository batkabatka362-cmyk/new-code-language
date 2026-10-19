# Module `stdp`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct STDPLayer`

| Field | Type |
|---|---|
| `num_pre` | `i32` |
| `num_post` | `i32` |
| `a_plus` | `f64` |
| `a_minus` | `f64` |
| `tau_plus` | `f64` |
| `tau_minus` | `f64` |
| `learning_rate` | `f64` |

### `struct SynapseTrace`

| Field | Type |
|---|---|
| `pre_trace` | `f64` |
| `post_trace` | `f64` |
| `current_weight` | `f64` |

## ⚡ Functions & Intrinsics

### `fn create_stdp_layer(num_pre: i32, num_post: i32, lr: f64) -> STDPLayer`

### `fn stdp_compute_weight_delta(layer: STDPLayer, delta_t_ms: f64) -> f64`

### `fn stdp_step(layer: STDPLayer, synapse: SynapseTrace, delta_t_ms: f64) -> SynapseTrace`

