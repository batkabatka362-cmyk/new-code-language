// CRON Standard Library - 0-Cycle Region Arena Allocator
// Module: cron.core.arena
// Target: 256-Core 4D-Torus Photonic Neuromorphic Processor

.MODULE cron.core.arena

struct RegionArena {
    base_addr: u64,
    capacity_bytes: u32,
    current_offset: u32,
    allocated_chunks: u32
}

def create_arena(base: u64, capacity: u32) -> RegionArena {
    return RegionArena {
        base_addr: base,
        capacity_bytes: capacity,
        current_offset: 0,
        allocated_chunks: 0
    }
}

// 0-Cycle Regional Allocation: Advances offset pointer without OS malloc/free overhead
def arena_alloc(lin arena: linear RegionArena, size_bytes: u32) -> (linear RegionArena, u64) {
    let offset = arena.current_offset
    let new_offset = offset + size_bytes
    let ptr = arena.base_addr + (offset as u64)

    let updated_arena = RegionArena {
        base_addr: arena.base_addr,
        capacity_bytes: arena.capacity_bytes,
        current_offset: new_offset,
        allocated_chunks: arena.allocated_chunks + 1
    }
    consume(arena)
    return (updated_arena, ptr)
}

// 0-Cycle Instant Arena Reset (Maps directly to machine opcode _RS)
def arena_reset(lin arena: linear RegionArena) -> linear RegionArena {
    let reset_arena = RegionArena {
        base_addr: arena.base_addr,
        capacity_bytes: arena.capacity_bytes,
        current_offset: 0,
        allocated_chunks: 0
    }
    consume(arena)
    return reset_arena
}

// Linear arena destructor: requires arena to be fully reset before dropping
def drop_arena(lin arena: linear RegionArena) {
    consume(arena)
}
