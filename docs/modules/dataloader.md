# Module `dataloader`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct TokenBatch`

| Field | Type |
|---|---|
| `inputs` | `vec8f` |
| `targets` | `vec8f` |
| `seq_len` | `i32` |
| `batch_idx` | `i32` |

### `struct BinaryTokenDataset`

| Field | Type |
|---|---|
| `total_tokens` | `i32` |
| `current_offset` | `i32` |
| `seq_len` | `i32` |
| `is_exhausted` | `bool` |

## ⚡ Functions & Intrinsics

### `fn dataset_open(total_tokens: i32, seq_len: i32) -> BinaryTokenDataset`

### `fn dataset_next_batch(dataset: BinaryTokenDataset, step: i32) -> TokenBatch`

### `fn dataset_reset(dataset: BinaryTokenDataset) -> BinaryTokenDataset`

