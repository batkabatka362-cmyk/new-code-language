# Module `attention`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct FlashAttentionConfig`

| Field | Type |
|---|---|
| `seq_len` | `i32` |
| `num_heads` | `i32` |
| `head_dim` | `i32` |
| `is_causal` | `bool` |

### `struct AttentionOutput`

| Field | Type |
|---|---|
| `context` | `wave_t` |
| `entropy` | `f64` |
| `peak_phase` | `i32` |

## ⚡ Functions & Intrinsics

### `fn init_flash_attention_config(seq_len: i32, num_heads: i32, head_dim: i32, is_causal: bool) -> FlashAttentionConfig`

### `fn photonic_qkv_gemm(q: wave_t, k: wave_t, v: wave_t, phase_shift: i32) -> wave_t`

### `fn photonic_flash_attention_forward(query: wave_t, key: wave_t, value: wave_t, config: FlashAttentionConfig) -> AttentionOutput`

