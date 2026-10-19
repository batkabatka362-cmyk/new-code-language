# Module `channel`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct Channel`

| Field | Type |
|---|---|
| `id` | `i64` |
| `capacity` | `i64` |

**Methods:**

- `fn new(capacity: i64) -> Channel<T>`
- `fn send(self: Self, val: T) -> i64`
- `fn recv(self: Self) -> i64`
- `fn try_recv(self: Self) -> i64`
- `fn close(self: Self) -> i64`

## ⚡ Functions & Intrinsics

### `fn channel_create(cap: i64) -> Channel<T>`

### `fn channel_shutdown(id: i64) -> i64`

