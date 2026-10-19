# Module `tensor4d`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct TorusCoord`

| Field | Type |
|---|---|
| `x` | `u8` |
| `y` | `u8` |
| `z` | `u8` |
| `w` | `u8` |

### `struct Tensor4D`

| Field | Type |
|---|---|
| `data` | `T` |
| `dim0` | `i32` |
| `dim1` | `i32` |
| `dim2` | `i32` |
| `dim3` | `i32` |
| `stride0` | `i32` |
| `stride1` | `i32` |
| `stride2` | `i32` |
| `stride3` | `i32` |

## ⚡ Functions & Intrinsics

### `fn tensor_stride_offset(dim0: i32, dim1: i32, dim2: i32, dim3: i32, i0: i32, i1: i32, i2: i32, i3: i32) -> i32`

### `fn create_tensor4d(dim0: i32, dim1: i32, dim2: i32, dim3: i32, init_val: T) -> Tensor4D<T>`

### `fn tensor_to_torus_coord(t: Tensor4D<T>, i0: i32, i1: i32, i2: i32, i3: i32) -> TorusCoord`

### `fn tensor4d_total_elements(t: Tensor4D<T>) -> i32`

