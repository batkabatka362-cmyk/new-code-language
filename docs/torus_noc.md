# Module `torus_noc`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct Torus4DCoordinate`

| Field | Type |
|---|---|
| `x` | `u32` |
| `y` | `u32` |
| `z` | `u32` |
| `w` | `u32` |

## ⚡ Functions & Intrinsics

### `fn create_torus_coord(x: u32, y: u32, z: u32, w: u32) -> Torus4DCoordinate`

### `fn compute_manhattan_distance_4d(src: Torus4DCoordinate, dst: Torus4DCoordinate) -> u32`

