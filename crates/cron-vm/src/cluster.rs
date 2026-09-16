//! CRON Multi-Chip Cluster Architecture (Pages 636–665)
//!
//! Scalable distributed multi-processor cluster architecture interconnecting
//! multiple 256-core 4D-Torus neuromorphic/photonic chips into a unified
//! high-performance computing fabric (default 16 Chips = 4,096 Cores).
//!
//! Features:
//! - 6D Hierarchical Coordinate System: (chip_x, chip_y, core_x, core_y, core_z, core_w)
//! - Low-Latency Optical Waveguide & RDMA Interconnects (up to 3.2 Tbps per chip)
//! - Cross-Chip Zero-Overhead Direct DMA (`DD`) & Cluster-Wide Barrier (`bb`)
//! - Distributed SPMD/MIMD VLIW Execution Engine with Telemetry Aggregation

use crate::core_engine::CoreEngine;
use crate::simulator::{Simulator, VliwInstruction};
use crate::torus_mesh::Coord4D;
use std::collections::HashMap;

/// 6D Hierarchical Coordinate addressing any core across the entire multi-chip cluster.
/// (chip_x, chip_y) locate the physical chip on the inter-chip board/wafer (e.g. 4x4 grid),
/// while (core_x, core_y, core_z, core_w) locate the physical core within the chip's 4D-Torus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClusterCoord {
    pub chip_x: usize,
    pub chip_y: usize,
    pub core_x: usize,
    pub core_y: usize,
    pub core_z: usize,
    pub core_w: usize,
}

impl ClusterCoord {
    pub fn new(
        chip_x: usize,
        chip_y: usize,
        core_x: usize,
        core_y: usize,
        core_z: usize,
        core_w: usize,
    ) -> Self {
        Self {
            chip_x: chip_x % 4,
            chip_y: chip_y % 4,
            core_x: core_x % 4,
            core_y: core_y % 4,
            core_z: core_z % 4,
            core_w: core_w % 4,
        }
    }

    /// Convert from linear global core ID (0..4095 for 16 chips)
    pub fn from_global_id(global_id: usize) -> Self {
        let chip_id = (global_id / 256) % 16;
        let local_id = global_id % 256;

        let chip_x = chip_id % 4;
        let chip_y = chip_id / 4;

        let coord4d = Coord4D::from_core_id(local_id);
        Self {
            chip_x,
            chip_y,
            core_x: coord4d.x,
            core_y: coord4d.y,
            core_z: coord4d.z,
            core_w: coord4d.w,
        }
    }

    /// Convert to linear global core ID (0..4095 for 16 chips)
    pub fn to_global_id(&self) -> usize {
        let chip_id = self.chip_x + 4 * self.chip_y;
        let local_id = self.core_x + 4 * self.core_y + 16 * self.core_z + 64 * self.core_w;
        chip_id * 256 + local_id
    }

    /// Get the chip ID (0..15)
    pub fn chip_id(&self) -> usize {
        self.chip_x + 4 * self.chip_y
    }

    /// Get the local intra-chip core ID (0..255)
    pub fn local_core_id(&self) -> usize {
        self.core_x + 4 * self.core_y + 16 * self.core_z + 64 * self.core_w
    }
}

/// Packet routed across the optical / RDMA inter-chip interconnect fabric.
#[derive(Debug, Clone)]
pub struct InterChipPacket {
    pub src_chip: usize,
    pub src_core: usize,
    pub dst_chip: usize,
    pub dst_core: usize,
    pub payload: u32,
    pub hop_count: usize,
    pub wavelength_nm: u32,
}

/// Represents a high-speed optical / PCIe Gen5 / CXL inter-chip communication link.
#[derive(Debug, Clone)]
pub struct InterChipLink {
    pub from_chip: usize,
    pub to_chip: usize,
    pub bandwidth_gbps: f64,
    pub latency_cycles: usize,
    pub queue: Vec<InterChipPacket>,
}

impl InterChipLink {
    pub fn new(from_chip: usize, to_chip: usize) -> Self {
        Self {
            from_chip,
            to_chip,
            bandwidth_gbps: 3200.0, // 3.2 Tbps per optical transceiver
            latency_cycles: 2,      // 2 cycles optical transit
            queue: Vec::new(),
        }
    }
}

/// A physical chip node inside the multi-chip cluster.
pub struct ClusterNode {
    pub chip_id: usize,
    pub chip_x: usize,
    pub chip_y: usize,
    pub simulator: Simulator,
    pub cross_chip_inbox: Vec<InterChipPacket>,
    pub cross_chip_outbox: Vec<InterChipPacket>,
}

impl ClusterNode {
    pub fn new(chip_id: usize) -> Self {
        let chip_x = chip_id % 4;
        let chip_y = chip_id / 4;
        let mut sim = Simulator::new();
        // Give each core a global sense of its chip ID
        for (i, core) in sim.cores.iter_mut().enumerate() {
            let global_id = chip_id * 256 + i;
            core.registers[14] = global_id as u32; // R14 holds global core ID
            core.registers[15] = chip_id as u32;   // R15 holds chip ID
        }
        Self {
            chip_id,
            chip_x,
            chip_y,
            simulator: sim,
            cross_chip_inbox: Vec::new(),
            cross_chip_outbox: Vec::new(),
        }
    }
}

