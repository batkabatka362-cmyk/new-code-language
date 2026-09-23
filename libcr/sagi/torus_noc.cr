// CRON Standard Library: 4D Torus Lockless DOR Micro-Packet Routing
// Target: 256-Core 4D-Torus Photonic Neuromorphic Hardware

.MODULE cron.sagi.torus_noc

pub struct Torus4DCoordinate {
    x: u32,
    y: u32,
    z: u32,
    w: u32
}

pub def create_torus_coord(x: u32, y: u32, z: u32, w: u32) -> Torus4DCoordinate {
    return Torus4DCoordinate { x: x, y: y, z: z, w: w }
}

pub def compute_manhattan_distance_4d(src: Torus4DCoordinate, dst: Torus4DCoordinate) -> u32 {
    let dx = if src.x > dst.x { src.x - dst.x } else { dst.x - src.x }
    let dy = if src.y > dst.y { src.y - dst.y } else { dst.y - src.y }
    let dz = if src.z > dst.z { src.z - dst.z } else { dst.z - src.z }
    let dw = if src.w > dst.w { src.w - dst.w } else { dst.w - src.w }
    return dx + dy + dz + dw
}
