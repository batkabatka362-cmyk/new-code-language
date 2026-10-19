# Module `arena_server`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct StringView`

| Field | Type |
|---|---|
| `ptr` | `u64` |
| `len` | `u32` |

### `struct HttpRequest`

| Field | Type |
|---|---|
| `method` | `StringView` |
| `path` | `StringView` |
| `content_length` | `u32` |
| `body` | `StringView` |

### `struct HttpResponse`

| Field | Type |
|---|---|
| `status_code` | `i32` |
| `content_type` | `StringView` |
| `body` | `StringView` |

### `struct BackendWorker`

| Field | Type |
|---|---|
| `worker_id` | `i32` |
| `request_counter` | `u64` |
| `arena_base` | `u64` |
| `arena_capacity` | `u32` |

## ⚡ Functions & Intrinsics

### `fn create_backend_worker(id: i32, arena_base: u64, capacity: u32) -> BackendWorker`

### `fn parse_method_fast(buf_ptr: u64, len: u32) -> StringView`

### `fn handle_request_zero_alloc(arena: linear RegionArena, raw_req_ptr: u64, raw_req_len: u32) -> (linear RegionArena, HttpResponse)`

### `fn process_transaction(arena: linear RegionArena, req_ptr: u64, req_len: u32) -> (linear RegionArena, i32)`

