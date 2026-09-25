# Module `sentry`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct SentryConfig`

| Field | Type |
|---|---|
| `max_thermal_thresh_c` | `u32` |
| `warning_thresh_c` | `u32` |
| `fallback_axis` | `u32` |
| `self_heal_enabled` | `bool` |

## ⚡ Functions & Intrinsics

### `fn init_sentry_daemon(threshold_c: u32, fallback: u32) -> SentryConfig`

### `fn evaluate_thermal_safety(config: SentryConfig, current_temp_c: u32) -> (bool, u32)`

### `fn remap_faulty_core(faulty_id: u32, backup_id: u32) -> u32`

