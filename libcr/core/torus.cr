// CRON Standard Library - 4D Torus Topology & Mesh Routing
// Module: cron.core.torus
// Target Architecture: 256-Core ($4 \times 4 \times 4 \times 4$) 4D-Torus Hardware Mesh

.MODULE cron.core.torus

struct TorusCoord {
    x: u8,
    y: u8,
    z: u8,
    w: u8
}

// Convert linear core ID (0..255) to 4D Torus Coordinate (x, y, z, w in 0..3)
def core_id_to_coord(core_id: u32) -> TorusCoord {
    let w = ((core_id / 64) % 4) as u8
    let z = ((core_id / 16) % 4) as u8
    let y = ((core_id / 4) % 4) as u8
    let x = (core_id % 4) as u8
    return TorusCoord { x: x, y: y, z: z, w: w }
}

// Convert 4D Torus Coordinate to linear core ID (0..255)
def coord_to_core_id(coord: TorusCoord) -> u32 {
    return ((coord.w as u32) * 64) + ((coord.z as u32) * 16) + ((coord.y as u32) * 4) + (coord.x as u32)
}

// Toroidal wrap distance along single dimension (mod 4: max distance is 2)
def torus_dim_distance(a: u8, b: u8) -> u8 {
    let diff = if a >= b { a - b } else { b - a }
    return if diff > 2 { 4 - diff } else { diff }
}

// 4D Manhattan hop distance between any two cores on the torus
def manhattan_distance_4d(src: TorusCoord, dst: TorusCoord) -> u32 {
    let dx = torus_dim_distance(src.x, dst.x) as u32
    let dy = torus_dim_distance(src.y, dst.y) as u32
    let dz = torus_dim_distance(src.z, dst.z) as u32
    let dw = torus_dim_distance(src.w, dst.w) as u32
    return dx + dy + dz + dw
}

// Compute next neighbor core ID along specified axis direction (+-X, +-Y, +-Z, +-W)
def get_torus_neighbor(coord: TorusCoord, axis: u32, positive: bool) -> u32 {
    let step = if positive { 1 } else { 3 }
    let next_c = if axis == 0 {
        TorusCoord { x: ((coord.x + step) % 4) as u8, y: coord.y, z: coord.z, w: coord.w }
    } else if axis == 1 {
        TorusCoord { x: coord.x, y: ((coord.y + step) % 4) as u8, z: coord.z, w: coord.w }
    } else if axis == 2 {
        TorusCoord { x: coord.x, y: coord.y, z: ((coord.z + step) % 4) as u8, w: coord.w }
    } else {
        TorusCoord { x: coord.x, y: coord.y, z: coord.z, w: ((coord.w + step) % 4) as u8 }
    }
    return coord_to_core_id(next_c)
}

// Dimension-Order Routing (DOR: X -> Y -> Z -> W)
// Resolves next immediate hop toward destination core on the 4D torus
def route_next_hop_dor(curr: TorusCoord, dst: TorusCoord) -> u32 {
    if curr.x != dst.x {
        let forward = ((dst.x + 4 - curr.x) % 4) <= 2
        return get_torus_neighbor(curr, axis=0, positive=forward)
    }
    if curr.y != dst.y {
        let forward = ((dst.y + 4 - curr.y) % 4) <= 2
        return get_torus_neighbor(curr, axis=1, positive=forward)
    }
    if curr.z != dst.z {
        let forward = ((dst.z + 4 - curr.z) % 4) <= 2
        return get_torus_neighbor(curr, axis=2, positive=forward)
    }
    if curr.w != dst.w {
        let forward = ((dst.w + 4 - curr.w) % 4) <= 2
        return get_torus_neighbor(curr, axis=3, positive=forward)
    }
    return coord_to_core_id(curr)
}

// Sentry Deflection Routing: If primary DOR axis is blocked/faulted, deflect to orthogonal neighbor
def route_deflect_on_fault(curr: TorusCoord, dst: TorusCoord, faulted_axis: u32) -> u32 {
    let alt_axis = (faulted_axis + 1) % 4
    return get_torus_neighbor(curr, axis=alt_axis, positive=true)
}
