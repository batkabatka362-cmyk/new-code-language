# Module `dvs_stream`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct DvsPixelEvent`

| Field | Type |
|---|---|
| `x` | `i32` |
| `y` | `i32` |
| `timestamp_us` | `i64` |
| `polarity` | `i32` |

## ⚡ Functions & Intrinsics

### `fn create_dvs_event(x: i32, y: i32, ts: i64, pol: i32) -> DvsPixelEvent`

### `fn accumulate_event_surface(evt: DvsPixelEvent, decay_rate: f32) -> f32`

### `fn dispatch_dvs_spike_to_cortex(evt: DvsPixelEvent, target_core: i32) -> i32`