/// Aggregated telemetry and hardware performance metrics across the entire cluster.
#[derive(Debug, Clone, Default)]
pub struct ClusterStats {
    pub num_chips: usize,
    pub total_cores: usize,
    pub total_cycles: usize,
    pub cross_chip_packets: usize,
    pub cross_chip_dma_bursts: usize,
    pub cluster_barrier_syncs: usize,
    pub aggregate_gemm_ops: usize,
    pub aggregate_reversible_ops: usize,
    pub aggregate_stdp_updates: usize,
    pub aggregate_energy_saved_uw: u64,
    pub peak_cluster_temperature_c: u32,
    pub aggregate_bandwidth_tbps: f64,
}

/// Multi-Chip 4,096-Core Distributed Cluster Simulator.
pub struct ClusterSimulator {
    pub num_chips: usize,
    pub total_cores: usize,
    pub nodes: Vec<ClusterNode>,
    pub inter_chip_links: HashMap<(usize, usize), InterChipLink>,
    pub stats: ClusterStats,
    pub step_index: usize,
}

impl ClusterSimulator {
    /// Initialize a new Multi-Chip Cluster Simulator.
    /// Default `num_chips = 16` creates 16 Chips * 256 Cores = 4,096 Cores.
    pub fn new(num_chips: usize) -> Self {
        let chips = if num_chips == 0 { 16 } else { num_chips };
        let mut nodes = Vec::with_capacity(chips);
        for id in 0..chips {
            nodes.push(ClusterNode::new(id));
        }

        // Initialize 2D Torus inter-chip links between neighboring chips
        let mut links = HashMap::new();
        for id in 0..chips {
            let cx = id % 4;
            let cy = id / 4;
            // 4 neighbors in 2D grid with wrap-around
            let neighbors = [
                ((cx + 1) % 4) + 4 * cy,
                ((cx + 3) % 4) + 4 * cy,
                cx + 4 * ((cy + 1) % 4),
                cx + 4 * ((cy + 3) % 4),
            ];
            for &n in &neighbors {
                if n < chips && n != id {
                    links.entry((id, n)).or_insert_with(|| InterChipLink::new(id, n));
                }
            }
        }

        Self {
            num_chips: chips,
            total_cores: chips * 256,
            nodes,
            inter_chip_links: links,
            stats: ClusterStats {
                num_chips: chips,
                total_cores: chips * 256,
                total_cycles: 0,
                cross_chip_packets: 0,
                cross_chip_dma_bursts: 0,
                cluster_barrier_syncs: 0,
                aggregate_gemm_ops: 0,
                aggregate_reversible_ops: 0,
                aggregate_stdp_updates: 0,
                aggregate_energy_saved_uw: 0,
                peak_cluster_temperature_c: 25,
                aggregate_bandwidth_tbps: (chips as f64) * 3.2,
            },
            step_index: 0,
        }
    }

    /// Load identical or partitioned VLIW program across all chips in the cluster
    pub fn load_machine_code(&mut self, cl_code: &str) {
        for node in &mut self.nodes {
            node.simulator.load_machine_code(cl_code);
        }
        self.step_index = 0;
    }

    /// Load pre-parsed instructions into all chips
    pub fn load_program(&mut self, instructions: Vec<VliwInstruction>) {
        for node in &mut self.nodes {
            node.simulator.load_program(instructions.clone());
        }
        self.step_index = 0;
    }

    /// Send a cross-chip packet directly between any two cores in the 4,096-core fabric
    pub fn send_cross_chip_packet(
        &mut self,
        src_global_id: usize,
        dst_global_id: usize,
        payload: u32,
    ) {
        let src_chip = (src_global_id / 256) % self.num_chips;
        let src_core = src_global_id % 256;
        let dst_chip = (dst_global_id / 256) % self.num_chips;
        let dst_core = dst_global_id % 256;

        let packet = InterChipPacket {
            src_chip,
            src_core,
            dst_chip,
            dst_core,
            payload,
            hop_count: 1,
            wavelength_nm: 1550,
        };

        if src_chip == dst_chip {
            // Intra-chip local routing
            self.nodes[src_chip]
                .simulator
                .mesh
                .packet_queues[dst_core]
                .push(crate::torus_mesh::MeshPacket {
                    source_id: src_core,
                    target_id: dst_core,
                    payload,
                    hop_count: 1,
                });
        } else {
            // Cross-chip optical routing
            self.nodes[src_chip].cross_chip_outbox.push(packet);
            self.stats.cross_chip_packets += 1;
        }
    }

