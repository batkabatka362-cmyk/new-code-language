# Module `runtime`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct HardToolCatalog`

| Field | Type |
|---|---|
| `tool_torus_distance` | `i32` |
| `tool_ternary_quantize` | `i32` |
| `tool_scratchpad_lookup` | `i32` |
| `tool_fused_fma` | `i32` |

### `struct AgentFrame`

| Field | Type |
|---|---|
| `agent_id` | `i32` |
| `step_count` | `i32` |
| `max_steps` | `i32` |
| `last_tool_id` | `i32` |
| `last_observation` | `f32` |
| `is_halted` | `bool` |

## ⚡ Functions & Intrinsics

### `fn init_tool_catalog() -> HardToolCatalog`

### `fn init_agent_frame(agent_id: i32, max_steps: i32) -> AgentFrame`

### `fn dispatch_hard_tool(tool_id: i32, arg0: f32, arg1: f32) -> f32`

### `fn agent_execute_step(frame: AgentFrame, tool_id: i32, arg0: f32, arg1: f32) -> f32`

