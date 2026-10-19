# Module `photonic_autodiff`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct OpticalAutodiffTap`

| Field | Type |
|---|---|
| `tap_ratio_percent` | `u8` |
| `polarization_angle` | `u16` |
| `spectral_bandwidth` | `u32` |

## ⚡ Functions & Intrinsics

### `fn init_autodiff_tap(tap_ratio: u8) -> OpticalAutodiffTap`

### `fn tap_forward_gradient(wave: linear wave_t, tap: OpticalAutodiffTap) -> (linear wave_t, linear wave_t)`

