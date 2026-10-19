# Module `micro_circuit`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct NeuromodulatorCocktail`

| Field | Type |
|---|---|
| `dopamine` | `i32` |
| `acetylcholine` | `i32` |
| `serotonin` | `i32` |
| `noradrenaline` | `i32` |

### `struct CorticalLayerState`

| Field | Type |
|---|---|
| `layer_id` | `i32` |
| `excitatory_count` | `i32` |
| `inhibitory_count` | `i32` |
| `membrane_tau_us` | `i32` |
| `spike_threshold_mv` | `i32` |
| `resting_potential_mv` | `i32` |
| `current_voltage_mv` | `i32` |
| `e_i_ratio_scaled` | `i32` |

### `struct CorticalColumn`

| Field | Type |
|---|---|
| `column_id` | `i32` |
| `core_id` | `i32` |
| `torus_x` | `i32` |
| `torus_y` | `i32` |
| `torus_z` | `i32` |
| `torus_w` | `i32` |
| `neuromodulators` | `NeuromodulatorCocktail` |
| `layer_l23` | `CorticalLayerState` |
| `layer_l4` | `CorticalLayerState` |
| `layer_l5` | `CorticalLayerState` |
| `layer_l6` | `CorticalLayerState` |
| `is_active` | `bool` |

## ⚡ Functions & Intrinsics

### `fn create_default_neuromodulators() -> NeuromodulatorCocktail`

### `fn create_layer_spec(layer_id: i32, exc: i32, inh: i32, tau_us: i32, threshold_mv: i32) -> CorticalLayerState`

### `fn initialize_cortical_column(column_id: i32, core_id: i32, tx: i32, ty: i32, tz: i32, tw: i32) -> CorticalColumn`

### `fn inject_sensory_spike_to_layer4(column: CorticalColumn, spike_magnitude_mv: i32) -> CorticalColumn`

### `fn modulate_dopamine_level(column: CorticalColumn, delta_dopamine: i32) -> CorticalColumn`

