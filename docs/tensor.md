# Module `tensor`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct Tensor`

| Field | Type |
|---|---|
| `data` | `T` |
| `d0` | `i32` |
| `d1` | `i32` |
| `d2` | `i32` |
| `d3` | `i32` |
| `s0` | `i32` |
| `s1` | `i32` |
| `s2` | `i32` |
| `s3` | `i32` |
| `requires_grad` | `bool` |
| `grad_val` | `f32` |

### `struct TensorView`

| Field | Type |
|---|---|
| `data` | `T` |
| `offset` | `i32` |
| `length` | `i32` |
| `stride` | `i32` |

## ⚡ Functions & Intrinsics

### `fn tensor_create(d0: i32, d1: i32, d2: i32, d3: i32, init_val: T, requires_grad: bool) -> Tensor<T>`

### `fn tensor_linear_offset(t: Tensor<T>, i0: i32, i1: i32, i2: i32, i3: i32) -> i32`

### `fn tensor_get_d0(t: Tensor<T>) -> i32`

### `fn tensor_get_d1(t: Tensor<T>) -> i32`

### `fn tensor_get_d2(t: Tensor<T>) -> i32`

### `fn tensor_get_d3(t: Tensor<T>) -> i32`

### `fn tensor_slice(t: Tensor<T>, offset: i32, length: i32, stride: i32) -> TensorView<T>`

### `fn tensor_view_subslice(v: TensorView<T>, sub_offset: i32, sub_len: i32) -> TensorView<T>`

### `fn tensor_reshape(t: Tensor<T>, new_d0: i32, new_d1: i32, new_d2: i32, new_d3: i32) -> Tensor<T>`

### `fn tensor_total_elements(t: Tensor<T>) -> i32`

### `fn tensor_fma_simd(a: vec8f, b: vec8f, c: vec8f) -> vec8f`

### `fn tensor_dot_simd(a: vec8f, b: vec8f) -> f32`

### `fn tensor_gemm_simd(a: vec8f, b: vec8f, bias: f32) -> f32`

### `fn tensor_add_bias_simd(a: vec8f, bias: vec8f) -> vec8f`

### `fn tensor_accumulate_grad(t: Tensor<T>, inc: f32) -> Tensor<T>`

