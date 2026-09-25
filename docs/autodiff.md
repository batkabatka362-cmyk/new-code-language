# Module `autodiff`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## ⚡ Functions & Intrinsics

### `fn relu(x: f32) -> f32`

### `fn sigmoid(x: f32) -> f32`

### `fn mse_loss(pred: f32, target: f32) -> f32`

### `fn vector_mse_loss(pred: vec8f, target: vec8f) -> f32`

### `fn linear_neuron_forward(weights: vec8f, inputs: vec8f, bias: f32) -> f32`

### `fn sgd_step(param: f32, grad_val: f32, lr: f32) -> f32`

### `fn simd_sgd_step(param: vec8f, grad_val: vec8f, lr: f32) -> vec8f`

