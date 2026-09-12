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
}
