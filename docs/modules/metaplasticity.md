# Module `metaplasticity`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct NeuromodulatorPool`

| Field | Type |
|---|---|
| `dopamine` | `i32` |
| `serotonin` | `i32` |
| `acetylcholine` | `i32` |
| `noradrenaline` | `i32` |

## ⚡ Functions & Intrinsics

### `fn create_neuromodulator_pool(da: i32, ser: i32, ach: i32, ne: i32) -> NeuromodulatorPool`

### `fn step_bcm_weight(pre_activity: i32, post_activity: i32, theta_m: i32, learning_rate: i32) -> i32`

