# Module `snn_autograd`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct LIFNeuron`

| Field | Type |
|---|---|
| `u_membrane` | `f32` |
| `u_threshold` | `f32` |
| `v_rest` | `f32` |
| `v_reset` | `f32` |
| `decay_beta` | `f32` |
| `spike_count` | `i32` |

### `struct SNNLayer`

| Field | Type |
|---|---|
| `weights` | `vec8f` |
| `bias` | `f32` |
| `neuron` | `LIFNeuron` |
| `alpha_surrogate` | `f32` |

### `struct SNNTapeEntry`

| Field | Type |
|---|---|
| `u_before_spike` | `f32` |
| `spike_emitted` | `f32` |
| `input_current` | `f32` |

### `struct LIFStepResult`

| Field | Type |
|---|---|
| `neuron` | `LIFNeuron` |
| `spike` | `f32` |

### `struct SNNLayerOutput`

| Field | Type |
|---|---|
| `layer` | `SNNLayer` |
| `spike` | `f32` |
| `tape` | `SNNTapeEntry` |

## ⚡ Functions & Intrinsics

### `fn create_lif_neuron(threshold: f32, decay_beta: f32) -> LIFNeuron`

### `fn fast_sigmoid_surrogate_grad(u: f32, u_th: f32, alpha: f32) -> f32`

### `fn atan_surrogate_grad(u: f32, u_th: f32, alpha: f32) -> f32`

### `fn lif_step_forward(neuron: LIFNeuron, input_current: f32) -> LIFStepResult`

### `fn lif_step_backward(grad_output: f32, u_before_spike: f32, u_th: f32, alpha: f32) -> f32`

### `fn snn_layer_forward(layer: SNNLayer, inputs: vec8f) -> SNNLayerOutput`

### `fn snn_layer_backward(grad_loss: f32, tape: SNNTapeEntry, u_th: f32, alpha: f32, inputs: vec8f) -> vec8f`

