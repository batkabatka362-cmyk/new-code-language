# Module `backprop`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct ReversibleLayer`

| Field | Type |
|---|---|
| `layer_id` | `u32` |
| `weight_ref` | `u64` |
| `entropy_loss_bits` | `u32` |

## ⚡ Functions & Intrinsics

### `fn init_reversible_layer(id: u32, weights: u64) -> ReversibleLayer`

### `fn reversible_backward_step(output_state: linear u32, upstream_grad: linear u32, layer: ReversibleLayer) -> (linear u32, linear u32)`

