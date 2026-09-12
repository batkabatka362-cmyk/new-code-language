use crate::core_engine::CoreEngine;
use crate::torus_mesh::TorusMesh;

#[derive(Debug, Clone)]
pub struct VliwInstruction {
    pub cycle: usize,
    pub slots: Vec<String>,
}

#[derive(Debug, Default)]
pub struct HardwareStats {
    pub total_cycles: usize,
    pub optical_gemm_ops: usize,
    pub reversible_gate_ops: usize,
    pub stdp_synapse_updates: usize,
    pub mesh_packets_routed: usize,
    pub peak_temperature_c: u32,
    pub dram_bandwidth_saved_mb: f64,
}

pub struct Simulator {
    pub cores: Vec<CoreEngine>,
    pub mesh: TorusMesh,
    pub instructions: Vec<VliwInstruction>,
    pub stats: HardwareStats,
    pub trace_enabled: bool,
    pub step_index: usize,
}

impl Simulator {
    pub fn new() -> Self {
        let mut cores = Vec::with_capacity(256);
        for id in 0..256 {
            cores.push(CoreEngine::new(id));
        }
        Self {
            cores,
            mesh: TorusMesh::new(),
            instructions: Vec::new(),
            stats: HardwareStats::default(),
            trace_enabled: false,
            step_index: 0,
        }
    }

    pub fn set_trace(&mut self, enabled: bool) {
        self.trace_enabled = enabled;
        for core in &mut self.cores {
            core.trace_enabled = enabled;
        }
    }

    pub fn core_dump(&self, core_id: usize) -> [u32; 16] {
        if core_id < self.cores.len() {
            self.cores[core_id].registers
        } else {
            [0; 16]
        }
    }

    pub fn load_machine_code(&mut self, cl_content: &str) {
        self.instructions.clear();
        self.step_index = 0;
        for line in cl_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
                continue;
            }

            // e.g. B0001: '=00#0A04> '=10#040B> _SH00$01B4> _SP00#0088>
            if let Some((cycle_part, slots_part)) = trimmed.split_once(':') {
                let cycle_str = cycle_part.trim().trim_start_matches('B');
                let cycle = cycle_str.parse::<usize>().unwrap_or(0);
                let slots: Vec<String> = slots_part
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

                self.instructions.push(VliwInstruction { cycle, slots });
            }
        }
    }

    pub fn step(&mut self) -> bool {
        if self.step_index >= self.instructions.len() {
            return false;
        }
        let inst = self.instructions[self.step_index].clone();
        self.step_index += 1;
        self.stats.total_cycles += 1;

        // Execute across cores (Primary core 0 with parallel mesh propagation)
        for core in &mut self.cores {
            core.cycle_count += 1;
            for slot in &inst.slots {
                if let Some(broadcast_payload) = core.execute_slot(slot) {
                    self.mesh.broadcast(core.id, broadcast_payload);
                    self.stats.mesh_packets_routed += 8; // 8 neighbors in 4D torus
                }
            }
        }

        // Process NoC packet deliveries
        for i in 0..256 {
            let recvd = self.mesh.deliver_packets(i);
            if !recvd.is_empty() {
                // Update core state with recvd packet
                self.cores[i].registers[1] = recvd[0].payload;
            }
        }

        true
    }

    pub fn run(&mut self) -> &HardwareStats {
        while self.step() {}

        // Aggregate stats from all cores
        let mut total_mzi = 0;
        let mut total_rev = 0;
        let mut total_stdp = 0;
        let mut peak_temp = 25;

        for core in &self.cores {
            total_mzi += core.optical_gemm_count;
            total_rev += core.reversible_ops_count;
            total_stdp += core.stdp_updates_count;
            if core.thermal_level > peak_temp {
                peak_temp = core.thermal_level;
            }
        }

        self.stats.optical_gemm_ops = total_mzi;
        self.stats.reversible_gate_ops = total_rev;
        self.stats.stdp_synapse_updates = total_stdp;
        self.stats.peak_temperature_c = peak_temp;
        // Reversible memory saves ~128 bytes per reverse auto-diff step per core
        self.stats.dram_bandwidth_saved_mb = (total_rev as f64 * 128.0) / (1024.0 * 1024.0);

        &self.stats
    }
}
