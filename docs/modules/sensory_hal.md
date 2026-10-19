# Module `sensory_hal`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct DvsEventPacket`

| Field | Type |
|---|---|
| `pixel_x` | `i64` |
| `pixel_y` | `i64` |
| `polarity` | `i64` |
| `timestamp_us` | `i64` |

### `struct CochleaAudioSpike`

| Field | Type |
|---|---|
| `channel_idx` | `i64` |
| `intensity` | `i64` |

## ⚡ Functions & Intrinsics

### `fn create_dvs_event(x: i64, y: i64, pol: i64, t: i64) -> DvsEventPacket`

### `fn ingest_dvs_event(event: DvsEventPacket) -> i64`

### `fn create_cochlea_spike(ch: i64, amp: i64) -> CochleaAudioSpike`

### `fn ingest_cochlea_spike(spike: CochleaAudioSpike) -> i64`

