# Module `clifford4d`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct Multivector16`

| Field | Type |
|---|---|
| `s` | `f32` |
| `e1` | `f32` |
| `e2` | `f32` |
| `e12` | `f32` |
| `e3` | `f32` |
| `e13` | `f32` |
| `e23` | `f32` |
| `e123` | `f32` |
| `e4` | `f32` |
| `e14` | `f32` |
| `e24` | `f32` |
| `e124` | `f32` |
| `e34` | `f32` |
| `e134` | `f32` |
| `e234` | `f32` |
| `e1234` | `f32` |

### `struct Rotor4D`

| Field | Type |
|---|---|
| `s` | `f32` |
| `e12` | `f32` |
| `e13` | `f32` |
| `e14` | `f32` |
| `e23` | `f32` |
| `e24` | `f32` |
| `e34` | `f32` |
| `p` | `f32` |

### `struct Vector4D`

| Field | Type |
|---|---|
| `x` | `f32` |
| `y` | `f32` |
| `z` | `f32` |
| `w` | `f32` |

## ⚡ Functions & Intrinsics

### `fn blade_index(a: i32, b: i32) -> i32`

### `fn prefix_xor(b: i32) -> i32`

### `fn blade_sign(a: i32, b: i32) -> f32`

### `fn rotor_identity() -> Rotor4D`

### `fn rotor_plane12(half_cos: f32, half_sin: f32) -> Rotor4D`

### `fn rotor_rotate_vector(r: Rotor4D, v: Vector4D) -> Vector4D`

### `fn main() -> i32`

