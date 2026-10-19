# Module `linear`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct LinearGuard`

| Field | Type |
|---|---|
| `resource` | `T` |
| `consumed` | `bool` |

## ⚡ Functions & Intrinsics

### `fn wrap_linear(val: T) -> linear LinearGuard<T>`

### `fn unwrap_linear(guard: linear LinearGuard<T>) -> T`

### `fn linear_swap(a: linear LinearGuard<T>, b: linear LinearGuard<T>) -> (linear LinearGuard<T>, linear LinearGuard<T>)`

