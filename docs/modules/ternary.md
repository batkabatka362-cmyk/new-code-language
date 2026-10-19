# Module `ternary`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct PackedTrit128`

| Field | Type |
|---|---|
| `low_bits` | `i64` |
| `high_bits` | `i64` |

## ⚡ Functions & Intrinsics

### `fn create_packed_trits(low: i64, high: i64) -> PackedTrit128`

### `fn trit_get_polarity(val: i64) -> i64`

### `fn trit_zero_mul_accumulate(trit_sign: i64, activation: i64, current_acc: i64) -> i64`