    /// Perform a Zero-Overhead Cross-Chip Direct DMA (`DD`) burst from src to dst core
    pub fn cross_chip_dma_transfer(
        &mut self,
        _src_global_id: usize,
        dst_global_id: usize,
        payload: u32,
    ) {
        let dst_chip = (dst_global_id / 256) % self.num_chips;
        let dst_core = dst_global_id % 256;

        // Zero-overhead DMA directly deposits payload into target core's R0/R1 register
        self.nodes[dst_chip].simulator.cores[dst_core].registers[0] = payload;
        self.nodes[dst_chip].simulator.cores[dst_core].dma_transfers += 1;
        self.stats.cross_chip_dma_bursts += 1;
    }

    /// Step all chips simultaneously in cycle-accurate lockstep
    pub fn step(&mut self) -> bool {
        let mut any_active = false;
        let mut has_barrier = false;
        let mut has_cache_inv = false;
        let mut has_dvfs = false;

        // 1. Check if any node's current instruction contains cluster-wide macro opcodes
        for node in &self.nodes {
            if node.simulator.step_index < node.simulator.instructions.len() {
                let inst = &node.simulator.instructions[node.simulator.step_index];
                for s in &inst.slots {
                    if s.contains("bb") {
                        has_barrier = true;
                    }
                    if s.contains("CC") {
                        has_cache_inv = true;
                    }
                    if s.contains("EE") {
                        has_dvfs = true;
                    }
                }
            }
        }

        // 2. Step all chips and collect cross-chip packets
        let mut all_outbox = Vec::new();
        for node in &mut self.nodes {
            let active = node.simulator.step();
            if active {
                any_active = true;
            }
            all_outbox.extend(std::mem::take(&mut node.cross_chip_outbox));
        }

        // 3. Deliver cross-chip packets to target nodes
        for pkt in all_outbox {
            let dst_chip = pkt.dst_chip;
            let dst_core = pkt.dst_core;
            let payload = pkt.payload;
            if dst_chip < self.nodes.len() {
                self.nodes[dst_chip].cross_chip_inbox.push(pkt);
                self.nodes[dst_chip].simulator.cores[dst_core].registers[1] = payload;
            }
        }

        if !any_active {
            return false;
        }

        self.stats.total_cycles += 1;
        self.step_index += 1;

        // 3. Cluster-Wide Synchronizations
        if has_barrier {
            self.stats.cluster_barrier_syncs += 1;
            for node in &mut self.nodes {
                for core in &mut node.simulator.cores {
                    core.barrier_count += 1;
                }
            }
        }

        if has_cache_inv {
            for node in &mut self.nodes {
                for core in &mut node.simulator.cores {
                    core.csr_stall_cnt = 0;
                    core.cache_invalidations += 1;
                }
            }
        }

        if has_dvfs {
            for node in &mut self.nodes {
                for core in &mut node.simulator.cores {
                    core.thermal_level = 25;
                    core.dvfs_energy_state = 1;
                    core.energy_saved_uw += 450;
                }
            }
        }

        true
    }

    /// Run simulation across all chips until complete or limit reached
    pub fn run(&mut self, max_cycles: usize) {
        let limit = if max_cycles == 0 { 10_000 } else { max_cycles };
        for _ in 0..limit {
            if !self.step() {
                break;
            }
        }
        self.aggregate_stats();
    }

    /// Aggregate hardware performance counters across all chips
    pub fn aggregate_stats(&mut self) {
        let mut gemm = 0;
        let mut rev = 0;
        let mut stdp = 0;
        let mut energy: u64 = 0;
        let mut max_temp = 25;

        for node in &self.nodes {
            gemm += node.simulator.stats.optical_gemm_ops;
            rev += node.simulator.stats.reversible_gate_ops;
            stdp += node.simulator.stats.stdp_synapse_updates;
            let node_energy: u64 = node.simulator.cores.iter().map(|c| c.energy_saved_uw).sum();
            energy += node_energy;
            let node_max_temp = node.simulator.cores.iter().map(|c| c.thermal_level).max().unwrap_or(25);
            if node_max_temp > max_temp {
                max_temp = node_max_temp;
            }
        }

        self.stats.aggregate_gemm_ops = gemm;
        self.stats.aggregate_reversible_ops = rev;
        self.stats.aggregate_stdp_updates = stdp;
        self.stats.aggregate_energy_saved_uw = energy;
        self.stats.peak_cluster_temperature_c = max_temp;
    }

    /// Query register dump from any global core ID (0..total_cores-1)
    pub fn get_global_core_dump(&self, global_core_id: usize) -> [u32; 16] {
        if global_core_id >= self.total_cores {
            return [0; 16];
        }
        let chip_id = global_core_id / 256;
        let local_id = global_core_id % 256;
        self.nodes[chip_id].simulator.cores[local_id].registers
    }

    /// Query a specific core's state by 6D cluster coordinate
    pub fn get_core_by_coord(&self, coord: ClusterCoord) -> Option<&CoreEngine> {
        let chip_id = coord.chip_id();
        let local_id = coord.local_core_id();
        if chip_id < self.nodes.len() && local_id < 256 {
            Some(&self.nodes[chip_id].simulator.cores[local_id])
        } else {
            None
        }
    }

    /// Return immutable reference to cluster statistics
    pub fn cluster_stats(&self) -> &ClusterStats {
        &self.stats
    }
}
