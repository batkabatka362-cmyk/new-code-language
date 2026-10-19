# Module `bpe`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct BpeVocabConfig`

| Field | Type |
|---|---|
| `vocab_size` | `i32` |
| `base_byte_count` | `i32` |
| `special_token_start` | `i32` |
| `end_of_text_id` | `i32` |

## ⚡ Functions & Intrinsics

### `fn init_bpe_config(vocab_size: i32) -> BpeVocabConfig`

### `fn is_raw_byte_token(token_id: i32) -> bool`

### `fn is_special_token(cfg: BpeVocabConfig, token_id: i32) -> bool`

### `fn bpe_merge_pair(token_a: i32, token_b: i32, vocab_limit: i32) -> i32`

### `fn bpe_encode_4bytes(b0: i32, b1: i32, b2: i32, b3: i32, vocab_size: i32) -> i32`

### `fn bpe_decode_to_lead_byte(token_id: i32) -> i32`

