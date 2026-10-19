# Module `thermodynamics`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct ThermalState`

| Field | Type |
|---|---|
| `temperature_kelvin` | `f32` |
| `entropy_s` | `f32` |
| `free_energy_f` | `f32` |
| `is_quenched` | `bool` |

## ⚡ Functions & Intrinsics

### `fn create_thermal_state(temp_k: f32) -> ThermalState`

### `fn evaluate_quench(state: ThermalState, activity_flux: f32, threshold: f32) -> bool`

