# Module `synapse_routing`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct AerSpikePacket`

| Field | Type |
|---|---|
| `raw_packet` | `u32` |

## ⚡ Functions & Intrinsics

### `fn pack_aer_spike(target_core: u8, synapse_idx: u8, timestamp: u16) -> AerSpikePacket`

### `fn unpack_target_core(packet: AerSpikePacket) -> u8`

### `fn unpack_synapse_index(packet: AerSpikePacket) -> u8`

### `fn unpack_timestamp(packet: AerSpikePacket) -> u16`

### `fn dispatch_aer_spike(spike: linear AerSpikePacket) -> ()`

