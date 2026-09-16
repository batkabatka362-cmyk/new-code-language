#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord4D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
}

impl Coord4D {
    pub fn new(x: usize, y: usize, z: usize, w: usize) -> Self {
        Self {
            x: x % 4,
            y: y % 4,
            z: z % 4,
            w: w % 4,
        }
    }

    pub fn to_core_id(&self) -> usize {
        self.x + 4 * self.y + 16 * self.z + 64 * self.w
    }

    pub fn from_core_id(id: usize) -> Self {
        let x = id % 4;
        let y = (id / 4) % 4;
        let z = (id / 16) % 4;
        let w = (id / 64) % 4;
        Self { x, y, z, w }
    }

    pub fn neighbor(&self, axis: &str) -> Self {
        match axis {
            "X+" => Self::new(self.x + 1, self.y, self.z, self.w),
            "X-" => Self::new((self.x + 3) % 4, self.y, self.z, self.w),
            "Y+" => Self::new(self.x, self.y + 1, self.z, self.w),
            "Y-" => Self::new(self.x, (self.y + 3) % 4, self.z, self.w),
            "Z+" => Self::new(self.x, self.y, self.z + 1, self.w),
            "Z-" => Self::new(self.x, self.y, (self.z + 3) % 4, self.w),
            "W+" => Self::new(self.x, self.y, self.z, self.w + 1),
            "W-" => Self::new(self.x, self.y, self.z, (self.w + 3) % 4),
            _ => *self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshPacket {
    pub source_id: usize,
    pub target_id: usize,
    pub payload: u32,
    pub hop_count: usize,
}

pub struct TorusMesh {
    pub total_cores: usize,
    pub packet_queues: Vec<Vec<MeshPacket>>,
}

impl Default for TorusMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl TorusMesh {
    pub fn new() -> Self {
        let total = 256; // 4x4x4x4
        Self {
            total_cores: total,
            packet_queues: vec![Vec::new(); total],
        }
    }

    pub fn broadcast(&mut self, source_id: usize, payload: u32) {
        let src_coord = Coord4D::from_core_id(source_id);
        // Broadcast to 8 immediate 4D neighbors
        for axis in &["X+", "X-", "Y+", "Y-", "Z+", "Z-", "W+", "W-"] {
            let neighbor_id = src_coord.neighbor(axis).to_core_id();
            self.packet_queues[neighbor_id].push(MeshPacket {
                source_id,
                target_id: neighbor_id,
                payload,
                hop_count: 1,
            });
        }
    }

    pub fn deliver_packets(&mut self, core_id: usize) -> Vec<MeshPacket> {
        if core_id < self.packet_queues.len() {
            std::mem::take(&mut self.packet_queues[core_id])
        } else {
            Vec::new()
        }
    }

    /// 16-Way Branchless 4D Hyper-Tree Traversal (Pages 268-275)
    /// Decodes a 4-bit directional mask per child index without branches.
    /// Calculates Torus wrap-around & stride projection: (X + Y*2 + Z*4 + W*8) & 0x7F.
    pub fn branchless_traverse_16way(base: Coord4D, active_mask: u16) -> [(Coord4D, usize); 16] {
        let mut results = [(Coord4D::new(0, 0, 0, 0), 0); 16];
        for i in 0..16 {
            let dx = i & 1;
            let dy = (i >> 1) & 1;
            let dz = (i >> 2) & 1;
            let dw = (i >> 3) & 1;

            let nx = (base.x + dx) % 4;
            let ny = (base.y + dy) % 4;
            let nz = (base.z + dz) % 4;
            let nw = (base.w + dw) % 4;

            let coord = Coord4D { x: nx, y: ny, z: nz, w: nw };
            // Hyper-tree stride projection hash
            let hash = (nx + ny * 2 + nz * 4 + nw * 8) & 0x7F;

            let is_active = (active_mask >> i) & 1;
            results[i] = (coord, hash * (is_active as usize));
        }
        results
    }
}

