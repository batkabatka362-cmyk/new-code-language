# Module `kv_cache`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct KvBlockHeader`

| Field | Type |
|---|---|
| `block_id` | `i32` |
| `token_capacity` | `i32` |
| `current_tokens` | `i32` |
| `bank_index` | `i32` |

## ⚡ Functions & Intrinsics

### `fn alloc_kv_block(id: i32, capacity: i32) -> KvBlockHeader`

### `fn append_token_kv(header: KvBlockHeader) -> (KvBlockHeader, i32)`

### `fn main() -> i32`

