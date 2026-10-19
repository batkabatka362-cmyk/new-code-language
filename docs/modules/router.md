# Module `router`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct RouterConfig`

| Field | Type |
|---|---|
| `ports` | `u32` |
| `virtual_channels` | `u32` |
| `buffer_depth` | `u32` |
| `flit_width` | `u32` |

### `struct RouterTelemetry`

| Field | Type |
|---|---|
| `total_cycles` | `u32` |
| `flits_traversed` | `u32` |
| `aggregate_gbps` | `f32` |
| `zero_drop_verified` | `bool` |

## ⚡ Functions & Intrinsics

### `fn init_router(ports: u32, vcs: u32) -> RouterConfig`

### `fn evaluate_router_throughput(cfg: RouterConfig, cycles: u32, flits: u32) -> RouterTelemetry`

