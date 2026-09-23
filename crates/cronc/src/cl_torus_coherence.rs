//! 4D Torus Lockless Mesh Coherence Engine for CRON (.cl)
//!
//! Provides deterministic micro-packet dimension-order routing (DOR)
//! across 4D spatial topologies (X, Y, Z, W) with zero locks and single-cycle forwarding.

/// 4D Torus Coordinate (x, y, z, w)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Torus4DCoord {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}

impl Torus4DCoord {
    pub fn new(x: u8, y: u8, z: u8, w: u8) -> Self {
        Self { x, y, z, w }
    }

    /// Manhattan distance on a torus of shape (dim_x, dim_y, dim_z, dim_w)
    pub fn torus_distance(&self, other: &Self, dims: (u8, u8, u8, u8)) -> u32 {
        let dx = (self.x as i32 - other.x as i32).abs() as u32;
        let dx = dx.min(dims.0 as u32 - dx);

        let dy = (self.y as i32 - other.y as i32).abs() as u32;
        let dy = dy.min(dims.1 as u32 - dy);

        let dz = (self.z as i32 - other.z as i32).abs() as u32;
        let dz = dz.min(dims.2 as u32 - dz);

        let dw = (self.w as i32 - other.w as i32).abs() as u32;
        let dw = dw.min(dims.3 as u32 - dw);

        dx + dy + dz + dw
    }
}

/// Torus Micro-Packet for Core-to-Core Transmission
#[derive(Debug, Clone, PartialEq)]
pub struct TorusMicroPacket {
    pub src: Torus4DCoord,
    pub dst: Torus4DCoord,
    pub tag: u16,
    pub data: [u64; 2], // 128-bit payload
    pub hops_taken: u32,
}

/// Lockless 4D Torus Mesh Router
#[derive(Debug, Clone, PartialEq)]
pub struct Torus4DMesh {
    pub dims: (u8, u8, u8, u8),
    pub total_nodes: usize,
    pub routed_packets_count: u64,
}

impl Torus4DMesh {
    pub fn new(dim_x: u8, dim_y: u8, dim_z: u8, dim_w: u8) -> Self {
        let total_nodes = (dim_x as usize) * (dim_y as usize) * (dim_z as usize) * (dim_w as usize);
        Self {
            dims: (dim_x, dim_y, dim_z, dim_w),
            total_nodes,
            routed_packets_count: 0,
        }
    }

    /// Dimension-Order-Routing (DOR) step from current node to next hop towards dst
    pub fn next_hop(&self, current: &Torus4DCoord, dst: &Torus4DCoord) -> Torus4DCoord {
        let mut next = *current;

        // Route in X dimension first
        if next.x != dst.x {
            let forward_dist = (dst.x + self.dims.0 - next.x) % self.dims.0;
            let backward_dist = (next.x + self.dims.0 - dst.x) % self.dims.0;
            if forward_dist <= backward_dist {
                next.x = (next.x + 1) % self.dims.0;
            } else {
                next.x = (next.x + self.dims.0 - 1) % self.dims.0;
            }
            return next;
        }

        // Route in Y dimension second
        if next.y != dst.y {
            let forward_dist = (dst.y + self.dims.1 - next.y) % self.dims.1;
            let backward_dist = (next.y + self.dims.1 - dst.y) % self.dims.1;
            if forward_dist <= backward_dist {
                next.y = (next.y + 1) % self.dims.1;
            } else {
                next.y = (next.y + self.dims.1 - 1) % self.dims.1;
            }
            return next;
        }

        // Route in Z dimension third
        if next.z != dst.z {
            let forward_dist = (dst.z + self.dims.2 - next.z) % self.dims.2;
            let backward_dist = (next.z + self.dims.2 - dst.z) % self.dims.2;
            if forward_dist <= backward_dist {
                next.z = (next.z + 1) % self.dims.2;
            } else {
                next.z = (next.z + self.dims.2 - 1) % self.dims.2;
            }
            return next;
        }

        // Route in W dimension fourth
        if next.w != dst.w {
            let forward_dist = (dst.w + self.dims.3 - next.w) % self.dims.3;
            let backward_dist = (next.w + self.dims.3 - dst.w) % self.dims.3;
            if forward_dist <= backward_dist {
                next.w = (next.w + 1) % self.dims.3;
            } else {
                next.w = (next.w + self.dims.3 - 1) % self.dims.3;
            }
            return next;
        }

        next
    }

    /// Simulate full packet route from src to dst
    pub fn route_packet(&mut self, mut packet: TorusMicroPacket) -> TorusMicroPacket {
        let mut curr = packet.src;
        while curr != packet.dst {
            curr = self.next_hop(&curr, &packet.dst);
            packet.hops_taken += 1;
        }
        self.routed_packets_count += 1;
        packet
    }
}
