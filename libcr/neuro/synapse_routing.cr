// CRON Standard Library - Event-Driven AER Spike Packet Routing
// Module: cron.neuro.synapse_routing
// Brain 4: Address-Event Representation (AER) Multicast over 4D Torus NoC

.MODULE cron.neuro.synapse_routing

// 32-bit Hardware AER Spike Packet:
// [31..24] Target Core ID (0..255)
// [23..16] Dendrite / Synapse Index (0..255)
// [15..0]  Spike Timestamp (16-bit hardware clock tick)
struct AerSpikePacket {
    raw_packet: u32
}

def pack_aer_spike(target_core: u8, synapse_idx: u8, timestamp: u16) -> AerSpikePacket {
    let raw = ((target_core as u32) << 24) | ((synapse_idx as u32) << 16) | (timestamp as u32)
    return AerSpikePacket { raw_packet: raw }
}

def unpack_target_core(packet: AerSpikePacket) -> u8 {
    return ((packet.raw_packet >> 24) & 0xFF) as u8
}

def unpack_synapse_index(packet: AerSpikePacket) -> u8 {
    return ((packet.raw_packet >> 16) & 0xFF) as u8
}

def unpack_timestamp(packet: AerSpikePacket) -> u16 {
    return (packet.raw_packet & 0xFFFF) as u16
}

// Multicast neuromorphic spike event across nearest neighbor 4D torus dimensions
def dispatch_aer_spike(lin spike: linear AerSpikePacket) {
    spatial_broadcast(consume(spike.raw_packet))
}
