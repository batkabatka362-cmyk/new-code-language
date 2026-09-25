# Module `torus`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct TorusCoord`

| Field | Type |
|---|---|
| `x` | `u8` |
| `y` | `u8` |
| `z` | `u8` |
| `w` | `u8` |

## ⚡ Functions & Intrinsics

### `fn core_id_to_coord(core_id: u32) -> TorusCoord`

### `fn coord_to_core_id(coord: TorusCoord) -> u32`

### `fn torus_dim_distance(a: u8, b: u8) -> u8`

### `fn manhattan_distance_4d(src: TorusCoord, dst: TorusCoord) -> u32`

### `fn get_torus_neighbor(coord: TorusCoord, axis: u32, positive: bool) -> u32`

### `fn route_next_hop_dor(curr: TorusCoord, dst: TorusCoord) -> u32`

### `fn route_deflect_on_fault(curr: TorusCoord, dst: TorusCoord, faulted_axis: u32) -> u32`

