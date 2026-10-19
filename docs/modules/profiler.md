# Module `profiler`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct PpaProfileConfig`

| Field | Type |
|---|---|
| `clock_ghz` | `f32` |
| `supply_voltage_v` | `f32` |
| `target_cores` | `u32` |

### `struct KernelPpaSummary`

| Field | Type |
|---|---|
| `ipc` | `f32` |
| `chip_tflops` | `f32` |
| `tops_per_watt` | `f32` |
| `junction_temp_c` | `f32` |

## ⚡ Functions & Intrinsics

### `fn init_ppa_profiler(freq: f32, voltage: f32) -> PpaProfileConfig`

### `fn evaluate_kernel_ppa(config: PpaProfileConfig, cycles: u32, flops: f32) -> KernelPpaSummary`

