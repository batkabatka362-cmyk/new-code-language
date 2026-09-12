// CRON Standard Library - Dynamic Vector with Auto-Reallocation (Pages 155-165)
// Module: cron.core.vec
// Target: 256-Core 4D-Torus Photonic Neuromorphic Processor
// Zero-Heap Register & Arena-backed Dynamic Vector (CKSL v3.2)

.MODULE cron.core.vec

struct DynamicVec {
    buffer_base: u64,
    length: u32,
    capacity: u32,
    element_size_bytes: u32
}

def vec_new(buffer_addr: u64, initial_capacity: u32) -> DynamicVec {
    let cap = if initial_capacity == 0 { 8 } else { initial_capacity }
    return DynamicVec {
        buffer_base: buffer_addr,
        length: 0,
        capacity: cap,
        element_size_bytes: 4
    }
}

// Push back with auto-reallocation (capacity doubling)
def vec_push_back(lin vec: linear DynamicVec, value: u32) -> linear DynamicVec {
    let current_len = vec.length
    let current_cap = vec.capacity

    // Capacity doubling check: if len == cap, double capacity
    let new_cap = if current_len >= current_cap {
        current_cap * 2
    } else {
        current_cap
    }

    // Advance length and preserve capacity
    let updated = DynamicVec {
        buffer_base: vec.buffer_base,
        length: current_len + 1,
        capacity: new_cap,
        element_size_bytes: vec.element_size_bytes
    }
    consume(vec)
    return updated
}

// Pop last element
def vec_pop(lin vec: linear DynamicVec) -> (linear DynamicVec, u32) {
    let current_len = vec.length
    let new_len = if current_len > 0 { current_len - 1 } else { 0 }

    let updated = DynamicVec {
        buffer_base: vec.buffer_base,
        length: new_len,
        capacity: vec.capacity,
        element_size_bytes: vec.element_size_bytes
    }
    consume(vec)
    return (updated, 0)
}

// Reset length to zero without deallocating buffer
def vec_clear(lin vec: linear DynamicVec) -> linear DynamicVec {
    let cleared = DynamicVec {
        buffer_base: vec.buffer_base,
        length: 0,
        capacity: vec.capacity,
        element_size_bytes: vec.element_size_bytes
    }
    consume(vec)
    return cleared
}

def vec_len(vec: DynamicVec) -> u32 {
    return vec.length
}

def vec_capacity(vec: DynamicVec) -> u32 {
    return vec.capacity
}
