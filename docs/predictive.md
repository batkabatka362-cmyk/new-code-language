# Module `predictive`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct DendriticUnit`

| Field | Type |
|---|---|
| `neuron_id` | `i64` |
| `somatic_state` | `i64` |
| `basal_input` | `i64` |
| `apical_prediction` | `i64` |
| `prediction_error` | `i64` |

## ⚡ Functions & Intrinsics

### `fn create_dendritic_unit(id: i64, basal: i64, apical: i64) -> DendriticUnit`

### `fn evaluate_local_error(unit: DendriticUnit) -> i64`

### `fn step_predictive_synapse(weight: i64, error: i64, higher_state: i64, eta_q8: i64) -> i64`

