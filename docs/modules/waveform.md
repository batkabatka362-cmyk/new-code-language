# Module `waveform`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct VcdProbeConfig`

| Field | Type |
|---|---|
| `timescale_ns` | `u32` |
| `clock_period_ns` | `u32` |
| `sample_rate_mhz` | `u32` |

### `struct SignalProbe`

| Field | Type |
|---|---|
| `name` | `string` |
| `width_bits` | `u32` |
| `current_value` | `u32` |

## ⚡ Functions & Intrinsics

### `fn init_vcd_probe(clock_mhz: u32) -> VcdProbeConfig`

### `fn probe_signal(probe: SignalProbe, new_val: u32) -> SignalProbe`

