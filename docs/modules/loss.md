# Module `loss`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## ⚡ Functions & Intrinsics

### `fn cross_entropy_loss_scalar(logit_pred: f32, logit_other: f32, target_is_pred: bool) -> f32`

### `fn cross_entropy_backward_scalar(logit_pred: f32, logit_other: f32, target_is_pred: bool) -> f32`

### `fn simd_cross_entropy_loss(logits: vec8f, target_idx: i32) -> f32`

### `fn label_smoothed_cross_entropy(loss_hard: f32, num_classes: i32, smoothing: f32) -> f32`

