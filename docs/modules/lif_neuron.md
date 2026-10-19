# Module `lif_neuron`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct LIFNeuronState`

| Field | Type |
|---|---|
| `v_membrane` | `f32` |
| `threshold` | `f32` |
| `decay_beta` | `f32` |
| `reset_voltage` | `f32` |
| `refractory_timer` | `i32` |
| `refractory_cycles` | `i32` |

## ⚡ Functions & Intrinsics

### `fn create_lif_neuron(decay: f32, thresh: f32) -> LIFNeuronState`

### `fn lif_integrate_spike(neuron: LIFNeuronState, current: f32) -> i32`

