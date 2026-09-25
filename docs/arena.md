# Module `arena`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct RegionArena`

| Field | Type |
|---|---|
| `base_addr` | `u64` |
| `capacity_bytes` | `u32` |
| `current_offset` | `u32` |
| `allocated_chunks` | `u32` |

## ⚡ Functions & Intrinsics

### `fn create_arena(base: u64, capacity: u32) -> RegionArena`

### `fn arena_alloc(arena: linear RegionArena, size_bytes: u32) -> (linear RegionArena, u64)`

### `fn arena_reset(arena: linear RegionArena) -> linear RegionArena`

### `fn drop_arena(arena: linear RegionArena) -> ()`

