# Module `stdp_synapse`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct STDPSynapseState`

| Field | Type |
|---|---|
| `weight` | `f32` |
| `a_plus` | `f32` |
| `a_minus` | `f32` |
| `w_min` | `f32` |
| `w_max` | `f32` |

## ⚡ Functions & Intrinsics

### `fn create_stdp_synapse(init_weight: f32, ltp: f32, ltd: f32) -> STDPSynapseState`

### `fn stdp_compute_dw(syn: STDPSynapseState, delta_t: i32) -> f32`

