# Module `vec`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct DynamicVec`

| Field | Type |
|---|---|
| `buffer_base` | `u64` |
| `length` | `u32` |
| `capacity` | `u32` |
| `element_size_bytes` | `u32` |

## ⚡ Functions & Intrinsics

### `fn vec_new(buffer_addr: u64, initial_capacity: u32) -> DynamicVec`

### `fn vec_push_back(vec: linear DynamicVec, value: u32) -> linear DynamicVec`

### `fn vec_pop(vec: linear DynamicVec) -> (linear DynamicVec, u32)`

### `fn vec_clear(vec: linear DynamicVec) -> linear DynamicVec`

### `fn vec_len(vec: DynamicVec) -> u32`

### `fn vec_capacity(vec: DynamicVec) -> u32`

