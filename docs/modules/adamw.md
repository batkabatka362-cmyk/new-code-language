# Module `adamw`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct AdamWConfig`

| Field | Type |
|---|---|
| `lr` | `f32` |
| `beta1` | `f32` |
| `beta2` | `f32` |
| `eps` | `f32` |
| `weight_decay` | `f32` |

### `struct AdamWState`

| Field | Type |
|---|---|
| `m` | `vec8f` |
| `v` | `vec8f` |
| `step` | `i32` |

### `struct ScalarAdamWState`

| Field | Type |
|---|---|
| `m` | `f32` |
| `v` | `f32` |
| `step` | `i32` |

## ⚡ Functions & Intrinsics

### `fn create_adamw_config(lr: f32, weight_decay: f32) -> AdamWConfig`

### `fn create_adamw_state() -> AdamWState`

### `fn create_scalar_adamw_state() -> ScalarAdamWState`

### `fn cosine_annealing_lr(current_step: i32, total_steps: i32, max_lr: f32, min_lr: f32) -> f32`

### `fn adamw_step_simd(param: vec8f, grad_val: vec8f, state: AdamWState, cfg: AdamWConfig) -> vec8f`

### `fn adamw_step_scalar(param: f32, grad_val: f32, state: ScalarAdamWState, cfg: AdamWConfig) -> f32`

