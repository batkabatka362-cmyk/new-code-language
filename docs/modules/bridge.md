# Module `bridge`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct SagiMailboxFrame`

| Field | Type |
|---|---|
| `source_brain` | `u32` |
| `target_brain` | `u32` |
| `command_opcode` | `u32` |
| `payload_0` | `i32` |
| `payload_1` | `i32` |
| `payload_2` | `i32` |
| `payload_3` | `i32` |
| `sequence_id` | `u64` |
| `acknowledged` | `bool` |

## ⚡ Functions & Intrinsics

### `fn create_sagi_mailbox(src: u32, dst: u32, opcode: u32, p0: i32, p1: i32, p2: i32, p3: i32, seq: u64) -> SagiMailboxFrame`

### `fn dispatch_sagi_directive(frame: SagiMailboxFrame) -> bool`

