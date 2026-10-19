# Module `transformer`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct TransformerConfig`

| Field | Type |
|---|---|
| `d_model` | `i32` |
| `num_heads` | `i32` |
| `d_k` | `i32` |
| `d_ff` | `i32` |
| `vocab_size` | `i32` |
| `eps` | `f32` |

### `struct MultiHeadAttention`

| Field | Type |
|---|---|
| `w_q` | `vec8f` |
| `w_k` | `vec8f` |
| `w_v` | `vec8f` |
| `w_o` | `vec8f` |
| `scale` | `f32` |

### `struct FeedForward`

| Field | Type |
|---|---|
| `w_gate` | `vec8f` |
| `w_up` | `vec8f` |
| `w_down` | `vec8f` |

### `struct RMSNorm`

| Field | Type |
|---|---|
| `weight` | `vec8f` |
| `eps` | `f32` |

### `struct TransformerBlock`

| Field | Type |
|---|---|
| `norm1` | `RMSNorm` |
| `mha` | `MultiHeadAttention` |
| `norm2` | `RMSNorm` |
| `ffn` | `FeedForward` |

## ⚡ Functions & Intrinsics

### `fn create_transformer_config(d_model: i32, num_heads: i32, d_ff: i32) -> TransformerConfig`

### `fn rmsnorm_forward(x: vec8f, weight: vec8f, eps: f32) -> vec8f`

### `fn swiglu_forward(x: vec8f, w_gate: vec8f, w_up: vec8f, w_down: vec8f) -> vec8f`

### `fn multihead_attention_forward(x: vec8f, mha: MultiHeadAttention) -> vec8f`

### `fn transformer_block_forward(x: vec8f, block: TransformerBlock) -> vec8f`

