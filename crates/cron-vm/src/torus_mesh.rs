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

    /// Minimal Manhattan distance on 4x4x4x4 4D Torus
    pub fn manhattan_distance(&self, other: &Coord4D) -> usize {
        let wrap4 = |a: usize, b: usize| -> usize {
            let d = (a as isize - b as isize).unsigned_abs();
            d.min(4 - d)
        };
        wrap4(self.x, other.x) + wrap4(self.y, other.y) + wrap4(self.z, other.z) + wrap4(self.w, other.w)
    }

    /// Dimension-Order Routing (4D-DOR): X -> Y -> Z -> W
    pub fn dor_step(&self, target: &Coord4D) -> (Coord4D, &'static str) {
        if self.x != target.x {
            let diff = (target.x as isize - self.x as isize).rem_euclid(4) as usize;
            if diff <= 2 {
                (self.neighbor("X+"), "X+")
            } else {
                (self.neighbor("X-"), "X-")
            }
        } else if self.y != target.y {
            let diff = (target.y as isize - self.y as isize).rem_euclid(4) as usize;
            if diff <= 2 {
                (self.neighbor("Y+"), "Y+")
            } else {
                (self.neighbor("Y-"), "Y-")
            }
        } else if self.z != target.z {
            let diff = (target.z as isize - self.z as isize).rem_euclid(4) as usize;
            if diff <= 2 {
                (self.neighbor("Z+"), "Z+")
            } else {
                (self.neighbor("Z-"), "Z-")
            }
        } else if self.w != target.w {
            let diff = (target.w as isize - self.w as isize).rem_euclid(4) as usize;
            if diff <= 2 {
                (self.neighbor("W+"), "W+")
            } else {
                (self.neighbor("W-"), "W-")
            }
        } else {
            (*self, "STAY")
        }
    }

    /// Adaptive Thermal Deflection Routing:
    /// If natural DOR next-hop core is thermally throttled or hotspot,
    /// deflects packet to orthogonal cooler 4D neighbor (X+, Y-, Z+, W-) without stalling!
    pub fn resilient_step_with_thermal_deflection(
        &self,
        target: &Coord4D,
        hot_cores: &[bool; 256],
    ) -> (Coord4D, &'static str, bool) {
        if self == target {
            return (*self, "STAY", false);
        }

        let (dor_next, dor_axis) = self.dor_step(target);
        let next_id = dor_next.to_core_id();

        // If target reached or next hop is cool, use standard DOR
        if dor_next == *target || !hot_cores[next_id] {
            return (dor_next, dor_axis, false);
        }

        // Hotspot detected! Select best orthogonal deflection axis that bypasses the hotspot
        let axes = ["X+", "X-", "Y+", "Y-", "Z+", "Z-", "W+", "W-"];
        let mut best_detour = dor_next;
        let mut best_axis = dor_axis;
        let mut min_dist = usize::MAX;
        let mut found_cool_detour = false;

        for &ax in &axes {
            if ax == dor_axis {
                continue; // Skip the hot direct route
            }
            let cand = self.neighbor(ax);
            let cid = cand.to_core_id();
            if !hot_cores[cid] {
                let dist = cand.manhattan_distance(target);
                if dist < min_dist {
                    min_dist = dist;
                    best_detour = cand;
                    best_axis = ax;
                    found_cool_detour = true;
                }
            }
        }

        if found_cool_detour {
            (best_detour, best_axis, true)
        } else {
            // All neighbors congested/hot - proceed on standard DOR with emergency DVFS
            (dor_next, dor_axis, false)
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshPacket {
    pub source_id: usize,
    pub target_id: usize,
    pub payload: u32,
    pub hop_count: usize,
    pub virtual_channel: u8, // 0 = VC0_Direct, 1 = VC1_Wrap, 2 = VC2_Deflection, 3 = VC3_Priority
    pub was_deflected: bool,
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
                virtual_channel: 0,
                was_deflected: false,
            });
        }
    }

    /// Routes a point-to-point packet across 4D Torus with live thermal deflection
    pub fn route_packet_with_thermal_deflection(
        &mut self,
        source_id: usize,
        target_id: usize,
        payload: u32,
        hot_cores: &[bool; 256],
    ) -> (usize, bool) {
        if source_id == target_id {
            return (0, false);
        }

        let mut curr = Coord4D::from_core_id(source_id);
        let target = Coord4D::from_core_id(target_id);
        let mut hops = 0;
        let mut any_deflected = false;

        // Traverse in 4D with max 16 hops to prevent any infinite loops
        while curr != target && hops < 16 {
            let (next, _axis, was_deflected) = curr.resilient_step_with_thermal_deflection(&target, hot_cores);
            if was_deflected {
                any_deflected = true;
            }
            curr = next;
            hops += 1;
        }

        let dest_id = curr.to_core_id();
        let vc = if any_deflected { 2 } else { 0 };
        self.packet_queues[dest_id].push(MeshPacket {
            source_id,
            target_id: dest_id,
            payload,
            hop_count: hops,
            virtual_channel: vc,
            was_deflected: any_deflected,
        });

        (hops, any_deflected)
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

