# Module `types`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct wave_t`

| Field | Type |
|---|---|
| `amp` | `[u8; 4]` |
| `phase` | `[u8; 4]` |

### `struct qubit_state_t`

| Field | Type |
|---|---|
| `alpha_real` | `i16` |
| `alpha_imag` | `i16` |
| `beta_real` | `i16` |
| `beta_imag` | `i16` |

### `struct torus_packet_t`

| Field | Type |
|---|---|
| `src_coord` | `topo_coord` |
| `dst_coord` | `topo_coord` |
| `virtual_channel` | `u8` |
| `payload` | `u32` |

### `struct sentry_health_t`

| Field | Type |
|---|---|
| `core_id` | `u32` |
| `temperature_c` | `u16` |
| `parity_error_count` | `u16` |
| `is_throttled` | `bool` |

### `struct topo_tensor`

| Field | Type |
|---|---|
| `coords` | `topo_coord` |
| `data_ref` | `ext_addr_t` |

## ⚡ Functions & Intrinsics

### `fn pack_wave(amp: [u8; 4], phase: [u8; 4]) -> wave_t`

### `fn make_coord(x: u8, y: u8, z: u8, w: u8) -> topo_coord`

### `fn pack_ternary_word(vals: [i8; 16]) -> ternary2`

### `fn make_qubit_ground() -> qubit_state_t`

