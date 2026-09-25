# Module `muon`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct MuonConfig`

| Field | Type |
|---|---|
| `lr` | `f32` |
| `momentum` | `f32` |
| `nesterov` | `bool` |
| `ns_steps` | `i32` |
| `weight_decay` | `f32` |

### `struct MuonState`

| Field | Type |
|---|---|
| `buf` | `vec8f` |
| `step` | `i32` |

## ⚡ Functions & Intrinsics

### `fn create_muon_config(lr: f32, weight_decay: f32) -> MuonConfig`

### `fn create_muon_state() -> MuonState`

### `fn newton_schulz_step_simd(g: vec8f) -> vec8f`

### `fn muon_step_simd(param: vec8f, grad_val: vec8f, state: MuonState, cfg: MuonConfig) -> vec8f`

